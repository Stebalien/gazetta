//  Copyright (C) 2015 Steven Allen
//
//  This file is part of gazetta.
//
//  This program is free software: you can redistribute it and/or modify it under the terms of the
//  GNU General Public License as published by the Free Software Foundation version 3 of the
//  License.
//
//  This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
//  without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See
//  the GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License along with this program.  If
//  not, see <http://www.gnu.org/licenses/>.
//

use std::fs::{self, File};
use std::io::{self, BufWriter};
use std::path::Path;

use horrorshow::prelude::*;
use horrorshow::{html, xml};
use std::collections::hash_map::DefaultHasher;
use str_stack::StrStack;

use crate::error::{AnnotatedError, RenderError};
use crate::model::{IndexedSource, Meta};
use crate::util::{self, StreamHasher};
use crate::view::{Context, Index, Page, Paginate, Site};

/// Compiles a set of files into a single asset by concatinating them. This
/// function also hashes the files so they can be cached.
fn compile_asset<P>(
    paths: &[P],
    target: &Path,
    prefix: &str,
    ext: &str,
) -> Result<String, AnnotatedError<io::Error>>
where
    P: AsRef<Path>,
{
    let mut tmp_path = target.join("assets");
    tmp_path.push(prefix);
    tmp_path.set_extension(ext);

    let hash = {
        let output = try_annotate!(File::create(&tmp_path), tmp_path);
        let mut output = StreamHasher::<_, DefaultHasher>::new(output);
        util::concat(paths, &mut output)?;
        output.finish()
    };

    let href = format!("assets/{}-{:x}.{}", prefix, hash, ext);
    let final_path = target.join(&href);
    try_annotate!(fs::rename(tmp_path, &final_path), final_path);
    Ok(href)
}

pub trait Gazetta: Sized {
    type SiteMeta: Meta;
    type PageMeta: Meta;

    /// The page rendering function.
    fn render_page(&self, context: &Context<Self>, tmpl: &mut TemplateBuffer);

    /// Render static content.
    ///
    /// By default, this just copies. Override to compile.
    #[allow(unused_variables)]
    fn render_static(&self, site: &Site<Self>, source: &Path, output: &Path) -> io::Result<()> {
        util::copy_recursive(source, output)
    }

    /// Render any additional feed "head" elements (usually author information, etc.). All the
    /// necessary fields (title, id, updated, link, etc.) will already have been included.
    #[allow(unused_variables)]
    fn render_feed_head(&self, context: &Context<Self>, tmpl: &mut TemplateBuffer) {}

    /// Render a page for syndication as a feed entry. By default, only the page summary
    /// (description) is included in the feed but this method can be overridden to include
    /// additional elements such as authorship information, the feed content, etc.
    #[allow(unused_variables)]
    fn render_feed_entry(&self, context: &Context<Self>, tmpl: &mut TemplateBuffer) {}

    /// Render the feed for a page. In general, you'll want to override
    /// [`Gazetta::render_feed_entry`] and [`Gazetta::render_feed_head`], not this method, unless
    /// you want to override how the entire feed is rendered.
    fn render_feed(&self, context: &Context<Self>, tmpl: &mut TemplateBuffer) {
        let Some(index) = context.page.index else {
            return;
        };
        let Some(feed_url) = index.feed else { return };
        tmpl << xml! {
            feed(xmlns="http://www.w3.org/2005/Atom", xml:base=context.site.base()) {
                id : context.canonical_url();
                title : &context.page.title;
                link(href = context.canonical_url());
                link(rel = "self", href = feed_url);
                updated : context.page.updated.to_rfc3339();

                @ if let Some(icon) = &context.site.icon {
                    icon : icon;
                }

                |tmpl| self.render_feed_head(context, tmpl);

                @ for pctx in index.entries.iter().map(|page| Context{site: context.site, page}) {
                    entry {
                        id : pctx.canonical_url();
                        title : &pctx.page.title;
                        link(href = pctx.canonical_url(), rel="alternate");
                        updated : pctx.page.updated.to_rfc3339();
                        @ if let Some(date) = pctx.page.date {
                            published : date.to_rfc3339();
                        }
                        @ if let Some(summary) = pctx.page.description {
                            summary(type="text") : summary;
                        }
                        |tmpl| self.render_feed_entry(&pctx, tmpl);
                    }
                }
            }
        }
    }

