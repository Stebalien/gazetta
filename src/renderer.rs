use std::io;
use std::fs::{self, File};
use std::io::prelude::*;
use std::fmt::Write as FmtWrite;
use markdown::Markdown;
use std::path::{Path, PathBuf};

use model::Site;
use view::{Page, Index, Paginate};

fn copy_dir(src: &Path, dest: &Path) -> io::Result<()> {
    try!(fs::create_dir(dest));
    for dir_entry in try!(fs::walk_dir(src)) {
        let dir_entry = try!(dir_entry);
        let from = dir_entry.path();
        let to = dest.join(from.relative_from(src).unwrap());
        if fs::metadata(&to).is_ok() {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "target path already exists"));
        }

        if try!(dir_entry.file_type()).is_dir() {
            try!(fs::create_dir(to));
        } else {
            // TODO: Use cow. Note: Don't use fs::copy because we just want to copy the files, not the
            // permisions.
            try!(io::copy(&mut try!(File::open(from)), &mut try!(File::create(to))));
        }
    }
    Ok(())
}

pub trait Renderer {
    fn render_page<W: Write>(&self, site: &Site, page: &Page, output: &mut W) -> io::Result<()>;
    fn render<P: AsRef<Path>>(&self, site: &Site, output: P) -> io::Result<()> {
        let output = output.as_ref();

        // In general, the system calls here will dwarf the cost of a couple of allocations.
        // However, putting all content in a single string buffer may improve cache behavior.
        // TODO: Test this.
        let mut buf = String::with_capacity(4096);

        // Define this as a macro because we need to go from a mutable borrow to an immutable borrow...
        macro_rules! read_children {
            ($buf:ident, $entries:ident, $site:ident)  => {{
                let mut children = Vec::with_capacity($entries.len());
                let mut strides = Vec::with_capacity($entries.len());

                let mut start = $buf.len();

                {
                    for child in $entries {
                        let page = &$site.entries[child];
                        strides.push(try!(page.read_content(&mut $buf)));
                        children.push(Page {
                            title: &page.title,
                            date: page.date.as_ref(),
                            content: None,
                            href: &child,
                            index: None,
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

        for (target, entry) in &site.entries {
            let content_len = try!(entry.read_content(&mut buf));

            {
                // Create output.
                let mut dest_dir = output.join(target);
                try!(fs::create_dir_all(&dest_dir));

                // Copy static.
                let src_dir = entry.src.join("static");
                if fs::metadata(&src_dir).is_ok() {
                    dest_dir.push("static");
                    try!(copy_dir(&src_dir, &dest_dir));
                }
            }

            if let Some(ref index) = entry.index {
                let entries = match index.max {
                    Some(max) => &index.entries[..(max as usize)],
                    None => &index.entries[..],
                };

                if let Some(paginate) = index.paginate {
                    // TODO: Assert that these casts are correct!
                    let paginate = paginate as usize;
                    let num_pages = ((entries.len() / paginate)
                                     + if entries.len() % paginate == 0 { 0 } else { 1 }) as u32;


                    for (page_num, chunk) in entries.chunks(paginate).enumerate() {
                        buf.truncate(content_len);

                        let page_num = page_num as u32;

                        let children = read_children!(buf, chunk, site);
                        let content = &buf[..content_len];

                        let has_prev = page_num > 0;
                        let has_next = page_num + 1 < num_pages;

                        // Initialize links.

                        let mut cur_href = String::with_capacity(target.len() + 10);
                        let mut next_href = String::with_capacity(target.len() + 10);
                        let mut prev_href = String::with_capacity(target.len() + 10);

                        if page_num == 0 {
                            cur_href.push_str(&target);
                        } else {
                            let _ = write!(cur_href, "{}/index/{}", target, page_num);
                            try!(fs::create_dir_all(output.join(&cur_href)));
                        }

                        if has_prev {
                            let _ = write!(prev_href, "{}/index/{}", target, page_num - 1);
                        }

                        if has_next {
                            let _ = write!(next_href, "{}/index/{}", target, page_num + 1);
                        }

                        let index_file: PathBuf  = [
                            output,
                            cur_href.as_ref(),
                            "index.html".as_ref()
                        ].into_iter().collect();

                        try!(self.render_page(site, &Page {
                            title: &entry.title,
                            date: entry.date.as_ref(),
                            content: if content.trim().is_empty() {
                                None
                            } else {
                                Some(Markdown::new(content, &target))
                            },
                            href: &cur_href,
                            index: Some(Index {
                                paginate: Some(Paginate {
                                    prev: if has_prev { Some((page_num - 1, &prev_href)) } else { None },
                                    next: if has_next { Some((page_num + 1, &next_href)) } else { None },
                                    page_number: page_num,
                                    total: num_pages,
                                }),
                                entries: &children[..]
                            }),
                        }, &mut try!(File::create(index_file))));
                    }
                } else {
                    let children = read_children!(buf, entries, site);
                    let content = &buf[..content_len];

                    let index_file: PathBuf  = [
                        output,
                        target.as_ref(),
                        "index.html".as_ref()
                    ].into_iter().collect();

                    try!(self.render_page(site, &Page {
                        title: &entry.title,
                        date: entry.date.as_ref(),
                        content: if content.trim().is_empty() {
                            None
                        } else {
                            Some(Markdown::new(content, &target))
                        },
                        href: &target,
                        index: Some(Index {
                            paginate: None,
                            entries: &children[..]
                        })
                    }, &mut try!(File::create(index_file))));
                }
            } else {
                let index_file: PathBuf  = [
                    output,
                    target.as_ref(),
                    "index.html".as_ref()
                ].into_iter().collect();

                try!(self.render_page(site, &Page {
                    title: &entry.title,
                    date: entry.date.as_ref(),
                    content: if buf.trim().is_empty() { None } else { Some(Markdown::new(&buf[..], &target)) },
                    href: &target,
                    index: None,
                }, &mut try!(File::create(index_file))));
            }
            buf.clear();
        }
        Ok(())
    }
}

pub struct DebugRenderer;

impl Renderer for DebugRenderer {
    fn render_page<W: Write>(&self, _site: &Site, page: &Page, output: &mut W) -> io::Result<()> {
        writeln!(output, "{:#?}", page)
    }
}
