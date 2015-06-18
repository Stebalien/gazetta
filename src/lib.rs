//! TODO: Document.

#![feature(path_relative_from)]

extern crate chrono;
extern crate glob;
extern crate yaml_rust;
extern crate pulldown_cmark;

#[macro_use]
extern crate horrorshow;

#[macro_use]
extern crate lazy_static;

mod error;
mod render;

pub mod prelude {
    pub use chrono::Datelike;
    pub use render::Gazetta;
}

pub use view::{Page, Site};
pub use render::Gazetta;
pub use model::{Source, Entry, Meta};
pub use error::{AnnotatedError, SourceError, RenderError};

pub mod model;
pub mod yaml;
pub mod view;
pub mod markdown;
