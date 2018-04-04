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

use horrorshow::prelude::*;
use pulldown_cmark::{Event, Parser, OPTION_ENABLE_FOOTNOTES, OPTION_ENABLE_TABLES};
use std::borrow::Cow;
use std::collections::HashMap;

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
        Markdown { data, base }
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
            footnotes: HashMap::new(),
            iter: Parser::new_ext(self.data, OPTION_ENABLE_TABLES | OPTION_ENABLE_FOOTNOTES),
            base: self.base,
        }
    }
}

struct RenderMarkdown<'a, I> {
    iter: I,
    footnotes: HashMap<Cow<'a, str>, u32>,
    base: &'a str,
}

impl<'a, I> RenderMarkdown<'a, I> {
    fn footnote(&mut self, name: Cow<'a, str>) -> u32 {
        let next_idx = (self.footnotes.len() as u32) + 1;
        *self.footnotes.entry(name).or_insert(next_idx)
    }

    fn make_relative<'b>(&self, dest: Cow<'b, str>) -> Cow<'b, str> {
        if dest.starts_with("./") {
            if self.base.is_empty() {
                match dest {
                    Cow::Borrowed(v) => Cow::Borrowed(&v[2..]),
                    Cow::Owned(mut v) => {
                        // There has to be a better way...
                        v.remove(0);
                        v.remove(0);
                        Cow::Owned(v)
                    }
                }
            } else {
                Cow::Owned(format!("{}/{}", self.base, &dest[2..]))
            }
        } else {
            dest
        }
    }
}

impl<'a, I: Iterator<Item = Event<'a>>> RenderOnce for RenderMarkdown<'a, I> {
    fn render_once(mut self, tmpl: &mut TemplateBuffer) {
        self.render_mut(tmpl)
    }
}

impl<'a, I: Iterator<Item = Event<'a>>> RenderMut for RenderMarkdown<'a, I> {
    fn render_mut(&mut self, tmpl: &mut TemplateBuffer) {
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
                        Tag::FootnoteDefinition(name) => {
                            tmpl << html! {
                                div(class="footnote", id=format_args!("footnote-{}", name)) {
                                    sup(class="footnote-label") : s.footnote(name);
                                    : s;
                                }
                            }
                        }
                        Tag::Paragraph => tmpl << html! { p : s },
                        Tag::Rule => {
                            // Eat the end tag.
                            match s.iter.next() {
                                Some(End(Tag::Rule)) => (),
                                _ => panic!("stuff inside horizontal rule tag?"),
                            }
                            tmpl << html! { hr; }
                        }
                        Tag::BlockQuote => tmpl << html! { blockquote : s },
                        Tag::Table(_) => tmpl << html! { table : s },
                        Tag::TableHead => tmpl << html! { thead { tr : s } },
                        Tag::TableRow => tmpl << html! { tr : s },
                        Tag::TableCell => tmpl << html! { td : s },
                        Tag::List(Some(0)) => tmpl << html! { ol : s },
                        Tag::List(Some(start)) => tmpl << html! { ol(start = start) : s },
                        Tag::List(None) => tmpl << html! { ul : s },
                        Tag::Item => tmpl << html! { li : s },
                        Tag::Emphasis => tmpl << html! { em: s },
                        Tag::Strong => tmpl << html! { strong: s },
                        Tag::Code => tmpl << html! { code: s },
                        Tag::Header(level) => match level {
                            1 => tmpl << html! { h1 : s },
                            2 => tmpl << html! { h2 : s },
                            3 => tmpl << html! { h3 : s },
                            4 => tmpl << html! { h4 : s },
                            5 => tmpl << html! { h5 : s },
                            6 => tmpl << html! { h6 : s },
                            _ => panic!(),
                        },
                        Tag::Link(dest, title) => {
                            tmpl << html! {
                                // TODO: Escape href?
                                a(href = &*s.make_relative(dest),
                                  title? = if !title.is_empty() { Some(&*title) } else { None }) : s
                            }
                        }
                        Tag::Image(dest, title) => {
                            tmpl << html! {
                                img(src = &*s.make_relative(dest),
                                    title? = if !title.is_empty() { Some(&*title) } else { None },
                                    alt = FnRenderer::new(|tmpl| {
                                        let mut nest = 0;
                                        while let Some(event) = s.iter.next() {
                                            let tmpl = &mut *tmpl;
                                            match event {
                                                Start(_) => nest += 1,
                                                End(_) if nest == 0 => break,
                                                End(_) => nest -= 1,
                                                Text(txt) => tmpl << &*txt,
                                                SoftBreak | HardBreak => tmpl << " ",
                                                FootnoteReference(_)
                                                    | InlineHtml(_)
                                                    | Html(_) => (),
                                            }
                                        }
                                    }))
                            }
                        }
                        Tag::CodeBlock(info) => {
                            // TODO Highlight code.
                            let lang = &*info.split(' ').next().unwrap();
                            // Why? Because the format_args and lifetimes...
                            match format_args!("language-{}", lang) {
                                f => {
                                    tmpl << html! {
                                        pre {
                                            code(class? = if !lang.is_empty() {
                                                Some(f)
                                            } else {
                                                None
                                            }) : s
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                End(_) => break,
                FootnoteReference(name) => {
                    tmpl << html! {
                        sup(class="footnote-reference") {
                            a(href=format_args!("{}/#footnote-{}", self.base, name)) : self.footnote(name);
                        }
                    }
                }
                Text(text) => tmpl << &*text,
                Html(html) | InlineHtml(html) => tmpl << Raw(html),
                SoftBreak => tmpl << "\n",
                HardBreak => tmpl << html! { br },
            };
        }
    }
}
