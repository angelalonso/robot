#!/usr/bin/env bash

# test connect:
# sudo avrdude -c linuxgpio -p atmega328p -v
HEX_SECS=000_blick_internal_led_seconds.ino.hex
HEX_SECS_INV=000_blick_internal_led_seconds_inverted.ino.hex
while true
do
	sudo avrdude -c linuxgpio -p atmega328p -v -U flash:w:$HEX_SECS:i 
	sleep 5
	sudo avrdude -c linuxgpio -p atmega328p -v -U flash:w:$HEX_SECS_INV:i
	sleep 5
done
