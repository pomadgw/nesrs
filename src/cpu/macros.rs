macro_rules! set_instruction {
    ($self:expr, $cycles:expr, $block:block) => {{
        if $self.steps == 0 {
            $block

            let cycle_required = if $self.is_crossing_page {
                $cycles
            } else {
                $cycles - 1
            };

            $self.steps = cycle_required + 1;
        }
    }};
}
