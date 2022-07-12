#pragma once
// No need to make main visible to anything outside the file including this header file.
// Right?
static inline int main();

// code is volatile so that it is put on the stack, so when i analyse the memory manually, i can extract the return code from the stack
static inline void exit(volatile int code){ while(1); }

void _start(void){
	exit(main());
}

