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
use inkjet::theme::Theme;

const DARK_THEME: &str = inkjet::theme::vendored::NORD;
const LIGHT_THEME: &str = inkjet::theme::vendored::PAPERCOLOR_LIGHT;

enum ThemeChoice {
    One(Theme),
    Two { light: Theme, dark: Theme },
}

fn main() -> inkjet::Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let choice = match &args[..] {
        [theme] => match theme == "DEFAULT" {
            true => ThemeChoice::Two {
                light: Theme::from_helix(LIGHT_THEME)?,
                dark: Theme::from_helix(DARK_THEME)?,
            },
            false => ThemeChoice::One(Theme::from_helix(&std::fs::read_to_string(theme)?)?),
        },
        [light, dark] => ThemeChoice::Two {
            light: Theme::from_helix(&std::fs::read_to_string(light)?)?,
            dark: Theme::from_helix(&std::fs::read_to_string(dark)?)?,
        },
        _ => {
            eprintln!("Output the default stylesheet:");
            eprintln!("  gazetta-syntax-css DEFAULT");
            eprintln!();
            eprintln!("Convert a Helix theme to a stylesheet:");
            eprintln!("  gazetta-syntax-css theme.toml");
            eprintln!();
            eprintln!("Convert a light & a dark Helix theme into to a single stylesheet:");
            eprintln!("  gazetta-syntax-css light.toml dark.toml");
            std::process::exit(1)
        }
    };

    let mut writer = std::io::stdout().lock();
    match choice {
        ThemeChoice::One(theme) => write_stylesheet(&mut writer, &theme)?,
        ThemeChoice::Two { light, dark } => {
            writer.write_all(b"@media (prefers-color-scheme: light) {\n")?;
            write_stylesheet(&mut writer, &light)?;
            writer.write_all(b"}\n@media (prefers-color-scheme: dark) {\n")?;
            write_stylesheet(&mut writer, &dark)?;
            writer.write_all(b"}")?;
        }
    }
    Ok(())
}
