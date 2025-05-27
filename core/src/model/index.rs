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

use glob;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub enum SortField {
    Date,
    #[default]
    Title,
}

impl SortField {
    pub fn default_direction(&self) -> SortDirection {
        use SortDirection::*;
        use SortField::*;
        match self {
            Date => Descending,
            Title => Ascending,
        }
    }
}

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub struct Sort {
    pub field: SortField,
    pub direction: SortDirection,
}

impl Default for Sort {
    fn default() -> Self {
        let field = SortField::default();
        Self {
            field,
            direction: field.default_direction(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Index {
    pub sort: Sort,
    pub directories: Vec<glob::Pattern>,
    pub paginate: Option<u32>,
    pub max: Option<u32>,
    pub compact: bool,
}
