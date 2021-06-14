#pragma once

unsigned int __umodsi3(unsigned int a, unsigned int b){
	for(; a >= b; a -=b );
	return a;
}
