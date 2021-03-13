# JMP (JuMP)

Affects Flags: none

## Addresing Modes

|MODE        |SYNTAX       |HEX |LEN |TIM|
|------------|-------------|----|----|---|
|Absolute    |JMP $5597    |$4C |3   |3  |
|Indirect    |JMP ($5597)  |$6C |3   |5  |

## Note

JMP transfers program execution to the following address (absolute) or to the location contained in the following address (indirect). Note that there is no carry associated with the indirect jump so:

AN INDIRECT JUMP MUST NEVER USE A
VECTOR BEGINNING ON THE LAST BYTE
OF A PAGE

For example if address $3000 contains $40, $30FF contains $80, and $3100 contains $50, the result of JMP ($30FF) will be a transfer of control to $4080 rather than $5080 as you intended i.e. the 6502 took the low byte of the address from $30FF and the high byte from $3000.

## Implementation

```rs
$self.pc = $self.absolute_address;
```
