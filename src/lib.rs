#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate modular_bitfield;

#[macro_use]
pub mod macros;

pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod memory;
pub mod ppu;

pub use cartridge::CartridgeRef;
