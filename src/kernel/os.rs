use lazy_static::lazy_static;
use spin::Mutex;
use alloc::{string::String};

lazy_static! {
	pub static ref OS: Mutex<SysInfo> = Mutex::new(SysInfo {
		os: String::from("CrystalOS Alpha"),
		version: String::from("0.2.1"),
	});
}

pub struct SysInfo {
	pub os: String,
	pub version: String,
}

