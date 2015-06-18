use std::io::{self, BufWriter};
use std::fs::{self, File};
use std::io::prelude::*;
use std::fmt::Write as FmtWrite;
use ::horrorshow::prelude::*;
use std::path::Path;

use model::{Source, Meta};
use view::{Site, Page, Index, Paginate, Content};

use ::{RenderError, AnnotatedError};

static MATCH_OPTIONS: ::glob::MatchOptions = ::glob::MatchOptions {
    case_sensitive: true,
    require_literal_separator: true,
    require_literal_leading_dot: false,
};

/// Recursivly copy a directory.
pub fn copy_recursive(src: &Path, dest: &Path) -> io::Result<()> {
    if try!(fs::metadata(&src)).is_dir() {
        copy_dir(src, dest)
    } else {
        copy_file(src, dest)
    }
}

#[cfg(not(linux))]
fn copy_file(src: &Path, dest: &Path) -> io::Result<()> {
    io::copy(&mut try!(File::open(&src)), &mut try!(File::create(&dest))).map(|_|())
}


#[cfg(linux)]
fn copy_file(src: &Path, dest: &Path) -> io::Result<()> {
    const BTRFS_IOC_CLONE: ioctl::libc::c_ulong = iow!(0x94, 9, 4) as ioctl::libc::c_ulong;
    let src = try!(File::open(&src));
    let dest = try!(File::create(&dest));
    if unsafe { ioctl::ioctl(dest.as_raw_fd(), BTRFS_IOC_CLONE, src.as_raw_fd() ) } != 0 {
        io::copy(&mut src, &mut dest).map(|_|())
    } else {
        Ok(())
    }
}

fn copy_dir(src: &Path, dest: &Path) -> io::Result<()> {
    try!(fs::create_dir(dest));
    for dir_entry in try!(fs::read_dir(src)) {
        let dir_entry = try!(dir_entry);
        let from = dir_entry.path();
        let to = dest.join(from.relative_from(src).unwrap());
        if fs::metadata(&to).is_ok() {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "target path already exists"));
        }

        try!(if try!(dir_entry.file_type()).is_dir() {
            copy_dir(&from, &to)
        } else {
            copy_file(&from, &to)
        });
    }
    Ok(())
}

pub trait Gazetta: Sized {
    type SiteMeta: Meta;
    type PageMeta: Meta;

    /// The page rendering function.
    fn render_page(&self, site: &Site<Self>, page: &Page<Self>, tmpl: &mut TemplateBuffer);

    /// Render static content.
    ///
    /// By default, this just copies. Override to compile.
    #[allow(unused_variables)]
    fn render_static(&self, site: &Site<Self>, source: &Path, output: &Path) -> io::Result<()> {
        copy_recursive(source, output)
    }

