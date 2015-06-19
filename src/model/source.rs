use std::fs;
use std::path::{Path, PathBuf};

use ::{AnnotatedError, SourceError};
use super::{ Entry, Person, StaticEntry};
use super::yaml::{self, Yaml};
use super::Meta;

static MATCH_OPTIONS: ::glob::MatchOptions = ::glob::MatchOptions {
    case_sensitive: true,
    require_literal_separator: true,
    require_literal_leading_dot: false,
};

#[derive(Debug, Clone)]
pub struct Source<SourceMeta=(), EntryMeta=()>
    where SourceMeta: Meta,
          EntryMeta: Meta
{
    pub title: String,
    pub author: Person,
    pub root: PathBuf,
    pub entries: Vec<Entry<EntryMeta>>,
    pub static_entries: Vec<StaticEntry>,
    pub meta: SourceMeta,
}

impl<SourceMeta, EntryMeta> Source<SourceMeta, EntryMeta>
    where SourceMeta: Meta,
          EntryMeta: Meta
{
    /// Build an index for an entry.
    pub fn build_index(&self, entry: &Entry<EntryMeta>) -> Vec<&Entry<EntryMeta>> {
        use ::model::index::SortDirection::*;
        use ::model::index::SortField::*;

        if let Some(ref index) = entry.index {
            let mut child_entries: Vec<_> = self.entries.iter().filter(|child| {
                child.cc.contains(&entry.name) || index.directories.iter().any(|d| {
                    d.matches_with(&child.name, &MATCH_OPTIONS)
                })
            }).collect();


            match (index.sort.direction, index.sort.field) {
                (Ascending,     Title) => child_entries.sort_by(|e1, e2| e1.title.cmp(&e2.title)),
                (Descending,    Title) => child_entries.sort_by(|e1, e2| e2.title.cmp(&e1.title)),
                (Ascending,     Date)  => child_entries.sort_by(|e1, e2| e1.date.cmp(&e2.date)),
                (Descending,    Date)  => child_entries.sort_by(|e1, e2| e2.date.cmp(&e1.date)),
            }

            if let Some(max) = index.max {
                child_entries.truncate(max as usize);
            }
            child_entries
        } else {
            Vec::new()
        }
    }

    /// Parse a data directory to create a new source.
    pub fn new<P: AsRef<Path>>(root: P) -> Result<Self, AnnotatedError<SourceError>> {
        let root = root.as_ref();

        let config_path = root.join("config.yaml");

        let mut site = try!(Source::new_inner(root, &config_path).map_err(|e| AnnotatedError::new(config_path, e)));

        try!(site.load(root));
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
            entries: Vec::new(),
            static_entries: Vec::new(),
            meta: try!(SourceMeta::from_yaml(config)),
        })
    }

    fn load(&mut self, dir: &Path) -> Result<(), AnnotatedError<SourceError>> {
        fn path_to_href(root: &Path, path: &Path) -> Result<String, AnnotatedError<SourceError>> {
            let mut path_str = String::with_capacity(256);
            path_str.push('/');
            let relative_path = path.relative_from(root).unwrap();
            match relative_path.to_str() {
                Some(s) => path_str.push_str(s),
                None => return Err(AnnotatedError::new(path.to_owned(), "file names must be valid utf8".into())),
            };
            Ok(path_str)
        }

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
                if full_path.file_stem().unwrap() == "index" {
                    let path_str = try!(path_to_href(&self.root, full_path.parent().unwrap()));
                    let entry = try_annotate!(Entry::from_file(full_path, path_str), dir_entry.path());
                    self.entries.push(entry);
                }
            } else if file_type.is_dir() {
                if full_path.ends_with("static") {
                    let path_str = try!(path_to_href(&self.root, &full_path));
                    self.static_entries.push(StaticEntry { name: path_str, source: full_path });
                } else {
                    try!(self.load(&full_path));
                }
            }
        }
        Ok(())
    }

}
