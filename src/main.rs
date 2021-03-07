use nesrs::*;
use std::fs;

fn main() {
    let filename = "./roms/nestest.nes";
    println!("{}", nesrs::hello());

    let file = fs::read(filename).unwrap();
    let cartridge = Cartridge::new_from_file(file);
    println!("Mapper Id: {}", cartridge.mapperid);

    let mut nes = NES::new();
    nes.bus.insert_cartridge(cartridge);

    println!("VAL: {:02X}", nes.bus.read(0xc000, false));

    nes.reset();
    // nes.cpu.pc = 0xc000;
    nes.clock();
    println!("VAL: ${:04X}", nes.cpu.pc);
    println!("VAL: ${:02X}", nes.cpu.p);
}
