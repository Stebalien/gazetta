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

use std::fs;
use std::path::{Path, PathBuf};

use url::Url;

use crate::error::{AnnotatedError, SourceError};
use crate::util;

use super::Meta;
use super::yaml::{self, Yaml};
use super::{Entry, StaticEntry};

const MATCH_OPTIONS: ::glob::MatchOptions = ::glob::MatchOptions {
    case_sensitive: true,
    require_literal_separator: true,
    require_literal_leading_dot: false,
};

/// The Source object reads and interprets a source directory.
///
/// The fields are intentionally public. Feel free to manually generate or modify this structure.
#[derive(Debug, Clone)]
pub struct Source<SourceMeta = (), EntryMeta = ()>
where
    SourceMeta: Meta,
    EntryMeta: Meta,
{
    /// The website's title.
    ///
    /// By default, this field is read from `gazetta.yaml`.
    pub title: String,
    /// The source root directory.
    ///
    /// This is specified on construction.
    pub root: PathBuf,
    /// The website origin (http://mydomain.com:1234)
    ///
    /// By default, this field is derived from the value of `base` in `gazetta.yaml`.
    pub origin: String,
    /// The directory under the origin at which this site will be hosted (e.g. "/").
    ///
    /// By default, this field is derived from the value of `base` in `gazetta.yaml`.
    pub prefix: String,
    /// The website content to be rendered.
    ///
    /// By default, this list is populated with Entries generated from files with the basename
    /// index under the root directory excluding:
    ///
    ///  1. Files *under* directories named "static".
    ///
    ///  2. Files under `assets/`.
    pub entries: Vec<Entry<EntryMeta>>,
    /// The website content to be deployed as-is (no rendering).
    ///
    /// By default, this list is populated with directories under the root directory named "static"
    /// excluding:
    ///
    ///  1. Directories *under* directories named "static".
    ///
    ///  2. Directories under `assets/`.
    pub static_entries: Vec<StaticEntry>,
    /// The website stylesheets. When rendered, these will be concatenated into a single
    /// stylesheet.
    ///
    /// By default, this list is populated by the files in is `assets/stylesheets/` in
    /// lexicographical order.
    pub stylesheets: Vec<PathBuf>,
    /// The website javascript. When rendered, these will be concatenated into a single
    /// javascript file.
    ///
    /// By default, this list is populated by the files in is `assets/javascript/` in
    /// lexicographical order.
    pub javascript: Vec<PathBuf>,
    /// The path to the website's icon.
    ///
    /// By default, this points to `assets/icon.png` (if it exists).
    pub icon: Option<PathBuf>,
    /// The path to the `.well-known` directory.
    ///
    /// By default, this points to `.well-known`.
    pub well_known: Option<PathBuf>,
    /// Additional metadata read from `gazetta.yaml`.
    pub meta: SourceMeta,
}

