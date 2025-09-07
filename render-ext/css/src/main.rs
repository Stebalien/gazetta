//  Copyright (C) 2025 Steven Allen
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

use std::io::Write;

use gazetta_syntax_css::write_stylesheet;
use autumnus::themes::{self, Theme};

// Use some default theme names that are available in autumnus
const DARK_THEME: &str = "nord";
const LIGHT_THEME: &str = "papercolor_light";

enum ThemeChoice {
    One(&'static Theme),
    Two { light: &'static Theme, dark: &'static Theme },
}

fn get_theme_by_name(name: &str) -> Option<&'static Theme> {
    themes::available_themes().iter().find(|t| t.name == name).copied()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let choice = match &args[..] {
        [theme] => match theme == "DEFAULT" {
            true => ThemeChoice::Two {
                light: get_theme_by_name(LIGHT_THEME).ok_or("light theme not found")?,
                dark: get_theme_by_name(DARK_THEME).ok_or("dark theme not found")?,
            },
            false => {
                // Try to load as a theme name
                if let Some(theme) = get_theme_by_name(theme) {
                    ThemeChoice::One(theme)
                } else {
                    let available: Vec<_> = themes::available_themes().iter().map(|t| &t.name).collect();
                    return Err(format!("Theme '{}' not found. Available themes: {:?}", theme, available).into());
                }
            }
        },
        [light_name, dark_name] => {
            let light = get_theme_by_name(light_name).ok_or_else(|| format!("Light theme '{}' not found", light_name))?;
            let dark = get_theme_by_name(dark_name).ok_or_else(|| format!("Dark theme '{}' not found", dark_name))?;
            ThemeChoice::Two { light, dark }
        },
        _ => {
            eprintln!("Output the default stylesheet:");
            eprintln!("  gazetta-syntax-css DEFAULT");
            eprintln!();
            eprintln!("Convert a theme to a stylesheet:");
            eprintln!("  gazetta-syntax-css dracula");
            eprintln!();
            eprintln!("Convert a light & a dark theme into to a single stylesheet:");
            eprintln!("  gazetta-syntax-css papercolor_light dracula");
            eprintln!();
            let available: Vec<_> = themes::available_themes().iter().map(|t| &t.name).collect();
            eprintln!("Available themes: {:?}", available);
            std::process::exit(1)
        }
    };

    let mut writer = std::io::stdout().lock();
    match choice {
        ThemeChoice::One(theme) => write_stylesheet(&mut writer, theme)?,
        ThemeChoice::Two { light, dark } => {
            writer.write_all(b"@media (prefers-color-scheme: light) {\n")?;
            write_stylesheet(&mut writer, light)?;
            writer.write_all(b"}\n@media (prefers-color-scheme: dark) {\n")?;
            write_stylesheet(&mut writer, dark)?;
            writer.write_all(b"}")?;
        }
    }
    Ok(())
}
