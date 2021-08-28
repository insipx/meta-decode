// Copyright 2019 Parity Technologies (UK) Ltd.
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

use crate::error::Error;
use core::{regex, EnumField, RustTypeMarker, SetField, StructField};
use serde::{
	de::{Deserializer, MapAccess, Visitor},
	Deserialize, Serialize,
};
use std::{collections::HashMap, fmt};

/// Types for each substrate Module
#[derive(Serialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Modules {
	/// module name -> Type Map of module
	modules: HashMap<String, ModuleTypes>,
}

impl Modules {
	/// Construct this struct from JSON
	pub fn new(raw_json: &str) -> Result<Self, Error> {
		let modules: Modules = serde_json::from_str(raw_json)?;
		Ok(modules)
	}

	pub fn get(&self, ty: &str) -> Option<&ModuleTypes> {
		self.modules.get(ty)
	}

	pub fn get_type(&self, module: &str, ty: &str) -> Option<&RustTypeMarker> {
		self.modules.get(module)?.types.get(ty)
	}

	/// Iterate over all the types in each module
	pub fn iter_types(&self) -> impl Iterator<Item = (&String, &RustTypeMarker)> {
		self.modules.values().map(|v| v.types.iter()).flatten()
	}
}

/// Map of types to their Type Markers
#[derive(Serialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct ModuleTypes {
	/// Type Name -> Type
	types: HashMap<String, RustTypeMarker>,
}

impl ModuleTypes {
	pub fn get(&self, ty: &str) -> Option<&RustTypeMarker> {
		self.types.get(ty)
	}

	/// Merges a ModuleTypes struct with another, to create a new HashMap
	/// The `other` struct takes priority if there are type conflicts
	pub fn merge(&self, other: &ModuleTypes) -> ModuleTypes {
		let mut types = self.types.clone();
		let other = other.clone();
		types.extend(other.types.into_iter());

		ModuleTypes { types }
	}
}

impl<'de> Deserialize<'de> for Modules {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct ModulesVisitor;

		impl<'de> Visitor<'de> for ModulesVisitor {
			type Value = Modules;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("map types")
			}

			fn visit_map<V>(self, mut map: V) -> Result<Modules, V::Error>
			where
				V: MapAccess<'de>,
			{
				let mut modules: HashMap<String, ModuleTypes> = HashMap::new();
				while let Some(key) = map.next_key::<&str>()? {
					let val: ModuleTypes = map.next_value()?;
					modules.insert(key.to_string(), val);
				}
				Ok(Modules { modules })
			}
		}
		deserializer.deserialize_map(ModulesVisitor)
	}
}

impl<'de> Deserialize<'de> for ModuleTypes {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_map(ModuleTypeVisitor)
	}
}

struct ModuleTypeVisitor;

impl<'de> Visitor<'de> for ModuleTypeVisitor {
	type Value = ModuleTypes;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("Map or string")
	}

	fn visit_map<V>(self, mut map: V) -> Result<ModuleTypes, V::Error>
	where
		V: MapAccess<'de>,
	{
		let mut module_types: HashMap<String, RustTypeMarker> = HashMap::new();

		while let Some(key) = map.next_key::<&str>()? {
			match key {
				// skip over "types" key, this encapsulates the types we actually care
				// about
				"types" => {
					let val: serde_json::Value = map.next_value()?;
					let val = val.as_object().expect("Types must refer to an object");
					for (key, val) in val.iter() {
						parse_mod_types(&mut module_types, key, val);
					}
				}
				m => {
					let val: serde_json::Value = map.next_value()?;
					//let val = val.as_object().expect("Types must refer to an object");
					parse_mod_types(&mut module_types, m, &val);
				}
			}
		}
		Ok(ModuleTypes { types: module_types })
	}
}

// FIXME: This whole function should return a Result<_,_>
fn parse_mod_types(module_types: &mut HashMap<String, RustTypeMarker>, key: &str, val: &serde_json::Value) {
	if val.is_string() {
		module_types.insert(key.to_string(), regex::parse(val.as_str().expect("Checked; qed")).expect("not a type"));
	} else if val.is_object() {
		let obj = val.as_object().expect("checked for object before unwrap; qed");
		if obj.contains_key("_enum") {
			module_types.insert(key.to_string(), parse_enum(&obj["_enum"]).unwrap()); // FIXME
		} else if obj.contains_key("_set") {
			let obj = obj["_set"].as_object().expect("_set is a map");
			module_types.insert(key.to_string(), parse_set(obj));
		} else if obj.contains_key("_alias") {
			let mut fields = Vec::new();
			for (key, val) in obj.iter() {
				if key == "_alias" {
					continue;
				} else {
					let field = StructField::new(key, regex::parse(&val_to_str(val)).expect("Not a type"));
					fields.push(field);
				}
			}
			module_types.insert(key.to_string(), RustTypeMarker::Struct(fields));
		} else {
			let mut fields = Vec::new();
			for (key, val) in obj.iter() {
				let field = StructField::new(key, regex::parse(&val_to_str(val)).expect("Not a type"));
				fields.push(field);
			}
			module_types.insert(key.to_string(), RustTypeMarker::Struct(fields));
		}
	}
}

