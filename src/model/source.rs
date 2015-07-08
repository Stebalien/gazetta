/* Copyright (2015) Stevem Allen
 *
 * This file is part of gazetta.
 * 
 * gazetta-bin is free software: you can redistribute it and/or modify it under the terms of the
 * GNU Affero General Public License (version 3) as published by the Free Software Foundation.
 * 
 * Foobar is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
 * the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU Affero General Public License along with Foobar.  If
 * not, see <http://www.gnu.org/licenses/>.
 */

use std::fs;
use std::path::{Path, PathBuf};

use ::{AnnotatedError, SourceError};
use ::util;
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
    pub stylesheets: Vec<PathBuf>,
    pub javascript: Vec<PathBuf>,
    pub icon: Option<PathBuf>,
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

        try!(source.reload());
        Ok(source)
    }

    /// Reload from the source directory.
    pub fn reload(&mut self) -> Result<(), AnnotatedError<SourceError>> {
        self.static_entries.clear();
        self.entries.clear();
        self.stylesheets.clear();
        self.javascript.clear();
        self.icon = None;
        try!(self.load_entries("/"));
        try!(self.load_assets());
        Ok(())
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
            stylesheets: Vec::new(),
            javascript: Vec::new(),
            icon: None,
            meta: try!(SourceMeta::from_yaml(config)),
        })
    }

    fn load_assets(&mut self) -> Result<(), AnnotatedError<SourceError>> {
        let mut path = self.root.join("assets");

        path.push("icon.png");
        if try_annotate!(util::exists(&path), path) {
            self.icon = Some(path.clone());
        }

        path.set_file_name("javascript");
        if try_annotate!(util::exists(&path), path) {
            self.javascript = try_annotate!(util::walk_sorted(&path), path);
        }

        path.set_file_name("stylesheets");
        if try_annotate!(util::exists(&path), path) {
            self.stylesheets = try_annotate!(util::walk_sorted(&path), path);
        }
        Ok(())
    }

    fn load_entries(&mut self, dir: &str) -> Result<(), AnnotatedError<SourceError>> {
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

            // Skip assets.
            if dir == "/" && file_name == "assets" {
                continue;
            }

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
                    _ => try!(self.load_entries(&name)),
                }
            } else if file_type.is_symlink() {
                // TODO: Symlinks
                unimplemented!();
            }
        }
        Ok(())
    }

}
