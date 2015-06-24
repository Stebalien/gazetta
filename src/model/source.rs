use std::fs;
use std::path::{Path, PathBuf};

use ::{AnnotatedError, SourceError};
use super::{ Entry, StaticEntry};
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

        let mut source = try!(Source::from_config(root, &config_path).map_err(|e| AnnotatedError::new(config_path, e)));

        try!(source.load("/"));
        Ok(source)
    }

    #[inline(always)]
    fn from_config(root: &Path, config_path: &Path) -> Result<Self, SourceError> {
        let mut config = try!(yaml::load(&config_path));

        Ok(Source {
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

    fn load(&mut self, dir: &str) -> Result<(), AnnotatedError<SourceError>> {
        macro_rules! try_annotate {
            ($e:expr, $l:expr) => {
                match $e {
                    Ok(v) => v,
                    Err(e) => return Err(AnnotatedError::new(($l).to_owned(), SourceError::from(e))),
                }
            }
        }
        let base_dir = self.root.join(&dir[1..]);

        for dir_entry in try_annotate!(fs::read_dir(&base_dir), base_dir) {
            let dir_entry = try_annotate!(dir_entry, base_dir);
            let file_name = match dir_entry.file_name().into_string() {
                Ok(s) => if s.starts_with('.') { continue } else { s },
                Err(s) => {
                    // Can't possibly be a file we care about but, if it isn't hidden, we want to
                    // return an error and bail.
                    // FIXME: OsStr::starts_with
                    if s.to_string_lossy().starts_with('.') {
                        continue
                    } else {
                        return Err(AnnotatedError::new(dir_entry.path(), "file names must be valid utf8".into()));
                    }
                }
            };

            let file_type = try_annotate!(dir_entry.file_type(), dir_entry.path());

            if file_type.is_file() {
                if Path::new(&file_name).file_stem().unwrap() == "index" {
                    let entry = try_annotate!(Entry::from_file(dir_entry.path(), dir), dir_entry.path());
                    self.entries.push(entry);
                }
            } else if file_type.is_dir() {
                let name = if dir == "/" {
                    format!("/{}", &file_name)
                } else {
                    format!("{}/{}", dir, &file_name)
                };
                match &file_name[..] {
                    "static" => self.static_entries.push(StaticEntry { name: name, source: dir_entry.path() }),
                    "index" => return Err(AnnotatedError::new(dir_entry.path(), "paths ending in index are reserved for indices".into())),
                    _ => try!(self.load(&name)),
                }
            } else if file_type.is_symlink() {
                // TODO: Symlinks
                unimplemented!();
            }
        }
        Ok(())
    }

}
