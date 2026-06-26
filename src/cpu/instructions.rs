use super::CPU;
use crate::bus::Bus;

impl CPU {
    pub fn nop(&mut self, _bus: &mut Bus) {
        // Do nothing
    }

    pub fn di(&mut self, _bus: &mut Bus) {
        self.ime = false;
    }

    pub fn ei(&mut self, _bus: &mut Bus) {
        self.ime = true;
    }

    pub fn jp_a16(&mut self, bus: &mut Bus) {
        self.pc = self.fetch16(bus);
    }

    pub fn ld_sp_d16(&mut self, bus: &mut Bus) {
        self.sp = self.fetch16(bus);
    }

    pub fn ld_a16_a(&mut self, bus: &mut Bus) {
        let addr = self.fetch16(bus);
        bus.write8(addr, self.a);
    }

    pub fn ld_a_d8(&mut self, bus: &mut Bus) {
        let data = self.fetch8(bus);
        self.a = data;
    }

    pub fn ld_hl_d16(&mut self, bus: &mut Bus) {
        self.l = self.fetch8(bus);
        self.h = self.fetch8(bus);

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
    }

    pub fn bit_5_h(&mut self, _bus: &mut Bus) {
        let bit = self.h & (1 << 5) != 0;
        // self.f.zero = bit;
        self.f.zero = !bit;
    }

    pub fn jr_z_s8(&mut self, bus: &mut Bus) {
        let offset = self.fetch8(bus) as i8;
        if self.f.zero {
            self.pc = ((self.pc as i16) + (offset as i16)) as u16;
        }
    }

    pub fn ld_a8_a(&mut self, bus: &mut Bus) {
        let addr = 0xFF00 + self.fetch8(bus) as u16;
        bus.write8(addr, self.a);
    }

    pub fn ld_de_d16(&mut self, bus: &mut Bus) {
        let data = self.fetch16(bus);

        self.e = data.to_le_bytes()[0];
        self.d = data.to_le_bytes()[1];
    }

    pub fn ld_a_de(&mut self, bus: &mut Bus) {
        let addr = u16::from_le_bytes([self.e, self.d]);
        self.a = bus.read8(addr);
    }

    pub fn ld_hl_c(&mut self, bus: &mut Bus) {
        let addr = u16::from_le_bytes([self.l, self.h]);
        bus.write8(addr, self.c);
    }

    pub fn ld_b_a(&mut self, _bus: &mut Bus) {
        self.b = self.a;
    }

    pub fn call_a16(&mut self, bus: &mut Bus) {
        let func_addr = self.fetch16(bus);

        let [lo, hi] = u16::to_le_bytes(self.pc);
        self.sp = self.sp.wrapping_sub(1);
        bus.write8(self.sp, hi);
        self.sp = self.sp.wrapping_sub(1);
        bus.write8(self.sp, lo);

        self.pc = func_addr;
    }

    pub fn ld_c_d8(&mut self, bus: &mut Bus) {
        self.c = self.fetch8(bus);
    }

    pub fn jr_nz_s8(&mut self, bus: &mut Bus) {
        if !self.f.zero {
            let offset = self.fetch8(bus) as i8;
            self.pc = ((self.pc as i16) + (offset as i16)) as u16;
        }
    }

    pub fn sla_b(&mut self, _bus: &mut Bus) {
        let mask = 1 << 7;
        let carry = (mask & self.b) > 0;
        self.f.carry = carry;

        self.b = self.b << 1;
    }
    
    pub fn add_a_b(&mut self, _bus: &mut Bus) {
        Self::add(&mut self.a, self.b);
    }

    pub fn add_a_c(&mut self, _bus: &mut Bus) {
        Self::add(&mut self.a, self.c);
    }

    pub fn add_a_d(&mut self, _bus: &mut Bus) {
        Self::add(&mut self.a, self.d);
    }

    pub fn add_a_e(&mut self, _bus: &mut Bus) {
        Self::add(&mut self.a, self.e);
    }

    pub fn add_a_h(&mut self, _bus: &mut Bus) {
        Self::add(&mut self.a, self.h);
    }

    pub fn add_a_l(&mut self, _bus: &mut Bus) {
        Self::add(&mut self.a, self.l);
    }

    pub fn add_a_a(&mut self, _bus: &mut Bus) {
        self.a += self.a;
    }
}