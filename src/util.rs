use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io;

/// Recursivly copy a directory.
///
/// Does not preserve permissions.
pub fn copy_recursive(src: &Path, dest: &Path) -> io::Result<()> {
    if try!(fs::metadata(&src)).is_dir() {
        copy_dir(src, dest)
    } else {
        copy_file(src, dest)
    }
}

fn copy_file(src: &Path, dest: &Path) -> io::Result<()> {
    io::copy(&mut try!(File::open(&src)), &mut try!(File::create(&dest))).map(|_|())
}

fn copy_dir(src: &Path, dest: &Path) -> io::Result<()> {
    try!(fs::create_dir(dest));
    for dir_entry in try!(fs::read_dir(src)) {
        let dir_entry = try!(dir_entry);
        let file_name = dir_entry.file_name();
        let from = src.join(&file_name);
        let to = dest.join(&file_name);
        if try!(exists(&to)) {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "target path already exists"));
        }

        try!(if try!(dir_entry.file_type()).is_dir() {
            copy_dir(&from, &to)
        } else {
            copy_file(&from, &to)
        });
    }
    Ok(())
}

/// Check if a file exists.
pub fn exists(path: &Path) -> io::Result<bool> {
    match fs::metadata(&path) {
        Ok(_) => Ok(true),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => Ok(false),
            _ => Err(e),
        }
    }
}

/// Walk a file tree and return a sorted vector of paths.
pub fn walk_sorted(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    try!(walk_into(path, &mut out));
    Ok(out)
}

fn walk_into(path: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    let mut files = try!(try!(fs::read_dir(path)).map(|e| {
        let e = try!(e);
        Ok((e.path(), try!(e.file_type()).is_dir()))
    }).collect::<io::Result<Vec<(PathBuf, bool)>>>());

    // Don't need to sort by whole path. We're walking in sort-order.
    files.sort_by(|a, b| a.0.file_name().unwrap().cmp(b.0.file_name().unwrap()));
    for (path, is_dir) in files {
        if is_dir {
            try!(walk_into(&path, out));
        } else {
            out.push(path);
        }
    }
    Ok(())
}

/// Concatinate source paths into an output file.
pub fn concat<W, I>(paths: I, output: &mut W) -> io::Result<u64>
    where W: io::Write,
          I: IntoIterator,
          I::Item: AsRef<Path>,
{
    let mut bytes = 0;
    for p in paths {
        bytes += try!(io::copy(&mut try!(File::open(p)), output))
    }
    Ok(bytes)
}
