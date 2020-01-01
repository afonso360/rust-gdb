/*
 * This file is part of rust-gdb.
 *
 * rust-gdb is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * rust-gdb is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with rust-gdb.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::{
    io,
    fmt,
    error::Error,
};


pub type GDBResult<T> = Result<T, GDBError>;

#[derive(Debug)]
pub enum GDBError {
    IOError(io::Error),
    ParseError,
    IgnoredOutput
}

impl fmt::Display for GDBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &GDBError::IOError(ref err) => write!(f, "{}", err.to_string()),
            &GDBError::ParseError => write!(f, "cannot parse response from gdb"),
            &GDBError::IgnoredOutput => write!(f, "ignored output")
        }
    }
}

impl Error for GDBError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            &GDBError::IOError(ref err) => Some(err),
            _ => None
        }
    }
}

impl From<io::Error> for GDBError {
    fn from(err: io::Error) -> GDBError {
        GDBError::IOError(err)
    }
}