# LDA (LoaD Accumulator)

Affects Flags: N Z

## Addresing Modes

|MODE        |SYNTAX       |HEX |LEN |TIM|
|------------|-------------|----|----|---|
|Immediate   |LDA #$44     |$A9 |2   |2  |
|Zero Page   |LDA $44      |$A5 |2   |3  |
|Zero Page,X |LDA $44,X    |$B5 |2   |4  |
|Absolute    |LDA $4400    |$AD |3   |4  |
|Absolute,X  |LDA $4400,X  |$BD |3   |4+ |
|Absolute,Y  |LDA $4400,Y  |$B9 |3   |4+ |
|Indirect,X  |LDA ($44,X)  |$A1 |2   |6  |
|Indirect,Y  |LDA ($44),Y  |$B1 |2   |5+ |


+ add 1 cycle if page boundary crossed

## Implementation

```rs
$self.a = $memory.read($self.absolute_address, false);
$self.set_nz($self.a);
```
