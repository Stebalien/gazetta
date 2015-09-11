/*  Copyright (C) 2015 Steven Allen
 *
 *  This file is part of gazetta.
 *
 *  This program is free software: you can redistribute it and/or modify it under the terms of the
 *  GNU General Public License as published by the Free Software Foundation version 3 of the
 *  License.
 *
 *  This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 *  without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See
 *  the GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License along with this program.  If
 *  not, see <http://www.gnu.org/licenses/>.
 */
use horrorshow::prelude::*;
use gazetta_core::render::Gazetta;
use gazetta_core::view::Page;

/// Renders a page's content
pub struct Content<'a, G>(pub &'a Page<'a, G>)
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
          G::PageMeta: 'a;

impl<'a, G> RenderOnce for Content<'a, G>
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
          G::PageMeta: 'a
{
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        self.render(tmpl);
    }
}

impl<'a, G> RenderMut for Content<'a, G>
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
          G::PageMeta: 'a
{
    fn render_mut(&mut self, tmpl: &mut TemplateBuffer) {
        self.render(tmpl);
    }
}

impl<'a, G> Render for Content<'a, G>
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
          G::PageMeta: 'a
{
    fn render(&self, tmpl: &mut TemplateBuffer) {
        match self.0.content.format {
            "mkd"|"md"|"markdown"   => tmpl << ::Markdown::new(self.0.content.data, self.0.href),
            "html"                  => tmpl << raw!(self.0.content.data),
            ""|"text"|"txt"         => tmpl << self.0.content.data,
            format                  => tmpl.record_error(format!("unknown format '{}'", format)),
        }
    }
}

