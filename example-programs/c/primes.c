#include <stdbool.h>

#ifdef RISCV_BARE_METAL
	#include "ulib/prelude.h"
#else
	#include <stdio.h>
#endif

#ifdef RISCV_BARE_METAL
	#define show(x) mmapedio_show(x)
#else
	#define show(x) printf("%i\n", x)
#endif


#define MMAPED_IO_BASE 0x10000

// Sidefects may include writing to RAM
void mmapedio_show(int val){
	static unsigned int slot = 0;
	*((volatile int*)MMAPED_IO_BASE + slot) = val;
	slot++;
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


int main(){
	unsigned int c = 0;
	for(unsigned int i = 0; i <= (1 << 31); i++)
		if(is_prime(i)) show(i); 
	return 0;
}
