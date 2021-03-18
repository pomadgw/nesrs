# INC (INCrement memory)

Affects Flags: N Z

## Addresing Modes

|MODE        |SYNTAX       |HEX |LEN |TIM|
|------------|-------------|----|----|---|
|Zero Page   |INC $44      |$E6 |2   |5  |
|Zero Page,X |INC $44,X    |$F6 |2   |6  |
|Absolute    |INC $4400    |$EE |3   |6  |
|Absolute,X  |INC $4400,X  |$FE |3   |7  |

## Implementation

```rs
$memory.read($self.absolute_address, false);
let mut temp = $memory.read($self.absolute_address, false);

$memory.write($self.absolute_address, temp);
temp = temp.wrapping_add(1);
$memory.write($self.absolute_address, temp);
$self.set_nz(temp);

$self.is_crossing_page = false;
```

## Additional Codes

```rs
on_step!(self, 1, {
  self.is_writing = true;
});

on_step!(self, 0, {
  self.is_writing = true;
});
```
