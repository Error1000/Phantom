#include <stdio.h>
#include "ulib/math.h"

int main(){
	int r;
	printf("Running blen test ...\n");
	r = test_blen();
	if(r != 1){
		printf("Blen failed with %d!\n", r);
		return r;
	}
	printf("Running isqrt test ...\n");
	r = test_isqrt();
	if(r != 1){
		printf("Isqrt failed with %d!\n", r ^ 0x80000000);
		return r;
	}
	printf("All testes ran succsessfully!\n");
	return 0;
}