/// internal api to convert a serde value to str
///
/// # Panics
/// panics if the value is not a string
fn val_to_str(v: &serde_json::Value) -> String {
	v.as_str().expect("will be string").to_string()
}

/*
/// In Polkadot-JS Definitions, an object can be:
/// - Struct (no identifier),
/// - Enum (`_enum` identifier)
/// - Set (`_set`)
/// This functions decides which is what and dispatches a call
/// to the appropriate parse fn.
fn deliberate_object(_obj: serde_json::Map<String, serde_json::Value>) -> Result<RustTypeMarker, Error> {
	todo!();
}
*/

// TODO: Account for 'bitlength' in _set
fn parse_set(obj: &serde_json::map::Map<String, serde_json::Value>) -> RustTypeMarker {
	let mut set_vec = Vec::new();
	for (key, value) in obj.iter() {
		let num: u8 = serde_json::from_value(value.clone()).expect("Must be u8");
		let set_field = SetField::new(key, num);
		set_vec.push(set_field)
	}
	RustTypeMarker::Set(set_vec)
}

/// Process the enum and return the representation as a Rust Type
///
/// # Panics
fn parse_enum(value: &serde_json::Value) -> Result<RustTypeMarker, Error> {
	// println!("{:?}", value);
	if value.is_array() {
		let arr = value.as_array().expect("checked before cast; qed");
		let rust_enum = arr.iter().map(|u| {
			let name = u.as_str().expect("Will be string according to polkadot-js defs").to_string();
			EnumField::new(name, None)
		}).collect::<Vec<_>>();
		Ok(RustTypeMarker::Enum(rust_enum))
	} else if value.is_object() {
		let value = value.as_object().expect("Checked before casting; qed");
		// If all the values are numbers then we need to order the enum according to those numbers.
		// Some types like `ProxyType` in the runtime may vary from chain-to-chain.
		// So afaict Polkadot-Js types solve this by attaching a number to each variant according to index.
		let rust_enum = if value.values().fold(true, |_, v| v.is_number()) { // TODO: TEST THIS AGAINST PROXYTYPE
			let mut tuples = value.values()
				.map(|v| v.as_u64().expect("Must be u64"))
				.zip(
					value.keys().map(|k| EnumField::new(k.into(), None))
				)
				.collect::<Vec<(u64, EnumField)>>();
			tuples.sort_by_key(|(num, _)| *num);
			tuples.into_iter().map(|t| t.1).collect::<Vec<_>>()
		} else {
			let mut rust_enum = Vec::new();
			for (key, value) in value.iter() {
				match value {
					serde_json::Value::Null => rust_enum.push(EnumField::new(key.into(), Some(RustTypeMarker::Null))),
					serde_json::Value::String(s) => {
						let field = regex::parse(s).ok_or(Error::from(s.to_string()))?;
						rust_enum.push(EnumField::new(key.into(), Some(field)));
					},
					serde_json::Value::Object(o) => {
						let rust_struct = parse_struct(o)?;
						rust_enum.push(EnumField::new(key.into(), Some(rust_struct)));
					},
					_ => return Err(Error::UnexpectedType),
				};
			}
			rust_enum
		};
		Ok(RustTypeMarker::Enum(rust_enum))
	} else {
		panic!("Unkown type")
	}
}

/// Parses a rust struct representation from a JSON Map.
fn parse_struct(rust_struct: &serde_json::Map<String, serde_json::Value>) -> Result<RustTypeMarker, Error> {
	let mut fields = Vec::new();
	for (key, value) in rust_struct.iter() {
		match value {
			serde_json::Value::Null => {
				let field = StructField::new(key, RustTypeMarker::Null);
				fields.push(field);
			},
			serde_json::Value::String(s) => { // points to some other type
				let ty = regex::parse(s).ok_or(s.to_string())?;
				let field = StructField::new(key, ty);
				fields.push(field);
			},
			serde_json::Value::Object(o) => { // struct-within-a-struct
				let inner_struct = parse_struct(o)?;
				let field = StructField::new(key, inner_struct);
				fields.push(field);
			},
			serde_json::Value::Array(a) => {
				let tuples = parse_tuple(a)?;
				let field = StructField::new(key, tuples);
				fields.push(field);
			},
			_ => return Err(Error::UnexpectedType),
		}
	}
	Ok(RustTypeMarker::Null)
}

