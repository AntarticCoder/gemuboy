mod bus;
mod cpu;
mod logger;
mod window;

use std::env;
use bus::Bus;
use cpu::CPU;
use window::SDLWindow;

use crate::logger::{disable_trace_logging, write_logs};

fn main() {
    let mut window = SDLWindow::new();

    let rom: Vec<u8> = match std::fs::read("roms/boot.bin") {
        Ok(data) => data,
        Err(e) => {
            panic!("Failed to read ROM!: {}", e);
        }
    };

    let mut bus = Bus::new(rom);
    let mut cpu = CPU::new();

    gemuerror!("Hello");

    // disable_trace_logging();

    loop {
        window.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        window.canvas.clear();

        for event in window.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    write_logs();
                    return;
                }
                _ => {}
            }
        }

        cpu.decode_and_execute(&mut bus);

        window.canvas.present();
    }
}
