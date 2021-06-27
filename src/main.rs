// extern crate sdl2;

use nesrs;
use nesrs::bus::*;
use nesrs::cartridge::*;

use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;

#[macro_use]
mod macros;

fn main() -> std::io::Result<()> {
    let mut file = File::open("./rom/nestest.nes")?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    let cartridge = Cartridge::parse(&buffer);

    let mut bus = Bus::new(cartridge);

    bus.cpu.debug = true;
    bus.reset();
    // bus.cpu.regs.pc = 0xc000;

    let now = Instant::now();
    let start = now.elapsed().as_micros();

    let mut ppucycle = 0;
    let mut ppuscanline = 0;

    let mut debug_print_tick = 0;

    for _i in 0..(30000 * 3) {
        bus.clock();

        if bus.cpu.done() {
            if debug_print_tick == 0 {
                println!(
                    "{}",
                    bus.cpu
                        .debug_with_other_info(&format!("PPU:{:3},{:3}", ppuscanline, ppucycle))
                );
                ppucycle = bus.ppu.borrow().cycle();
                ppuscanline = bus.ppu.borrow().scanline();
                debug_print_tick = 3;
            }

            debug_print_tick -= 1;
        }
    }

    let end = now.elapsed().as_micros();
    let dur = end - start;
    let dur_per_cycle = dur as f32 / bus.cpu.total_cycles as f32;
    let freq = bus.cpu.total_cycles as f32 / (dur as f32 / 1_000_000.0);

    eprintln!("duration: {:}us, cycles: {}", dur, bus.cpu.total_cycles);
    eprintln!("duration / cycles: {} us/cycle", dur_per_cycle);
    if freq < 1_000_000.0 {
        eprintln!("freq: {} Hz", freq);
    } else if freq > 1_000_000_000.0 {
        eprintln!("freq: {} KHz", freq / 1_000.0);
    } else {
        eprintln!("freq: {} MHz", freq / 1_000_000.0);
    }
    /*
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("nesrs", 512, 480)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGBA32,
            nesrs::ppu::NES_WIDTH_SIZE as u32,
            nesrs::ppu::NES_HEIGHT_SIZE as u32,
        )
        .map_err(|e| e.to_string())
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        bus.clock_until_frame_done();

        canvas.clear();
        texture
            .update(
                None,
                bus.ppu.borrow().screen(),
                nesrs::ppu::NES_WIDTH_SIZE * 3,
            )
            .unwrap();
        canvas
            .copy(
                &texture,
                None,
                Some(sdl2::rect::Rect::new(
                    0,
                    0,
                    nesrs::ppu::NES_WIDTH_SIZE as u32 * 2,
                    nesrs::ppu::NES_HEIGHT_SIZE as u32 * 2,
                )),
            )
            .unwrap();

        canvas.present();
    }
    */
    Ok(())
}
