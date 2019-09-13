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
use crate::yaml::*;

#[derive(Clone, Debug)]
pub struct Link {
    pub text: String,
    pub url: String,
}

impl Link {
    pub fn from_yaml(yaml: Yaml) -> Result<Self, &'static str> {
        match yaml {
            Yaml::Hash(link) => {
                if link.len() != 1 {
                    Err("links must have exactly one entry")
                } else {
                    match link.into_iter().next().unwrap() {
                        (Yaml::String(k), Yaml::String(v)) => Ok(Link { text: k, url: v }),
                        _ => Err("links must be in the form `name: url`"),
                    }
                }
            }
            _ => Err("links must be in the form `name: url`"),
        }
    }

    pub fn many_from_yaml(links: Yaml) -> Result<Vec<Self>, &'static str> {
        Ok(match links {
            Yaml::Array(links) => links
                .into_iter()
                .map(Link::from_yaml)
                .collect::<Result<_, _>>()?,
            _ => return Err("lists of links need to be arrays"),
        })
    }
}
