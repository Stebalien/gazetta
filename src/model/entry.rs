use std::fs::{self, File};
use std::io::prelude::*;
use std::io;
use std::path::{Path, PathBuf};

use ::Date;

use super::Error;
use super::index::{self, Index};

use super::yaml::{self, Yaml};

#[derive(Debug, Clone)]
pub struct Entry {
    pub title: String,
    pub date: Option<Date>,
    pub index: Option<Index>,
    pub cc: Vec<String>,
    pub content: PathBuf,
    pub content_offset: usize,
}

impl Entry {
    pub fn read_content(&self, buf: &mut String) -> io::Result<usize> {
        let mut f = try!(File::open(&self.content));
        if (self.content_offset as u64) != try!(f.seek(io::SeekFrom::Start(self.content_offset as u64))) {
            return Err(io::Error::new(io::ErrorKind::Other, "content missing"));
        }
        f.read_to_string(buf)
    }
    pub fn from_file(root: &Path, path: &Path) -> Result<Entry, Error> {
        let mut full_path = root.join(path);
        full_path.push("index.md");
        let (offset, mut meta) = try!(yaml::load_front(&full_path));

        let mut entry = Entry {
            content_offset: offset,
            content: full_path,
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
                Some(Yaml::Boolean(b)) => if b { Some(Index::default()) } else { None },
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
                    entries: Vec::new(),
                }),
                Some(..) => return Err("invalid index value".into()),
                None => None,
            },
            cc: match meta.remove(&yaml::CC) {
                Some(Yaml::String(cc)) => vec![cc],
                Some(Yaml::Array(cc)) => try!(cc.into_iter().map(|v| match v {
                    Yaml::String(ci) => Ok(ci),
                    _ => Err(Error::from("invlaid cc value")),
                }).collect()),
                Some(..) => return Err("invalid cc value".into()),
                None => Vec::new(),
            },
        };

        if let Some(ref mut idx) = entry.index {
            for dir_entry in try!(fs::read_dir(&entry.content.parent().unwrap())) {
                let dir_entry = try!(dir_entry);
                if try!(fs::metadata(dir_entry.path())).is_dir() {
                    match path.join(dir_entry.file_name()).into_os_string().into_string() {
                        Ok(s) => idx.entries.push(s),
                        Err(_) => return Err("paths must be valid utf8".into()),
                    }
                }
            }
        }
        Ok(entry)
    }
}

