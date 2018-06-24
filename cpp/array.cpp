#include <iostream>
#include <cstdio>

typedef struct {
	int v[10];
} Fu;

void foo(int * v) {
	v[2] = 9999;
}

void foo_fu(int * v) {
	v[2] = 9999;
}

int main() {
	int v[6] {1,2,3,4,5,6};
	foo(v);
	printf("%d\n", v[2]);
	Fu fu;
	fu
}