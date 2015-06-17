use ::yaml;

pub trait Meta {
    fn from_yaml(yaml: yaml::Hash) -> Result<Self, &'static str>;
}

impl Meta for yaml::Hash {
    fn from_yaml(yaml: yaml::Hash) -> Result<Self, &'static str> {
        Ok(yaml)
    }
}

impl Meta for () {
    fn from_yaml(_: yaml::Hash) -> Result<Self, &'static str> {
        Ok(())
    }
}
