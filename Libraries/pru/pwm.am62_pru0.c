/* SPDX-License-Identifier: GPL-2.0-or-later */ 
/* Copyright (C) 2025, Jason Kridner, BeagleBoard.org Foundation <jkridner@beagleboard.org> */

/*
 * pwm.am62_pru0.c - Example PWM firmware for AM62
 *
 * References:
 * * SPRUHV7C - PRU Optimizing C/C++ Compiler v2.3 User's Guide
 */

#include <stdint.h>

#define NUM_GPO 20
#pragma DATA_SECTION(commands, "pru_sharedmem")
struct cmd {
	uint32_t high;
	uint32_t low;
} commands[NUM_GPO] =
{
	{ 0, 0 },
	{ 0, 0 },
	{ 0, 0 },
	{ 0, 0 },
	{ 0, 0 },
	{ 0, 0 },
	{ 0, 0 },
	{ 0, 0 },
	{ 0, 0 },
	{ 0, 0 },
	{ 0, 0 },
	{ 0, 0 },
	{ 0, 0 },
	{ 0, 0 },
	{ 0, 0 },
	{ 0, 0 },
	{ 0, 0 },
	{ 0, 0 },
	{ 0, 0 },
	{ 0, 0 },
};

volatile register unsigned int __R31;

void main() {
	int i;
	while(1) {
		for(i=0; i<NUM_GPO; i++) {

		}
	}
}
