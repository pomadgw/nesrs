.segment "DATA"
data:
  .asciiz "Hello, world"

.segment "VECTORS"
.word nmi
.word reset
.word irq

screen := $0200

.segment "CODE"
reset:
	sei
	lda data
  ldx #0
  ldy #$0a
loop:
  sta screen,x
  inx
  lda data,x
  beq done
  jmp loop

done:
  sta $01,x
  sta ($01),y
  jmp done

.segment "CODE"
nmi:
  rti

irq:
  rti
