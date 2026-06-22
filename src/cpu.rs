use crate::bus::Bus;

struct FlagRegister {
    pub zero: bool,
    pub sub: bool,
    pub half_carry: bool,
    pub carry: bool
}

pub struct CPU {
    a: u8,
    f: FlagRegister,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,

    halt: bool,
    ime: bool,
    
    is_16bit_opcode: bool
}

type Opcode = fn(&mut CPU, &mut Bus);

const OPCODE_LUT: [Opcode; 256]  = {
    let mut table = [CPU::unimplemented_instruction as Opcode; 256];

    table[0x00] = CPU::nop;
    table[0x0E] = CPU::ld_c_d8;
    table[0x11] = CPU::ld_de_d16;
    table[0x1A] = CPU::ld_a_de;
    table[0x20] = CPU::jr_nz_s8;
    table[0x21] = CPU::ld_hl_d16;
    table[0x22] = CPU::ld_hl_plus_a;
    table[0x28] = CPU::jr_z_s8;
    table[0x31] = CPU::ld_sp_d16;
    table[0x47] = CPU::ld_b_a;
    table[0x71] = CPU::ld_hl_c;
    table[0x3E] = CPU::ld_a_d8;
    table[0xC3] = CPU::jp_a16;
    table[0xCD] = CPU::call_a16;
    table[0xE0] = CPU::ld_a8_a;
    table[0xEA] = CPU::ld_a16_a;
    table[0xF3] = CPU::di;

    table
};

const OPCODE_CB_LUT: [Opcode; 256]  = {
    let mut table = [CPU::unimplemented_instruction as Opcode; 256];

    table[0x20] = CPU::sla_b;
    table[0x6C] = CPU::bit_5_h;

    table
};

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            f: FlagRegister {
                zero: false,
                sub: false,
                half_carry: false,
                carry: false,
            },
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            // pc: 0x0100,
            pc: 0,
            halt: false,
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
        self.halt = true;
    }

    pub fn nop(&mut self, _bus: &mut Bus) {
        // Do nothing
        println!("NOP");
    }

    pub fn di(&mut self, _bus: &mut Bus) {
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

        let [lo, hi] = u16::to_le_bytes(addr.wrapping_add(1));
        self.l = lo;
        self.h = hi;
        println!("LD (HL+), A")
    }

    pub fn bit_5_h(&mut self, bus: &mut Bus) {
        let bit = self.h & (1 << 5) != 0;
        // self.f.zero = bit;
        self.f.zero = !bit;

        println!("BIT 5, H")
    }

    pub fn jr_z_s8(&mut self, bus: &mut Bus) {
        let offset = self.fetch8(bus) as i8;
        if self.f.zero {
            self.pc = ((self.pc as i16) + (offset as i16)) as u16;
        }
        println!("JR Z, S8")
    }

    pub fn ld_a8_a(&mut self, bus: &mut Bus) {
        let addr = 0xFF00 + self.fetch8(bus) as u16;
        bus.write8(addr, self.a);
        println!("LD (A8), A")
    }

    pub fn ld_de_d16(&mut self, bus: &mut Bus) {
        let data = self.fetch16(bus);

        self.e = data.to_le_bytes()[0];
        self.d = data.to_le_bytes()[1];
        println!("LD DE, D16")
    }

    pub fn ld_a_de(&mut self, bus: &mut Bus) {
        let addr = u16::from_le_bytes([self.e, self.d]);
        self.a = bus.read8(addr);

        println!("LD A, DE")
    }

    pub fn ld_hl_c(&mut self, bus: &mut Bus) {
        let addr = u16::from_le_bytes([self.l, self.h]);
        bus.write8(addr, self.c);

        println!("LD HL, C")
    }

    pub fn ld_b_a(&mut self, bus: &mut Bus) {
        self.b = self.a;
        println!("LD B, A")
    }

    pub fn call_a16(&mut self, bus: &mut Bus) {
        println!("SP: {}", self.sp);

        let func_addr = self.fetch16(bus);

        let [lo, hi] = u16::to_le_bytes(self.pc);
        self.sp = self.sp.wrapping_sub(1);
        bus.write8(self.sp, hi);
        self.sp = self.sp.wrapping_sub(1);
        bus.write8(self.sp, lo);

        self.pc = func_addr;

        println!("CALL A16")
    }

    pub fn ld_c_d8(&mut self, bus: &mut Bus) {
        self.c = self.fetch8(bus);
        println!("LD C, D8")
    }

    pub fn jr_nz_s8(&mut self, bus: &mut Bus) {
        if !self.f.zero {
            let offset = self.fetch8(bus) as i8;
            self.pc = ((self.pc as i16) + (offset as i16)) as u16;
        }

        println!("JR NZ, S8")
    }

    pub fn sla_b(&mut self, bus: &mut Bus) {
        let mask = 1 << 7;
        let carry = (mask & self.b) > 0;
        self.f.carry = carry;

        self.b = self.b << 1;

        println!("SLA B")
    }

    pub fn decode_and_execute(&mut self, bus: &mut Bus) {
        if self.halt {
            return;
        }

        let opcode = bus.read8(self.pc);
        self.pc += 1;

        if opcode == 0xCB {
            println!("CB PREFIX");
            let cb_opcode = self.fetch8(bus);
            OPCODE_CB_LUT[cb_opcode as usize](self, bus);
            return;
        }

        OPCODE_LUT[opcode as usize](self, bus);
    }
}   