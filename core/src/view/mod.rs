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

use crate::render::Gazetta;

mod index;
mod page;
mod site;

pub use self::index::{Index, Paginate};
pub use self::page::{Content, Page};
pub use self::site::Site;

pub struct Context<'a, G>
where
    G: Gazetta,
{
    pub page: &'a Page<'a, G>,
    pub site: &'a Site<'a, G>,
}

impl<'a, G> Copy for Context<'a, G>
where
    G: Gazetta + 'a,
    G::PageMeta: 'a,
    G::SiteMeta: 'a,
{
}

impl<'a, G> Clone for Context<'a, G>
where
    G: Gazetta + 'a,
    G::PageMeta: 'a,
    G::SiteMeta: 'a,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, G> fmt::Debug for Context<'a, G>
where
    G: Gazetta + 'a,
    G::PageMeta: fmt::Debug + 'a,
    G::SiteMeta: fmt::Debug + 'a,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Context")
            .field("site", &self.site)
            .field("page", &self.page)
            .finish()
    }
}
