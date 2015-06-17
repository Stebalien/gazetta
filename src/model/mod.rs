pub type Date = ::chrono::naive::date::NaiveDate;

pub mod index;

mod meta;
mod error;
mod entry;
mod person;
mod source;
mod yaml_keys;

pub use self::error::Error;
pub use self::source::Source;
pub use self::entry::Entry;
pub use self::person::Person;
pub use self::meta::Meta;
