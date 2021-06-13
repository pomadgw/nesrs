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
loop:
  jmp loop

.segment "CODE"
nmi:
  rti

irq:
  rti
