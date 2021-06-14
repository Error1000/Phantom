#include <stdbool.h>
#include "ulib/starter.h"
#include "ulib/core.h"
#include "ulib/math.h"

#define MMAPED_IO_BASE 0x10000

// Sidefects may include writing to RAM
void mmapedio_show(int val, unsigned int slot){
	*((volatile int*)MMAPED_IO_BASE + slot) = val;
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
		if(is_prime(i)) mmapedio_show(i, c++);
	return 0;
}
