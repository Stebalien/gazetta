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

use std::usize;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::path::Path;
use yaml_rust::{YamlLoader, yaml};

use error::SourceError;

pub use yaml::*;

lazy_static! {
    pub static ref TITLE: Yaml = Yaml::String("title".into());
    pub static ref INDEX: Yaml = Yaml::String("index".into());
    pub static ref DATE: Yaml = Yaml::String("date".into());
    pub static ref SORT: Yaml = Yaml::String("sort".into());
    pub static ref PAGINATE: Yaml = Yaml::String("paginate".into());
    pub static ref CC: Yaml = Yaml::String("cc".into());
    pub static ref MAX: Yaml = Yaml::String("max".into());
    pub static ref BASE: Yaml = Yaml::String("base".into());
    pub static ref COMPACT: Yaml = Yaml::String("compact".into());
    pub static ref DESCRIPTION: Yaml = Yaml::String("description".into());
}

pub fn load_front<P: AsRef<Path>>(path: P) -> Result<(yaml::Hash, String), SourceError> {
    _load_front(path.as_ref())
}

fn _load_front(path: &Path) -> Result<(yaml::Hash, String), SourceError> {
    let mut buf = String::with_capacity(100);
    let mut file = BufReader::new(File::open(path)?);
    file.read_line(&mut buf)?;
    if buf.trim_right() == "---" {
        // Parse yaml header.
        loop {
            match file.read_line(&mut buf) {
                Ok(len) => {
                    match buf[buf.len() - len..].trim_right() {
                        "..." => break,
                        "---" => {
                            let end = buf.len() - len;
                            // Allow jekyll style metadata.
                            buf.truncate(end);
                            buf.push_str("...\n");
                            break;
                        }
                        _ => (),
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e.into()),
            }
        }
    } else {
        return Ok((yaml::Hash::new(), String::new()));
    }
    // Parse yaml
    let mut docs = YamlLoader::load_from_str(&buf)?;
    docs.truncate(1);
    let meta = match docs.pop() {
        Some(Yaml::Hash(data)) => data,
        _ => return Err("Yaml must be a key -> value mapping".into()),
    };

    buf.clear();
    file.read_to_string(&mut buf)?;
    Ok((meta, buf))
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<yaml::Hash, SourceError> {
    _load(path.as_ref())
}
fn _load(path: &Path) -> Result<yaml::Hash, SourceError> {
    let mut file = File::open(path)?;
    let mut buf = String::with_capacity(file.metadata().ok().map_or(100, |m| {
        let len = m.len();
        if len > usize::MAX as u64 {
            usize::MAX
        } else {
            len as usize
        }
    }));
    file.read_to_string(&mut buf)?;
    let mut docs = YamlLoader::load_from_str(&buf)?;
    docs.truncate(1);
    match docs.pop() {
        Some(Yaml::Hash(data)) => Ok(data),
        _ => Err("Yaml must be a key -> value mapping".into()),
    }
}
