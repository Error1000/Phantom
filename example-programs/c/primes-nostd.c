#include <stdbool.h>


void mmapedio_show(int val, unsigned int slot);
unsigned int blen(unsigned int x);
unsigned int __umodsi3(unsigned int a, unsigned int b);
unsigned int isqrt(unsigned int x);
bool is_prime(unsigned int x);

// Since we use a hack to load programs that dosen't respect elf
// we have to rely on the semi-compiler depentent behaviour that
// the first function in the c code is the first function in the .text
// so that the entry point is the first instruction in the .text
// since we just copy the .text to raw memory from 0x0 for now
void _start(){
        unsigned int c = 0;
        for(unsigned int i = 0; i <= (1 << 31); i++)
                if(is_prime(i)) mmapedio_show(i, c++);
        while(1);
}

#define MMAPED_IO_BASE 0x10000

// Sidefects may include writing to RAM
void mmapedio_show(int val, unsigned int slot){
	*((volatile int*)MMAPED_IO_BASE + slot) = val;
}



// Bit Length, calculates the length, in binary of the number
// A.K.A the number of digits in binary ( without precedding zeros )
unsigned int blen(unsigned int x){
/*
	Old impl:
	unsgined int l = 0;
	for(; x != 0; x >>= 1) l++;
	return l;
*/
	// New impl:
        // Binary search
        unsigned int left = 32;
        unsigned int right = 0;
        while(left > right){
                unsigned int mid = (left+right)/2;
                /// Mask should be from left to mid
                unsigned int mask = ((1 << (left-mid)) - 1) << mid;

                // Necessary parantheses >:(
                if((x & mask) != 0){
                        // We need to go to left side so we adjust right bound
                        right = mid+1;
                }else{
                        // We need to go to right side so we adjust left side
                        left = mid;
                }
        }
        return left;
}


// We don't use stdlib since i can't be bothered to build a rv32e full blown toolchain so i just use the rv64i compiler and "cross-compile" to rv32e
unsigned int __umodsi3(unsigned int a, unsigned int b){
	for(; a >= b; a -= b);
	return a;
}

// Probablly not the most efficient way to sqrt, but gets the job done in about O(log2(n)) where n is the number to sqrt
unsigned int isqrt(unsigned int x){
	if(x == 0) return 0;
	else if(x == 1) return 1;
	unsigned int res = 1;
	unsigned int l = blen(x);
	if(l%2 == 1) l++;
	unsigned int w = (x >> (l-2))-1;

	for(unsigned int i = 4; i <= l; i += 2){
		// Prep for next calc
		// Update working group
		w <<= 2;
		w |= (x >> (l-i)) & 0b11;
		// Find suitable number to try to subtract from working group
		unsigned int t = (res << 2) | 1;
		// Calc
		res <<= 1; // Make space for new bit
		if(w - t >= 0){ // If we can subtract t from w and get a positive answer
			w -= t; // Subtract the number
			res |= 1; // Put a one
		}else{
			// Don't subtract the number, instead wait for the working group calc to try a new group
			res |= 0; // Put a zero
		}
	}
	return res;
}

bool is_prime(unsigned int x){
	if(x == 2) return true;
	if(x%2 == 0) return false;
	for(unsigned int d = 3; d <= isqrt(x); d += 2){
		if(x%d == 0) return false; // If x is divisable by d, then x/d is also a divisor
		/// NOTE: For counting divisors:
		// if(d == x/d) then we need to make sure we don't double count divisors
	}
	return true;
}

