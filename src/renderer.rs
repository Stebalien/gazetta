use std::io;
use std::io::prelude::*;
use std::fmt::Write as FmtWrite;

use model::Site;
use builder::SiteBuilder;
use view::{Page, Index, Paginate};

pub trait Renderer {
    fn render_page<W: Write>(&self, site: &Site, page: &Page, output: &mut W) -> io::Result<()>;
    fn render<B: SiteBuilder>(&self, site: &Site, output: &mut B) -> io::Result<()> {
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
                        child_entry.content = Some(s)
                    }
                }
                children
            }}
        }

        for (target, entry) in &site.entries {
            let content_len = try!(entry.read_content(&mut buf));
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

                    let mut cur_href = String::with_capacity(target.len() + 10);
                    let mut next_href = String::with_capacity(target.len() + 10);
                    let mut prev_href = String::with_capacity(target.len() + 10);

                    for (page_num, chunk) in entries.chunks(paginate).enumerate() {
                        {
                            let page_num = page_num as u32;

                            let children = read_children!(buf, chunk, site);
                            let content = &buf[..content_len];

                            let has_prev = page_num > 0;
                            let has_next = page_num + 1 < num_pages;

                            if page_num == 0 {
                                cur_href.push_str(&target);
                            } else {
                                let _ = write!(cur_href, "{}/index/{}", target, page_num);
                            }

                            if has_prev {
                                let _ = write!(prev_href, "{}/index/{}", target, page_num - 1);
                            }

                            if has_next {
                                let _ = write!(next_href, "{}/index/{}", target, page_num + 1);
                            }

                            try!(output.build_page(&target, |w| self.render_page(site, &Page {
                                title: &entry.title,
                                date: entry.date.as_ref(),
                                content: if content.trim().is_empty() { None } else { Some(content) },
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
                            }, w)));
                        }
                        cur_href.clear();
                        next_href.clear();
                        prev_href.clear();
                        buf.truncate(content_len);
                    }
                } else {
                    let children = read_children!(buf, entries, site);
                    let content = &buf[..content_len];
                    try!(output.build_page(&target, |w| self.render_page(site, &Page {
                        title: &entry.title,
                        date: entry.date.as_ref(),
                        content: {
                            if content.trim().is_empty() {
                                None
                            } else {
                                Some(content)
                            }
                        },
                        href: &target,
                        index: Some(Index {
                            paginate: None,
                            entries: &children[..]
                        })
                    }, w)));
                }
            } else {
                try!(output.build_page(&target, |w| self.render_page(site, &Page {
                    title: &entry.title,
                    date: entry.date.as_ref(),
                    content: if buf.trim().is_empty() { None } else { Some(&buf[..]) },
                    href: &target,
                    index: None,
                }, w)));
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

