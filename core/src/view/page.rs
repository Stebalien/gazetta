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

use crate::model::DateTime;
use crate::model::Entry;
use crate::render::Gazetta;

use super::Index;

/// Represents an indevidual page to be rendered.
pub struct Page<'a, G>
where
    G: Gazetta + 'a,
    G::PageMeta: 'a,
    G::SiteMeta: 'a,
{
    /// The page's title.
    pub title: &'a str,

    /// An optional description of the page.
    pub description: Option<&'a str>,

    /// The date the page was created (specified in the metadata).
    pub date: Option<&'a DateTime>,

    /// The date the page was last modified (derived from the file metadata and used for
    /// syndication).
    pub updated: &'a DateTime,

    /// The page's location, relative to the site's base.
    pub href: &'a str,

    /// The index contained in this page, if any.
    pub index: Option<Index<'a, G>>,

    /// Extra metadata specified in the Entry.
    pub meta: &'a G::PageMeta,

    /// The page's content.
    ///
    /// If you want to use the default renderer, just render the page itself.
    ///
    /// ```norun
    /// html! {
    ///     div(id="content") : page;
    /// }
    /// ```
    pub content: Content<'a>,
}

impl<'a, G> Page<'a, G>
where
    G: Gazetta + 'a,
    G::PageMeta: 'a,
    G::SiteMeta: 'a,
{
    /// Creates a page for an entry. This does *not* fill in the index.
    pub fn for_entry(entry: &'a Entry<G::PageMeta>) -> Self {
        Page {
            title: &entry.title,
            date: entry.date.as_ref(),
            updated: &entry.updated,
            description: entry.description.as_deref(),
            content: Content {
                data: &entry.content,
                format: &entry.format,
            },
            href: &entry.name,
            index: None,
            meta: &entry.meta,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Content<'a> {
    pub data: &'a str,
    pub format: &'a str,
}

// Implement these manually. Derive requires that G: Trait.

impl<'a, G> Copy for Page<'a, G>
where
    G: Gazetta + 'a,
    G::PageMeta: 'a,
    G::SiteMeta: 'a,
{
}

impl<'a, G> Clone for Page<'a, G>
where
    G: Gazetta + 'a,
    G::PageMeta: 'a,
    G::SiteMeta: 'a,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, G> fmt::Debug for Page<'a, G>
where
    G: Gazetta + 'a,
    G::PageMeta: fmt::Debug + 'a,
    G::SiteMeta: 'a,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Page")
            .field("title", &self.title)
            .field("date", &self.date)
            .field("href", &self.href)
            .field("index", &self.index)
            .field("meta", &self.meta)
            .field("content", &self.content)
            .finish()
    }
}

impl<'a, G> Deref for Page<'a, G>
where
    G: Gazetta + 'a,
    G::PageMeta: 'a,
    G::SiteMeta: 'a,
{
    type Target = G::PageMeta;
    fn deref(&self) -> &Self::Target {
        self.meta
    }
}
