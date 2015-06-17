#![feature(path_relative_from)]
// TODO: Better errors.
// Check url paths?

extern crate chrono;
extern crate glob;
extern crate yaml_rust;
extern crate pulldown_cmark;

#[macro_use]
extern crate horrorshow;

#[macro_use]
extern crate lazy_static;

pub mod prelude {
    pub use chrono::Datelike;
    pub use view::{Page, Site};
    pub use renderer::Renderer;
    pub use model::Source;
    pub use model::Meta;
}

pub use view::{Page, Site};
pub use renderer::Renderer;
pub use model::Source;
pub use model::Meta;

pub mod model;
pub mod yaml;
pub mod view;
pub mod renderer;
pub mod markdown;

#[test]
fn it_works() {
    use renderer::DebugRenderer;
    DebugRenderer.render(&Source::new("data").unwrap(), "/tmp/outdir").unwrap();
}
