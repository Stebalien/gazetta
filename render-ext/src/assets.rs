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
use gazetta_core::view::Site;
use gazetta_core::render::Gazetta;

/// Renders common head tags for a site and page.
pub struct Assets<'a, G>(pub &'a Site<'a, G>)
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
          G::PageMeta: 'a;

impl<'a, G> RenderOnce for Assets<'a, G>
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
          G::PageMeta: 'a
{
    fn render_once(self, tmpl: &mut TemplateBuffer) { self.render(tmpl) }
}

impl<'a, G> RenderMut for Assets<'a, G>
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
          G::PageMeta: 'a
{
    fn render_mut(&mut self, tmpl: &mut TemplateBuffer) { self.render(tmpl) }
}

impl<'a, G> Render for Assets<'a, G>
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
          G::PageMeta: 'a
{
    fn render(&self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            base(href=self.0.prefix);
            @ if let Some(css) = self.0.stylesheets {
                link(rel="stylesheet", href=css);
            }
            @ if let Some(js) = self.0.javascript {
                script(async, src=js) {}
            }
            @ if let Some(icon) = self.0.icon {
                link(rel="shortcut icon", href=icon);
            }
        };
    }
}

