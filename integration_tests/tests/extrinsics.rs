extern crate extras;

// TODO: Make test structure into a macro

use crate::test_suite;
use desub_core::decoder::{Chain, Decoder, Metadata};

pub fn init() {
	let _ = pretty_env_logger::try_init();
}

#[test]
pub fn should_decode_ext342962() {
	init();
	let types = extras::TypeResolver::default();
	let mut decoder = Decoder::new(types, Chain::Kusama);

	let (meta, ext) = test_suite::extrinsics_block_342962();
	let meta = Metadata::new(meta.as_slice());
	let ext = ext.into_iter().flatten().collect::<Vec<u8>>();

	// block 6 of KSM CC3 is spec 1020
	decoder.register_version(1031, &meta);
	let ext = decoder.decode_extrinsics(1031, &ext).expect("should decode");

	// assert_eq!(vec![("now".to_string(), SubstrateType::U64(1577070096000))], decoded);
	// 1577070096000 is the UNIX timestamp in milliseconds of
	// Monday, December 23, 2019 3:01:36 AM
	// when block 342,962 was processed
}

#[test]
pub fn should_decode_ext422871() {
	init();
	let types = extras::TypeResolver::default();
	let mut decoder = Decoder::new(types, Chain::Kusama);

	let (meta, ext) = test_suite::extrinsics_block_422871();
	let meta = Metadata::new(meta.as_slice());
	let ext = ext.into_iter().flatten().collect::<Vec<u8>>();

	decoder.register_version(1031, &meta);

	let ext = decoder.decode_extrinsics(1031, &ext).expect("should decode");

}

#[test]
pub fn should_decode_ext50970() {
	init();
	let types = extras::TypeResolver::default();
	let mut decoder = Decoder::new(types, Chain::Kusama);

	let (meta, ext) = test_suite::extrinsics_block_50970();
	let meta = Metadata::new(meta.as_slice());
	decoder.register_version(1031, &meta);
	let ext = ext.into_iter().flatten().collect::<Vec<u8>>();
	let ext = decoder.decode_extrinsics(1031, &ext).expect("should decode");

}

#[test]
pub fn should_decode_ext_106284() {
	init();

	let types = extras::TypeResolver::default();
	let mut decoder = Decoder::new(types, Chain::Kusama);

	let (meta, ext) = test_suite::extrinsics_block_106284();
	let meta = Metadata::new(meta.as_slice());

	decoder.register_version(1042, &meta);
	let ext = ext.into_iter().flatten().collect::<Vec<u8>>();
	let ext = decoder.decode_extrinsics(1042, &ext).expect("should decode");

}

#[test]
pub fn should_decode_ext_1674683() {
	init();

	let types = extras::TypeResolver::default();
	let mut decoder = Decoder::new(types, Chain::Kusama);

	let (meta, ext) = test_suite::extrinsics_block_1674683();
	let meta = Metadata::new(meta.as_slice());

	decoder.register_version(1055, &meta);
	let ext = ext.into_iter().flatten().collect::<Vec<u8>>();
	let ext = decoder.decode_extrinsics(1055, &ext).expect("should decode");

}

#[test]
pub fn should_decode_ext_1677621() {
	init();

	let types = extras::TypeResolver::default();
	let mut decoder = Decoder::new(types, Chain::Kusama);

	let (meta, ext) = test_suite::extrinsics_block_1677621();
	let meta = Metadata::new(meta.as_slice());

	decoder.register_version(1055, &meta);

	let ext = ext.into_iter().flatten().collect::<Vec<u8>>();
	let ext = decoder.decode_extrinsics(1055, &ext).expect("should decode");
}

#[test]
fn should_decode_ext_1702023() {
	init();

	let types = extras::TypeResolver::default();
	let mut decoder = Decoder::new(types, Chain::Kusama);

	let (meta, ext) = test_suite::extrinsics_block_1702023();
	let meta = Metadata::new(meta.as_slice());

	decoder.register_version(1055, &meta);
	let ext = ext.into_iter().flatten().collect::<Vec<u8>>();
	let ext = decoder.decode_extrinsics(1055, &ext).expect("should decode");
}

#[test]
fn should_decode_ext_1714495() {
	init();

	let types = extras::TypeResolver::default();
	let mut decoder = Decoder::new(types, Chain::Kusama);

	let (meta, ext) = test_suite::extrinsics_block_1714495();
	let meta = Metadata::new(meta.as_slice());

	decoder.register_version(1055, &meta);
	let ext = ext.into_iter().flatten().collect::<Vec<u8>>();

}

#[test]
fn should_decode_ext_1717926() {
	init();
	let types = extras::TypeResolver::default();
	let mut decoder = Decoder::new(types, Chain::Kusama);

	let (meta, ext) = test_suite::extrinsics_block_1717926();
	let meta = Metadata::new(meta.as_slice());

	decoder.register_version(1055, &meta);
	let ext = ext.into_iter().flatten().collect::<Vec<u8>>();
	let ext = decoder.decode_extrinsics(1055, &ext).expect("should decode");
}

