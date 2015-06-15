#![feature(fs_walk, path_relative_from, collections_drain)]
// TODO: Better errors.
// Check url paths?

extern crate chrono;
extern crate yaml_rust;
extern crate pulldown_cmark;

#[macro_use]
extern crate horrorshow;

#[macro_use]
extern crate lazy_static;


pub type Date = ::chrono::naive::date::NaiveDate;

pub mod model;
pub mod view;
pub mod renderer;
pub mod markdown;

pub use model::Site;
pub use renderer::Renderer;

#[test]
fn it_works() {
    use renderer::DebugRenderer;
    DebugRenderer.render(&Site::new("data").unwrap(), "/tmp/outdir").unwrap();
}

