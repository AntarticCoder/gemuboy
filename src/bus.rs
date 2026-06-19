pub struct Bus {
    // rom: [u8; 0x3FFF],
    rom: Vec<u8>,
    wram: [u8; 0xDFFF - 0xC000]
}

impl Bus {
    pub fn new(rom: Vec<u8>) -> Self {
        let mut bus = Bus {
            rom: rom,
            wram: [0; 0xDFFF - 0xC000]
        };
        bus
        // bus.rom.copy_from_slice(&rom);
        // bus
    }

    pub fn read8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.rom[addr as usize],
            _ => 0,
        }
    }

    pub fn write8(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x3FFF => panic!("Invalid write of data:[{}] to address:[{}] in rom", data, addr),
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = data,
            _ => panic!("Failed write. \nData: [{}]\nAddress:[{}]", data, addr)
        }
    }
}