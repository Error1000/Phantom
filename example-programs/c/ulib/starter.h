#pragma once
// No need to make main visible to anything outside the file including this header file
// Right?
static inline int main();

void _start(void){
	main();
	while(1);
}
