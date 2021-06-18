#pragma once

#include "starter.h"

unsigned int __umodsi3(unsigned int a, unsigned int b){
	for(; a >= b; a -=b );
	return a;
}

void *memcpy(void* dest, const void* src, unsigned int count){
	// Count is in bytes
	int *destc = dest;
	const int *srcc = src;
	if(count%sizeof(int) != 0){
		exit(1);
	}
	count /= sizeof(int);
	for(; count > 0; --count) *(destc++) = *(srcc++);
	return dest;
}
