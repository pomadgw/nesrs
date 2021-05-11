use nesrs;

fn main() {
    nesrs::cpu::hello();
    let cpu = nesrs::cpu::CPU::new();

    println!("{}", cpu.regs.a);
}
