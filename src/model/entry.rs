use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::PathBuf;
use std::borrow::Cow;

use ::{glob, SourceError, AnnotatedError};

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
    pub content: ContentSource,
}

#[derive(Debug, Clone)]
pub enum ContentSource {
    File {
        content_path: PathBuf,
        content_offset: u64,
    },
    String {
        data: Cow<'static, str>,
        format: Cow<'static, str>,
    }
}

impl ContentSource {
    /// Read the content into the specified buffer.
    pub fn read_into(&self, buf: &mut String) -> Result<usize, AnnotatedError<io::Error>> {
        match *self {
            ContentSource::File { ref content_path, content_offset } => (|| {
                let mut f = try!(File::open(&content_path));
                if content_offset != try!(f.seek(io::SeekFrom::Start(content_offset))) {
                    return Err(io::Error::new(io::ErrorKind::Other, "content missing"));
                }
                f.read_to_string(buf)
            })().map_err(|e| AnnotatedError::new(content_path.clone(), e)),
            ContentSource::String { ref data, .. } => {
                buf.push_str(&data);
                Ok(data.len())
            }
        }
    }

    /// The content format.
    ///
    /// If this is a file, this returns the file extension.
    /// If this content exists in-memory, this returns the format.
    ///
    /// Note: if the file extension is not UTF-8 encoded, this returns an empty string.
    ///
    pub fn format(&self) -> &str {
        match *self {
            ContentSource::File { ref content_path, .. } => {
                content_path.extension().and_then(|e|e.to_str()).unwrap_or("")
            },
            ContentSource::String { ref format, .. } => &format,
        }
    }
}

impl<EntryMeta> Entry<EntryMeta> where EntryMeta: Meta {

    pub fn from_file(full_path: PathBuf, name: String) -> Result<Self, SourceError> {
        // Helpers

        const U32_MAX_AS_I64: i64 = ::std::u32::MAX as i64;

        fn dir_to_glob(mut dir: String) -> Result<glob::Pattern, SourceError> {
            if !dir.starts_with("/") {
                dir.insert(0, '/');
            }
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

        let (offset, mut meta) = try!(yaml::load_front(&full_path));

        Ok(Entry {
            content: ContentSource::File {
                content_offset: offset,
                content_path: full_path,
            },
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
                        None => vec![name_to_glob(&name)],
                    }
                }),
                Some(..) => return Err("invalid index value".into()),
                None => None,
            },
            cc: match meta.remove(&yaml::CC) {
                Some(Yaml::String(cc)) => vec![cc],
                Some(Yaml::Array(cc)) => try!(cc.into_iter().map(|v| match v {
                    Yaml::String(mut ci) => {
                        ci.insert(0, '/');
                        Ok(ci)
                    },
                    _ => Err(SourceError::from("invlaid cc value")),
                }).collect()),
                Some(..) => return Err("invalid cc value".into()),
                None => Vec::new(),
            },
            meta: try!(EntryMeta::from_yaml(meta)),
            name: name,
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