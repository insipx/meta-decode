// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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

use crate::ScaleInfoTypeId;

/// This represents the ID of a type found in the metadata. A scale info type representation can
/// be converted into this, and we get this back directly when decoding types into Values.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
pub struct TypeId(u32);

impl TypeId {
	/// Create a new `TypeId` from a `u32`.
	pub(crate) fn from_u32(id: u32) -> TypeId {
		TypeId(id)
	}
	/// Return the u32 ID expected by a PortableRegistry.
	pub(crate) fn id(self) -> u32 {
		self.0
	}
}

impl From<ScaleInfoTypeId> for TypeId {
	fn from(id: ScaleInfoTypeId) -> Self {
		TypeId(id.id())
	}
}

impl From<&ScaleInfoTypeId> for TypeId {
	fn from(id: &ScaleInfoTypeId) -> Self {
		TypeId(id.id())
	}
}

impl From<&TypeId> for TypeId {
	fn from(id: &TypeId) -> Self {
		*id
	}
}
