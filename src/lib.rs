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

extern crate gazetta_core;
extern crate gazetta_cli;
extern crate gazetta_render_ext;
extern crate gazetta_model_ext;

pub mod view {
    pub use gazetta_core::view::*;
}

pub mod error {
    pub use gazetta_core::error::*;
}

pub mod model {
    pub use gazetta_core::model::*;
    pub use gazetta_model_ext::*;
}

pub mod render {
    pub use gazetta_core::render::*;
    pub use gazetta_render_ext::*;
}

pub mod prelude {
    pub use gazetta_core::prelude::*;
}

pub use gazetta_cli as cli;

#[doc(no_inline)]
pub use render::Gazetta;
#[doc(no_inline)]
pub use view::{Page, Site};
#[doc(no_inline)]
pub use model::{Source, Meta, SourceMeta, EntryMeta};
#[doc(no_inline)]
pub use error::{AnnotatedError, SourceError, RenderError};

