use yaml::*;

#[derive(Clone, Debug)]
pub struct Link {
    pub text: String,
    pub url: String
}

impl Link {
    pub fn from_yaml(yaml: Yaml) -> Result<Self, &'static str> {
        match yaml {
            Yaml::Hash(link) => {
                if link.len() != 1 {
                    Err("links must have exactly one entry".into())
                } else {
                    match link.into_iter().next().unwrap() {
                        (Yaml::String(k), Yaml::String(v)) => Ok(Link {
                            text: k,
                            url: v,
                        }),
                        _ => Err("links must be in the form `name: url`"),
                    }
                }
            },
            _ => Err("links must be in the form `name: url`")
        }
    }

    pub fn many_from_yaml(links: Yaml) -> Result<Vec<Self>, &'static str> {
        Ok(match links {
            Yaml::Array(links) => try!(links.into_iter().map(Link::from_yaml).collect()),
            _ => return Err("lists of links need to be arrays"),
        })
    }

}

