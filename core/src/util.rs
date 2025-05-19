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

use std::fs::{self, File};
use std::hash::Hasher;
use std::io;
use std::path::{Path, PathBuf};

use crate::error::AnnotatedError;

/// Recursivly copy a directory.
///
/// Does not preserve permissions.
pub fn copy_recursive(src: &Path, dest: &Path) -> io::Result<()> {
    if fs::metadata(src)?.is_dir() {
        copy_dir(src, dest)
    } else {
        copy_file(src, dest)
    }
}

fn copy_file(src: &Path, dest: &Path) -> io::Result<()> {
    io::copy(&mut File::open(src)?, &mut File::create(dest)?).map(|_| ())
}

fn copy_dir(src: &Path, dest: &Path) -> io::Result<()> {
    fs::create_dir(dest)?;
    for dir_entry in fs::read_dir(src)? {
        let dir_entry = dir_entry?;
        let file_name = dir_entry.file_name();
        let from = src.join(&file_name);
        let to = dest.join(&file_name);
        if exists(&to)? {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "target path already exists",
            ));
        }

        if dir_entry.file_type()?.is_dir() {
            copy_dir(&from, &to)?;
        } else {
            copy_file(&from, &to)?;
        }
    }
    Ok(())
}

/// Check if a file exists.
pub fn exists(path: &Path) -> io::Result<bool> {
    match fs::metadata(path) {
        Ok(_) => Ok(true),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => Ok(false),
            _ => Err(e),
        },
    }
}

/// Walk a file tree and return a sorted vector of paths.
pub fn walk_sorted(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    walk_into(path, &mut out)?;
    Ok(out)
}

fn walk_into(path: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    let mut files = fs::read_dir(path)?
        .map(|e| {
            let e = e?;
            Ok((e.path(), e.file_type()?.is_dir()))
        })
        .collect::<io::Result<Vec<_>>>()?;

    // Don't need to sort by whole path. We're walking in sort-order.
    files.sort_by(|a, b| a.0.file_name().unwrap().cmp(b.0.file_name().unwrap()));
    for (path, is_dir) in files {
        if is_dir {
            walk_into(&path, out)?;
        } else {
            out.push(path);
        }
    }
    Ok(())
}

pub struct StreamHasher<W, H> {
    hash: H,
    inner: W,
}
impl<W, H> StreamHasher<W, H>
where
    H: Hasher,
    W: io::Write,
{
    pub fn new(inner: W) -> Self
    where
        H: Default,
    {
        StreamHasher {
            hash: H::default(),
            inner,
        }
    }
    pub fn with_hasher(inner: W, hash: H) -> Self {
        StreamHasher { hash, inner }
    }
    pub fn finish(&self) -> u64 {
        self.hash.finish()
    }
}
impl<W, H> io::Write for StreamHasher<W, H>
where
    W: io::Write,
    H: Hasher,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let size = self.inner.write(buf)?;
        self.hash.write(&buf[..size]);
        Ok(size)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

/// Concatinate source paths into an output file.
pub fn concat<W, I>(paths: I, output: &mut W) -> Result<u64, AnnotatedError<io::Error>>
where
    W: io::Write,
    I: IntoIterator,
    I::Item: AsRef<Path>,
{
    let mut bytes = 0;
    for p in paths {
        let p = p.as_ref();
        bytes += try_annotate!(io::copy(&mut try_annotate!(File::open(p), p), output), p)
    }
    Ok(bytes)
}
