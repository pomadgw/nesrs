use nesrs;
use nesrs::bus::*;
use nesrs::cartridge::*;
use nesrs::controller::ButtonStatus;

use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;

use font_kit::family_name::FamilyName;
use font_kit::handle::Handle;
use font_kit::properties::{Properties, Weight};
use font_kit::source::SystemSource;

#[macro_use]
mod macros;

use std::cell::RefCell;
use std::rc::Rc;

use std::env;

use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};

pub struct TextRenderer<'s, 't> {
    font: Rc<&'s Font<'s, 't>>,
    texture_creator: Rc<TextureCreator<WindowContext>>,
    canvas: Rc<&'s RefCell<Canvas<Window>>>,
    offset_y: i32,
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
            offset_y: 0,
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
                Some(Rect::new(
                    x,
                    y + self.offset_y,
                    text_query.width,
                    text_query.height,
                )),
            )
            .unwrap();
    }

    pub fn recommended_line_spacing(&self) -> i32 {
        self.font.recommended_line_spacing()
    }

    pub fn newline(&mut self) {
        self.offset_y += self.recommended_line_spacing();
    }

    pub fn reset_newline(&mut self) {
        self.offset_y = 0;
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    let cartridge = Cartridge::parse(&buffer);

    let mut bus = Bus::new(cartridge);

    bus.cpu.debug = false;
    bus.reset();
    // bus.cpu.regs.pc = 0xc000;

    // // let mut ppucycle = 0;
    // // let mut ppuscanline = 0;

    // // let mut debug_print_tick = 0;

    // for _i in 0..(400000 * 3) {
    //     bus.clock();

    //     // if bus.cpu.done() {
    //     //     if debug_print_tick == 0 {
    //     //         eprintln!(
    //     //             "{}",
    //     //             bus.cpu
    //     //                 .debug_with_other_info(&format!("PPU:{:3},{:3}", ppuscanline, ppucycle))
    //     //         );
    //     //         ppucycle = bus.ppu.borrow().cycle();
    //     //         ppuscanline = bus.ppu.borrow().scanline();
    //     //         debug_print_tick = 3;
    //     //     }

    //     //     debug_print_tick -= 1;
    //     // }
    // }

    // let end = now.elapsed().as_micros();
    // let dur = end - start;
    // let dur_per_cycle = dur as f32 / bus.cpu.total_cycles as f32;
    // let freq = bus.cpu.total_cycles as f32 / (dur as f32 / 1_000_000.0);

    // eprintln!("duration: {:}us, cycles: {}", dur, bus.cpu.total_cycles);
    // eprintln!("duration / cycles: {} us/cycle", dur_per_cycle);
    // if freq < 1_000_000.0 {
    //     eprintln!("freq: {} Hz", freq);
    // } else if freq > 1_000_000_000.0 {
    //     eprintln!("freq: {} KHz", freq / 1_000.0);
    // } else {
    //     eprintln!("freq: {} MHz", freq / 1_000_000.0);
    // }

    // match bus.ppu.borrow_mut().get_color(0, 0) {
    //     (r, g, b) => println!("{}", "█".truecolor(r, g, b)),
    // }

    // println!("");
    // for palette in 0..8 {
    //     for pixel in 0..4 {
    //         match bus.ppu.borrow_mut().get_color(palette, pixel) {
    //             (r, g, b) => print!("{}", "█".truecolor(r, g, b)),
    //         }
    //     }
    //     println!("");
    // }

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let font_size = 12i32;

    let font = SystemSource::new()
        .select_best_match(&[FamilyName::Monospace], &Properties::new().weight(Weight::BOLD))
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
        .create_texture_streaming(PixelFormatEnum::RGBA32, 256, 128)
        .map_err(|e| e.to_string())
        .unwrap();

    let mut text_renderer = TextRenderer::new(Rc::new(&font), Rc::new(&canvas));

    canvas.borrow_mut().set_draw_color(Color::RGB(0, 0, 0));
    canvas.borrow_mut().clear();
    canvas.borrow_mut().present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut palette = 0;
    let now = Instant::now();
    let frame_regulator = Instant::now();
    let mut start = 0;
    let mut end;
    let mut fps;

    let mut show_debug = false;

    let mut frame_time = frame_regulator.elapsed().as_micros();

    use sdl2::keyboard::Scancode;

    macro_rules! update_controllers {
        ( $($event_pump:ident, $actual_button:ident, $button:ident),+ ) => (
            $(
                if $event_pump.keyboard_state().is_scancode_pressed(Scancode::$actual_button)
                {
                    bus.press_controller_button(0, ButtonStatus::$button, true);
                } else {
                    bus.press_controller_button(0, ButtonStatus::$button, false);
                }
            )+
        )
    }

    'running: loop {
        update_controllers!(event_pump, X, A);
        update_controllers!(event_pump, Z, B);
        update_controllers!(event_pump, A, SELECT);
        update_controllers!(event_pump, S, START);
        update_controllers!(event_pump, Up, UP);
        update_controllers!(event_pump, Down, DOWN);
        update_controllers!(event_pump, Left, LEFT);
        update_controllers!(event_pump, Right, RIGHT);

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
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    show_debug = !show_debug;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    bus.reset();
                }
                _ => {}
            }
        }

        if (frame_regulator.elapsed().as_micros() - frame_time) < 13_333 {
            continue;
        }

        bus.clock_until_frame_done();
        end = now.elapsed().as_micros();
        fps = (1_000_000) / (end - start);

        start = now.elapsed().as_micros();
        frame_time = frame_regulator.elapsed().as_micros();

        if show_debug {
            for patternindex in 0..2 {
                bus.ppu
                    .borrow_mut()
                    .set_debug_pattern_screen(patternindex, palette);
            }
        }

        canvas.borrow_mut().set_draw_color(Color::RGB(0, 0, 0));

        canvas.borrow_mut().clear();

        let mut ppu_ref = bus.ppu.borrow_mut();

        texture
            .update(None, ppu_ref.screen().image(), ppu_ref.screen().width() * 4)
            .unwrap();

        canvas
            .borrow_mut()
            .copy(
                &texture,
                None,
                Some(sdl2::rect::Rect::new(
                    0,
                    0,
                    (ppu_ref.screen().width() * 2) as u32,
                    (ppu_ref.screen().height() * 2) as u32,
                )),
            )
            .unwrap();

        if show_debug {
            text_renderer.reset_newline();

            text_renderer.render("DEBUG MODE", Color::RGB(0xc0, 0xc0, 0xc0), 0, 0);

            text_renderer.newline();

            text_renderer.render(
                &format!("FPS: {:4} fps", fps),
                Color::RGB(0xc0, 0xc0, 0xc0),
                0,
                0,
            );

            text_renderer.newline();
            text_renderer.newline();
            text_renderer.render("OAMS", Color::RGB(0xc0, 0xc0, 0xc0), 0, 0);

            for oam in 0..64 {
                text_renderer.newline();
                let oam_data = ppu_ref.oams.get(oam);
                let oam_y = oam_data.y;
                let oam_id = oam_data.id;
                let oam_attr = oam_data.attr;
                let oam_x = oam_data.x;

                text_renderer.render(
                    &format!(
                        "{:02X}: [{:3}, {:3}] ID: {:02X} AT: {:02X}",
                        oam, oam_x, oam_y, oam_id, oam_attr
                    ),
                    Color::RGB(0xc0, 0xc0, 0xc0),
                    0,
                    0,
                );
            }

            text_renderer.reset_newline();
            text_renderer.newline();
            text_renderer.newline();
            text_renderer.newline();
            text_renderer.render("SECONDARY OAMS", Color::RGB(0xc0, 0xc0, 0xc0), 256, 0);

            for oam in 0..8 {
                text_renderer.newline();
                let oam_data = ppu_ref.internal_oams_debug.get(oam);
                let oam_y = oam_data.y;
                let oam_id = oam_data.id;
                let oam_attr = oam_data.attr;
                let oam_x = oam_data.x;

                text_renderer.render(
                    &format!(
                        "{:02X}: [{:3}, {:3}] ID: {:02X} AT: {:02X}",
                        oam, oam_x, oam_y, oam_id, oam_attr
                    ),
                    Color::RGB(0x00, 0xc0, 0xc0),
                    256,
                    0,
                );
            }
        }

        if show_debug {
            let width = ppu_ref.screen_debug_pattern[0].width() * 2;
            let height = ppu_ref.screen_debug_pattern[0].height();
            texture_debug_pattern
                .update(
                    None,
                    ppu_ref.screen_debug_pattern[0].image(),
                    ppu_ref.screen_debug_pattern[0].width() * 4,
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
                        (width * 2) as u32,
                        (height * 2) as u32,
                    )),
                )
                .unwrap();
            texture_debug_pattern
                .update(
                    None,
                    ppu_ref.screen_debug_pattern[1].image(),
                    ppu_ref.screen_debug_pattern[1].width() * 4,
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
                        (width * 2) as u32,
                        (height * 2) as u32,
                    )),
                )
                .unwrap();
        }

        // let debug_nametable = ppuref.debug_nametable(0);

        // let mut row = 0;
        // let offset = text_renderer.recommended_line_spacing();
        // // debug_nametable.iter().for_each(|item| {
        // //     text_renderer.render(&item, Color::RGB(0x30, 0x30, 0xff), 0, 0 + offset * row);
        // //     row += 1;
        // // });

        if show_debug {
            let n_swatch_size: i32 = 6;

            for p in 0..8i32 {
                for s in 0..4i32 {
                    let color = ppu_ref.get_color(p as usize, s as usize);
                    let color = Color::RGB(color.0, color.1, color.2);

                    canvas.borrow_mut().set_draw_color(color);
                    canvas
                        .borrow_mut()
                        .fill_rect(Rect::new(
                            256 + p * (n_swatch_size * 5) + s * n_swatch_size,
                            512 - n_swatch_size,
                            n_swatch_size as u32,
                            n_swatch_size as u32,
                        ))
                        .unwrap();
                }
            }
        }

        canvas.borrow_mut().present();
    }

    Ok(())
}
