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
pub use gazetta_core::yaml::Yaml;
use std::sync::LazyLock;

pub static KEYS: LazyLock<YamlKeys> = LazyLock::new(|| YamlKeys {
    nav: Yaml::String("nav".into()),
    name: Yaml::String("name".into()),
    about: Yaml::String("about".into()),
    author: Yaml::String("author".into()),
    email: Yaml::String("email".into()),
    key: Yaml::String("key".into()),
    url: Yaml::String("url".into()),
    fingerprint: Yaml::String("fingerprint".into()),
    nicknames: Yaml::String("nicknames".into()),
    photo: Yaml::String("photo".into()),
    also: Yaml::String("also".into()),
    robots: Yaml::String("robots".into()),
});

pub struct YamlKeys {
    pub nav: Yaml,
    pub name: Yaml,
    pub about: Yaml,
    pub author: Yaml,
    pub email: Yaml,
    pub key: Yaml,
    pub url: Yaml,
    pub fingerprint: Yaml,
    pub nicknames: Yaml,
    pub photo: Yaml,
    pub also: Yaml,
    pub robots: Yaml,
}