impl<SourceMeta, EntryMeta> Source<SourceMeta, EntryMeta>
where
    SourceMeta: Meta,
    EntryMeta: Meta,
{
    /// Build an index for an entry.
    ///
    /// This index includes all entries that "cc" this entry and all entries specified in this
    /// entry's index pattern.
    pub fn build_index(&self, entry: &Entry<EntryMeta>) -> Vec<&Entry<EntryMeta>> {
        use crate::model::index::SortDirection::*;

        if let Some(ref index) = entry.index {
            let mut child_entries: Vec<_> = self
                .entries
                .iter()
                .filter(|child| {
                    child.cc.contains(&entry.name)
                        || index
                            .directories
                            .iter()
                            .any(|d| d.matches_with(&child.name, MATCH_OPTIONS))
                })
                .collect();

            match index.sort.direction {
                Ascending => child_entries.sort_by(|e1, e2| index.sort.field.compare(e1, e2)),
                Descending => child_entries.sort_by(|e1, e2| index.sort.field.compare(e2, e1)),
            }

            if let Some(max) = index.max {
                child_entries.truncate(max as usize);
            }
            child_entries
        } else {
            Vec::new()
        }
    }

    /// Parse a source directory to create a new source.
    pub fn new<P: AsRef<Path>>(root: P) -> Result<Self, AnnotatedError<SourceError>> {
        Self::_new(root.as_ref())
    }

    // avoid exporting large generic functions.
    fn _new(root: &Path) -> Result<Self, AnnotatedError<SourceError>> {
        let config_path = root.join("gazetta.yaml");
        let mut source = Source::from_config(root, &config_path)
            .map_err(|e| AnnotatedError::new(config_path, e))?;
        source.reload()?;
        Ok(source)
    }

    /// Reload from the source directory.
    ///
    /// Call this after changing source files.
    pub fn reload(&mut self) -> Result<(), AnnotatedError<SourceError>> {
        self.static_entries.clear();
        self.entries.clear();
        self.stylesheets.clear();
        self.javascript.clear();
        self.icon = None;
        self.well_known = None;
        self.load_entries("")?;
        self.load_assets()?;
        self.load_well_known()?;
        Ok(())
    }

    #[inline(always)]
    fn from_config(root: &Path, config_path: &Path) -> Result<Self, SourceError> {
        let mut config = yaml::load(config_path)?;
        let (origin, prefix) = match config.remove(&yaml::KEYS.base) {
            Some(Yaml::String(base)) => {
                let mut url = Url::parse(&base)?;
                if url.cannot_be_a_base() {
                    return Err("url cannot be a base".into());
                }
                if url.fragment().is_some() {
                    return Err("base url must not specify a fragment".into());
                }
                if url.query().is_some() {
                    return Err("base url must not specify a query".into());
                }

                let prefix = url.path().to_string();

                url.set_path("");
                let mut origin = url.to_string();
                // Trim a trailing /, if any.
                if origin.ends_with("/") {
                    origin.pop();
                }

                (origin, prefix)
            }
            Some(..) => return Err("the base url must be a string".into()),
            None => return Err("you must specify a base url".into()),
        };

        Ok(Source {
            title: match config.remove(&yaml::KEYS.title) {
                Some(Yaml::String(title)) => title,
                Some(..) => return Err("title must be a string".into()),
                None => return Err("must specify title".into()),
            },
            origin,
            prefix,
            root: root.to_owned(),
            well_known: None,
            entries: Vec::new(),
            static_entries: Vec::new(),
            stylesheets: Vec::new(),
            javascript: Vec::new(),
            icon: None,
            meta: SourceMeta::from_yaml(config)?,
        })
    }

    fn load_well_known(&mut self) -> Result<(), AnnotatedError<SourceError>> {
        let path = self.root.join(".well-known");
        if try_annotate!(util::exists(&path), path) {
            self.well_known = Some(path.clone());
        }
        Ok(())
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
        let base_dir = self.root.join(dir);

        for dir_entry in try_annotate!(fs::read_dir(&base_dir), base_dir) {
            let dir_entry = try_annotate!(dir_entry, base_dir);
            let file_name = match dir_entry.file_name().into_string() {
                Ok(s) => {
                    if s.starts_with('.') {
                        continue;
                    } else {
                        s
                    }
                }
                Err(s) => {
                    // Can't possibly be a file we care about but, if it isn't hidden, we want to
                    // return an error and bail.
                    // FIXME: OsStr::starts_with
                    if s.to_string_lossy().starts_with('.') {
                        continue;
                    } else {
                        return Err(AnnotatedError::new(
                            dir_entry.path(),
                            "file names must be valid utf8".into(),
                        ));
                    }
                }
            };

            // Skip assets.
            if dir.is_empty() && file_name == "assets" {
                continue;
            }

            let file_type = try_annotate!(dir_entry.file_type(), dir_entry.path());

            if file_type.is_file() {
                if Path::new(&file_name).file_stem().unwrap() == "index" {
                    let entry =
                        try_annotate!(Entry::from_file(dir_entry.path(), dir), dir_entry.path());
                    self.entries.push(entry);
                }
            } else if file_type.is_dir() {
                let name = if dir.is_empty() {
                    file_name.to_owned()
                } else {
                    format!("{}/{}", dir, &file_name)
                };
                match &*file_name {
                    "static" => self.static_entries.push(StaticEntry {
                        name,
                        source: dir_entry.path(),
                    }),
                    "index" => {
                        return Err(AnnotatedError::new(
                            dir_entry.path(),
                            "paths ending in index are reserved for \
                             indices"
                                .into(),
                        ));
                    }
                    _ => self.load_entries(&name)?,
                }
            } else if file_type.is_symlink() {
                // TODO: Symlinks
                unimplemented!();
            }
        }
        Ok(())
    }
}
