use std::fs;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use ::{AnnotatedError, SourceError};
use super::{ Entry, Person };
use super::yaml::{self, Yaml};
use super::Meta;

#[derive(Debug, Clone)]
pub struct Source<SourceMeta=(), EntryMeta=()>
    where SourceMeta: Meta,
          EntryMeta: Meta
{
    pub title: String,
    pub author: Person,
    pub root: PathBuf,
    pub entries: BTreeMap<String, Entry<EntryMeta>>,
    pub meta: SourceMeta,
}


impl<SourceMeta, EntryMeta> Source<SourceMeta, EntryMeta>
    where SourceMeta: Meta,
          EntryMeta: Meta
{
    fn read_entries(&mut self, dir: &Path) -> Result<(), AnnotatedError<SourceError>> {
        macro_rules! try_annotate {
            ($e:expr, $l:expr) => {
                match $e {
                    Ok(v) => v,
                    Err(e) => return Err(AnnotatedError::new(($l).to_owned(), SourceError::from(e))),
                }
            }
        }
        for dir_entry in try_annotate!(fs::read_dir(dir), dir) {
            let dir_entry = try_annotate!(dir_entry, dir);
            let full_path = dir_entry.path();
            let file_type = try_annotate!(dir_entry.file_type(), full_path);
            if file_type.is_file() {
                if full_path.file_name().unwrap() == "index.md" {
                    let mut path_str = String::with_capacity(256);
                    {
                        let path = full_path.parent().unwrap().relative_from(&self.root).unwrap();
                        path_str.push('/');
                        match path.to_str() {
                            Some(s) => path_str.push_str(s),
                            None => return Err(AnnotatedError::new(full_path.clone(), "file names must be valid utf8".into())),
                        };
                    }
                    let entry = try_annotate!(Entry::from_file(full_path, &path_str), dir_entry.path());
                    self.entries.insert(path_str, entry);
                }
                // TODO: index.html?
            } else if file_type.is_dir() && !full_path.ends_with("/static") {
                try!(self.read_entries(&full_path));
            }
        }
        Ok(())
    }

    fn check_entries(&self) -> Result<(), AnnotatedError<SourceError>> {
        for (entry_path, entry) in &self.entries {
            for other_entry_path in &entry.cc {
                if other_entry_path == entry_path {
                    return Err(AnnotatedError::new(entry.content_path.clone(), "entry CCed to itself".into()));
                }
                let other_entry = match self.entries.get(other_entry_path) {
                    None => return Err(AnnotatedError::new(entry.content_path.clone(), "cc points to missing entry".into())),
                    Some(p) => p,
                };
                if other_entry.index.is_none() {
                    return Err(AnnotatedError::new(entry.content_path.clone(), "cc points to entry without index".into()));
                }
            }
        }
        Ok(())
    }

    /// Parse a data directory to create a new source.
    pub fn new<P: AsRef<Path>>(root: P) -> Result<Self, AnnotatedError<SourceError>> {
        let root = root.as_ref();

        let config_path = root.join("config.yaml");

        let mut site = try!(Source::new_inner(root, &config_path).map_err(|e| AnnotatedError::new(config_path, e)));

        try!(site.read_entries(root));
        try!(site.check_entries());
        Ok(site)
    }

    #[inline(always)]
    fn new_inner(root: &Path, config_path: &Path) -> Result<Self, SourceError> {
        let mut config = try!(yaml::load(&config_path));

        Ok(Source {
            author: match config.remove(&yaml::AUTHOR) {
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
            },
            title: match config.remove(&yaml::TITLE) {
                Some(Yaml::String(title)) => title,
                Some(..) => return Err("title must be a string".into()),
                None => return Err("must specify title".into()),
            },
            root: root.to_owned(),
            entries: BTreeMap::new(),
            meta: try!(SourceMeta::from_yaml(config)),
        })
    }
}