#[test]
fn should_decode_ext_1718223() {
	init();
	let types = extras::TypeResolver::default();
	let mut decoder = Decoder::new(types, Chain::Kusama);

	let (meta, ext) = test_suite::extrinsics_block_1718223();
	let meta = Metadata::new(meta.as_slice());

	decoder.register_version(1055, &meta);
	let ext = ext.into_iter().flatten().collect::<Vec<u8>>();
	let ext = decoder.decode_extrinsics(1055, &ext).expect("should decode");
}

#[test]
fn should_decode_ext_1732321() {
	init();

	let types = extras::TypeResolver::default();
	let mut decoder = Decoder::new(types, Chain::Kusama);

	let (meta, ext) = test_suite::extrinsics_block_1732321();
	let meta = Metadata::new(meta.as_slice());

	decoder.register_version(1055, &meta);
	let ext = ext.into_iter().flatten().collect::<Vec<u8>>();
	decoder.decode_extrinsics(1055, &ext).expect("should decode");

}

#[test]
fn should_decode_ext_1731904() {
	init();

	let types = extras::TypeResolver::default();
	let mut decoder = Decoder::new(types, Chain::Kusama);

	let (meta, ext) = test_suite::extrinsics_block_1731904();
	let meta = Metadata::new(meta.as_slice());

	decoder.register_version(1055, &meta);
	let ext = ext.into_iter().flatten().collect::<Vec<u8>>();
	decoder.decode_extrinsics(1055, &ext).expect("should decode");
}

#[test]
fn should_decode_ext_1768321() {
	init();

	let types = extras::TypeResolver::default();
	let mut decoder = Decoder::new(types, Chain::Kusama);

	let (meta, ext) = test_suite::extrinsics_block_1768321();
	let meta = Metadata::new(meta.as_slice());

	decoder.register_version(1055, &meta);
	let ext = ext.into_iter().flatten().collect::<Vec<u8>>();
	decoder.decode_extrinsics(1055, &ext).expect("should decode");
}

#[test]
fn should_decode_ext_6144() {
	init();

	let types = extras::TypeResolver::default();
	let mut decoder = Decoder::new(types, Chain::Kusama);

	let (meta, ext) = test_suite::extrinsics_block_6144();
	let meta = Metadata::new(meta.as_slice());

	decoder.register_version(1020, &meta);
	let ext = ext.into_iter().flatten().collect::<Vec<u8>>();
	decoder.decode_extrinsics(1020, &ext).expect("should decode");
}

#[test]
fn should_decode_ext_779410_ksm() {
	init();

	let types = extras::TypeResolver::default();
	let mut decoder = Decoder::new(types, Chain::Kusama);

	let (meta, ext) = test_suite::extrinsics_block_779410();
	let meta = Metadata::new(meta.as_slice());

	decoder.register_version(1042, &meta);
	let ext = ext.into_iter().flatten().collect::<Vec<u8>>();
	decoder.decode_extrinsics(1042, &ext).expect("should decode");
}

#[test]
fn should_decode_ext_899638_ksm() {
	init();
	let types = extras::TypeResolver::default();
	let mut decoder = Decoder::new(types, Chain::Kusama);

	let (meta, ext) = test_suite::extrinsics_block_899638();
	let meta = Metadata::new(meta.as_slice());

	decoder.register_version(1042, &meta);
	let ext = ext.into_iter().flatten().collect::<Vec<u8>>();
	decoder.decode_extrinsics(1042, &ext).expect("should decode");
}

#[test]
fn should_decode_ext_233816_ksm() {
	init();

	let types = extras::TypeResolver::default();
	let mut decoder = Decoder::new(types, Chain::Kusama);

	let (meta, ext) = test_suite::extrinsics_block_233816();
	let meta = Metadata::new(meta.as_slice());

	decoder.register_version(1030, &meta);
	let ext = ext.into_iter().flatten().collect::<Vec<u8>>();
	decoder.decode_extrinsics(1030, &ext).expect("should decode");

}

#[test]
fn should_decode_ext_607421_ksm() {
	init();

	let types = extras::TypeResolver::default();
	let mut decoder = Decoder::new(types, Chain::Kusama);

	let (meta, ext) = test_suite::extrinsics_block_607421();
	let meta = Metadata::new(meta.as_slice());

	decoder.register_version(1039, &meta);
	let ext = ext.into_iter().flatten().collect::<Vec<u8>>();
	decoder.decode_extrinsics(1039, &ext).expect("should decode");

}

#[test]
fn should_decode_ext_892_dot() {
	init();

	let types = extras::TypeResolver::default();
	let mut decoder = Decoder::new(types, Chain::Polkadot);

	let (meta, ext) = test_suite::extrinsics_block_892();
	let meta = Metadata::new(meta.as_slice());

	decoder.register_version(0, &meta);
	let ext = ext.into_iter().flatten().collect::<Vec<u8>>();
	decoder.decode_extrinsics(0, &ext).expect("should decode");

}

#[test]
fn should_decode_ext_1191_wnd() {
	init();

	let types = extras::TypeResolver::default();
	let mut decoder = Decoder::new(types, Chain::Polkadot);

	let (meta, ext) = test_suite::extrinsics_block_1191();
	let meta = Metadata::new(meta.as_slice());

	decoder.register_version(0, &meta);
	let ext = ext.into_iter().flatten().collect::<Vec<u8>>();
	decoder.decode_extrinsics(0, &ext).expect("should decode");
}
