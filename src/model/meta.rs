/* Copyright (2015) Stevem Allen
 *
 * This file is part of gazetta.
 * 
 * gazetta-bin is free software: you can redistribute it and/or modify it under the terms of the
 * GNU Affero General Public License (version 3) as published by the Free Software Foundation.
 * 
 * Foobar is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
 * the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU Affero General Public License along with Foobar.  If
 * not, see <http://www.gnu.org/licenses/>.
 */

use ::yaml;

pub trait Meta: Sized {
    fn from_yaml(yaml: yaml::Hash) -> Result<Self, &'static str>;
}

impl Meta for yaml::Hash {
    fn from_yaml(yaml: yaml::Hash) -> Result<Self, &'static str> {
        Ok(yaml)
    }
}

impl Meta for () {
    fn from_yaml(_: yaml::Hash) -> Result<Self, &'static str> {
        Ok(())
    }
}
