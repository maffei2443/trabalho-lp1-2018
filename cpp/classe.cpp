#include <iostream>
#include <cstdio>
using namespace std;
class Foo {
public:
	Foo() {
		printf("lack of orthogonality\n");
	}
};

int main() {
	Foo foo;
	Foo foo2 = Foo();	
}