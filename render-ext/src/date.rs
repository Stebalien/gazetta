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
use gazetta_core::prelude::*;
use gazetta_core::model::Date as DateModel;

pub struct Date<'a>(pub &'a DateModel);

impl<'a> RenderOnce for Date<'a> {
    fn render_once(self, tmpl: &mut TemplateBuffer) { self.render(tmpl) }
}

impl<'a> RenderMut for Date<'a> {
    fn render_mut(&mut self, tmpl: &mut TemplateBuffer) { self.render(tmpl) }
}

impl<'a> Render for Date<'a> {
    fn render(&self, tmpl: &mut TemplateBuffer) { 
        tmpl << html! {
            time(datetime=format_args!("{:04}-{:02}-{:02}",
                                       self.0.year(),
                                       self.0.month(),
                                       self.0.day())
                ) : format_args!("{:04}-{:02}-{:02}",
                                 self.0.year(),
                                 self.0.month(),
                                 self.0.day())
        }
    }
}

