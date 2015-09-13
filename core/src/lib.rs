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

//! Gazetta is a static site generator and this crate contains the core functionality.
//!
//! If you just want to use gazetta, see [the project page][project].
//!
//! If you want to extend gazetta, you should fork [gazetta-bin][] and then read the
//! [gazetta][] documentation. [gazetta][] contains some extra abstractions for generating static
//! websites but aren't absolutely essential (so they aren't built-in to gazetta-core).
//!
//!
//! # Design
//!
//! Gazetta's data flow is as follows:
//!
//!     Input → Models (Entries) → Views (Pages) → Output
//!
//! TODO: Actually fill this in.
//!
//! [gazetta-bin]: https://github.com/Stebalien/gazetta-bin
//! [project]: http://stebalien.com/projects/gazetta
//! [gazetta]: http://github.com/Stebalien/gazetta

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
