/* Copyright (2015) Stevem Allen
 *
 * This file is part of gazetta.
 * 
 * gazetta-bin is free software: you can redistribute it and/or modify it under the terms of the
 * GNU Affero General Public License (version 3) as published by the Free Software Foundation.
 * 
 * Foobar is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
 * the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU Affero General Public License along with Foobar.  If
 * not, see <http://www.gnu.org/licenses/>.
 */

use std::usize;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::path::Path;
use std::collections::BTreeMap;

use ::yaml_rust::YamlLoader;

use ::SourceError;

pub use ::yaml::*;

lazy_static! {
    pub static ref TITLE: Yaml = Yaml::String("title".into());
    pub static ref INDEX: Yaml = Yaml::String("index".into());
    pub static ref DATE: Yaml = Yaml::String("date".into());
    pub static ref SORT: Yaml = Yaml::String("sort".into());
    pub static ref PAGINATE: Yaml = Yaml::String("paginate".into());
    pub static ref CC: Yaml = Yaml::String("cc".into());
    pub static ref MAX: Yaml = Yaml::String("max".into());
    pub static ref BASE: Yaml = Yaml::String("base".into());
}

pub fn load_front<P: AsRef<Path>>(path: P) -> Result<(u64, BTreeMap<Yaml, Yaml>), SourceError> {
    let mut buf = String::with_capacity(100);
    let mut file = BufReader::new(try!(File::open(path.as_ref())));
    let mut offset: u64 = 0;
    offset += try!(file.read_line(&mut buf)) as u64;
    if buf.trim_right() == "---" {
        // Parse yaml header.
        loop {
            match file.read_line(&mut buf) {
                Ok(len) => {
                    offset += len as u64;

                    match buf[buf.len()-len..].trim_right() {
                        "..." => break,
                        "---" => {
                            let end = buf.len() - len;
                            // Allow jekyll style metadata.
                            buf.truncate(end);
                            buf.push_str("...\n");
                            break;
                        },
                        _ => ()
                    }
                },
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {},
                Err(e) => return Err(e.into()),
            }
        }
    } else {
        return Ok((0, BTreeMap::new()));
    }
    let mut docs = try!(YamlLoader::load_from_str(&buf));
    docs.truncate(1);
    match docs.pop() {
        Some(Yaml::Hash(data)) => Ok((offset, data)),
        _ => Err("Yaml must be a key -> value mapping".into()),
    }
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<BTreeMap<Yaml, Yaml>, SourceError> {
    let mut file = try!(File::open(path.as_ref()));
    let mut buf = String::with_capacity(file.metadata().ok().map_or(100, |m| {
        let len = m.len();
        if len > usize::MAX as u64 {
            usize::MAX
        } else {
            len as usize
        }
    }));
    try!(file.read_to_string(&mut buf));
    let mut docs = try!(YamlLoader::load_from_str(&buf));
    docs.truncate(1);
    match docs.pop() {
        Some(Yaml::Hash(data)) => Ok(data),
        _ => Err("Yaml must be a key -> value mapping".into()),
    }
}
