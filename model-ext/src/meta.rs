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

use gazetta_core::yaml::Hash;
use gazetta_core::model::Meta;
use link::Link;
use person::Person;
use util::BubbleResult;
use yaml::*;

pub struct SourceMeta {
    pub nav: Vec<Link>,
    pub author: Person,
}

impl Meta for SourceMeta {
    fn from_yaml(mut meta: Hash) -> Result<SourceMeta, &'static str> {
        Ok(SourceMeta {
            nav: try! {
                meta.remove(&NAV).map(Link::many_from_yaml).bubble_result()
            }.unwrap_or(Vec::new()),
            author: try! {
                try! {
                    meta.remove(&AUTHOR).map(Person::from_yaml).bubble_result()
                }.ok_or("websites must have authors")
            }
        })
    }
}

pub struct EntryMeta {
    pub author: Option<Person>,
    pub about: Option<Person>,
}

impl Meta for EntryMeta {
    fn from_yaml(mut meta: Hash) -> Result<EntryMeta, &'static str> {
        Ok(EntryMeta {
            author: try!(meta.remove(&AUTHOR).map(Person::from_yaml).bubble_result()),
            about: try!(meta.remove(&ABOUT).map(Person::from_yaml).bubble_result()),
        })
    }
}