    /// Creates pages from a site defined by a source and renders them into output.
    ///
    /// Call this to render your site.
    ///
    /// Note: You *can* override this but you **really** shouldn't. This function contains pretty
    /// much all of the provided render logic.
    fn render<P: AsRef<Path>>(&self,
                              source: &Source<Self::SiteMeta, Self::PageMeta>,
                              output: P) -> Result<(), AnnotatedError<RenderError>>
    {
        macro_rules! try_annotate {
            ($e:expr, $l:expr) => {
                match $e {
                    Ok(v) => v,
                    Err(e) => return Err(AnnotatedError::new(($l).to_owned(), RenderError::from(e))),
                }
            }
        }
        let output = output.as_ref();

        let site = Site {
            title: &source.title,
            author: &source.author,
            meta: &source.meta,
        };

        // In general, the system calls here will dwarf the cost of a couple of allocations.
        // However, putting all content in a single string buffer may improve cache behavior.
        // TODO: Test this.
        let mut buf = String::with_capacity(4096);

        // Define this as a macro because we need to go from a mutable borrow to an immutable borrow...
        macro_rules! read_children {
            ($buf:ident, $entries:expr)  => {{
                let mut children = Vec::with_capacity($entries.len());
                let mut strides = Vec::with_capacity($entries.len());

                let mut start = $buf.len();

                {
                    for child in $entries {
                        strides.push(try!(child.content.read_into(&mut $buf)));
                        children.push(Page {
                            title: &child.title,
                            date: child.date.as_ref(),
                            content: Content { 
                                data: "",
                                format: child.content.format()
                            },
                            href: &child.name,
                            index: None,
                            meta: &child.meta,
                        });
                    }
                }

                for (&len, child_entry) in strides.iter().zip(children.iter_mut()) {
                    let s  = &$buf[start..(start + len)];
                    start += len;
                    child_entry.content.data = s;
                }
                children
            }}
        }

        for static_entry in &source.static_entries {
            let dst = output.join(&static_entry.name[1..]);
            if let Some(parent) = dst.parent() {
                try_annotate!(fs::create_dir_all(parent), parent.clone());
            }
            try_annotate!(self.render_static(&site, &static_entry.source, &dst), static_entry.source.clone());
        }

        for entry in &source.entries {
            let content_len = try!(entry.content.read_into(&mut buf));

            let dest_dir = output.join(&entry.name[1..]);
            try_annotate!(fs::create_dir_all(&dest_dir), dest_dir);

            if let Some(ref index) = entry.index {

                let mut child_entries: Vec<_> = source.entries.iter().filter(|child| {
                    child.cc.contains(&entry.name) || index.directories.iter().any(|d| {
                        d.matches_with(&child.name, &MATCH_OPTIONS)
                    })
                }).collect();

                {
                    use ::model::index::SortDirection::*;
                    use ::model::index::SortField::*;

                    match (index.sort.direction, index.sort.field) {
                        (Ascending,     Title) => child_entries.sort_by(|e1, e2| e1.title.cmp(&e2.title)),
                        (Descending,    Title) => child_entries.sort_by(|e1, e2| e2.title.cmp(&e1.title)),
                        (Ascending,     Date)  => child_entries.sort_by(|e1, e2| e1.date.cmp(&e2.date)),
                        (Descending,    Date)  => child_entries.sort_by(|e1, e2| e2.date.cmp(&e1.date)),
                    }
                }

                if let Some(max) = index.max {
                    child_entries.truncate(max as usize);
                }

                if let Some(paginate) = index.paginate {
                    // TODO: Assert that these casts are correct!
                    let paginate = paginate as usize;
                    let num_pages = (child_entries.len() / paginate) + if child_entries.len() % paginate == 0 { 0 } else { 1 };
                    if num_pages == 0 {
                        let content = &buf[..content_len];

                        let mut index_file_path = dest_dir;
                        index_file_path.push("index.html");

                        let index_file = try_annotate!(File::create(&index_file_path), index_file_path);

                        try_annotate!(html! {
                            |tmpl| self.render_page(&site, &Page {
                                title: &entry.title,
                                date: entry.date.as_ref(),
                                content: Content {
                                    data: content,
                                    format: entry.content.format(),
                                },
                                href: &entry.name,
                                index: Some(Index {
                                    paginate: Some(Paginate {
                                        pages: &[&entry.name],
                                        current: 0,
                                    }),
                                    entries: &[]
                                }),
                                meta: &entry.meta,
                            }, tmpl);
                        }.write_to_io(&mut BufWriter::new(index_file)), index_file_path);
                    } else {
                        let mut page_buf = String::with_capacity((num_pages-1) * (entry.name.len() + 10));
                        let mut pages: Vec<&str> = Vec::with_capacity(num_pages);
                        pages.push(&entry.name);
                        {
                            let mut page_offsets = Vec::with_capacity(num_pages-1);
                            for page_num in 1..num_pages {
                                let _ = write!(page_buf, "{}/index/{}", &entry.name, page_num);
                                page_offsets.push(page_buf.len());
                            }
                            let mut start = 0;
                            for end in page_offsets {
                                pages.push(&page_buf[start..end]);
                                start = end;
                            }
                        }

                        for (page_num, (chunk, href)) in child_entries.chunks(paginate).zip(&pages).enumerate() {
                            buf.truncate(content_len);

                            let children = read_children!(buf, chunk);
                            let content = &buf[..content_len];

                            let mut index_file_path = output.join(&href[1..]);
                            try_annotate!(fs::create_dir_all(&index_file_path), index_file_path);
                            index_file_path.push("index.html");

                            let index_file = try_annotate!(File::create(&index_file_path), index_file_path);

                            try_annotate!(html! {
                                |tmpl| self.render_page(&site, &Page {
                                    title: &entry.title,
                                    date: entry.date.as_ref(),
                                    content: Content {
                                        data: content,
                                        format: entry.content.format(),
                                    },
                                    href: &href,
                                    index: Some(Index {
                                        paginate: Some(Paginate {
                                            pages: &pages,
                                            current: page_num,
                                        }),
                                        entries: &children[..]
                                    }),
                                    meta: &entry.meta,
                                }, tmpl);
                            }.write_to_io(&mut BufWriter::new(index_file)), index_file_path);
                        }
                    }
                } else {
                    let children = read_children!(buf, &child_entries);
                    let content = &buf[..content_len];

                    let mut index_file_path = dest_dir;
                    index_file_path.push("index.html");

                    let index_file = try_annotate!(File::create(&index_file_path), index_file_path);

                    try_annotate!(html! {
                        |tmpl| self.render_page(&site, &Page {
                            title: &entry.title,
                            date: entry.date.as_ref(),
                            content: Content {
                                data: content,
                                format: entry.content.format(),
                            },
                            href: &entry.name,
                            meta: &entry.meta,
                            index: Some(Index {
                                paginate: None,
                                entries: &children[..]
                            })
                        }, tmpl);
                    }.write_to_io(&mut BufWriter::new(index_file)), index_file_path);
                }
            } else {
                let mut index_file_path = dest_dir;
                index_file_path.push("index.html");

                let index_file = try_annotate!(File::create(&index_file_path), index_file_path);

                try_annotate!(html! {
                    |tmpl| self.render_page(&site, &Page {
                        title: &entry.title,
                        date: entry.date.as_ref(),
                        content: Content {
                            data: &buf[..],
                            format: entry.content.format(),
                        },
                        href: &entry.name,
                        meta: &entry.meta,
                        index: None,
                    }, tmpl);
                }.write_to_io(&mut BufWriter::new(index_file)), index_file_path);
            }
            buf.clear();
        }
        Ok(())
    }
}
