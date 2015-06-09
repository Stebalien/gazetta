use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Person {
    pub name: String,
    pub email: Option<String>,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.name));
        if let Some(ref email) = self.email {
            try!(write!(f, " <{}>", email));
        }
        Ok(())
    }
}


