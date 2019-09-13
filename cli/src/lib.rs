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

use std::borrow::Cow;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, process};

use clap::{App, Arg, SubCommand};
use gazetta_core::model::Source;
use gazetta_core::render::Gazetta;
use slug::slugify;

use chrono::offset::Local as Date;

// Internal trait to use dynamic dispatch instead of monomorphizing run.
trait RenderPaths {
    fn render_paths(&self, source_path: &Path, dest_path: &Path) -> Result<(), Box<dyn Error>>;
}

impl<G: Gazetta> RenderPaths for G {
    fn render_paths(&self, source_path: &Path, dest_path: &Path) -> Result<(), Box<dyn Error>> {
        let source = Source::new(&source_path)?;
        self.render(&source, &dest_path)?;
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

pub fn gen_completions<P: AsRef<Path>>(name: &str, directory: P) {
    let directory = directory.as_ref();
    let mut app = build_argparser(name);
    app.gen_completions(name, clap::Shell::Bash, directory);
}

/// Build the argument parser.
fn build_argparser(name: &str) -> App {
    App::new(name)
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("SOURCE")
                .short("s")
                .long("source")
                .takes_value(true)
                .help("Specify the source directory (defaults to the current directory)"),
        )
        .subcommand(
            SubCommand::with_name("render")
                .about("Render the website")
                .arg(
                    Arg::with_name("FORCE")
                        .short("f")
                        .long("force")
                        .help("Overwrite any existing DEST."),
                )
                .arg(
                    Arg::with_name("DEST")
                        .required(true)
                        .index(1)
                        .help("Destination directory"),
                ),
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("Create a new page")
                .arg(
                    Arg::with_name("EDIT")
                        .short("e")
                        .long("edit")
                        .help("Edit new page in your $EDITOR"),
                )
                .arg(
                    Arg::with_name("WHERE")
                        .required(true)
                        .index(1)
                        .help("Directory in which to create the page"),
                )
                .arg(
                    Arg::with_name("TITLE")
                        .required(true)
                        .index(2)
                        .help("The page title"),
                ),
        )
}

fn _run(render_paths: &dyn RenderPaths) -> Result<i32, Box<dyn Error>> {
    let name = env::args().next().unwrap();
    let matches = build_argparser(&name).get_matches();
    let source_path: Cow<Path> = matches
        .value_of("SOURCE")
        .map(|v| Cow::Borrowed(v.as_ref()))
        .or_else(|| {
            let mut path = PathBuf::new();
            path.push(".");
            while path.exists() {
                path.push("gazetta.yaml");
                let is_root = path.exists();
                path.pop();
                if is_root {
                    return Some(Cow::Owned(path));
                }
                path.push("..");
            }
            None
        })
        .ok_or("Could not find a gazetta config in this directory or any parent directories.")?;

    match matches.subcommand() {
        ("render", Some(matches)) => {
            let dest_path: &Path = matches.value_of("DEST").unwrap().as_ref();
            if fs::metadata(&dest_path).is_ok() {
                if matches.is_present("FORCE") {
                    fs::remove_dir_all(dest_path).map_err(|e| {
                        format!("Failed to remove '{}': {}", dest_path.display(), e)
                    })?;
                } else {
                    return Err(format!("Target '{}' exists.", dest_path.display()).into());
                }
            }
            render_paths.render_paths(&source_path, dest_path)?;
            Ok(0)
        }
        ("new", Some(matches)) => {
            let mut path: PathBuf = matches.value_of("WHERE").unwrap().into();
            let title = matches.value_of("TITLE").unwrap();
            path.push(slugify(&title));
            if path.exists() {
                return Err(format!("Directory '{}' exists.", path.display()).into());
            }
            fs::create_dir(&path)
                .map_err(|e| format!("Failed to create directory '{}': {}", path.display(), e))?;

            path.push("index.md");
            let mut file = File::create(&path)?;
            writeln!(file, "---")?;
            writeln!(file, "title: {}", &title)?;
            writeln!(file, "date: {}", Date::today().format("%Y-%m-%d"))?;
            writeln!(file, "---")?;
            println!("Created page: {}", path.display());
            if matches.is_present("EDIT") {
                path.pop();
                match Command::new(
                    env::var_os("EDITOR")
                        .as_ref()
                        .map(|p| &**p)
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
        _ => Err(matches.usage().into()),
    }
}
