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

use ::glob;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SortField {
    Date,
    Title,
}

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Default, Clone, Debug, Copy, Eq, PartialEq)]
pub struct Sort {
    pub field: SortField,
    pub direction: SortDirection,
}

impl Default for SortField {
    fn default() -> SortField {
        SortField::Title
    }
}

impl Default for SortDirection {
    fn default() -> SortDirection {
        SortDirection::Descending
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
