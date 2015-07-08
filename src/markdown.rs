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

use pulldown_cmark::{Parser, Event};
use horrorshow::prelude::*;
use std::borrow::Cow;

fn is_absolute(mut url: &str) -> bool {
    if url.starts_with("/") {
        return true;
    }
    // strip hash
    if let Some(hash) = url.find('#') {
        url = &url[..hash];
    }
    // If there is a protocol, we should see a ':'.
    // This will obviously only work for well-formed urls.
    url.contains(':')
}

/// Markdown renderer
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Markdown<'a> {
    data: &'a str,
    base: &'a str,
}

impl<'a> Markdown<'a> {
    /// Create a new markdown renderer.
    ///
    /// `data` should contain the markdown to be rendered and `base` should specify a relative url
    /// prefix (for relative links and images).
    ///
    /// Note: `base` will only affect markdown links and images, not inline html ones.
    pub fn new(data: &'a str, base: &'a str) -> Markdown<'a> {
        Markdown {
            data: data,
            base: base,
        }
    }
}

impl<'a> RenderOnce for Markdown<'a> {
    #[inline]
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        self.render(tmpl)
    }
}

impl<'a> RenderMut for Markdown<'a> {
    #[inline]
    fn render_mut(&mut self, tmpl: &mut TemplateBuffer) {
        self.render(tmpl)
    }
}

impl<'a> Render for Markdown<'a> {
    #[inline]
    fn render(&self, tmpl: &mut TemplateBuffer) {
        tmpl << RenderMarkdown {
            iter: Parser::new(&self.data),
            base: &self.base
        };
    }
}

struct RenderMarkdown<'a, I> {
    iter: I,
    base: &'a str,
}

impl<'a, I: Iterator<Item=Event<'a>>> RenderOnce for RenderMarkdown<'a, I> {
    fn render_once(mut self, mut tmpl: &mut TemplateBuffer) {
        self.render_mut(tmpl)
    }
}

impl<'a, I: Iterator<Item=Event<'a>>> RenderMut for RenderMarkdown<'a, I> {
    fn render_mut(&mut self, mut tmpl: &mut TemplateBuffer) {
        use pulldown_cmark::Event::*;
        use pulldown_cmark::Tag;

        while let Some(event) = self.iter.next() {
            // manually reborrow
            let tmpl = &mut *tmpl;
            match event {
                Start(tag) => {
                    // Because rust doesn't reborrow? (WTF?)
                    let s: &mut Self = &mut *self;
                    match tag {
                        Tag::Paragraph          => tmpl << html! { p : s },
                        Tag::Rule               => tmpl << html! { hr: s },
                        Tag::BlockQuote         => tmpl << html! { blockquote : s },
                        Tag::List(Some(0))      => tmpl << html! { ol : s },
                        Tag::List(Some(start))  => tmpl << html! { ol(start = start) : s },
                        Tag::List(None)         => tmpl << html! { ul : s },
                        Tag::Item               => tmpl << html! { li : s },
                        Tag::Emphasis           => tmpl << html! { em: s },
                        Tag::Strong             => tmpl << html! { strong: s },
                        Tag::Code               => tmpl << html! { code: s },
                        Tag::Header(level) => match level {
                            1 => tmpl << html! { h1 : s },
                            2 => tmpl << html! { h2 : s },
                            3 => tmpl << html! { h3 : s },
                            4 => tmpl << html! { h4 : s },
                            5 => tmpl << html! { h5 : s },
                            6 => tmpl << html! { h6 : s },
                            _ => panic!(),
                        },
                        Tag::Link(mut dest, title)  => {
                            if !is_absolute(&*dest) {
                                dest = Cow::Owned(format!("{}/{}", &*self.base, &*dest));
                            }

                            tmpl << html! {
                                // TODO: Escape href?
                                a(href = &*dest, title? = if !title.is_empty() { Some(&*title) } else { None }) : s
                            }
                        }
                        Tag::Image(mut dest, title) => {
                            if !is_absolute(&*dest) {
                                dest = Cow::Owned(format!("{}/{}", &*self.base, &*dest));
                            }

                            tmpl << html! {
                                img(src = &*dest,
                                    title? = if !title.is_empty() { Some(&*title) } else { None },
                                    alt = FnRenderer::new(|tmpl| {
                                        let mut nest = 0;
                                        while let Some(event) = s.iter.next() {
                                            let tmpl = &mut *tmpl;
                                            match event {
                                                Start(_) => nest += 1,
                                                End(_) if nest == 0 => break,
                                                End(_) => nest -= 1,
                                                Text(txt) | InlineHtml(txt) => tmpl << &*txt,
                                                SoftBreak | HardBreak => tmpl << " ",
                                                Html(_) => (),
                                            }
                                        }
                                    }))
                            }
                        },
                        Tag::CodeBlock(info)    => {
                            // TODO Highlight code.
                            let lang = &*info.split(" ").next().unwrap();
                            // Why? Because the format_args...
                            (|f| tmpl << html! {
                                pre {
                                    code(class? = if !lang.is_empty() { Some(f) } else { None }) : s
                                }
                            })(format_args!("language-{}", lang))
                        },
                    }
                },
                End(_) => break,
                Text(text) => tmpl << &*text,
                Html(html) | InlineHtml(html) => tmpl << raw!(html),
                SoftBreak => tmpl << "\n",
                HardBreak => tmpl << html! { br },
            };
        }
    }
}
