// #include <threads.h>
// #include <stdlib.h>

static int sub1(void) {
	return sub2(); 
	// return (0); // EXIT_SUCCESS
}

volatile int main(void) {
	int (*func)(void) = sub1;
	return func();
	// return (func() == -1) ? EXIT_FAILURE : EXIT_SUCCESS;
}
