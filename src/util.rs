use std::path::Path;
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
        let from = dir_entry.path();
        let to = dest.join(from.relative_from(src).unwrap());
        if fs::metadata(&to).is_ok() {
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

