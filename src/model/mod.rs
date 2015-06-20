pub type Date = ::chrono::naive::date::NaiveDate;

pub mod index;

mod meta;
mod entry;
mod source;
mod yaml;

pub use self::source::Source;
pub use self::entry::{Entry, StaticEntry, ContentSource};
pub use self::meta::Meta;
