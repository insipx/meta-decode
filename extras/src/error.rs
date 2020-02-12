// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of substrate-desub.
//
// substrate-desub is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// substrate-desub is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with substrate-desub.  If not, see <http://www.gnu.org/licenses/>.

use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Decode {}", _0)]
    Decode(#[fail(cause)] serde_json::Error)
}


impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Decode(err)
    }
}