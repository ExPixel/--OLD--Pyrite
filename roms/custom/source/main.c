#include <gba_console.h>
#include <gba_video.h>
#include <gba_interrupt.h>
#include <gba_systemcalls.h>
#include <gba_input.h>
#include <stdio.h>
#include "e_printf.h"


int main(void) {
	int a = 2374;
	while(a >= 0) {
		a++;
	}
	a += 7;
	a--;
	a *= 3;
	e_printf("The value of a at the end is %d\n", a);
}


