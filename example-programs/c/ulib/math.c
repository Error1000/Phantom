#include "math.h"

unsigned int blen(unsigned int x){
	unsigned int left = sizeof(x)*8;
	unsigned int right = 0;

	// We increase to go to the left
	while(left > right){
		unsigned int mid = (left+right)/2;
		// Mask, filled with ones from left to mid
		unsigned int mask = ((1 << (left-mid)) - 1) << mid;
		if((x & mask) != 0){
			// We need to go to left side so we adjust right
			right = mid+1;
		}else{
			// We need to go to right side so we adjust left
			left = mid;
		}
	}
	return left;
}

unsigned int isqrt(unsigned int x){
	if(x == 0) return 0;
	else if(x == 1) return 1;
	// Variation of the digit-by-digit algorithm
	unsigned int res = 1;
	unsigned int l = blen(x);
	if(l%2 == 1) l++;
	unsigned int w = (x >> (l-2))-1; // -1 to subtract res from group
	for(unsigned int i = 4; i <= l; i += 2){
		// Update working group
		w <<= 2; // make space
		w |= (x >> (l-i)) & 0b11; // pull the top two bits from x
		// Calculate number that we should try to subtract from group
		// The number should be of the form t = ((x << 1)+a)*a, where a is a digit and x = res*2, and less than the working group
		// Since we are in binary the only possibilites are either (res << 2) | 1 or 0
		// We calculate the bigger value and then do an if to cover both possibilites
		unsigned int t = (res << 2) | 1;
		res <<= 1; // Make space for new result digit
		if(w >= t) { // Check if t is actaully less than working group
			w -= t; // Subtract t if it is less than w
			res |= 1;
		}else{
			// Don't subtract it, instead put a zero and try a new group
			res |= 0;
		}
	}
	return res;
}

unsigned int __umodsi3(unsigned int a, unsigned int b) {
    unsigned int lb = blen(b);

    while(a > b){
        unsigned int la = blen(a);
        if(la == lb) break;
        b <<= (la-lb)-1;
        while(a >= b) a -= b;
        b >>= (la-lb)-1;
    }
    while(a >= b) a -= b;
    return a;
}



#ifdef ULIB_COMPILE_TESTS
int test_blen(){
	unsigned int refb = 0;
	for(unsigned int i = 0; i <= (1 << 21); i++){ // We should have a better way to pick the upper value
		unsigned int i_copy = i;
		unsigned int b = blen(i_copy);
		refb = 0;
		for(; i_copy != 0; i_copy >>= 1) refb += 1;
		if(b != refb) return 2;
	}

	refb = 1;
	for(unsigned int i = 1; i < (1 << 31); i <<= 1){
		if(blen(i) != refb) return 3;
		refb += 1;
	}
	return 1;
}

int test_isqrt(){
	for(unsigned int i = 0; i < (1 << (sizeof(unsigned int)*8/2)); i++)
		if(isqrt(i*i) != i) return (i | 0x80000000);
	return 1;
}
#endif
