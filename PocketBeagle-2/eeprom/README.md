# EEPROM Example

## Goal

Read and parse contents of built-in EEPROM to check the hardware information.

## Overview

Boards by BeagleBoard.org contain an [EEPROM](https://en.wikipedia.org/wiki/EEPROM) to store the information regarding the exact board revision for easy debugging and transparency. This example demonstrates reading and parsing PocketBeagle 2 EEPROM contents, to use it for board detection, etc.

This example also serves as a great reference on parsing byte information in the language.

## Challenges

1. Can you figure out the EEPROM format, with the byte length of each field?
