use std::io::{self, BufWriter};
use std::fs::{self, File};
use std::io::prelude::*;
use std::fmt::Write as FmtWrite;
use markdown::Markdown;
use std::path::{Path, PathBuf};

use model::{Source, Meta};
use view::{Site, Page, Index, Paginate};

static MATCH_OPTIONS: ::glob::MatchOptions = ::glob::MatchOptions {
    case_sensitive: true,
    require_literal_separator: true,
    require_literal_leading_dot: false,
};

fn copy_dir(src: &Path, dest: &Path) -> io::Result<()> {
    try!(fs::create_dir(dest));
    for dir_entry in try!(fs::read_dir(src)) {
        let dir_entry = try!(dir_entry);
        let from = dir_entry.path();
        let to = dest.join(from.relative_from(src).unwrap());
        if fs::metadata(&to).is_ok() {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "target path already exists"));
        }

        if try!(dir_entry.file_type()).is_dir() {
            try!(copy_dir(&from, &to));
        } else {
            // TODO: Use cow. Note: Don't use fs::copy because we just want to copy the files, not the
            // permisions.
            try!(io::copy(&mut try!(File::open(from)), &mut try!(File::create(to))));
        }
    }
    Ok(())
}

pub trait Renderer {
    type SiteMeta: Meta;
    type PageMeta: Meta;

    fn render_page<W: Write>(&self,
                             site: &Site<Self::SiteMeta>,
                             page: &Page<Self::PageMeta>,
                             output: &mut W) -> Result<(), Box<::std::error::Error>>;

    fn render<P: AsRef<Path>>(&self,
                              source: &Source<Self::SiteMeta, Self::PageMeta>,
                              output: P) -> Result<(), Box<::std::error::Error>>
    {
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
                    for &(child_name, child) in $entries {
                        strides.push(try!(child.read_content(&mut $buf)));
                        children.push(Page {
                            title: &child.title,
                            date: child.date.as_ref(),
                            content: None,
                            href: &child_name,
                            index: None,
                            meta: &child.meta,
                        });
                    }
                }

                for (&len, child_entry) in strides.iter().zip(children.iter_mut()) {
                    let s  = &$buf[start..(start + len)];
                    start += len;

                    if !s.trim().is_empty() {
                        child_entry.content = Some(Markdown::new(s, child_entry.href))
                    }
                }
                children
            }}
        }

        for (name, entry) in &source.entries {
            let content_len = try!(entry.read_content(&mut buf));

            {
                // Create output.
                let mut dest_dir = output.join(&name[1..]);
                try!(fs::create_dir_all(&dest_dir));

                // Copy static.
                let src_dir = entry.content_path.parent().unwrap().join("static");
                if fs::metadata(&src_dir).is_ok() {
                    dest_dir.push("static");
                    try!(copy_dir(&src_dir, &dest_dir));
                }
            }

            if let Some(ref index) = entry.index {

                // TODO: We can optimize this because the btree is sorted.
                let mut child_entries: Vec<_> = source.entries.iter().filter(|&(ref child_name, ref child)| {
                    child.cc.contains(name) || index.directories.iter().any(|d| d.matches_with(child_name, &MATCH_OPTIONS))
                }).collect();

                {
                    use ::model::index::SortDirection::*;
                    use ::model::index::SortField::*;

                    match (index.sort.direction, index.sort.field) {
                        (Ascending,     Title) => child_entries.sort_by(|&(_, ref e1), &(_, ref e2)| e1.title.cmp(&e2.title)),
                        (Descending,    Title) => child_entries.sort_by(|&(_, ref e1), &(_, ref e2)| e2.title.cmp(&e1.title)),
                        (Ascending,     Date)  => child_entries.sort_by(|&(_, ref e1), &(_, ref e2)| e1.date.cmp(&e2.date)),
                        (Descending,    Date)  => child_entries.sort_by(|&(_, ref e1), &(_, ref e2)| e2.date.cmp(&e1.date)),
                    }
                }

                if let Some(max) = index.max {
                    child_entries.truncate(max as usize);
                }

                if let Some(paginate) = index.paginate {
                    // TODO: Assert that these casts are correct!
                    let paginate = paginate as usize;
                    let num_pages = (child_entries.len() / paginate) + if child_entries.len() % paginate == 0 { 0 } else { 1 };


                    let mut page_buf = String::with_capacity((num_pages-1) * (name.len() + 10));
                    let mut pages: Vec<&str> = Vec::with_capacity(num_pages);
                    pages.push(name);
                    {
                        let mut page_offsets = Vec::with_capacity(num_pages-1);
                        for page_num in 1..num_pages {
                            let _ = write!(page_buf, "{}/index/{}", name, page_num);
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

                        let mut index_file = output.join(&href[1..]);
                        try!(fs::create_dir_all(&index_file));
                        index_file.push("index.html");

                        try!(self.render_page(&site, &Page {
                            title: &entry.title,
                            date: entry.date.as_ref(),
                            content: if content.trim().is_empty() {
                                None
                            } else {
                                Some(Markdown::new(content, &name))
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
                        }, &mut BufWriter::new(try!(File::create(index_file)))));
                    }
                } else {
                    let children = read_children!(buf, &child_entries);
                    let content = &buf[..content_len];

                    let index_file: PathBuf  = [
                        output,
                        name[1..].as_ref(),
                        "index.html".as_ref()
                    ].into_iter().collect();

                    try!(self.render_page(&site, &Page {
                        title: &entry.title,
                        date: entry.date.as_ref(),
                        content: if content.trim().is_empty() {
                            None
                        } else {
                            Some(Markdown::new(content, &name))
                        },
                        href: &name,
                        meta: &entry.meta,
                        index: Some(Index {
                            paginate: None,
                            entries: &children[..]
                        })
                    }, &mut BufWriter::new(try!(File::create(index_file)))));
                }
            } else {
                let index_file: PathBuf  = [
                    output,
                    name[1..].as_ref(),
                    "index.html".as_ref()
                ].into_iter().collect();

                try!(self.render_page(&site, &Page {
                    title: &entry.title,
                    date: entry.date.as_ref(),
                    content: if buf.trim().is_empty() { None } else { Some(Markdown::new(&buf[..], &name)) },
                    href: &name,
                    meta: &entry.meta,
                    index: None,
                }, &mut BufWriter::new(try!(File::create(index_file)))));
            }
            buf.clear();
        }
        Ok(())
    }
}

pub struct DebugRenderer;

impl Renderer for DebugRenderer {
    type SiteMeta = ::yaml::Hash;
    type PageMeta = ::yaml::Hash;

    fn render_page<W: Write>(&self,
                             _site: &Site<Self::SiteMeta>,
                             page: &Page<Self::PageMeta>,
                             output: &mut W) -> Result<(), Box<::std::error::Error>>
    {
        Ok(try!(writeln!(output, "{:#?}", page)))
    }
}
