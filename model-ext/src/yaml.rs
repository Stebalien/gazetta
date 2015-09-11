pub use gazetta_core::yaml::Yaml;

lazy_static! {
    pub static ref NAV: Yaml = Yaml::String(String::from("nav"));
    pub static ref NAME: Yaml = Yaml::String("name".into());
    pub static ref ABOUT: Yaml = Yaml::String(String::from("about"));
    pub static ref AUTHOR: Yaml = Yaml::String("author".into());
    pub static ref EMAIL: Yaml = Yaml::String("email".into());
    pub static ref KEY: Yaml = Yaml::String("key".into());
    pub static ref URL: Yaml = Yaml::String("url".into());
    pub static ref FINGERPRINT: Yaml = Yaml::String("fingerprint".into());
    pub static ref NICKNAMES: Yaml = Yaml::String("nicknames".into());
    pub static ref PHOTO: Yaml = Yaml::String("photo".into());
    pub static ref ALSO: Yaml = Yaml::String("also".into());
}
