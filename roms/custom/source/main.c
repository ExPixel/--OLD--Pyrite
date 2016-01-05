#include <gba_console.h>
#include <gba_video.h>
#include <gba_interrupt.h>
#include <gba_systemcalls.h>
#include <gba_input.h>
#include <stdio.h>
// #include "e_printf.h"
// #include "e_debug.h"

#define DMA0SAD *((volatile u32*)(0x40000B0))
#define DMA0SAD_B *((volatile u8*)(0x40000B0))

/**
 * This instruction is used to signal the emulator.
 */
void deadbeef() {
	asm volatile("mov r7, r7" ::: "r7");
	DMA0SAD = 0xdeadbeef;
	int x = DMA0SAD_B;
	while(x);
	while(true);
}

int main(void) {
	deadbeef();
}


