use std::usize;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::path::Path;
use std::collections::BTreeMap;

use ::yaml_rust::YamlLoader;

use ::model::Error;

// Re-export
pub use yaml_rust::{Yaml, ScanError};
pub use yaml_rust::yaml::{Hash, Array};

pub fn load_front<P: AsRef<Path>>(path: P) -> Result<(usize, BTreeMap<Yaml, Yaml>), Error> {
    let mut buf = String::with_capacity(100);
    let mut file = BufReader::new(try!(File::open(path.as_ref())));
    let mut offset = 0;
    offset += try!(file.read_line(&mut buf));
    if buf.trim_right() == "---" {
        // Parse yaml header.
        loop {
            match file.read_line(&mut buf) {
                Ok(len) => {
                    offset += len;

                    if buf[buf.len()-len..].trim_right() == "..." {
                        break;
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

pub fn load<P: AsRef<Path>>(path: P) -> Result<BTreeMap<Yaml, Yaml>, Error> {
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
