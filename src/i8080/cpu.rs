use super::bus::*;
use super::util::*;

pub const CARRY_FLAG: usize = 0;
pub const PARITY_FLAG: usize = 2;
pub const AUX_CARRY_FLAG: usize = 4;
pub const ZERO_FLAG: usize = 6;
pub const SIGN_FLAG: usize = 7;

pub struct CPU {
    pub a: u8,
    pub flags: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub bus: Bus,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            a: 0,
            flags: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            bus: Bus::new(),
        }
    }

    pub fn read_byte(&mut self) -> u8 {
        let ret = self.bus.read_byte(self.pc);
        self.pc += 1;
        ret
    }

    pub fn read_word(&mut self) -> u16 {
        let low = self.read_byte();
        let high = self.read_byte();
        join_bytes(high, low)
    }

    pub fn jump(&mut self, address: u16) {
        self.pc = address;
    }

    pub fn push(&mut self, value: u16) {
        self.bus.write_byte(self.sp, get_low_byte(value));
        self.bus.write_byte(self.sp + 1, get_high_byte(value));
        self.sp += 2;
    }

    pub fn pop(&mut self) -> u16 {
        let high = self.bus.read_byte(self.sp - 1);
        let low = self.bus.read_byte(self.sp - 2);
        self.sp -= 2;

        join_bytes(high, low)
    }

    pub fn set_flag(&mut self, flag: usize) {
        set_bit(&mut self.flags, flag);
    }

    pub fn clear_flag(&mut self, flag: usize) {
        clear_bit(&mut self.flags, flag);
    }

    pub fn get_flag(&mut self, flag: usize) -> bool {
        get_bit(self.flags, flag)
    }

    pub fn get_psw(&self) -> u16 {
        join_bytes(self.a, self.flags)
    }

    pub fn get_bc(&self) -> u16 {
        join_bytes(self.b, self.c)
    }

    pub fn get_de(&self) -> u16 {
        join_bytes(self.d, self.e)
    }

    pub fn get_hl(&self) -> u16 {
        join_bytes(self.h, self.l)
    }

    pub fn set_psw(&mut self, value: u16) {
        self.a = get_high_byte(value);
        self.flags = get_low_byte(value);
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = get_high_byte(value);
        self.c = get_low_byte(value);
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = get_high_byte(value);
        self.e = get_low_byte(value);
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = get_high_byte(value);
        self.l = get_low_byte(value);
    }
}
