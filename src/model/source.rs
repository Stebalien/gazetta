use std::fs;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use ::yaml::{ self, Yaml };
use super::{ Error, Entry, Person };
use super::yaml_keys;
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

fn read_entries<EntryMeta>(root: &Path,
                dir: &Path,
                entries: &mut BTreeMap<String, Entry<EntryMeta>>) -> Result<(), Error>
    where EntryMeta: Meta
{
    for dir_entry in try!(fs::read_dir(dir)) {
        let dir_entry = try!(dir_entry);
        let full_path = dir_entry.path();
        let file_type = try!(dir_entry.file_type());
        if file_type.is_file() {
            if full_path.file_name().unwrap() == "index.md" {
                let mut path_str = String::with_capacity(256);
                {
                    let path = full_path.parent().unwrap().relative_from(&root).unwrap();
                    path_str.push('/');
                    match path.to_str() {
                        Some(s) => path_str.push_str(s),
                        None => return Err("file names must be valid utf8".into()),
                    };
                }
                let entry = try!(Entry::from_file(full_path, &path_str));
                entries.insert(path_str, entry);
            }
            // TODO: index.html?
        } else if file_type.is_dir() && !full_path.ends_with("/static") {
            try!(read_entries(root, &full_path, entries));
        }
    }
    Ok(())
}

impl<SourceMeta, EntryMeta> Source<SourceMeta, EntryMeta>
    where SourceMeta: Meta,
          EntryMeta: Meta
{
    pub fn new<P: AsRef<Path>>(root: P) -> Result<Self, Error> {
        let root = root.as_ref();
        let mut config = try!(yaml::load(root.join("config.yaml")));

        let author = match config.remove(&yaml_keys::AUTHOR) {
            Some(Yaml::Hash(mut author)) => Person {
                name: match author.remove(&yaml_keys::NAME) {
                    Some(Yaml::String(name)) => name,
                    None => return Err("missing author name".into()),
                    _ => return Err("author name must be a string".into()),
                },
                email: match author.remove(&yaml_keys::EMAIL) {
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

        let title = match config.remove(&yaml_keys::TITLE) {
            Some(Yaml::String(title)) => title,
            Some(..) => return Err("title must be a string".into()),
            None => return Err("must specify title".into()),
        };

        let mut entries = BTreeMap::<String, Entry<EntryMeta>>::new();

        try!(read_entries(&root, &root, &mut entries));

        // Validate cross links.
        for (entry_path, entry) in &entries {
            for other_entry_path in &entry.cc {
                if other_entry_path == entry_path {
                    return Err("entry CCed to itself".into());
                }
                let other_entry = match entries.get(other_entry_path) {
                    None => return Err("cc points to missing entry".into()),
                    Some(p) => p,
                };
                if other_entry.index.is_none() {
                    return Err("cc points to entry without index".into());
                }
            }
        }

        Ok(Source {
            title: title,
            author: author,
            root: root.into(),
            entries: entries,
            meta: try!(SourceMeta::from_yaml(config)),
        })
    }
}

