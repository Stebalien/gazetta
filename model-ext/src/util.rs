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
pub trait BubbleResult {
    type Value;
    type Error;
    fn bubble_result(self) -> Result<Self::Value, Self::Error>;
}

impl<V, E> BubbleResult for Result<V, E> {
    type Value = V;
    type Error = E;
    fn bubble_result(self) -> Self {
        self
    }
}

impl<I> BubbleResult for Option<I>
    where I: BubbleResult
{
    type Value = Option<I::Value>;
    type Error = I::Error;

    fn bubble_result(self) -> Result<Self::Value, Self::Error> {
        match self {
            Some(v) => match v.bubble_result() {
                Ok(v) => Ok(Some(v)),
                Err(e) => Err(e),
            },
            None => Ok(None),
        }
    }
}

