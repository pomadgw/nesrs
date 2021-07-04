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

use font_kit::family_name::FamilyName;
use font_kit::handle::Handle;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;

#[macro_use]
mod macros;

use std::cell::RefCell;
use std::rc::Rc;

use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};

pub struct TextRenderer<'s, 't> {
    font: Rc<&'s Font<'s, 't>>,
    texture_creator: Rc<TextureCreator<WindowContext>>,
    canvas: Rc<&'s RefCell<Canvas<Window>>>,
}

impl<'s, 't> TextRenderer<'s, 't> {
    pub fn new(
        font: Rc<&'s Font<'s, 't>>,
        canvas: Rc<&'s RefCell<Canvas<Window>>>,
    ) -> TextRenderer<'s, 't> {
        let texture_creator = canvas.borrow().texture_creator();

        TextRenderer {
            font: Rc::clone(&font),
            texture_creator: Rc::new(texture_creator),
            canvas: Rc::clone(&canvas),
        }
    }

    pub fn render(&self, text: &str, color: Color, x: i32, y: i32) {
        let surface = self
            .font
            .render(text)
            .blended(color)
            .map_err(|e| e.to_string())
            .unwrap();
        let texture_text = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();
        let text_query = texture_text.query();

        self.canvas
            .borrow_mut()
            .copy(
                &texture_text,
                None,
                Some(Rect::new(x, y, text_query.width, text_query.height)),
            )
            .unwrap();
    }

    pub fn recommended_line_spacing(&self) -> i32 {
        self.font.recommended_line_spacing()
    }
}

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

    // let mut ppucycle = 0;
    // let mut ppuscanline = 0;

    // let mut debug_print_tick = 0;

    for _i in 0..(400000 * 3) {
        bus.clock();

        // if bus.cpu.done() {
        //     if debug_print_tick == 0 {
        //         eprintln!(
        //             "{}",
        //             bus.cpu
        //                 .debug_with_other_info(&format!("PPU:{:3},{:3}", ppuscanline, ppucycle))
        //         );
        //         ppucycle = bus.ppu.borrow().cycle();
        //         ppuscanline = bus.ppu.borrow().scanline();
        //         debug_print_tick = 3;
        //     }

        //     debug_print_tick -= 1;
        // }
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

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let font_size = 6i32;

    let font = SystemSource::new()
        .select_best_match(&[FamilyName::Monospace], &Properties::new())
        .unwrap();
    let font_path = match font {
        Handle::Path { path, .. } => path
            .to_str()
            .unwrap_or("/usr/share/fonts/TTF/Roboto-Regular.ttf")
            .to_string(),
        _ => String::from("/usr/share/fonts/TTF/Roboto-Regular.ttf"),
    };

    println!("PATH: {:?}", font_path);
    let font = ttf_context.load_font(font_path, font_size as u16).unwrap();

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

    let canvas = Rc::new(RefCell::new(window.into_canvas().build().unwrap()));
    let texture_creator = canvas.borrow_mut().texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGBA32,
            nesrs::ppu::NES_WIDTH_SIZE as u32,
            nesrs::ppu::NES_HEIGHT_SIZE as u32,
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

    let text_renderer = TextRenderer::new(Rc::new(&font), Rc::new(&canvas));

    canvas.borrow_mut().set_draw_color(Color::RGB(0, 0, 0));
    canvas.borrow_mut().clear();
    canvas.borrow_mut().present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut palette = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    palette += 1;
                    palette &= 0x07;
                }
                _ => {}
            }
        }

        bus.clock_until_frame_done();

        for patternindex in 0..2 {
            bus.ppu
                .borrow_mut()
                .set_debug_pattern_screen(patternindex, palette);
        }

        canvas.borrow_mut().set_draw_color(Color::RGB(0, 0, 0));

        canvas.borrow_mut().clear();

        texture
            .update(
                None,
                bus.ppu.borrow().screen().image(),
                bus.ppu.borrow().screen().width() * 4,
            )
            .unwrap();

        canvas
            .borrow_mut()
            .copy(
                &texture,
                None,
                Some(sdl2::rect::Rect::new(
                    0,
                    0,
                    (bus.ppu.borrow().screen().width()) as u32,
                    (bus.ppu.borrow().screen().height()) as u32,
                )),
            )
            .unwrap();

        texture_debug_pattern
            .update(
                None,
                bus.ppu.borrow().screen_debug_pattern[0].image(),
                bus.ppu.borrow().screen_debug_pattern[0].width() * 4,
                // nesrs::ppu::NES_WIDTH_SIZE * 3,
            )
            .unwrap();
        canvas
            .borrow_mut()
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
        texture_debug_pattern
            .update(
                None,
                bus.ppu.borrow().screen_debug_pattern[1].image(),
                bus.ppu.borrow().screen_debug_pattern[1].width() * 4,
                // nesrs::ppu::NES_WIDTH_SIZE * 3,
            )
            .unwrap();
        canvas
            .borrow_mut()
            .copy(
                &texture_debug_pattern,
                None,
                Some(sdl2::rect::Rect::new(
                    256,
                    256,
                    (debug_pattern.width() * 2) as u32,
                    (debug_pattern.height() * 2) as u32,
                )),
            )
            .unwrap();

        // text_renderer.render("TEST", Color::RGB(12, 33, 145), 0, 0);

        let mut ppuref = bus.ppu.borrow_mut();
        let debug_nametable = ppuref.debug_nametable(0);

        let mut row = 0;
        let offset = text_renderer.recommended_line_spacing();
        // debug_nametable.iter().for_each(|item| {
        //     text_renderer.render(&item, Color::RGB(0x30, 0x30, 0xff), 0, 0 + offset * row);
        //     row += 1;
        // });

        let n_swatch_size: i32 = 6;

        for p in 0..8i32 {
            for s in 0..4i32 {
                let color = ppuref.get_color(p as usize, s as usize);
                let color = Color::RGB(color.0, color.1, color.2);

                canvas.borrow_mut().set_draw_color(color);
                canvas
                    .borrow_mut()
                    .fill_rect(Rect::new(
                        256 + p * (n_swatch_size * 5) + s * n_swatch_size,
                        250,
                        n_swatch_size as u32,
                        n_swatch_size as u32,
                    ))
                    .unwrap();
            }
        }

        canvas.borrow_mut().present();
    }

    Ok(())
}
