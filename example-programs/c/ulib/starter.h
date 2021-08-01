#pragma once
// No need to make main visible to anything outside the file including this header file
// Right?
static inline int main();

static inline void exit(volatile int code){ while(1); }

void _start(void){
	exit(main());
}

