use ::yaml::Yaml;

lazy_static! {
    pub static ref AUTHOR: Yaml = Yaml::String("author".into());
    pub static ref TITLE: Yaml = Yaml::String("title".into());
    pub static ref NAME: Yaml = Yaml::String("name".into());
    pub static ref INDEX: Yaml = Yaml::String("index".into());
    pub static ref EMAIL: Yaml = Yaml::String("email".into());
    pub static ref DATE: Yaml = Yaml::String("date".into());
    pub static ref SORT: Yaml = Yaml::String("sort".into());
    pub static ref PAGINATE: Yaml = Yaml::String("paginate".into());
    pub static ref CC: Yaml = Yaml::String("cc".into());
    pub static ref MAX: Yaml = Yaml::String("max".into());
}
