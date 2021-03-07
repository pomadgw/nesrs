# LDX (LoaD X register)

Affects Flags: N Z

## Addresing Modes

|MODE        |SYNTAX       |HEX |LEN |TIM|
|------------|-------------|----|----|---|
|Immediate   |LDX #$44     |$A2 |2   |2  |
|Zero Page   |LDX $44      |$A6 |2   |3  |
|Zero Page,Y |LDX $44,Y    |$B6 |2   |4  |
|Absolute    |LDX $4400    |$AE |3   |4  |
|Absolute,Y  |LDX $4400,Y  |$BE |3   |4+ |

+ add 1 cycle if page boundary crossed

## Implementation

```rs
$self.x = $memory.read($self.absolute_address, false);
$self.set_nz($self.x);
```