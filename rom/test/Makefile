all: test.rom

test.o: test.s
	ca65 test.s -g -o test.o

test.rom: test.o test.cfg
	ld65 -o test.rom -C test.cfg test.o
