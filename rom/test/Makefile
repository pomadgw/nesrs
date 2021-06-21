CONFIG := romconfig.cfg

all: 6502_test.rom test.rom

6502_test.o: 6502_test.s
	$(CA65_BIN) 6502_test.s -g -o 6502_test.o

6502_test.rom: 6502_test.o $(CONFIG)
	$(LD65_BIN) -o 6502_test.rom -C $(CONFIG) 6502_test.o

test.o: test.s
	$(CA65_BIN) test.s -g -o test.o

test.rom: test.o $(CONFIG)
	$(LD65_BIN) -o test.rom -C $(CONFIG) test.o
