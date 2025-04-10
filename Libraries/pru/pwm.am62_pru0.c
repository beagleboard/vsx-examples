/* SPDX-License-Identifier: GPL-2.0-or-later */ 
/* Copyright (C) 2025, Jason Kridner, BeagleBoard.org Foundation <jkridner@beagleboard.org> */

/*
 * pwm.am62_pru0.c - Example PWM firmware for AM62
 *
 * References:
 * * SPRUHV6C - PRU Assembly Language Tools v2.3 User's Guide
 * * SPRUHV7C - PRU Optimizing C/C++ Compiler v2.3 User's Guide
 * * SPRULV7B - AM62x Processors Silicon Revision 1.0 Texas Instruments Families of Products Technical Reference Manual
 * * https://git.ti.com/cgit/pru-software-support-package/pru-software-support-package/commit/?id=f3e3996c50b2c7bc7ed1f6f5374f3d8efb501a45
 */

#include <stdint.h>

#define NUM_GPO 20
#pragma DATA_SECTION(duty, ".pru_dmem0")
volatile uint32_t duty[NUM_GPO] = {
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
};

uint32_t count[NUM_GPO] = {
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
};

volatile register unsigned int __R30; /* R30 directly outputs to PRU GPO */

void main() {
	int i, mask;
	__R30 = 0;
	while(1) {
		mask = 0;
		for(i=0; i<NUM_GPO; i++) {
			if(count[i] == 0) {
				if(duty[i] > 0) {
					mask |= (1 << i);	/* Invert GPO i */
					count[i] = duty[i];	/* Start count */
				}
			}
			else {
				/* Need to make this the same number of cycles */
				count[i]--;			/* Decrement counter */
			}
		}
		__R30 ^= mask;					/* Apply updates at once */
	}
}

/* This section is required by Linux to load the firmware using remoteproc */
#pragma DATA_SECTION(pru_remoteproc_ResourceTable, ".resource_table")
#pragma RETAIN(pru_remoteproc_ResourceTable)
struct resource_table {
	uint32_t ver;
	uint32_t num;
	uint32_t reserved[2];
};
struct my_resource_table {
    struct resource_table base;
    uint32_t offset[1];
} pru_remoteproc_ResourceTable = { 1, 0, 0, 0, 0 };
