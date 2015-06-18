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

trait Iff: Sized {
    fn iff<F: FnOnce(&Self) -> bool>(self, f: F) -> Option<Self> {
        if (f)(&self) { Some(self) } else { None }
    }
}

impl<T> Iff for T {}

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
            match event {
                Start(tag) => {
                    // Because rust doesn't reborrow? (WTF?)
                    let s: &mut Self = &mut *self;
                    match tag {
                        Tag::Paragraph          => &mut *tmpl << html! { p : s },
                        Tag::Rule               => &mut *tmpl << html! { hr: s },
                        Tag::BlockQuote         => &mut *tmpl << html! { blockquote : s },
                        Tag::List(Some(0))      => &mut *tmpl << html! { ol : s },
                        Tag::List(Some(start))  => &mut *tmpl << html! { ol(start = start) : s },
                        Tag::List(None)         => &mut *tmpl << html! { ul : s },
                        Tag::Item               => &mut *tmpl << html! { li : s },
                        Tag::Emphasis           => &mut *tmpl << html! { em: s },
                        Tag::Strong             => &mut *tmpl << html! { strong: s },
                        Tag::Code               => &mut *tmpl << html! { code: s },
                        Tag::Header(level) => match level {
                            1 => &mut *tmpl << html! { h1 : s },
                            2 => &mut *tmpl << html! { h2 : s },
                            3 => &mut *tmpl << html! { h3 : s },
                            4 => &mut *tmpl << html! { h4 : s },
                            5 => &mut *tmpl << html! { h5 : s },
                            6 => &mut *tmpl << html! { h6 : s },
                            _ => panic!(),
                        },
                        Tag::Link(mut dest, title)  => {
                            if !is_absolute(&*dest) {
                                dest = Cow::Owned(format!("{}/{}", &*self.base, &*dest));
                            }

                            &mut *tmpl << html! {
                                // TODO: Escape href?
                                a(href = &*dest, title? = (&*title).iff(|&s|!s.is_empty())) : s
                            }
                        }
                        Tag::Image(dest, title) => &mut *tmpl << html! {
                            img(src = &*dest,
                                title? = (&*title).iff(|&s|!s.is_empty()),
                                alt = FnRenderer::new(|tmpl| {
                                    let mut nest = 0;
                                    while let Some(event) = s.iter.next() {
                                        match event {
                                            Start(_) => nest += 1,
                                            End(_) if nest == 0 => break,
                                            End(_) => nest -= 1,
                                            Text(txt) | InlineHtml(txt) => {&mut *tmpl << &*txt;},
                                            SoftBreak | HardBreak => {&mut *tmpl << " ";},
                                            Html(_) => (),
                                        }
                                    }
                                }))
                        },
                        Tag::CodeBlock(info)    => {
                            // TODO Highlight code.
                            let lang = &*info.split(" ").next().unwrap();
                            // Can't use map because format_args references the
                            // stack... Bad macro! Bad!
                            &mut *tmpl << html! {
                                pre {
                                    code(class? = lang.iff(|&s| {
                                        !s.is_empty()
                                    }).and(Some(format_args!("language-{}", lang)))) : s
                                }
                            }
                        },
                    }
                },
                End(_) => break,
                Text(text) => &mut *tmpl << &*text,
                Html(html) | InlineHtml(html) => &mut *tmpl << raw!(html),
                SoftBreak => &mut *tmpl << "\n",
                HardBreak => &mut *tmpl << html! { br },
            };
        }
    }
}
