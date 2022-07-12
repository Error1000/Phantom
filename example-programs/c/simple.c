#include "ulib/starter.h"
int val;

int main(){
	// Tests endianness
	int* base = &val;
	*base = 0x12345678;
	// In ram: 0x78563412 at base
	if(*((short*) base) == 0x5678){
		// 4 bytes == 2 shorts == 1 int
		*(((short*)base) + 2/*so we don't overwrite the int at base*/) = 0x12AB;
	}
	return 0;
}