    /// Creates pages from a site defined by a source and renders them into output.
    ///
    /// Call this to render your site.
    ///
    /// Note: You *can* override this but you **really** shouldn't. This function contains pretty
    /// much all of the provided render logic.
    fn render<P: AsRef<Path>>(
        &self,
        source: &IndexedSource<Self::SiteMeta, Self::PageMeta>,
        output: P,
    ) -> Result<(), AnnotatedError<RenderError>> {
        let output = output.as_ref();

        {
            let assets_path = output.join("assets");
            try_annotate!(fs::create_dir_all(&assets_path), assets_path);
        }

        let js_href = if !source.javascript.is_empty() {
            Some(compile_asset(&source.javascript, output, "main", "js")?)
        } else {
            None
        };

        let css_href = if !source.stylesheets.is_empty() {
            Some(compile_asset(&source.stylesheets, output, "main", "css")?)
        } else {
            None
        };

        let icon_href = if let Some(ref icon) = source.icon {
            Some(compile_asset(&[&icon], output, "icon", "png")?)
        } else {
            None
        };

        if let Some(ref src) = source.well_known {
            let dst = output.join(".well-known");
            try_annotate!(util::copy_recursive(src, &dst), src)
        }

        let site = Site {
            title: &source.title,
            origin: &source.origin,
            prefix: &source.prefix,
            meta: &source.meta,
            javascript: js_href.as_ref().map(|s| &s[..]),
            stylesheets: css_href.as_ref().map(|s| &s[..]),
            icon: icon_href.as_ref().map(|s| &s[..]),
        };

        for static_entry in &source.static_entries {
            let dst = output.join(&static_entry.name);
            if let Some(parent) = dst.parent() {
                try_annotate!(fs::create_dir_all(parent), parent);
            }
            try_annotate!(
                self.render_static(&site, &static_entry.source, &dst),
                static_entry.source.clone()
            );
        }

        for entry in &source.entries {
            let dest_dir = output.join(&entry.name);
            try_annotate!(fs::create_dir_all(&dest_dir), dest_dir);

            let page = Page::for_entry(entry);

            if let Some(ref index) = entry.index {
                let children: Vec<_> = source
                    .children(&entry.name)
                    .iter()
                    .copied()
                    .map(Page::for_entry)
                    .collect();

                let feed_path = if let Some(syndicate) = &index.syndicate {
                    let mut atom_file_path = dest_dir.clone();
                    atom_file_path.push("atom.xml");
                    let atom_file = try_annotate!(File::create(&atom_file_path), atom_file_path);
                    let to_syndicate = syndicate
                        .max
                        .map(|m| &children[..m as usize])
                        .unwrap_or(&children);

                    let feed_path = format!("{}/atom.xml", page.href);
                    try_annotate!(
                        xml! {
                            : Raw("<?xml version=\"1.0\" encoding=\"utf-8\"?>");
                            |tmpl| self.render_feed(&Context{
                                site: &site,
                                page: &Page {
                                    index: Some(Index {
                                        compact: index.compact,
                                        paginate: None,
                                        feed: Some(&feed_path),
                                        entries: to_syndicate,
                                    }),
                                    ..page
                                },
                            }, tmpl);
                        }
                        .write_to_io(&mut BufWriter::new(atom_file)),
                        atom_file_path
                    );
                    Some(feed_path)
                } else {
                    None
                };

                if let Some(paginate) = index.paginate {
                    // TODO: Assert that these casts are correct!
                    let paginate = paginate as usize;
                    let num_pages = (children.len() / paginate)
                        + if children.len() % paginate == 0 { 0 } else { 1 };

                    if num_pages == 0 {
                        let mut index_file_path = dest_dir;
                        index_file_path.push("index.html");

                        let index_file =
                            try_annotate!(File::create(&index_file_path), index_file_path);

                        try_annotate!(
                            html! {
                                |tmpl| self.render_page(&Context{
                                    site: &site,
                                    page: &Page {
                                        index: Some(Index {
                                            compact: index.compact,
                                            paginate: Some(Paginate {
                                                pages: &[page.href],
                                                current: 0,
                                            }),
                                            feed: feed_path.as_deref(),
                                            entries: &[],
                                        }),
                                        ..page
                                    },
                                }, tmpl);
                            }
                            .write_to_io(&mut BufWriter::new(index_file)),
                            index_file_path
                        );
                    } else {
                        let mut page_stack = StrStack::with_capacity(
                            (num_pages - 1) * (entry.name.len() + 10),
                            num_pages,
                        );
                        for page_num in 1..num_pages {
                            let _ = write!(page_stack, "{}/index/{}", &entry.name, page_num);
                        }
                        let mut pages = Vec::with_capacity(num_pages);
                        pages.push(&*entry.name);
                        pages.extend(&page_stack);

                        for (page_num, (children_range, href)) in
                            children.chunks(paginate).zip(&pages).enumerate()
                        {
                            let mut index_file_path = output.join(href);
                            try_annotate!(fs::create_dir_all(&index_file_path), index_file_path);
                            index_file_path.push("index.html");

                            let index_file =
                                try_annotate!(File::create(&index_file_path), index_file_path);
                            try_annotate!(
                                html! {
                                    |tmpl| self.render_page(&Context{
                                        site: &site,
                                        page: &Page {
                                        index: Some(Index {
                                            feed: feed_path.as_deref(),
                                            compact: index.compact,
                                            paginate: Some(Paginate {
                                                pages: &pages,
                                                current: page_num,
                                            }),
                                            entries: children_range,
                                        }),
                                        href,
                                        ..page
                                        },
                                    }, tmpl);
                                }
                                .write_to_io(&mut BufWriter::new(index_file)),
                                index_file_path
                            );
                        }
                    }
                } else {
                    let mut index_file_path = dest_dir;
                    index_file_path.push("index.html");

                    let index_file = try_annotate!(File::create(&index_file_path), index_file_path);

                    try_annotate!(
                        html! {
                            |tmpl| self.render_page(&Context {
                                site: &site,
                                page: &Page {
                                index:  Some(Index {
                                    feed: feed_path.as_deref(),
                                    compact: index.compact,
                                    paginate: None,
                                    entries: &children[..],
                                }),
                                ..page
                                },
                            }, tmpl);
                        }
                        .write_to_io(&mut BufWriter::new(index_file)),
                        index_file_path
                    );
                }
            } else {
                let mut index_file_path = dest_dir;
                index_file_path.push("index.html");

                let index_file = try_annotate!(File::create(&index_file_path), index_file_path);

                try_annotate!(
                    html! {
                        |tmpl| self.render_page(&Context{
                            site: &site,
                            page: &page,
                        }, tmpl);
                    }
                    .write_to_io(&mut BufWriter::new(index_file)),
                    index_file_path
                );
            }
        }
        Ok(())
    }
}
