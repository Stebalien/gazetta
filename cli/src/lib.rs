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

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, process};

use clap::{Parser, Subcommand};
use gazetta_core::model::Source;
use gazetta_core::render::Gazetta;
use slug::slugify;

// Internal trait to use dynamic dispatch instead of monomorphizing run.
trait RenderPaths {
    fn render_paths(&self, source_path: &Path, dest_path: &Path) -> Result<(), Box<dyn Error>>;
}

impl<G: Gazetta> RenderPaths for G {
    fn render_paths(&self, source_path: &Path, dest_path: &Path) -> Result<(), Box<dyn Error>> {
        let source = Source::new(source_path)?;
        self.render(&source, dest_path)?;
        Ok(())
    }
}

/// Run the CLI.
pub fn run<G: Gazetta>(gazetta: G) -> ! {
    process::exit(_run(&gazetta).unwrap_or_else(|e| {
        eprintln!("{}", e);
        1
    }))
}

lazy_static::lazy_static! {
    static ref CLI_NAME: String = std::env::current_exe()
        .ok()
        .and_then(|p| p.file_name().map(|s| s.to_string_lossy().into_owned()))
        .unwrap_or_else(|| "gazetta".into());
}

/// A fast static site generator written in Rust.
#[derive(Parser)]
#[command(version, long_about = None, name = &**CLI_NAME)]
pub struct Cli {
    /// Specify the source directory (defaults to the current directory)
    #[arg(short, long, value_name = "DIRECTORY")]
    source: Option<PathBuf>,
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Render the gazetta site into the target directory.
    Render {
        /// Overwrite any existing
        #[arg(short, long)]
        force: bool,
        /// The output directory
        destination: PathBuf,
    },
    /// Create a new post in the target directory.
    New {
        /// Edit the new page in your $EDITOR
        #[arg(short, long)]
        edit: bool,
        /// Directory in which to create the page
        directory: PathBuf,
        /// The page title.
        title: String,
    },
    /// Edit a the target post.
    Edit {
        /// The file (page) to edit.
        file: PathBuf,
    },
}

fn current_date() -> String {
    ::chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

fn edit_file(path: &Path) -> Result<i32, Box<dyn Error>> {
    let cwd = path
        .parent()
        .ok_or_else(|| format!("path is not a file: {}", path.display()))?;
    let fname: &Path = path
        .file_name()
        .ok_or_else(|| format!("path is not a file: {}", path.display()))?
        .as_ref();
    match Command::new(
        env::var_os("EDITOR")
            .as_deref()
            .unwrap_or_else(|| "vim".as_ref()),
    )
    .arg(fname)
    .current_dir(cwd)
    .status()
    {
        Ok(status) => match status.code() {
            Some(code) => Ok(code),
            None => Err("Editor was killed.".into()),
        },
        Err(e) => Err(format!("Failed to spawn editor: {}", e).into()),
    }
}

fn modify_updated(path: &Path) -> Result<(), Box<dyn Error>> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(false)
        .truncate(false)
        .open(path)?;

    // Read the file.
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // Find the "updated: ..." metadata line.
    let mut reader = std::io::Cursor::new(contents);
    let mut line = String::new();

    // If the file is empty, don't do anything.
    if reader.read_line(&mut line)? == 0 {
        return Ok(());
    }
    // If the file has no metadata, it isn't our problem.
    if line.trim_end() != "---" {
        return Ok(());
    }
    let range = loop {
        line.clear();
        let line_start = reader.position();
        if reader.read_line(&mut line)? == 0 {
            return Err("unexpected end of file metadata".into());
        }
        if line.trim_end() == "---" {
            break line_start..line_start;
        }
        if line.starts_with("updated:") {
            break line_start..reader.position();
        }
    };

    // Replace it with the new date.
    let contents = reader.into_inner();
    let date = current_date();

    file.seek(SeekFrom::Start(0))?;
    file.set_len(0)?;

    let mut writer = BufWriter::new(file);
    writer.write_all(&contents[..range.start as usize])?;
    writeln!(writer, "updated: {date}")?;
    writer.write_all(&contents[range.end as usize..])?;
    writer.flush()?;
    Ok(())
}

fn _run(render_paths: &dyn RenderPaths) -> Result<i32, Box<dyn Error>> {
    let cli = Cli::parse();
    let source_path = cli
        .source
        .or_else(|| {
            let mut path = PathBuf::new();
            path.push(".");
            while path.exists() {
                path.push("gazetta.yaml");
                let is_root = path.exists();
                path.pop();
                if is_root {
                    return Some(path);
                }
                path.push("..");
            }
            None
        })
        .ok_or("Could not find a gazetta config in this directory or any parent directories.")?;

    match cli.commands {
        Commands::Render { force, destination } => {
            if fs::metadata(&destination).is_ok() {
                if force {
                    fs::remove_dir_all(&destination).map_err(|e| {
                        format!("Failed to remove '{}': {}", destination.display(), e)
                    })?;
                } else {
                    return Err(format!("Target '{}' exists.", destination.display()).into());
                }
            }
            render_paths.render_paths(&source_path, &destination)?;
            Ok(0)
        }
        Commands::New {
            edit,
            directory,
            title,
        } => {
            let mut path = directory;
            path.push(slugify(&title));
            if path.exists() {
                return Err(format!("Directory '{}' exists.", path.display()).into());
            }
            fs::create_dir(&path)
                .map_err(|e| format!("Failed to create directory '{}': {}", path.display(), e))?;
            path.push("index.md");

            let mut file = std::io::BufWriter::new(File::create(&path)?);
            let date = current_date();

            writeln!(file, "---")?;
            writeln!(file, "title: {}", &title)?;
            writeln!(file, "date: {}", date)?;
            writeln!(file, "updated: {}", date)?;
            writeln!(file, "---")?;
            println!("Created page: {}", path.display());
            file.flush()?;
            drop(file);

            if edit {
                edit_file(&path)
            } else {
                Ok(0)
            }
        }
        Commands::Edit { file } => match edit_file(&file)? {
            0 => modify_updated(&file).map(|_| 0),
            n => Ok(n),
        },
    }
}
