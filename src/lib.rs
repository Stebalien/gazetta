#![feature(fs_walk, dir_entry_ext, path_relative_from, collections_drain)]
// TODO: Better errors.
// Check url paths?

extern crate chrono;
extern crate yaml_rust;

#[macro_use]
extern crate lazy_static;


pub type Date = ::chrono::naive::date::NaiveDate;

pub mod model;
pub mod view;
pub mod renderer;
pub mod builder;

pub use model::Site;
pub use renderer::Renderer;
pub use builder::SiteBuilder;

#[test]
fn it_works() {
    use builder::DebugSiteBuilder;
    use renderer::DebugRenderer;
    DebugRenderer.render(&Site::new("data").unwrap(), &mut DebugSiteBuilder::new()).unwrap();
}

