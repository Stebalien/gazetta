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
pub use gazetta_core::yaml::Yaml;

lazy_static! {
    pub static ref NAV: Yaml = Yaml::String(String::from("nav"));
    pub static ref NAME: Yaml = Yaml::String("name".into());
    pub static ref ABOUT: Yaml = Yaml::String(String::from("about"));
    pub static ref AUTHOR: Yaml = Yaml::String("author".into());
    pub static ref EMAIL: Yaml = Yaml::String("email".into());
    pub static ref KEY: Yaml = Yaml::String("key".into());
    pub static ref URL: Yaml = Yaml::String("url".into());
    pub static ref FINGERPRINT: Yaml = Yaml::String("fingerprint".into());
    pub static ref NICKNAMES: Yaml = Yaml::String("nicknames".into());
    pub static ref PHOTO: Yaml = Yaml::String("photo".into());
    pub static ref ALSO: Yaml = Yaml::String("also".into());
}
