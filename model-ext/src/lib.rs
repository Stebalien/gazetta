/* Copyright (2015) Stevem Allen
 *
 * This file is part of gazetta.
 * 
 * gazetta is free software: you can redistribute it and/or modify it under the terms of the
 * GNU Affero General Public License (version 3) as published by the Free Software Foundation.
 * 
 * gazetta is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
 * the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU Affero General Public License along with gazetta.  If
 * not, see <http://www.gnu.org/licenses/>.
 */

#[macro_use]
extern crate lazy_static;
extern crate gazetta_core;

mod util;
mod person;
mod link;
mod meta;
mod yaml;

pub use meta::{SourceMeta, EntryMeta};
pub use person::{Person, Key};
pub use link::Link;
