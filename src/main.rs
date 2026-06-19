mod bus;
mod cpu;

use bus::Bus;
use cpu::CPU;

fn main() {
    let rom: Vec<u8> = match std::fs::read("roms/boot.bin") {
        Ok(data) => data,
        Err(e) => {
            panic!("Failed to read ROM!: {}", e);
        }
    };

    let mut  bus = Bus::new(rom);
    let mut cpu = CPU::new();

    while true {
        cpu.decode_and_execute(&mut bus);
    }
}
