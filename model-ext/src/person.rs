/* Copyright (2015) Stevem Allen
 *
 * This file is part of gazetta.
 * 
 * gazetta is free software: you can redistribute it and/or modify it under the terms of the
 * GNU Affero General Public License (version 3) as published by the Free Software Foundation.
 * 
 * gazetta is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
 * the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU Affero General Public License along with gazetta.  If
 * not, see <http://www.gnu.org/licenses/>.
 */

use std::fmt;
use link::Link;
use util::BubbleResult;
use yaml::*;

#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub email: Option<String>,
    pub photo: Option<String>,
    pub key: Option<Key>,
    pub nicknames: Vec<String>,
    pub also: Vec<Link>,
}

#[derive(Debug, Clone)]
pub struct Key {
    pub url: String,
    pub fingerprint: String,
}

impl Key {
    pub fn from_yaml(key: Yaml) -> Result<Self, &'static str> {
        match key {
            Yaml::Hash(mut key) => Ok(Key {
                url: match key.remove(&URL) {
                    Some(Yaml::String(url)) => url,
                    Some(..) => return Err("key url must be a string"),
                    None => return Err("key url missing"),
                },
                fingerprint: match key.remove(&FINGERPRINT) {
                    Some(Yaml::String(fprint)) => fprint,
                    Some(..) => return Err("key fingerprint must be a string"),
                    None => return Err("key fingerprint missing"),
                },
            }),
            _ => Err("if specified, key must be a hash"),
        }
    }
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

impl Person {
    pub fn from_yaml(person: Yaml) -> Result<Self, &'static str> {
        Ok(match person {
            Yaml::Hash(mut person) => Person {
                name: match person.remove(&NAME) {
                    Some(Yaml::String(name)) => name,
                    None => return Err("missing name"),
                    _ => return Err("name must be a string"),
                },
                photo: match person.remove(&PHOTO) {
                    Some(Yaml::String(photo)) => Some(photo),
                    None => None,
                    _ => return Err("if specified, photo must be a string"),
                },
                email: match person.remove(&EMAIL) {
                    Some(Yaml::String(email)) => Some(email),
                    None => None,
                    _ => return Err("if specified, email must be a string"),
                },
                nicknames: match person.remove(&NICKNAMES) {
                    Some(Yaml::String(nick)) => vec![nick],
                    Some(Yaml::Array(nicks)) => try!(nicks.into_iter().map(|nick| match nick {
                        Yaml::String(nick) => Ok(nick),
                        _ => Err("nicknames must be strings"),
                    }).collect()),
                    Some(..) => return Err("invalid nicknames value"),
                    None => vec![],
                },
                also: try!(person.remove(&ALSO)
                           .map(Link::many_from_yaml)
                           .bubble_result())
                    .unwrap_or(Vec::new()),
                key: try!(person.remove(&KEY).map(Key::from_yaml).bubble_result()),
            },
            Yaml::String(name) => Person {
                name: name,
                email: None,
                key: None,
                also: Vec::new(),
                photo: None,
                nicknames: Vec::new(),
            },
            _ => return Err("invalid person"),
        })
    }
}
