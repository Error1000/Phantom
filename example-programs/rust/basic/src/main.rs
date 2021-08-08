#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &::core::panic::PanicInfo) -> ! {
	mmapedio_show(0xDEADB411, 0);
	loop {}
}

const fn blen(x: usize) -> usize {
	use core::mem;
	// 0000  ... 100
	// |           |
	// bit n ...   bit 0
	// msb   ...   lsb
	
	// left and right are inclusive, a.k.a the value they "point" to is unchecked as well ( as opposed to pointing one to the left/right of where we need to check )
	let mut left: usize  = mem::size_of::<usize>() * 8;
	let mut right: usize = 0;
	while left > right {
		let mid = (left+right)/2; // Rounds to zero (right)
		let left_mask = ((1 << (left-mid)) - 1) << mid;
		if (x & left_mask) != 0 {
			right = mid+1;
		}else{
			left = mid;
		}
	}
	return left;
}


const MMAPED_IO_BASE: *mut usize = 0x10000 as *mut usize;
// SAFTEY: Since slot is a u8 you can't acces more than 256 bytes from the base, in hw the range from 0x1000 and 0x100F is reserved so this is fine
fn mmapedio_show(val: usize, slot: u8){
	unsafe{ ::core::ptr::write_volatile(MMAPED_IO_BASE.offset(slot.into()), val); }
}

#[no_mangle]
pub extern "C" fn _start() {
	mmapedio_show(blen(100), 0);
	loop {}
}
