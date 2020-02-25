use super::cpu::*;
use super::decoder::*;
use super::util::*;

use std::collections::HashMap;

pub struct Executor<'a> {
    cpu: &'a mut CPU,
    cycles: usize,
    cache: HashMap<u8, Instruction>,
}

impl<'a> Executor<'a> {
    pub fn new(cpu: &'a mut CPU) -> Self {
        Self {
            cpu,
            cycles: 0,
            cache: HashMap::new(),
        }
    }

    pub fn execute(&mut self) {
        let opcode = self.cpu.read_byte();

        println!("[EXECUTOR]: Executing opcode {:02X}h", opcode);

        if !self.cache.contains_key(&opcode) {
            self.cache
                .insert(opcode, decode_instruction(opcode, &mut self.cpu));
            println!("[EXECUTOR]: Saving instruction implementation to cache");
        } else {
            println!("[EXECUTOR]: Loading instruction implementation from cache");
        }

        let instruction = *self.cache.get(&opcode).unwrap();
        println!("[EXECUTOR]: Running cached instruction {:?}", instruction);
        let (size, cycles) = self.execute_instruction(instruction);
        self.cpu.pc += size;
        self.cycles += cycles;
    }

    pub fn get_cycles(&self) -> usize {
        self.cycles
    }

    pub fn reset_cycles(&mut self) {
        self.cycles = 0;
    }

    fn write_reg16(&mut self, reg: Register, value: u16) {
        println!("[EXECUTOR]: Writing {:04X}h to {:?}", value, reg);
        match reg {
            Register::A => self.cpu.set_psw(value),
            Register::B => self.cpu.set_bc(value),
            Register::D => self.cpu.set_de(value),
            Register::H => self.cpu.set_hl(value),
            Register::SP => self.cpu.sp = value,
            _ => panic!("{:?} is not a 16 Bit register", reg),
        };
    }

    fn read_reg16(&self, reg: Register) -> u16 {
        let result = match reg {
            Register::A => self.cpu.get_psw(),
            Register::B => self.cpu.get_bc(),
            Register::D => self.cpu.get_de(),
            Register::H => self.cpu.get_hl(),
            Register::SP => self.cpu.sp,
            _ => panic!("{:?} is not a 16 Bit register", reg),
        };
        println!("[EXECUTOR]: Reading {:04X}h from {:?}", result, reg);
        result
    }

    fn write_reg8(&mut self, reg: Register, value: u8) {
        println!("[EXECUTOR]: Writing {:02X}h to {:?}", value, reg);
        match reg {
            Register::A => self.cpu.a = value,
            Register::Flags => self.cpu.flags = value,
            Register::B => self.cpu.b = value,
            Register::C => self.cpu.c = value,
            Register::D => self.cpu.d = value,
            Register::E => self.cpu.e = value,
            Register::H => self.cpu.h = value,
            Register::L => self.cpu.l = value,
            Register::M => self.cpu.bus.write_byte(self.cpu.get_hl(), value),
            _ => panic!("{:?} is not a 8 Bit register", reg),
        };
    }

    fn read_reg8(&mut self, reg: Register) -> u8 {
        let result = match reg {
            Register::A => self.cpu.a,
            Register::Flags => self.cpu.flags,
            Register::B => self.cpu.b,
            Register::C => self.cpu.c,
            Register::D => self.cpu.d,
            Register::E => self.cpu.e,
            Register::H => self.cpu.h,
            Register::L => self.cpu.l,
            Register::M => self.cpu.bus.read_byte(self.cpu.get_hl()),
            _ => panic!("{:?} is not a 8 Bit register", reg),
        };
        println!("[EXECUTOR]: Reading {:02X}h from {:?}", result, reg);
        result
    }

    fn check_flags(&mut self, value: u8) {
        set_bit_enabled(&mut self.cpu.flags, ZERO_FLAG, value == 0);
        set_bit_enabled(&mut self.cpu.flags, SIGN_FLAG, (value >> 7) == 0x1);
        set_bit_enabled(
            &mut self.cpu.flags,
            PARITY_FLAG,
            get_enabled_bits(value) > 0,
        );
        set_bit_enabled(&mut self.cpu.flags, AUX_CARRY_FLAG, false);
    }

    fn check_carry(&mut self, value: u16) {
        set_bit_enabled(&mut self.cpu.flags, CARRY_FLAG, value > 0xFF);
    }

    fn execute_instruction(&mut self, instr: Instruction) -> (u16, usize) {
        match instr {
            Instruction::Nop => (1, 4),
            Instruction::Lxi(reg, value) => {
                self.write_reg16(reg, value);
                (3, 10)
            }
            Instruction::Stax(reg) => {
                self.cpu.bus.write_byte(self.read_reg16(reg), self.cpu.a);
                (1, 7)
            }
            Instruction::Shld(addr) => {
                self.cpu.bus.write_byte(addr, self.cpu.l);
                self.cpu.bus.write_byte(addr + 1, self.cpu.h);
                (3, 16)
            }
            Instruction::Sta(addr) => {
                self.cpu.bus.write_byte(addr, self.cpu.a);
                (3, 13)
            }
            Instruction::Inx(reg) => {
                let tmp = self.read_reg16(reg);
                self.write_reg16(reg, tmp.wrapping_add(1));
                (1, 5)
            }
            Instruction::Inr(reg) => {
                let result = self.read_reg8(reg).wrapping_add(1);
                self.check_flags(result);
                self.write_reg8(reg, result);
                (1, 5)
            }
            Instruction::Dcr(reg) => {
                let result = self.read_reg8(reg).wrapping_sub(1);
                self.check_flags(result);
                self.write_reg8(reg, result);
                (1, 5)
            }
            Instruction::Mvi(reg, value) => {
                self.write_reg8(reg, value);
                (2, 7)
            }
            Instruction::Rlc => {
                let bit = self.cpu.a >> 7;
                self.cpu.a = (self.cpu.a << 1) | bit;
                set_bit_enabled(&mut self.cpu.flags, CARRY_FLAG, bit == 1);
                (1, 4)
            }
            Instruction::Ral => {
                let bit = self.cpu.a >> 7;
                self.cpu.a = (self.cpu.a << 1) | (get_bit(self.cpu.flags, CARRY_FLAG) as u8);
                set_bit_enabled(&mut self.cpu.flags, CARRY_FLAG, bit == 1);
                (1, 4)
            }
            // Instruction::Daa
            Instruction::Stc => {
                set_bit(&mut self.cpu.flags, CARRY_FLAG);
                (1, 4)
            }
            _ => unimplemented!("[EXECUTOR]: {:?}", instr),
        }
    }
}
