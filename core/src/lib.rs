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

//! Gazetta is a static site generator and this crate contains the core functionality.
//!
//! If you just want to use gazetta, see [the project page][project].
//!
//! If you want to extend gazetta, you should fork [gazetta-bin][] and then read the
//! [gazetta][] documentation. [gazetta][] contains some extra abstractions for generating static
//! websites but aren't absolutely essential (so they aren't built-in to gazetta-core).
//!
//! [gazetta-bin]: https://github.com/Stebalien/gazetta-bin
//! [project]: http://stebalien.com/projects/gazetta
//! [gazetta]: http://github.com/Stebalien/gazetta

#[macro_use]
pub mod error;
pub mod model;
pub mod render;
pub mod util;
pub mod view;
pub mod yaml;

pub mod prelude {
    pub use chrono::Datelike;

    pub use crate::render::Gazetta;
}
