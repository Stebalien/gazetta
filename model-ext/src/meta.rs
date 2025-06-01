//  Copyright (C) 2015 Steven Allen
//
//  This file is part of gazetta.
//
//  This program is free software: you can redistribute it and/or modify it under the terms of the
//  GNU General Public License as published by the Free Software Foundation version 3 of the
//  License.
//
//  This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
//  without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See
//  the GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License along with this program.  If
//  not, see <http://www.gnu.org/licenses/>.
//

use gazetta_core::model::Meta;
use gazetta_core::yaml::{Hash, Yaml};

use crate::link::Link;
use crate::person::Person;
use crate::yaml;

pub struct SourceMeta {
    pub nav: Vec<Link>,
    pub author: Person,
}

impl Meta for SourceMeta {
    fn from_yaml(mut meta: Hash) -> Result<SourceMeta, &'static str> {
        Ok(SourceMeta {
            nav: meta
                .remove(&yaml::KEYS.nav)
                .map(Link::many_from_yaml)
                .transpose()?
                .unwrap_or_else(Vec::new),
            author: meta
                .remove(&yaml::KEYS.author)
                .map(Person::from_yaml)
                .transpose()?
                .ok_or("websites must have authors")?,
        })
    }
}

pub struct EntryMeta {
    pub author: Option<Person>,
    pub about: Option<Person>,
    pub robots: Option<Vec<String>>,
}

impl Meta for EntryMeta {
    fn from_yaml(mut meta: Hash) -> Result<EntryMeta, &'static str> {
        Ok(EntryMeta {
            author: meta
                .remove(&yaml::KEYS.author)
                .map(Person::from_yaml)
                .transpose()?,
            about: meta
                .remove(&yaml::KEYS.about)
                .map(Person::from_yaml)
                .transpose()?,
            robots: meta
                .remove(&yaml::KEYS.robots)
                .map(|r| match r {
                    Yaml::String(val) => Ok(vec![val]),
                    Yaml::Array(arr) => Ok(arr
                        .into_iter()
                        .map(|l| l.into_string().ok_or("robots directives must be strings"))
                        .collect::<Result<Vec<_>, _>>()?),
                    _ => Err("robots directives must be strings"),
                })
                .transpose()?,
        })
    }
}
