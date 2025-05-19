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

//! This is the API documentation for gazetta. If you just want to use gazetta, see the
//! [homepage](http://stebalien.com/projects/gazetta).

#![allow(clippy::redundant_field_names)]

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

pub mod cli {
    // Because rust can be stupid sometimes...
    // We can't just re-export private modules.
    pub use gazetta_cli::*;
}

#[doc(no_inline)]
pub use crate::error::{AnnotatedError, RenderError, SourceError};
#[doc(no_inline)]
pub use crate::model::{EntryMeta, Meta, Source, SourceMeta};
#[doc(no_inline)]
pub use crate::render::Gazetta;
#[doc(no_inline)]
pub use crate::view::{Context, Page, Site};
