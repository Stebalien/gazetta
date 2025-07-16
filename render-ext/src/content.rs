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
use horrorshow::{html, prelude::*};

/// Renders content:
///
/// - Raw text will be rendered as escaped paragraphs (separated by double newlines).
/// - HTML will be injected as-is with no processing.
/// - Markdown documents will be rendered as one might expect EXCEPT that:
///   - "./" at the beginning of links, image references, etc. will be replaced with `path`.
///   - If `root` is specified, all links will be made absolute (prefixed with `root`).
pub struct Content<'a> {
    /// The content to render.
    pub content: &'a gazetta_core::view::Content<'a>,
    /// The website's "root" (origin). If specified, links and references will be absolute (markdown
    /// only).
    pub root: Option<&'a str>,
    /// The current page, relative to the root (markdown only).
    pub path: &'a str,
    /// Whether or not to syntax highlight code blocks (markdown only).
    pub syntax_highlight: bool,
}

impl RenderOnce for Content<'_> {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        self.render(tmpl);
    }
}

impl RenderMut for Content<'_> {
    fn render_mut(&mut self, tmpl: &mut TemplateBuffer) {
        self.render(tmpl);
    }
}

impl Render for Content<'_> {
    fn render(&self, tmpl: &mut TemplateBuffer) {
        match self.content.format {
            "mkd" | "md" | "markdown" => {
                tmpl << crate::Markdown::new(
                    self.content.data,
                    self.root,
                    self.path,
                    self.syntax_highlight,
                )
            }
            "html" => tmpl << Raw(self.content.data),
            "" | "text" | "txt" => {
                tmpl << html! {
                    @ for p in self.content.data.split("\n\n") {
                        p : p;
                    }
                }
            }
            format => tmpl.record_error(format!("unknown format '{format}'")),
        }
    }
}
