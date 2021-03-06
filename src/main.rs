use nesrs;
use nesrs::bus::*;
use nesrs::cartridge::*;
use nesrs::utils::*;
use std::fs;

fn main() {
    let filename = "nestest.nes";
    println!("{}", nesrs::hello());

    let file = fs::read(filename).unwrap();
    let cartridge = Cartridge::new_from_file(file);
    println!("Mapper Id: {}", cartridge.mapperid);

    let mut bus = Bus::new();
    bus.insert_cartridge(cartridge);

    println!("VAL: {:02X}", bus.read(0xc000, false));
}
