#ifndef E_DEBUG_H
#define E_DEBUG_H

/*
 * Creates a debug marker.
 */
#ifndef __thumb__
	#define E_DEBUG(__e_debug_number__)\
		asm volatile("mov r11, #"#__e_debug_number__ ::: "r11");\
		asm volatile("mov r11, r11" ::: "r11")
#else
	#define E_DEBUG(__e_debug_number__)\
		asm volatile("mov r7, #"#__e_debug_number__ ::: "r7");\
		asm volatile("add r7, r7, #0" ::: "r7")
#endif

#endif