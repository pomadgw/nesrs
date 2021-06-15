.segment "DATA"
data:
  .byte $1, $2

.segment "VECTORS"
.word nmi
.word reset
.word irq

.segment "CODE"
reset:
	sei
	lda data
  jsr test
loop:
  jmp loop

test:
  ldx #10
  rts

.segment "CODE"
nmi:
  rti

irq:
  rti
