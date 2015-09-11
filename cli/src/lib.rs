/*  Copyright (C) 2015 Steven Allen
 *
 *  This file is part of gazetta.
 *
 *  This program is free software: you can redistribute it and/or modify it under the terms of the
 *  GNU General Public License as published by the Free Software Foundation version 3 of the
 *  License.
 *
 *  This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 *  without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See
 *  the GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License along with this program.  If
 *  not, see <http://www.gnu.org/licenses/>.
 */

extern crate gazetta_core;
extern crate chrono;
extern crate clap;
extern crate slug;

use std::{fs, io, process};
use std::process::Command;
use std::io::Write;
use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;

use clap::{Arg, App, SubCommand};
use gazetta_core::render::Gazetta;
use gazetta_core::model::Source;
use slug::slugify;

use chrono::offset::local::Local as Date;

/// Run the CLI.
pub fn run<G: Gazetta>(gazetta: G) -> ! {
    macro_rules! try_exit {
        ($e:expr) => {
            match $e {
                Ok(v) => v,
                Err(e) => bail!("{}", e),
            }
        }
    }

    macro_rules! bail {
        ($($toks:tt)*) => {{
            let stderr = io::stderr();
            let mut stderr = stderr.lock();
            let _ = writeln!(stderr, $($toks)*);
            process::exit(1)
        }}
    }

    let name = &env::args().next().unwrap();
    let matches = App::new(&name)
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("SOURCE")
             .short("s")
             .long("source")
             .takes_value(true)
             .help("Specify the source directory (defaults to the current directory)"))
        .subcommand(SubCommand::with_name("render")
                    .about("Render the website")
                    .arg(Arg::with_name("FORCE")
                         .short("f")
                         .long("force")
                         .help("Overwrite any existing DEST."))
                    .arg(Arg::with_name("DEST")
                         .required(true)
                         .index(1)
                         .help("Destination directory")))
        .subcommand(SubCommand::with_name("new")
                    .about("Create a new page")
                    .arg(Arg::with_name("EDIT")
                         .short("e")
                         .long("edit")
                         .help("Edit new page in your $EDITOR"))
                    .arg(Arg::with_name("WHERE")
                         .required(true)
                         .index(1)
                         .help("Directory in which to create the page"))
                    .arg(Arg::with_name("TITLE")
                         .required(true)
                         .index(2)
                         .help("The page title"))).get_matches();

    let source_path: &Path = matches.value_of("SOURCE").unwrap_or(".").as_ref();
    match matches.subcommand() {
        ("render", Some(matches)) => {
            let dest_path: &Path = matches.value_of("DEST").unwrap().as_ref();
            if fs::metadata(&dest_path).is_ok() {
                if matches.is_present("FORCE") {
                    match fs::remove_dir_all(dest_path) {
                        Ok(_) => (),
                        Err(e) => bail!("Failed to remove '{}': {}", dest_path.display(), e),
                    }
                } else {
                    bail!("Target '{}' exists.", dest_path.display());
                }
            }
            let source = try_exit!(Source::new(&source_path));
            try_exit!(gazetta.render(&source, &dest_path));
        }
        ("new", Some(matches)) => {
            let mut path: PathBuf = matches.value_of("WHERE").unwrap().into();
            let title = matches.value_of("TITLE").unwrap();
            path.push(slugify(&title));
            if fs::metadata(&path).is_ok() {
                bail!("Directory '{}' exists.", path.display());
            }
            if let Err(e) = fs::create_dir(&path) {
                bail!("Failed to create directory '{}': {}", path.display(), e);
            }
            path.push("index.md");
            let mut file = try_exit!(File::create(&path));
            try_exit!(writeln!(file, "---"));
            try_exit!(writeln!(file, "title: {}", &title));
            try_exit!(writeln!(file, "date: {}", Date::today().format("%Y-%m-%d")));
            try_exit!(writeln!(file, "---"));
            println!("Created page: {}", path.display());
            if matches.is_present("EDIT") {
                path.pop();
                match Command::new(env::var_os("EDITOR").as_ref().map(|p|&**p).unwrap_or("vim".as_ref()))
                    .arg("index.md")
                    .current_dir(path)
                    .status()
                {
                    Ok(status) => match status.code() {
                        Some(code) => process::exit(code),
                        None => bail!("Editor was killed."),
                    },
                    Err(e) => bail!("Failed to spawn editor: {}", e),
                }
            }
        },
        _ => {
            bail!("{}", matches.usage());
        }
    }
    process::exit(0)
}
