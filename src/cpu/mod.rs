mod instructions;

use std::u8;

use crate::bus::Bus;
use crate::{gemuerror, gemutrace};

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
}

type OpcodeExec = fn(&mut CPU, &mut Bus);

#[derive(Clone, Copy)]
pub struct Instruction {
    pub exec: OpcodeExec,
    pub name: &'static str,
}

pub const fn new_instruction(exec: OpcodeExec, name: &'static str) -> Instruction {
    Instruction { exec, name }
}

const OPCODE_LUT: [Instruction; 256]  = {
    let unimplemented_instruction = new_instruction(CPU::unimplemented_instruction, "Unimplemented instruction");
    let mut table = [unimplemented_instruction; 256];

    table[0x00] = new_instruction(CPU::nop, "NOP");
    table[0x0E] = new_instruction(CPU::ld_c_d8, "LD C, d8");
    table[0x11] = new_instruction(CPU::ld_de_d16, "LD DE, d16");
    table[0x1A] = new_instruction(CPU::ld_a_de, "LD A, (DE)");
    table[0x20] = new_instruction(CPU::jr_nz_s8, "JR NZ, s8");
    table[0x21] = new_instruction(CPU::ld_hl_d16, "LD HL, d16");
    table[0x22] = new_instruction(CPU::ld_hl_plus_a, "LD (HL+), A");
    table[0x28] = new_instruction(CPU::jr_z_s8, "JR Z, s8");
    table[0x31] = new_instruction(CPU::ld_sp_d16, "LD SP, d16");
    table[0x47] = new_instruction(CPU::ld_b_a, "LD B, A");
    table[0x71] = new_instruction(CPU::ld_hl_c, "LD (HL), C");

    table[0x80] = new_instruction(CPU::add_a_b, "ADD A, B");
    table[0x81] = new_instruction(CPU::add_a_c, "ADD A, C");
    table[0x82] = new_instruction(CPU::add_a_d, "ADD A, D");
    table[0x83] = new_instruction(CPU::add_a_e, "ADD A, E");
    table[0x84] = new_instruction(CPU::add_a_h, "ADD A, H");
    table[0x85] = new_instruction(CPU::add_a_e, "ADD A, L");
    table[0x87] = new_instruction(CPU::add_a_e, "ADD A, A");

    table[0x3E] = new_instruction(CPU::ld_a_d8, "LD A, d8");
    table[0x28] = new_instruction(CPU::jr_z_s8, "JR Z, s8");
    table[0xC3] = new_instruction(CPU::jp_a16, "JP a16");
    table[0xCD] = new_instruction(CPU::call_a16, "CALL a16");
    table[0xE0] = new_instruction(CPU::ld_a8_a, "LD (a8), A");
    table[0xEA] = new_instruction(CPU::ld_a16_a, "LD (a16), A");
    table[0xF3] = new_instruction(CPU::di, "DI");
    table[0xFB] = new_instruction(CPU::ei, "EI");
    table
};

const OPCODE_CB_LUT: [Instruction; 256]  = {
    let unimplemented_instruction = new_instruction(CPU::unimplemented_instruction, "Unimplemented instruction");
    let mut table = [unimplemented_instruction; 256];

    table[0x20] = new_instruction(CPU::sla_b, "SLA B");
    table[0x6C] = new_instruction(CPU::bit_5_h, "but 5, H");
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
            pc: 0,
            halt: false,
            ime: true,
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
        gemuerror!("Unimplemented instruction {:02x}", bus.read8(self.pc));
        self.halt = true;
    }

    pub fn add(dest: &mut u8, src: u8) {
        *dest += src;
    }

    pub fn decode_and_execute(&mut self, bus: &mut Bus) {
        if self.halt {
            return;
        }

        let opcode = bus.read8(self.pc);
        self.pc += 1;

        if opcode == 0xCB {
            let cb_opcode = self.fetch8(bus);
            (OPCODE_CB_LUT[cb_opcode as usize].exec)(self, bus);
            gemutrace!("CB: {}", OPCODE_CB_LUT[cb_opcode as usize].name);
            return;
        }

        (OPCODE_LUT[opcode as usize].exec)(self, bus);
        gemutrace!("{}", OPCODE_LUT[opcode as usize].name);
    }
}   