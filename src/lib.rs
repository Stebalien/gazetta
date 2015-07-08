/* Copyright (2015) Stevem Allen
 *
 * This file is part of gazetta.
 * 
 * gazetta-bin is free software: you can redistribute it and/or modify it under the terms of the
 * GNU Affero General Public License (version 3) as published by the Free Software Foundation.
 * 
 * Foobar is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
 * the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU Affero General Public License along with Foobar.  If
 * not, see <http://www.gnu.org/licenses/>.
 */

//! TODO: Document.

extern crate chrono;
extern crate glob;
extern crate yaml_rust;
extern crate pulldown_cmark;

#[macro_use]
extern crate horrorshow;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod error;

mod render;

pub mod model;
pub mod util;
pub mod yaml;
pub mod view;
pub mod markdown;

pub mod prelude {
    pub use chrono::Datelike;
    pub use render::Gazetta;
}

pub use render::Gazetta;
pub use view::{Page, Site};
pub use model::{Source, Meta};
pub use error::{AnnotatedError, SourceError, RenderError};
