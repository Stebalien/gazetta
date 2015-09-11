/*  Copyright (C) 2015 Steven Allen
 *
 *  This file is part of gazetta.
 *
 *  This program is free software: you can redistribute it and/or modify it under the terms of the
 *  GNU General Public License as published by the Free Software Foundation version 3 of the
 *  License.
 *
 *  This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 *  without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See
 *  the GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License along with this program.  If
 *  not, see <http://www.gnu.org/licenses/>.
 */

use std::path::PathBuf;

use glob;
use error::SourceError;

use super::{Meta, Date};
use super::index::{self, Index};
use super::yaml::{self, Yaml};

/// An entry in the website.
///
/// Note: *Pages* do not correspond one-to-one to Entries (due to pagination).
#[derive(Debug, Clone)]
pub struct Entry<EntryMeta> where EntryMeta: Meta {
    /// The entry's title.
    ///
    /// This is separate from the general metadata for sorting and linking. All entries should have
    /// titles.
    pub title: String,

    /// The entry's date.
    /// 
    /// This is separate from the general metadata for sorting. Many entries will have dates.
    pub date: Option<Date>,

    /// The entries index options (if specified).
    pub index: Option<Index>,

    /// Indices into which this entry should be linked.
    pub cc: Vec<String>,

    /// Extra metadata.
    pub meta: EntryMeta,

    /// The entry name.
    pub name: String,

    /// The content
    pub content: String,

    /// Content format
    pub format: String, // TODO: Use atoms (intern).
}

impl<EntryMeta> Entry<EntryMeta> where EntryMeta: Meta {

    pub fn from_file(full_path: PathBuf, name: &str) -> Result<Self, SourceError> {
        // Helpers

        const U32_MAX_AS_I64: i64 = ::std::u32::MAX as i64;

        fn dir_to_glob(mut dir: String) -> Result<glob::Pattern, SourceError> {
            if !dir.ends_with("/") {
                dir.push('/');
            }
            dir.push('*');
            Ok(try!(glob::Pattern::new(&dir)))
        }

        fn name_to_glob(name: &str) -> glob::Pattern {
            let mut s = glob::Pattern::escape(&name);
            s.push_str("/*");
            glob::Pattern::new(&s).unwrap()
        }

        // Load metadata

        let (mut meta, content) = try!(yaml::load_front(&full_path));

        Ok(Entry {
            content: content,
            format: full_path.extension().and_then(|e|e.to_str()).unwrap_or("").to_owned(),
            title: match meta.remove(&yaml::TITLE) {
                Some(Yaml::String(title)) => title,
                Some(..) => return Err("titles must be strings".into()),
                None => return Err("entries must have titles".into()),
            },
            date: match meta.remove(&yaml::DATE) {
                Some(Yaml::String(date)) => match Date::parse_from_str(&date, "%Y-%m-%d") {
                    Ok(date) => Some(date),
                    Err(_) => return Err("invalid date format".into()),
                },
                Some(..) => return Err("date must be a string".into()),
                None => None,
            },
            index: match meta.remove(&yaml::INDEX) {
                Some(Yaml::Boolean(b)) => if b {
                    Some(Index {
                        paginate: None,
                        max: None,
                        sort: index::Sort::default(),
                        directories: vec![name_to_glob(&name)],
                    })
                } else {
                    None
                },
                Some(Yaml::String(dir)) => Some(Index {
                    paginate: None,
                    max: None,
                    sort: index::Sort::default(),
                    directories: vec![try!(dir_to_glob(dir))],
                }),
                Some(Yaml::Array(array)) => Some(Index {
                    paginate: None,
                    max: None,
                    sort: index::Sort::default(),
                    directories: try!(array.into_iter().map(|i| match i {
                        Yaml::String(dir) => dir_to_glob(dir),
                        _ => Err(SourceError::from("index directories must be strings")),
                    }).collect())
                }),
                Some(Yaml::Hash(mut index)) => Some(Index {
                    paginate: match index.remove(&yaml::PAGINATE) {
                        Some(Yaml::Integer(i @ 1...U32_MAX_AS_I64)) => Some(i as u32),
                        Some(Yaml::Boolean(false)) | None => None,
                        Some(..) => return Err("invalid pagination setting".into()),
                    },
                    max: match index.remove(&yaml::MAX) {
                        Some(Yaml::Integer(i @ 1...U32_MAX_AS_I64)) => Some(i as u32),
                        Some(Yaml::Boolean(false)) | None => None,
                        Some(..) => return Err("invalid max setting".into()),
                    },
                    sort: match index.remove(&yaml::SORT) {
                        Some(Yaml::String(key)) => {
                            let (dir, key) = if key.starts_with("+") {
                                (index::SortDirection::Ascending, &key[1..])
                            } else if key.starts_with("-") {
                                (index::SortDirection::Descending, &key[1..])
                            } else {
                                (index::SortDirection::default(), &key[..])
                            };
                            index::Sort {
                                direction: dir,
                                field: match key {
                                    "date" => index::SortField::Date,
                                    "title" => index::SortField::Title,
                                    "default" => index::SortField::default(),
                                    _ => return Err("invalid sort value".into()),
                                }
                            }
                        },
                        Some(..) => return Err("invalid sort value".into()),
                        None => index::Sort::default(),
                    },
                    directories: match index.remove(&yaml::SORT) {
                        Some(Yaml::Array(array)) => try!(array.into_iter().map(|i| match i {
                            Yaml::String(dir) => dir_to_glob(dir),
                            _ => Err(SourceError::from("index directories must be strings")),
                        }).collect()),
                        Some(Yaml::String(dir)) => vec![try!(dir_to_glob(dir))],
                        Some(..) => return Err("invalid directory list in index".into()),
                        None => vec![name_to_glob(name)],
                    }
                }),
                Some(..) => return Err("invalid index value".into()),
                None => None,
            },
            cc: match meta.remove(&yaml::CC) {
                Some(Yaml::String(cc)) => vec![cc],
                Some(Yaml::Array(cc)) => try!(cc.into_iter().map(|v| match v {
                    Yaml::String(ci) => Ok(ci),
                    _ => Err(SourceError::from("invlaid cc value")),
                }).collect()),
                Some(..) => return Err("invalid cc value".into()),
                None => Vec::new(),
            },
            meta: try!(EntryMeta::from_yaml(meta)),
            name: name.to_owned(),
        })
    }
}

/// A static entry.
///
/// Represents a static file/directory to be deployed.
#[derive(Debug, Clone)]
pub struct StaticEntry {
    pub name: String,
    pub source: PathBuf,
}
