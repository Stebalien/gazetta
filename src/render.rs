/* Copyright (2015) Stevem Allen
 *
 * This file is part of gazetta.
 * 
 * gazetta-bin is free software: you can redistribute it and/or modify it under the terms of the
 * GNU Affero General Public License (version 3) as published by the Free Software Foundation.
 * 
 * Foobar is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
 * the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU Affero General Public License along with Foobar.  If
 * not, see <http://www.gnu.org/licenses/>.
 */

use std::io::{self, BufWriter};
use std::fs::{self, File};
use std::path::Path;

use horrorshow::prelude::*;
use util::{self, StreamHasher};
use model::{Source, Meta};
use view::{Site, Page, Index, Paginate, Content};
use error::{RenderError, AnnotatedError};
use std::hash::SipHasher;


/// Compiles a set of files into a single asset by concatinating them.
/// This function also hashes the files so they can be cached.
fn compile_asset<P>(paths: &[P],
                    target: &Path,
                    prefix: &str,
                    ext: &str) -> Result<String, AnnotatedError<io::Error>>
    where P: AsRef<Path>
{
    let mut tmp_path = target.join("assets");
    tmp_path.push(prefix);
    tmp_path.set_extension(ext);

    let hash = {
        let output = try_annotate!(File::create(&tmp_path), tmp_path);
        let mut output = StreamHasher::<_, SipHasher>::new(output);
        try!(util::concat(paths, &mut output));
        output.finish()
    };

    let href = format!("/assets/{}-{:x}.{}", prefix, hash, ext);
    let final_path = target.join(&href[1..]);
    try_annotate!(fs::rename(tmp_path, &final_path), final_path);
    Ok(href)
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
        util::copy_recursive(source, output)
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

        let output = output.as_ref();

        {
            let assets_path = output.join("assets");
            try_annotate!(fs::create_dir_all(&assets_path), assets_path);
        }

        let js_href = if !source.javascript.is_empty() {
            Some(try!(compile_asset(&source.javascript, output, "main", "js")))
        } else { None };

        let css_href = if !source.stylesheets.is_empty() {
            Some(try!(compile_asset(&source.stylesheets, output, "main", "css")))
        } else { None };

        let icon_href = if let Some(ref icon) = source.icon {
            Some(try!(compile_asset(&[&icon], output, "icon", "png")))
        } else { None };

        let site = Site {
            title: &source.title,
            meta: &source.meta,
            javascript: js_href.as_ref().map(|s|&s[..]),
            stylesheets: css_href.as_ref().map(|s|&s[..]),
            icon: icon_href.as_ref().map(|s|&s[..]),
        };

        for static_entry in &source.static_entries {
            let dst = output.join(&static_entry.name[1..]);
            if let Some(parent) = dst.parent() {
                try_annotate!(fs::create_dir_all(parent), parent.clone());
            }
            try_annotate!(self.render_static(&site, &static_entry.source, &dst), static_entry.source.clone());
        }

        // In general, the system calls here will dwarf the cost of a couple of allocations.
        // However, putting all content in a single string buffer may improve cache behavior.
        // TODO: Test this.
        let mut buf = String::with_capacity(4096);

        for entry in &source.entries {
            let content_len = try!(entry.content.read_into(&mut buf));

            let dest_dir = output.join(&entry.name[1..]);
            try_annotate!(fs::create_dir_all(&dest_dir), dest_dir);

            if let Some(ref index) = entry.index {

                let child_entries = source.build_index(entry);

                if let Some(paginate) = index.paginate {
                    // TODO: Assert that these casts are correct!
                    let paginate = paginate as usize;
                    let num_pages = (child_entries.len() / paginate) +
                        if child_entries.len() % paginate == 0 { 0 } else { 1 };

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
                                use std::fmt::Write;
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
