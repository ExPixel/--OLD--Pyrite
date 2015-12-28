// #include <gba_console.h>
// #include <gba_video.h>
// #include <gba_interrupt.h>
// #include <gba_systemcalls.h>
// #include <gba_input.h>
// #include <stdio.h>
// #include "e_printf.h"
// #include "e_debug.h"

/**
 * This instruction is used to signal the emulator.
 */
void deadbeef() {
	asm volatile("mov r10, #0xad" ::: "r10");
	asm volatile("mov r11, #0xde" ::: "r11");
	asm volatile("lsl r11, r11, #8" ::: "r11");
	asm volatile("orr r11, r10" ::: "r11", "r10");
	asm volatile("mov r10, #0xbe" ::: "r10");
	asm volatile("lsl r11, r11, #8" ::: "r11");
	asm volatile("orr r11, r10" ::: "r11", "r10");
	asm volatile("mov r10, #0xef" ::: "r10");
	asm volatile("lsl r11, r11, #8" ::: "r11");
	asm volatile("orr r11, r10" ::: "r11", "r10");
	asm volatile("andeq r0, r0" ::: "r0");
	// asm volatile("mov r10, r15" ::: "r10", "r15");
	// asm volatile("mov r15, #0" ::: "r15"); // back to 0
}

int main(void) {
	deadbeef();
}


