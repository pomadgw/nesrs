#![deny(clippy::all)]
#![forbid(unsafe_code)]

use crate::gui::Gui;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use nesrs::bus::*;
use nesrs::cartridge::*;
use nesrs::controller::ButtonStatus;
use nesrs::ppu::{NES_HEIGHT_SIZE, NES_WIDTH_SIZE};

use std::fs::File;
use std::io::prelude::*;

mod gui;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(NES_WIDTH_SIZE as f64, NES_HEIGHT_SIZE as f64);
        WindowBuilder::new()
            .with_title("NES RS")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let (mut pixels, mut gui) = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(
            NES_WIDTH_SIZE as u32,
            NES_HEIGHT_SIZE as u32,
            surface_texture,
        )?;
        let gui = Gui::new(
            window_size.width,
            window_size.height,
            scale_factor,
            pixels.context(),
        );

        (pixels, gui)
    };
    let mut nes = None;

    event_loop.run(move |event, _, control_flow| {
        // Update egui inputs
        gui.handle_event(&event);

        if let Some(path) = &gui.opened_fname {
            println!("Opening file: {:?}", path);

            let mut file = File::open(path).unwrap();
            let mut buffer = Vec::new();

            file.read_to_end(&mut buffer).unwrap();
            let cartridge = Cartridge::parse(&buffer);
            nes = Some(Bus::new(cartridge));
            nes.as_mut().unwrap().reset();

            gui.opened_fname = None;
        }

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            // Draw the world
            // world.draw(pixels.get_frame());
            if let Some(bus) = &mut nes {
                bus.clock_until_frame_done();
                bus.ppu.borrow().screen().copy_to(pixels.get_frame());
            }

            // Prepare egui
            gui.prepare();

            // Render everything together
            let render_result = pixels.render_with(|encoder, render_target, context| {
                // Render the world texture
                context.scaling_renderer.render(encoder, render_target);

                // Render egui
                gui.render(encoder, render_target, context);
            });

            // Basic error handling
            if render_result
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if let Some(bus) = &mut nes {
            match &event {
                Event::DeviceEvent {
                    event:
                        DeviceEvent::Key(KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(key),
                            ..
                        }),
                    ..
                } => {
                    let button = match &key {
                        VirtualKeyCode::Up => Some(ButtonStatus::UP),
                        VirtualKeyCode::Down => Some(ButtonStatus::DOWN),
                        VirtualKeyCode::Left => Some(ButtonStatus::LEFT),
                        VirtualKeyCode::Right => Some(ButtonStatus::RIGHT),
                        VirtualKeyCode::S => Some(ButtonStatus::START),
                        VirtualKeyCode::A => Some(ButtonStatus::SELECT),
                        VirtualKeyCode::Z => Some(ButtonStatus::A),
                        VirtualKeyCode::X => Some(ButtonStatus::B),
                        _ => None,
                    };

                    if let Some(button) = button {
                        bus.press_controller_button(0, button, true);
                    }
                }
                Event::DeviceEvent {
                    event:
                        DeviceEvent::Key(KeyboardInput {
                            state: ElementState::Released,
                            virtual_keycode: Some(key),
                            ..
                        }),
                    ..
                } => {
                    let button = match &key {
                        VirtualKeyCode::Up => Some(ButtonStatus::UP),
                        VirtualKeyCode::Down => Some(ButtonStatus::DOWN),
                        VirtualKeyCode::Left => Some(ButtonStatus::LEFT),
                        VirtualKeyCode::Right => Some(ButtonStatus::RIGHT),
                        VirtualKeyCode::S => Some(ButtonStatus::START),
                        VirtualKeyCode::A => Some(ButtonStatus::SELECT),
                        VirtualKeyCode::Z => Some(ButtonStatus::A),
                        VirtualKeyCode::X => Some(ButtonStatus::B),
                        _ => None,
                    };

                    if let Some(button) = button {
                        bus.press_controller_button(0, button, false);
                    }
                }
                _ => {}
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Update the scale factor
            if let Some(scale_factor) = input.scale_factor() {
                gui.scale_factor(scale_factor);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
                gui.resize(size.width, size.height);
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
}
