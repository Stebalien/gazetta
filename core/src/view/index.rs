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

use std::fmt;

use ::render::Gazetta;

use super::Page;

/// Page pagination information.
#[derive(Copy, Clone, Debug)]
pub struct Paginate<'a> {
    /// Index of current page.
    pub current: usize,
    /// The list of pages (links only) in this pagination.
    pub pages: &'a [&'a str],
}

/// Page index information
pub struct Index<'a, G>
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
          G::PageMeta: 'a
{
    /// Pages to be indexed.
    pub entries: &'a [Page<'a, G>],
    /// Pagination information (if any).
    pub paginate: Option<Paginate<'a>>,
}

// Implement these manually. Derive requires that G: Trait.

impl<'a, G> Copy for Index<'a, G>
    where G: Gazetta + 'a,
          G::PageMeta: 'a,
          G::SiteMeta: 'a
{}

impl<'a, G> Clone for Index<'a, G>
    where G: Gazetta + 'a,
          G::PageMeta: 'a,
          G::SiteMeta: 'a,
{
    fn clone(&self) -> Self {
        *self
    }
}

// Manually implement because rust isn't correctly adding the Debug constraint when deriving.
impl<'a, G> fmt::Debug for Index<'a, G>
    where G: Gazetta + 'a,
          G::PageMeta: fmt::Debug + 'a,
          G::SiteMeta: 'a,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Index")
            .field("entries", &self.entries)
            .field("paginate", &self.paginate)
            .finish()
    }
}

