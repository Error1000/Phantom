#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn _start(){
	let mut c = 0;
	for val in 3..2_u32.pow(30){
		if is_prime(val) {
			mmapedio_show(val, c);
			c += 1;
		}
	}
	loop{}
}

fn is_prime(val: u32) -> bool {
	if val == 2 { return true; }
	if val%2 == 0 { return false; }
	
	for d in (3 .. val).step_by(2) {
		if val%d == 0 { return false; }
	}
	return true;
}


#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
	loop {}
}

fn mmapedio_show(val: u32, slot: u8){
	use core::ptr::write_volatile;
	let base = 0x10000 as *mut u32;
	unsafe{
	 write_volatile(base.offset(slot as isize), val);
	}
}

