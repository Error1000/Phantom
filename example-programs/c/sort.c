#include <stdbool.h>
#include "ulib/core.h"
#include "ulib/starter.h"

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

// #include <stdio.h>

#define MMAPED_IO_BASE 0x10000

// Sidefects may include writing to RAM
void mmapedio_show(int val, unsigned int slot){
        *((volatile int*)MMAPED_IO_BASE + slot) = val;
/*
	printf("%d ", val);
	fflush(stdout);
*/
}

int main(){
	#define SIZE 11
	unsigned int slot = 0;
	int arr[] = {6, 8, 1, 4, 5, 2, 7, 9, 10, 0, 3};
	for(unsigned int i = 0; i < SIZE; ++i)
		mmapedio_show(arr[i], slot++);
	mmapedio_show(0xFFFF0000, slot++);

	merge_sort((Slice){arr, arr+SIZE});

	for(unsigned int i = 0; i < SIZE; ++i)
		mmapedio_show(arr[i], slot++);


	mmapedio_show(-1, slot++);
	return 0;
}
