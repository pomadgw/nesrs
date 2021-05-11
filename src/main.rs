use nesrs;

struct RAM {
    a: Vec<u8>,
}

impl nesrs::memory::Memory for RAM {
    fn read(&self, address: usize, _is_read_only: bool) -> u8 {
        self.a[address]
    }

    fn write(&mut self, address: usize, value: u8) {
        self.a[address] = value;
    }
}

fn main() {
    nesrs::cpu::hello();
    let mut cpu = nesrs::cpu::CPU::new();

    let mut memory = RAM {
        a: vec![0; 0x10000],
    };

    println!("{}", cpu.regs.a);
    cpu.clock(&mut memory);
    cpu.clock(&mut memory);
}
