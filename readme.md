# VREF Question

This repo creates an example to use VREF using embassy.

As of now, it doesn't appear to work.  The vref out pin remains low, and the ADC
reading gives 4095 (2^12-1).

To replicate, you can take a Nucleo-g0b1re and depopulate the 0 ohm resistor on
SB28.  This will disconnect VREF from VDD. and allow you to test the example.
