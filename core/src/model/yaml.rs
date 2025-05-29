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

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;
use std::sync::LazyLock;

use yaml_rust::{yaml, YamlLoader};

use crate::error::SourceError;

pub use crate::yaml::*;

pub static KEYS: LazyLock<YamlKeys> = LazyLock::new(|| YamlKeys {
    title: Yaml::String("title".into()),
    index: Yaml::String("index".into()),
    date: Yaml::String("date".into()),
    updated: Yaml::String("updated".into()),
    sort: Yaml::String("sort".into()),
    directories: Yaml::String("directories".into()),
    paginate: Yaml::String("paginate".into()),
    cc: Yaml::String("cc".into()),
    max: Yaml::String("max".into()),
    base: Yaml::String("base".into()),
    compact: Yaml::String("compact".into()),
    description: Yaml::String("description".into()),
    syndicate: Yaml::String("syndicate".into()),
});

pub struct YamlKeys {
    pub title: Yaml,
    pub index: Yaml,
    pub date: Yaml,
    pub updated: Yaml,
    pub sort: Yaml,
    pub directories: Yaml,
    pub paginate: Yaml,
    pub cc: Yaml,
    pub max: Yaml,
    pub base: Yaml,
    pub compact: Yaml,
    pub description: Yaml,
    pub syndicate: Yaml,
}

pub fn load_front(file: impl io::Read) -> Result<(yaml::Hash, String), SourceError> {
    let mut buf = String::with_capacity(100);
    let mut file = BufReader::new(file);
    file.read_line(&mut buf)?;
    if buf.trim_end() == "---" {
        // Parse yaml header.
        loop {
            match file.read_line(&mut buf) {
                Ok(len) => {
                    match buf[buf.len() - len..].trim_end() {
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
