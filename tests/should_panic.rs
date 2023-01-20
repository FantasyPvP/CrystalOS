
#![no_std]
#![no_main]


use core::panic::PanicInfo;
use CrystalOS::{QemuExitCode, exit, serial_println, serial_print};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	serial_println!("OK");
	exit(QemuExitCode::Ok);
	
	loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
	shouldpanic();
	serial_println!("Err: Test did not panic");
	exit(QemuExitCode::Err);
	loop {}
}

fn shouldpanic() {
	serial_print!("{}...\t", "should_panic::should_panic");
	assert_eq!(1,2);
}
