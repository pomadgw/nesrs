#!/bin/sh

for rom in 01-basics 03-immediate 05-zp_xy 07-abs_xy 09-ind_y 11-stack 13-rts 15-brk 02-implied 04-zero_page 06-absolute 08-ind_x 10-branches 12-jmp_jsr 14-rti 16-special; do
    ca65 -I common -o $rom.o $rom.s -D OFFICIAL_ONLY
    ld65 -C nes.cfg $rom.o -o $rom.nes
done
