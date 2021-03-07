mod bus;
mod cartridge;
mod cpu;
mod mappers;
#[macro_use]
mod utils;

pub use bus::*;
pub use cartridge::*;
pub use cpu::*;
pub use mappers::*;
pub use utils::*;

pub fn hello() -> String {
    String::from("Hello")
}
