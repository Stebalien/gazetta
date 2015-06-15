use std::cell::RefCell;
use std::fs;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::{ Error, Entry, Person };
use super::yaml::{ self, Yaml };

#[derive(Debug, Clone)]
pub struct Site {
    pub title: String,
    pub author: Person,
    pub root: PathBuf,
    pub entries: BTreeMap<String, Entry>,
}

impl Site {
    pub fn new<P: AsRef<Path>>(root: P) -> Result<Site, Error> {
        let root = root.as_ref();
        let mut config = try!(yaml::load(root.join("config.yaml")));

        let author = match config.remove(&yaml::AUTHOR) {
            Some(Yaml::Hash(mut author)) => Person {
                name: match author.remove(&yaml::NAME) {
                    Some(Yaml::String(name)) => name,
                    None => return Err("missing author name".into()),
                    _ => return Err("author name must be a string".into()),
                },
                email: match author.remove(&yaml::EMAIL) {
                    Some(Yaml::String(email)) => Some(email),
                    None => None,
                    _ => return Err("if specified, author email must be a string".into()),
                }
            },
            Some(Yaml::String(name)) => Person {
                    name: name,
                    email: None
            },
            Some(..) => return Err("invalid author".into()),
            None => return Err("must specify author".into()),
        };

        let title = match config.remove(&yaml::TITLE) {
            Some(Yaml::String(title)) => title,
            Some(..) => return Err("title must be a string".into()),
            None => return Err("must specify title".into()),
        };

        let mut entries = BTreeMap::<String, RefCell<Entry>>::new();

        for dir_entry in try!(fs::walk_dir(&root)) {
            let dir_entry = try!(dir_entry);
            let full_path = dir_entry.path();
            if try!(fs::metadata(&full_path)).is_file() {
                continue;
            }
            let path = full_path.relative_from(&root).unwrap();
            let path_str: String = match path.to_str() {
                Some(s) => s.into(),
                None => return Err("file names must be valid utf8".into()),
            };
            // Ignore static.
            if path_str.ends_with("/static") {
                continue;
            }
            entries.insert(path_str, RefCell::new(try!(Entry::from_file(root, path))));
        }

        // Cross link
        for (entry_path, entry) in &entries {
            for other_entry_path in &entry.borrow().cc {
                if other_entry_path == entry_path {
                    return Err("entry CCed to itself".into());
                }
                let other_entry = match entries.get(other_entry_path) {
                    None => return Err("cc points to missing entry".into()),
                    Some(p) => p,
                };
                let mut other_entry = other_entry.borrow_mut();
                match other_entry.index.as_mut() {
                    None => return Err("cc points to entry without index".into()),
                    Some(index) => index.entries.push(entry_path.clone()),
                }
            }
        }

        // Sort indexes.
        for entry in entries.values() {
            let mut entry = entry.borrow_mut();
            if let Some(idx) = entry.index.as_mut() {
                // TODO: Find some way to reuse children (without lifetime issues...)
                use super::index::SortDirection::*;
                use super::index::SortField::*;

                let mut children: Vec<_> = idx.entries.drain(..).map(|n| {
                    let e = entries[&n].borrow();
                    (n, e)
                }).collect();
                match (idx.sort.direction, idx.sort.field) {
                    (Ascending,     Title) => children.sort_by(|&(_, ref e1), &(_, ref e2)| e1.title.cmp(&e2.title)),
                    (Descending,    Title) => children.sort_by(|&(_, ref e1), &(_, ref e2)| e2.title.cmp(&e1.title)),
                    (Ascending,     Date)  => children.sort_by(|&(_, ref e1), &(_, ref e2)| e1.date.cmp(&e2.date)),
                    (Descending,    Date)  => children.sort_by(|&(_, ref e1), &(_, ref e2)| e2.date.cmp(&e1.date)),
                }
                idx.entries.extend(children.into_iter().map(|(n, _)| n));
            }
        }

        // Remove RefCells.
        let entries = entries.into_iter().map(|(p, e)| (p, e.into_inner())).collect();

        Ok(Site {
            title: title,
            author: author,
            root: root.into(),
            entries: entries,
        })
    }
}

