.segment "DATA"
data:
  .asciiz "Hello, world"

.segment "VECTORS"
.word nmi
.word reset
.word irq

SCREEN = $2200

.segment "CODE"
reset:
	sei
  ldx #0
	lda data,x
  ldy #$0a
loop:
  sta SCREEN,x
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
