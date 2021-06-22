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
  sta $00
  sta $00,x
  stx $00,y
  sta $1000
  sta $1000,x
  sta $1000,y
  sta $01,x
  sta ($01),y
  lda #$00
  sta $0010
  lda #$80
  sta $0011
  jmp ($0010)

indirect:
  lda #$00
end:
  jmp end

.segment "CODE"
nmi:
  rti

irq:
  rti
