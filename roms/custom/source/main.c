#include <gba_console.h>
#include <gba_video.h>
#include <gba_interrupt.h>
#include <gba_systemcalls.h>
#include <gba_input.h>
#include <stdio.h>
#include "e_printf.h"
#include "e_debug.h"


int main(void) {
	E_DEBUG(7);
	e_printf("The value of a at the end is %d\n", 43);
	E_DEBUG(8);
}


