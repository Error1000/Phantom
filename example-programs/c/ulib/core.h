#pragma once

#include "starter.h"

unsigned int __umodsi3(unsigned int a, unsigned int b){
	for(; a >= b; a -=b );
	return a;
}

void* memcpy(void* dest, const void* src, unsigned int count){
	// Count is in bytes
	int *destc = dest;
	const int *srcc = src;
	if(sizeof(int) != 4) exit(505);
	unsigned char offset = count%4;
	count /= sizeof(int);
	for(; count > 1; --count) *(destc++) = *(srcc++);
	// Assumes sizzeof(int) = 4;
	switch(offset){
		case 0:
			*destc = (*srcc & 0xFFFFFFFF) | (*destc & 0x00000000);
			break;
		case 1:
			*destc = (*srcc & 0xFF000000) | (*destc & 0x00FFFFFF);
			break;
		case 2:
			*destc = (*srcc & 0xFFFF0000) | (*destc & 0x0000FFFF);
			break;
		case 3:
			*destc = (*srcc & 0xFFFFFF00) | (*destc & 0x000000FF);
			break;
		default: exit(101);
	}
	return dest;
}
