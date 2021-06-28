// extern crate sdl2;

use nesrs;
use nesrs::bus::*;
use nesrs::cartridge::*;
use nesrs::utils::*;

use colored::*;
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

    bus.cpu.debug = false;
    bus.reset();
    // bus.cpu.regs.pc = 0xc000;

    let now = Instant::now();
    let start = now.elapsed().as_micros();

    let mut ppucycle = 0;
    let mut ppuscanline = 0;

    let mut debug_print_tick = 0;

    for _i in 0..(100000 * 3) {
        bus.clock();

        if bus.cpu.done() {
            if debug_print_tick == 0 {
                eprintln!(
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

    match bus.ppu.borrow_mut().get_color(0, 0) {
        (r, g, b) => println!("{}", "█".truecolor(r, g, b)),
    }

    println!("");
    for palette in 0..8 {
        for pixel in 0..4 {
            match bus.ppu.borrow_mut().get_color(palette, pixel) {
                (r, g, b) => print!("{}", "█".truecolor(r, g, b)),
            }
        }
        println!("");
    }

    // let ppuref = bus.ppu.borrow();

    // let nametable = ppuref.debug_nametable(1);

    // for y in 0..32 {
    //     for x in 0..32 {

    //         let index = y * 32 + x;
    //         print!("{:02X}", nametable[index]);
    //     }
    //     println!("");
    // }

    // for palette in 0..8 {
    //     for index in 0..4 {
    //         println!("{:?}", bus.ppu.borrow_mut().get_color(palette, index));
    //     }
    // }

    let mut debug_pattern: Screen = Screen::new(256, 128);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("nesrs", 512, 512)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGBA32,
            256,
            256,
            // nesrs::ppu::NES_WIDTH_SIZE as u32,
            // nesrs::ppu::NES_HEIGHT_SIZE as u32,
        )
        .map_err(|e| e.to_string())
        .unwrap();

    let mut texture_debug_pattern = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGBA32,
            (debug_pattern.width()) as u32,
            (debug_pattern.height()) as u32,
            // nesrs::ppu::NES_WIDTH_SIZE as u32,
            // nesrs::ppu::NES_HEIGHT_SIZE as u32,
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

        for patternindex in 0..2 {
            for tile_y in 0..16 {
                for tile_x in 0..16 {
                    let pattern_test =
                        bus.ppu
                            .borrow_mut()
                            .debug_pattern(patternindex, tile_x, tile_y);
                    for row in 0..8 {
                        let yy = tile_y * 8 + row;
                        for col in 0..8 {
                            let d = pattern_test[row * 8 + col];
                            let xx =
                                (tile_x * 8 + col) + (patternindex * debug_pattern.width() / 2);
                            debug_pattern.set_pixel(xx, yy, d);
                        }
                    }
                }
            }
        }

        bus.clock_until_frame_done();

        canvas.clear();
        texture_debug_pattern
            .update(
                None,
                debug_pattern.image(),
                debug_pattern.width() * 4,
                // nesrs::ppu::NES_WIDTH_SIZE * 3,
            )
            .unwrap();
        canvas
            .copy(
                &texture,
                None,
                Some(sdl2::rect::Rect::new(
                    0,
                    256,
                    (debug_pattern.width() * 2) as u32,
                    (debug_pattern.height() * 2) as u32,
                )),
            )
            .unwrap();
            canvas
                .copy(
                    &texture_debug_pattern,
                    None,
                    Some(sdl2::rect::Rect::new(
                        0,
                        256,
                        (debug_pattern.width() * 2) as u32,
                        (debug_pattern.height() * 2) as u32,
                    )),
                )
                .unwrap();

        canvas.present();
    }

    Ok(())
}
