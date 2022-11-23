#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &::core::panic::PanicInfo) -> ! {
	mmapedio_show(0xDEADB411, &mut 0);
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
		// Masked filled with ones from left to mid
		let left_mask = ((1 << (left-mid)) - 1) << mid;
		if (x & left_mask) != 0 /* a.k.a if there are ones to the left */{
			// Go left
			right = mid+1;
		}else{
			// Go right
			left = mid;
		}
	}
	return left;
}


const MMAPED_IO_BASE: *mut usize = 0x10000 as *mut usize;
fn mmapedio_show(val: usize, slot: &mut u16) {
	unsafe{ ::core::ptr::write_volatile(MMAPED_IO_BASE.offset(*slot as isize), val); }
	*slot += 1;
}

fn mmapedio_get(slot: &mut u16) -> usize {
	*slot += 1;
	unsafe{ ::core::ptr::read_volatile(MMAPED_IO_BASE.offset(*slot as isize - 1isize)) }
}


const fn integer_sqrt(val: usize) -> usize {
		if val == 0 { return 0; }
		else if val == 1 { return 1; }

		let mut val_len: usize = blen(val);
		if val_len%2 == 1 { val_len += 1; } // Must be even
		let val_len = val_len; // Downgrade mutability

		let mut result: usize = 1;
		let mut w: usize = (val >> (val_len-2))-result; // -1 to subtract result from group
		let mut i = 4;
		loop {
			if i > val_len { break; }
			w <<= 2;
			w |= (val >> (val_len-i)) & 0b11; // pull the top two bits from x
			// Calculate number that we should try to subtract from group
			// The number is t = ((x << 1)+a)*a, where a is a digit and x = result*2
			// t should be smaller than the working group
			// Since we are in binary the only possibilites are either 0 or (result << 2)|1 
			// We calculate the bigger value and then do an if to cover both possibilities
			let t = (result << 2) | 1;
			result <<= 1;
			if w >= t { // Check if t is less than working group
				w -= t; // Subtract if so
				result |= 1; // And put a 1 in the answer
			}else {
				// Don't subtract, instead put a zero and try a new group
				result |= 0;
			}
			i += 2;
		}
		result
}

const fn is_prime(x: usize) -> bool{
	if x == 1 { return false; }
	if x == 2 { return true; }
	if x%2 == 0 { return false; }
	let mut d = 3;
	let xsqrt = integer_sqrt(x);
	loop{
		if d > xsqrt { break; }
		if x%d == 0 { return false; }
		d += 2;
	}
	return true;

}


#[no_mangle]
pub extern "C" fn _start() -> ! {
	let mut cslot  = 0;
	for i in 1.. {
		if is_prime(i){
			mmapedio_show(i, &mut cslot);
		}
	}

	loop { }
}
