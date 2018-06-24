#include <iostream>
#include <cstdio>

typedef struct {
	int v[10];
} Fu;

void foo(int * v) {
	v[2] = 9999;
}

void foo_fu(Fu fu) {
	fu.v[2] = 9999;
	printf("fu.v[2] == %d\n", fu.v[2]);
}

int main() {
	int v[6] {1,2,3,4,5,6};
	foo(v);
	printf("%d\n", v[2]);
	Fu fu;
	fu.v[2] = 1901;
	foo_fu(fu);
	printf("%d\n", fu.v[2]);
}