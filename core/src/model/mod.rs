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

pub type DateTime = ::chrono::DateTime<Utc>;

pub mod index;

mod entry;
mod meta;
mod source;
mod yaml;

use chrono::Utc;

pub use self::entry::{Entry, StaticEntry};
pub use self::meta::Meta;
pub use self::source::{IndexedSource, Source};
