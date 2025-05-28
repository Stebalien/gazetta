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
use std::path::{Path, PathBuf};

use chrono::FixedOffset;
use glob;

use crate::error::SourceError;

use super::index::{self, Index};
use super::yaml::{self, Yaml};
use super::{DateTime, Meta};

/// An entry in the website.
///
/// Note: *Pages* do not correspond one-to-one to Entries (due to pagination).
#[derive(Debug, Clone)]
pub struct Entry<EntryMeta>
where
    EntryMeta: Meta,
{
    /// The entry's title.
    ///
    /// This is separate from the general metadata for sorting and linking. All entries should have
    /// titles.
    pub title: String,

    /// The entry's description.
    ///
    /// If present, this will be included in compact indices.
    pub description: Option<String>,

    /// The date & time at which the entry was created.
    ///
    /// This is separate from the general metadata for sorting.
    pub date: Option<DateTime>,

    /// The entry's last modification time.
    ///
    /// All entries have this. Usually this should be specified in the entry's metadata, but it can
    /// also be derived from the associated file's metadata.
    pub updated: DateTime,

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

fn parse_datetime(date: &str) -> Result<DateTime, SourceError> {
    Ok(if date.contains([' ', 't', 'T']) {
        chrono::DateTime::<FixedOffset>::parse_from_rfc3339(date)
            .map_err(|e| format!("invalid date: {e}"))?
            .to_utc()
    } else {
        let date = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .map_err(|e| format!("invalid date: {e}"))?;
        let datetime = chrono::NaiveDateTime::new(date, chrono::NaiveTime::default());
        chrono::DateTime::from_naive_utc_and_offset(datetime, chrono::Utc)
    })
}

impl<EntryMeta> Entry<EntryMeta>
where
    EntryMeta: Meta,
{
    pub fn from_file<P>(full_path: P, name: &str) -> Result<Self, SourceError>
    where
        P: AsRef<Path>,
    {
        Entry::_from_file(full_path.as_ref(), name)
    }

    fn _from_file(full_path: &Path, name: &str) -> Result<Self, SourceError> {
        // Helpers
        const U32_MAX_AS_I64: i64 = u32::MAX as i64;

        fn dir_to_glob(mut dir: String) -> Result<glob::Pattern, SourceError> {
            if !dir.ends_with('/') {
                dir.push('/');
            }
            dir.push('*');
            glob::Pattern::new(&dir).map_err(From::from)
        }

        fn name_to_glob(name: &str) -> glob::Pattern {
            let mut s = glob::Pattern::escape(name);
            s.push_str("/*");
            glob::Pattern::new(&s).unwrap()
        }

        // Load metadata

        let entry_file = File::open(full_path)?;
        let (mut meta, content) = yaml::load_front(&entry_file)?;

        let format = full_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_owned();
        let title = match meta.remove(&yaml::TITLE) {
            Some(Yaml::String(title)) => title,
            Some(..) => return Err("titles must be strings".into()),
            None => return Err("entries must have titles".into()),
        };
        let description = match meta.remove(&yaml::DESCRIPTION) {
            Some(Yaml::String(desc)) => Some(desc),
            None => None,
            Some(..) => return Err("invalid description type".into()),
        };
        let date = match meta.remove(&yaml::DATE) {
            Some(Yaml::String(date)) => Some(parse_datetime(&date)?),
            Some(..) => return Err("date must be a string".into()),
            None => None,
        };
        let updated = match meta.remove(&yaml::UPDATED) {
            Some(Yaml::String(date)) => parse_datetime(&date)?,
            Some(..) => return Err("date must be a string".into()),
            None => match date {
                Some(d) => d,
                None => entry_file.metadata()?.modified()?.into(),
            },
        };
        let index = match meta.remove(&yaml::INDEX) {
            Some(Yaml::Boolean(b)) => b.then(|| Index {
                paginate: None,
                max: None,
                compact: false,
                sort: index::Sort::default(),
                directories: vec![name_to_glob(name)],
            }),
            Some(Yaml::String(dir)) => Some(Index {
                paginate: None,
                max: None,
                compact: false,
                sort: index::Sort::default(),
                directories: vec![dir_to_glob(dir)?],
            }),
            Some(Yaml::Array(array)) => Some(Index {
                paginate: None,
                max: None,
                compact: false,
                sort: index::Sort::default(),
                directories: array
                    .into_iter()
                    .map(|i| match i {
                        Yaml::String(dir) => dir_to_glob(dir),
                        _ => Err(SourceError::from("index directories must be strings")),
                    })
                    .collect::<Result<_, _>>()?,
            }),
            Some(Yaml::Hash(mut index)) => Some(Index {
                paginate: match index.remove(&yaml::PAGINATE) {
                    Some(Yaml::Integer(i @ 1..=U32_MAX_AS_I64)) => Some(i as u32),
                    Some(Yaml::Boolean(false)) | None => None,
                    Some(..) => return Err("invalid pagination setting".into()),
                },
                max: match index.remove(&yaml::MAX) {
                    Some(Yaml::Integer(i @ 1..=U32_MAX_AS_I64)) => Some(i as u32),
                    Some(Yaml::Boolean(false)) | None => None,
                    Some(..) => return Err("invalid max setting".into()),
                },
                compact: match index.remove(&yaml::COMPACT) {
                    Some(Yaml::Boolean(b)) => b,
                    None => false,
                    Some(..) => return Err("invalid compact setting".into()),
                },
                sort: match index.remove(&yaml::SORT) {
                    Some(Yaml::String(key)) => key.parse()?,
                    Some(..) => return Err("invalid sort value".into()),
                    None => index::Sort::default(),
                },
                directories: match index.remove(&yaml::DIRECTORIES) {
                    Some(Yaml::Array(array)) => array
                        .into_iter()
                        .map(|i| match i {
                            Yaml::String(dir) => dir_to_glob(dir),
                            _ => Err(SourceError::from(
                                "index directories must be \
                                 strings",
                            )),
                        })
                        .collect::<Result<_, _>>()?,
                    Some(Yaml::String(dir)) => vec![dir_to_glob(dir)?],
                    Some(..) => return Err("invalid directory list in index".into()),
                    None => vec![name_to_glob(name)],
                },
            }),
            Some(..) => return Err("invalid index value".into()),
            None => None,
        };
        let cc = match meta.remove(&yaml::CC) {
            Some(Yaml::String(cc)) => vec![cc],
            Some(Yaml::Array(cc)) => cc
                .into_iter()
                .map(|v| match v {
                    Yaml::String(ci) => Ok(ci),
                    _ => Err(SourceError::from("invlaid cc value")),
                })
                .collect::<Result<_, _>>()?,
            Some(..) => return Err("invalid cc value".into()),
            None => Vec::new(),
        };
        let meta = EntryMeta::from_yaml(meta)?;
        let name = name.to_owned();

        Ok(Entry {
            title,
            description,
            date,
            updated,
            index,
            cc,
            meta,
            name,
            content,
            format,
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
