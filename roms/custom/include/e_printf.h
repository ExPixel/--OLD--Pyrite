#ifndef E_PRINTF_H
#define E_PRINTF_H

#include "e_debug.h"
#include <stdlib.h>
#include <stdarg.h>

/*
4000120h - SIODATA32_L - SIO Normal Communication lower 16bit data (R/W)
4000122h - SIODATA32_H - SIO Normal Communication upper 16bit data (R/W)
*/

#define SIODATA32_L *((volatile u16*)(0x4000120))
#define SIODATA32_H *((volatile u16*)(0x4000122))

void e_printc(char c) {
	SIODATA32_H = 0xdead; // Prepare for writing data.
	SIODATA32_L = (u16) c;
	SIODATA32_H = 0xbeef; // Flush the written data.
}

void e_print(char source[]) {
	u32 idx = 0;
	while(source[idx]) {
		e_printc(source[idx]);
		idx++;
	}
}

void e_println(char source[]) {
	u32 idx = 0;
	while(source[idx]) {
		e_printc(source[idx]);
		idx++;
	}
	e_printc('\n');
}

// void e_printf(char *format, ...) {
// 	va_list va;
// 	va_start(va, format);
// 	char buffer[256];
// 	sprintf(buffer, format, va);
// 	int idx = 0;
// 	char c = '\0';
// 	while(idx < 256 && (c = buffer[idx]) != '\0') {
// 		e_printc(c);
// 		idx++;
// 	}
// }

#endif