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

/// Renders a page's content
pub struct Content<'a> {
    pub content: &'a gazetta_core::view::Content<'a>,
    pub base: &'a str,
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
            "mkd" | "md" | "markdown" => tmpl << crate::Markdown::new(self.content.data, self.base),
            // TODO: error if heading-level is non-zero.
            "html" => tmpl << Raw(self.content.data),
            "" | "text" | "txt" => tmpl << self.content.data,
            format => tmpl.record_error(format!("unknown format '{}'", format)),
        }
    }
}
