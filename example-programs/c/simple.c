#include "ulib/starter.h"

int main(){
	// Tests endianness
	volatile int* base = 0x1000;
	*base = 0x12345678;
	// In ram: 0x78563412 at 0x1000
	if(*((volatile short*) base) == 0x5678){
	// *((volatile short*)0x1002) = 0xCDEF;
	*((volatile short*)0x1004) = 0x12AB;
	}
	return 0;
}
