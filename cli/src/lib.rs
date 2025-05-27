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
use std::io::Write;
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

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Specify the source directory (defaults to the current directory)
    #[arg(short, long, value_name = "DIRECTORY")]
    source: Option<PathBuf>,
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Render {
        /// Overwrite any existing
        #[arg(short, long)]
        force: bool,
        /// The output directory
        destination: PathBuf,
    },
    New {
        /// Edit the new page in your $EDITOR
        #[arg(short, long)]
        edit: bool,
        /// Directory in which to create the page
        directory: PathBuf,
        /// The page title.
        title: String,
    },
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

            let date = ::chrono::Local::now().to_rfc3339();

            path.push("index.md");
            let mut file = File::create(&path)?;
            writeln!(file, "---")?;
            writeln!(file, "title: {}", &title)?;
            writeln!(file, "date: {}", date)?;
            writeln!(file, "updated: {}", date)?;
            writeln!(file, "---")?;
            println!("Created page: {}", path.display());
            if edit {
                path.pop();
                match Command::new(
                    env::var_os("EDITOR")
                        .as_deref()
                        .unwrap_or_else(|| "vim".as_ref()),
                )
                .arg("index.md")
                .current_dir(path)
                .status()
                {
                    Ok(status) => match status.code() {
                        Some(code) => Ok(code),
                        None => Err("Editor was killed.".into()),
                    },
                    Err(e) => Err(format!("Failed to spawn editor: {}", e).into()),
                }
            } else {
                Ok(0)
            }
        }
    }
}
