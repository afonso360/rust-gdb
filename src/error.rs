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

use std::{io, fmt, error, result};

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    ParseError,
    IgnoredOutput
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::IOError(ref err) => write!(f, "{}", err.to_string()),
            &Error::ParseError => write!(f, "cannot parse response from gdb"),
            &Error::IgnoredOutput => write!(f, "ignored output")
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            &Error::IOError(ref err) => Some(err),
            _ => None
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IOError(err)
    }
}