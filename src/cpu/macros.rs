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

macro_rules! on_step {
    ($myname:ident : $cycles:expr, $block:block) => {{
        let $myname = $cycles;

        $block;
    }};
}
