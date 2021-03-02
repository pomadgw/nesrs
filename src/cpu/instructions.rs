macro_rules! set_instruction {
    ($self:expr, $cycles:expr, $block:block) => {{
        if $self.steps == 0 {
            $block
        }

        let cycle_required = if $self.is_crossing_page {
            $cycles
        } else {
            $cycles - 1
        };

        if $self.steps == cycle_required {
            $self.sync = true;
        }
    }};
}

/// LDA (LoaD Accumulator)
/// Affects Flags: N Z
///
/// MODE           SYNTAX       HEX LEN TIM
/// Immediate     LDA #$44      $A9  2   2
/// Zero Page     LDA $44       $A5  2   3
/// Zero Page,X   LDA $44,X     $B5  2   4
/// Absolute      LDA $4400     $AD  3   4
/// Absolute,X    LDA $4400,X   $BD  3   4+
/// Absolute,Y    LDA $4400,Y   $B9  3   4+
/// Indirect,X    LDA ($44,X)   $A1  2   6
/// Indirect,Y    LDA ($44),Y   $B1  2   5+
///
/// + add 1 cycle if page boundary crossed
macro_rules! lda {
    ($self:expr, $memory:expr) => {
        $self.a = $memory.read($self.absolute_address, false);
        $self.set_nz($self.a);
    };
}

// LDX (LoaD X register)
// Affects Flags: N Z
//
// MODE           SYNTAX       HEX LEN TIM
// Immediate     LDX #$44      $A2  2   2
// Zero Page     LDX $44       $A6  2   3
// Zero Page,Y   LDX $44,Y     $B6  2   4
// Absolute      LDX $4400     $AE  3   4
// Absolute,Y    LDX $4400,Y   $BE  3   4+
//
// + add 1 cycle if page boundary crossed

macro_rules! ldx {
    ($self:expr, $memory:expr) => {
        $self.x = $memory.read($self.absolute_address, false);
        $self.set_nz($self.x);
    };
}

/// INC (INCrement memory)
/// Affects Flags: N Z
//
/// MODE           SYNTAX       HEX LEN TIM
/// Zero Page     INC $44       $E6  2   5
/// Zero Page,X   INC $44,X     $F6  2   6
/// Absolute      INC $4400     $EE  3   6
/// Absolute,X    INC $4400,X   $FE  3   7

// INC opcode invokes double read-write
macro_rules! inc {
    ($self:expr, $memory:expr) => {
        $memory.read($self.absolute_address, false);
        let mut temp = $memory.read($self.absolute_address, false);

        $memory.write($self.absolute_address, temp);
        temp = temp.wrapping_add(1);
        $memory.write($self.absolute_address, temp);
        $self.set_nz(temp);

        $self.is_crossing_page = false;
    };
}
