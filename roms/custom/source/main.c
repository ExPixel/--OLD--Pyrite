#include <gba_console.h>
#include <gba_video.h>
#include <gba_interrupt.h>
#include <gba_systemcalls.h>
#include <gba_input.h>
#include <stdio.h>
#include "e_printf.h"
#include "e_debug.h"

#define DISPCNT *((volatile u16*)(0x04000000))
#define VCOUNT *((volatile u16*)(0x04000006))

#define SIODATA32_L *((volatile u16*)(0x4000120))
#define SIODATA32_H *((volatile u16*)(0x4000122))

void wait_for_line(u16 line);
u32 draw_line(u32 line, u32 data);

int main(void) {
	e_println("\"Hello from the other side.\" --Plies");
	DISPCNT = 0x3;
	DISPCNT |= (1 << 10);
	u16 line = 0;
	u32 data = 0;
	while(true) {
		data = draw_line(line, data);
		if(line++ > 159) line = 0;
	}
}

void wait_for_line(u16 line) {
	while((VCOUNT & 0xff) != line);
}

u32 draw_line(u32 line, u32 data) {
	wait_for_line(line + 1);
	volatile u16* frame_buffer = ((u16*)(0x06000000));
	u32 offset = line * 240 * 2;
	*(frame_buffer + offset) = 0xFFFF;
	return data;
}


