use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::PathBuf;

use ::{glob, SourceError};

use super::{Meta, Date};
use super::index::{self, Index};
use super::yaml::{self, Yaml};

#[derive(Debug, Clone)]
pub struct Entry<EntryMeta> where EntryMeta: Meta {
    pub title: String,
    pub date: Option<Date>,
    pub index: Option<Index>,
    pub cc: Vec<String>,
    pub meta: EntryMeta,
    pub content_path: PathBuf,
    pub content_offset: usize,
}

impl<EntryMeta> Entry<EntryMeta> where EntryMeta: Meta {
    pub fn read_content(&self, buf: &mut String) -> io::Result<usize> {
        let mut f = try!(File::open(&self.content_path));
        if (self.content_offset as u64) != try!(f.seek(io::SeekFrom::Start(self.content_offset as u64))) {
            return Err(io::Error::new(io::ErrorKind::Other, "content missing"));
        }
        f.read_to_string(buf)
    }

    pub fn from_file(full_path: PathBuf, name: &str) -> Result<Self, SourceError> {
        let (offset, mut meta) = try!(yaml::load_front(&full_path));

        Ok(Entry {
            content_offset: offset,
            content_path: full_path,
            title: match meta.remove(&yaml::TITLE) {
                Some(Yaml::String(title)) => title,
                Some(..) => return Err("titles must be strings".into()),
                None => return Err("entries must have titles".into()),
            },
            date: match meta.remove(&yaml::DATE) {
                Some(Yaml::String(date)) => match Date::parse_from_str(&date, "%Y-%m-%d") {
                    Ok(date) => Some(date),
                    Err(e) => {
                        println!("{} : {}", e, &date);
                        return Err("invalid date format".into());
                    }
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
                        directories: {
                            let mut s = glob::Pattern::escape(name);
                            s.push_str("/*");
                            vec![glob::Pattern::new(&s).unwrap()]
                        },
                    })
                } else {
                    None
                },
                Some(Yaml::String(mut dir)) => Some(Index {
                    paginate: None,
                    max: None,
                    sort: index::Sort::default(),
                    directories: {
                        if !dir.starts_with("/") {
                            dir.insert(0, '/');
                        }
                        if !dir.ends_with("/") {
                            dir.push('/');
                        }
                        dir.push('*');
                        vec![try!(glob::Pattern::new(&dir))]
                    },
                }),
                Some(Yaml::Array(array)) => Some(Index {
                    paginate: None,
                    max: None,
                    sort: index::Sort::default(),
                    directories: try!(array.into_iter().map(|i| match i {
                        Yaml::String(mut dir) => {
                            if !dir.starts_with("/") {
                                dir.insert(0, '/');
                            }
                            if !dir.ends_with("/") {
                                dir.push('/');
                            }
                            dir.push('*');
                            Ok(try!(glob::Pattern::new(&dir)))
                        },
                        _ => Err(SourceError::from("index directories must be strings")),
                    }).collect())
                }),
                Some(Yaml::Hash(mut index)) => Some(Index {
                    paginate: match index.remove(&yaml::PAGINATE) {
                        Some(Yaml::Integer(i)) if i > 0 && (i as u64) < (::std::usize::MAX as u64) => Some(i as u32),
                        Some(Yaml::Boolean(false)) | None => None,
                        Some(..) => return Err("invalid pagination setting".into()),
                    },
                    max: match index.remove(&yaml::MAX) {
                        Some(Yaml::Integer(i)) if i > 0 && (i as u64) < (::std::usize::MAX as u64) => Some(i as u32),
                        Some(Yaml::Boolean(false)) | None => None,
                        Some(..) => return Err("invalid max setting".into()),
                    },
                    sort: match index.remove(&yaml::SORT) {
                        Some(Yaml::String(key)) => match &key[..] {
                            "default" => index::Sort::default(),
                            "date" => index::Sort {
                                field: index::SortField::Date,
                                direction: index::SortDirection::default(),
                            },
                            "+date" => index::Sort {
                                field: index::SortField::Date,
                                direction: index::SortDirection::Ascending,
                            },
                            "-date" => index::Sort {
                                field: index::SortField::Date,
                                direction: index::SortDirection::Descending,
                            },
                            "title" => index::Sort {
                                field: index::SortField::Title,
                                direction: index::SortDirection::default(),
                            },
                            "-title" => index::Sort {
                                field: index::SortField::Title,
                                direction: index::SortDirection::Descending,
                            },
                            "+title" => index::Sort {
                                field: index::SortField::Title,
                                direction: index::SortDirection::Descending,
                            },
                            _ => return Err("invalid sort value".into()),
                        },
                        Some(..) => return Err("invalid sort value".into()),
                        None => index::Sort::default(),
                    },
                    directories: match index.remove(&yaml::SORT) {
                        Some(Yaml::Array(array)) => try!(array.into_iter().map(|i| match i {
                            Yaml::String(mut dir) => {
                                if !dir.starts_with("/") {
                                    dir.insert(0, '/');
                                }
                                if !dir.ends_with("/") {
                                    dir.push('/');
                                }
                                dir.push('*');
                                Ok(try!(glob::Pattern::new(&dir)))
                            },
                            _ => Err(SourceError::from("index directories must be strings")),
                        }).collect()),
                        Some(Yaml::String(mut dir)) => vec![{
                            if !dir.starts_with("/") {
                                dir.insert(0, '/');
                            }
                            if !dir.ends_with("/") {
                                dir.push('/');
                            }
                            dir.push('*');
                            try!(glob::Pattern::new(&dir))
                        }],
                        Some(..) => return Err("invalid directory list in index".into()),
                        None => {
                            let mut s = glob::Pattern::escape(name);
                            s.push_str("/*");
                            vec![glob::Pattern::new(&s).unwrap()]
                        }
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
        })
    }
}

