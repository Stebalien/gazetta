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

use std::fmt::{self, Display};
use std::ops::Deref;

use horrorshow::{Render, RenderMut, RenderOnce};

use crate::render::Gazetta;

/// A "website"
///
/// You should include this view at the top of your websites "head". It renders
/// into script, stylesheet, icon tags, metadata tags, and *importantly* the
/// base tag.
///
/// ```norun
/// html! {
///     html {
///         head {
///             : site;
///             // ...
///         }
///     }
/// }
/// ```
pub struct Site<'a, G>
where
    G: Gazetta + 'a,
    G::SiteMeta: 'a,
{
    /// The website's title
    pub title: &'a str,
    /// The "canonical" origin for the website. (i.e. the
    /// `<protocol>://<domain>:<port>` part of the url)
    pub origin: &'a str,
    /// The path prefix at which we're serving this website.
    pub prefix: &'a str,
    /// The concatenated stylesheets.
    pub stylesheets: Option<&'a str>,
    /// The concatenated javascript.
    pub javascript: Option<&'a str>,
    /// The icon.
    pub icon: Option<&'a str>,
    /// Extra metadata specified in the Source.
    pub meta: &'a G::SiteMeta,
}

impl<'a, G> Site<'a, G>
where
    G: Gazetta,
{
    pub fn base(&self) -> Base<'a> {
        Base {
            origin: self.origin,
            prefix: self.prefix,
        }
    }
}

impl<'a, G> Deref for Site<'a, G>
where
    G: Gazetta + 'a,
    G::SiteMeta: 'a,
{
    type Target = G::SiteMeta;
    fn deref(&self) -> &Self::Target {
        self.meta
    }
}

impl<'a, G> Copy for Site<'a, G>
where
    G: Gazetta + 'a,
    G::PageMeta: 'a,
    G::SiteMeta: 'a,
{
}

impl<'a, G> Clone for Site<'a, G>
where
    G: Gazetta + 'a,
    G::PageMeta: 'a,
    G::SiteMeta: 'a,
{
    fn clone(&self) -> Self {
        *self
    }
}

// Manually implement because rust isn't correctly adding the Debug constraint
// when deriving.
impl<'a, G> fmt::Debug for Site<'a, G>
where
    G: Gazetta + 'a,
    G::PageMeta: 'a,
    G::SiteMeta: fmt::Debug + 'a,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Site")
            .field("title", &self.title)
            .field("stylesheets", &self.stylesheets)
            .field("javascript", &self.javascript)
            .field("icon", &self.icon)
            .field("meta", &self.meta)
            .finish()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Base<'a> {
    origin: &'a str,
    prefix: &'a str,
}

impl<'a> Display for Base<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.origin)?;
        f.write_str(self.prefix)?;
        Ok(())
    }
}

impl Render for Base<'_> {
    fn render(&self, tmpl: &mut horrorshow::TemplateBuffer<'_>) {
        tmpl.write_fmt(format_args!("{}", self))
    }
}

impl RenderMut for Base<'_> {
    fn render_mut(&mut self, tmpl: &mut horrorshow::TemplateBuffer<'_>) {
        self.render(tmpl)
    }
}

impl RenderOnce for Base<'_> {
    fn render_once(self, tmpl: &mut horrorshow::TemplateBuffer<'_>)
    where
        Self: Sized,
    {
        self.render(tmpl)
    }

    fn size_hint(&self) -> usize {
        self.origin.len() + self.prefix.len()
    }
}
