use crate::bus::Bus;

pub struct CPU {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,

    ime: bool,

    is_16bit_opcode: bool
}

type Opcode = fn(&mut CPU, &mut Bus);

const OPCODE_LUT: [Opcode; 256]  = {
    let mut table = [CPU::unimplemented_instruction as Opcode; 256];

    table[0x00] = CPU::nop;
    table[0x21] = CPU::ld_hl_d16;
    table[0x22] = CPU::ld_hl_plus_a;
    table[0x31] = CPU::ld_sp_d16;
    table[0x3E] = CPU::ld_a_d8;
    table[0xC3] = CPU::jp_a16;
    table[0xEA] = CPU::ld_a16_a;
    table[0xF3] = CPU::di;

    table
};

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            // pc: 0x0100,
            pc: 0,
            ime: true,
            is_16bit_opcode: false
        }
    }

    fn fetch8(&mut self, bus: &mut Bus) -> u8 {
        let byte = bus.read8(self.pc);
        self.pc += 1;
        byte
    }

    fn fetch16(&mut self, bus: &mut Bus) -> u16 {
        let low = self.fetch8(bus);
        let high = self.fetch8(bus);
        u16::from_le_bytes([low, high])
    }

    pub fn unimplemented_instruction(&mut self, bus: &mut Bus) {
        self.pc -= 1;
        println!("Unimplemented instruction {}", bus.read8(self.pc));
        panic!()
    }

    pub fn nop(&mut self, bus: &mut Bus) {
        // Do nothing
        println!("NOP");
    }

    pub fn di(&mut self, bus: &mut Bus) {
        self.ime = false;
        println!("DI")
    }


    pub fn jp_a16(&mut self, bus: &mut Bus) {
        self.pc = self.fetch16(bus);
        println!("JP A16")
    }

    pub fn ld_sp_d16(&mut self, bus: &mut Bus) {
        self.sp = self.fetch16(bus);
        println!("LD SP, D16")
    }

    pub fn ld_a16_a(&mut self, bus: &mut Bus) {
        let addr = self.fetch16(bus);
        bus.write8(addr, self.a);
        println!("LD A16, A")
    }

    pub fn ld_a_d8(&mut self, bus: &mut Bus) {
        let data = self.fetch8(bus);
        self.a = data;
        println!("LD A, D8")
    }

    pub fn ld_hl_d16(&mut self, bus: &mut Bus) {
        self.l = self.fetch8(bus);
        self.h = self.fetch8(bus);
        println!("LD HL, D16")
    }

    pub fn ld_hl_plus_a(&mut self, bus: &mut Bus) {
        let addr = u16::from_le_bytes([self.l, self.h]);
        bus.write8(addr, self.a);

        match addr.checked_add(1) {
            Some(_) => {}
            None => panic!("HL register increment overflowed!"),
        }

        let hl = u16::to_le_bytes(addr);
        self.l = hl[0];
        self.h = hl[1];
        println!("LD (HL+), A")
    }


    pub fn decode_and_execute(&mut self, bus: &mut Bus) {
        let opcode = bus.read8(self.pc);
        self.pc += 1;
        OPCODE_LUT[opcode as usize](self, bus);
    }
}   