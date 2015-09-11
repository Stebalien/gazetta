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

//! TODO: Document.

extern crate chrono;
extern crate glob;
extern crate yaml_rust;
extern crate str_stack;
extern crate url;

#[macro_use]
extern crate horrorshow;

#[macro_use]
extern crate lazy_static;

#[macro_use]
pub mod error;
pub mod render;
pub mod model;
pub mod util;
pub mod yaml;
pub mod view;

pub mod prelude {
    pub use chrono::Datelike;
    pub use render::Gazetta;
}
