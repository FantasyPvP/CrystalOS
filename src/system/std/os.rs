use lazy_static::lazy_static;
use spin::Mutex;
use alloc::{string::String};

lazy_static! {
	pub static ref OS: Mutex<SysInfo> = Mutex::new(SysInfo {
		os: String::from("Zxq5-OS"),
		version: String::from("0.2.2"),
		url: String::from("https://git.zxq5.dev/OsDev/Zxq5-OS")
	});
}

pub struct SysInfo {
	pub os: String,
	pub version: String,
	pub url: String,
}

