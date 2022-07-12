#include <stdbool.h>
#ifdef RISCV_BARE_METAL
 #include "ulib/prelude.h"
#else
 #include <stdio.h>
 #include <stdlib.h>
#endif

#ifdef RISCV_BARE_METAL
	#define show(x) { mmapedio_show(x); }
#else
	#define show(x) { printf("%i\n", x); fflush(stdout); }
#endif



typedef struct {
	int *p;
	int *const e;
} Slice;

unsigned int len(Slice s){
	return s.e - s.p;
}

void merges(Slice a, Slice b, Slice r){
	if(len(r) < len(a)+len(b)) exit(2);
	while(len(a) > 0 && len(b) > 0)
		*(r.p++) = (*a.p < *b.p) ? *(a.p++) : *(b.p++);
	while(len(a) > 0) *(r.p++) = *(a.p++);
	while(len(b) > 0) *(r.p++) = *(b.p++);
}

void merge_sort(Slice v){
	if(len(v) == 1) return;
	if(len(v) == 2){
		if(*v.p > *(v.e-1)){
			*v.p ^= *(v.e-1);
			*(v.e-1) ^= *v.p;
			*v.p ^= *(v.e-1);
		}
		return;
	}
	// End points one past the end but begin does not, so we don't need to change v.p+len(v)/2
	Slice left  = (Slice){v.p, v.p+len(v)/2};
	Slice right = (Slice){v.p+len(v)/2, v.e};
	merge_sort(left);
	merge_sort(right);

	int buff[len(v)];
	Slice r = (Slice){buff, buff+len(v)};
	merges(left, right, r);
	for(unsigned int i = 0; i < len(v); ++i)
		v.p[i] = r.p[i];
}



#define MMAPED_IO_BASE 0x10000

// Sidefects may include writing to RAM
void mmapedio_show(int val){
	static unsigned int slot = 0;
	*((volatile int*)MMAPED_IO_BASE + slot) = val;
	slot++;
}


int main(){
	#define SIZE 11
	int arr[SIZE] = {6, 8, 1, 4, 5, 2, 7, 9, 10, 0, 3};
	for(unsigned int i = 0; i < SIZE; ++i)
		show(arr[i]);
	show(0xFFFF0000);

	merge_sort((Slice){arr, arr+SIZE});

	for(unsigned int i = 0; i < SIZE; ++i)
		show(arr[i]);


	show(-1);
	return 0;
}