fn parse_tuple(json_tuple: &[serde_json::Value]) -> Result<RustTypeMarker, Error> {
	let mut tuple = Vec::new();
	for value in json_tuple.iter() {
		match value {
			serde_json::Value::Null => tuple.push(RustTypeMarker::Null),
			serde_json::Value::String(s) => {
				let ty = regex::parse(s).ok_or(s.to_string())?;
				tuple.push(ty);
			},
			_ => return Err(Error::UnexpectedType)
		}
	}
	Ok(RustTypeMarker::Tuple(tuple))
}

#[cfg(test)]
mod tests {
	use super::Modules;

	use crate::error::Error;
	use crate::ModuleTypes;
	use core::{EnumField, RustTypeMarker, SetField, StructField};
	use std::collections::HashMap;
	const RAW_JSON: &'static str = r#"
{
	"runtime": {
		"types": {
			"Extrinsic": "GenericExtrinsic",
			"hash": "H512",
			"BlockNumber": "u64",
			"ChangesTrieConfiguration": {
				"digestInterval": "u32",
				"digestLevels": "u32"
			},
			"DispatchInfo": {
				"weight": "Weight",
				"class": "DispatchClass",
				"paysFee": "bool"
			},
			"MultiSignature": {
				"_enum": {
					"Ed25519": "Ed25519Signature",
					"Sr25519": "Sr25519Signature",
					"Ecdsa": "EcdsaSignature"
				}
			},
			"Reasons": {
				"_enum": [
					"Fee",
					"Misc",
					"All"
				]
			},
			"WithdrawReasons": {
				"_set": {
					"TransactionPayment": 1,
					"Transfer": 2,
					"Reserve": 4,
					"Fee": 8,
					"Tip": 16
				}
			}
		}
	}
}
"#;

	#[test]
	fn should_deserialize_modules() -> Result<(), Error> {
		let deser_dot_types = Modules::new(RAW_JSON)?;
		let mut modules = HashMap::new();
		let mut types = HashMap::new();
		types.insert("Extrinsic".to_string(), RustTypeMarker::TypePointer("GenericExtrinsic".to_string()));
		types.insert("hash".to_string(), RustTypeMarker::TypePointer("H512".to_string()));
		types.insert("BlockNumber".to_string(), RustTypeMarker::U64);
		types.insert(
			"ChangesTrieConfiguration".to_string(),
			RustTypeMarker::Struct(vec![
				StructField { name: "digestInterval".to_string(), ty: RustTypeMarker::U32 },
				StructField { name: "digestLevels".to_string(), ty: RustTypeMarker::U32 },
			]),
		);
		types.insert(
			"DispatchInfo".to_string(),
			RustTypeMarker::Struct(vec![
				StructField { name: "weight".to_string(), ty: RustTypeMarker::TypePointer("Weight".to_string()) },
				StructField { name: "class".to_string(), ty: RustTypeMarker::TypePointer("DispatchClass".to_string()) },
				StructField { name: "paysFee".to_string(), ty: RustTypeMarker::Bool },
			]),
		);
		types.insert(
			"MultiSignature".to_string(),
			RustTypeMarker::Enum(vec![
				EnumField {
					name: "Ed25519".to_string(),
					value: Some(RustTypeMarker::TypePointer("Ed25519Signature".to_string())),
				},
				EnumField {
					name: "Sr25519".to_string(),
					value: Some(RustTypeMarker::TypePointer("Sr25519Signature".to_string())),
				},
				EnumField {
					name: "Ecdsa".to_string(),
					value: Some(RustTypeMarker::TypePointer("EcdsaSignature".to_string())),
				},
			]),
		);
		types.insert(
			"Reasons".to_string(),
			RustTypeMarker::Enum(vec![
				EnumField { name: "Fee".into(), value: None },
				EnumField { name: "Misc".into(), value: None },
				EnumField { name: "All".into(), value: None },
			]),
		);
		types.insert(
			"WithdrawReasons".to_string(),
			RustTypeMarker::Set(vec![
				SetField { name: "TransactionPayment".to_string(), num: 1 },
				SetField { name: "Transfer".to_string(), num: 2 },
				SetField { name: "Reserve".to_string(), num: 4 },
				SetField { name: "Fee".to_string(), num: 8 },
				SetField { name: "Tip".to_string(), num: 16 },
			]),
		);

		for (key, val) in types.iter() {
			assert_eq!(val, &deser_dot_types.modules["runtime"].types[key]);
		}

		let mod_types = ModuleTypes { types };
		modules.insert("runtime".to_string(), mod_types);
		let dot_types = Modules { modules };
		assert_eq!(dot_types, deser_dot_types);
		Ok(())
	}
}
