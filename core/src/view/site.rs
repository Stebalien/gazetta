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

use std::fmt;
use std::ops::Deref;

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
    /// The path prefix at wich we're serving this website.
    pub prefix: &'a str,
    /// The concatinated stylesheets.
    pub stylesheets: Option<&'a str>,
    /// The concatinated javascript.
    pub javascript: Option<&'a str>,
    /// The icon.
    pub icon: Option<&'a str>,
    /// Extra metadata specified in the Source.
    pub meta: &'a G::SiteMeta,
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
