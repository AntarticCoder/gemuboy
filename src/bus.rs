pub struct Bus {
    // rom: [u8; 0x3FFF],
    boot_rom: [u8; 0x00FF - 0x0000 + 1],
    rom: Vec<u8>,
    wram: [u8; 0xDFFF - 0xC000 + 1],
    vram_tile: [u8; 0x9FFF - 0x8000 + 1],
    io_registers: [u8; 0xFF7F - 0xFF00 + 1],
    hram: [u8; 0xFFFE - 0xFF80 + 1]
}

impl Bus {
    pub fn new(rom: Vec<u8>) -> Self {
        let bus = Bus {
            boot_rom: rom.clone().try_into().unwrap(),
            rom: rom,
            wram: [0; 0xDFFF - 0xC000 + 1],
            vram_tile: [0; 0x9FFF - 0x8000 + 1],
            io_registers: [0; 0xFF7F - 0xFF00 + 1],
            hram: [0; 0xFFFE - 0xFF80 + 1]
        };
        bus
    }

    pub fn read8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x00FF => self.boot_rom[addr as usize],
            0x0100..=0x3FFF => self.rom[(addr - 0x0100) as usize],
            0x8000..=0x9FFF => self.vram_tile[(addr - 0x8000) as usize],
            0xFF00..=0xFF7F => self.io_registers[(addr - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            _ => 0,
        }
    }

    pub fn write8(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x00FF => panic!("Invalid write of data:[{}] to address:[{}] in boot rom", data, addr),
            0x0100..=0x3FFF => panic!("Invalid write of data:[{}] to address:[{}] in rom", data, addr),
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = data,
            0x8000..=0x9FFF => self.vram_tile[(addr - 0x8000) as usize] = data,
            0xFF00..=0xFF7F => self.io_registers[(addr - 0xFF00) as usize] = data,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = data,
            _ => panic!("Failed write. \nData: [{}]\nAddress:[{}]", data, addr)
        }
    }
}
