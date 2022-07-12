#pragma once

unsigned int isqrt(unsigned int x);
unsigned int blen(unsigned int x);
unsigned int __umodsi3(unsigned int a, unsigned int b);


#ifdef ULIB_COMPILE_TESTS
int test_isqrt();
int test_blen();
#endif
