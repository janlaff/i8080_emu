use super::cpu::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Register {
    A,
    Flags,
    B,
    C,
    D,
    E,
    H,
    L,
    SP,
    M,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instruction {
    Nop,
    Lxi(Register, u16),
    Stax(Register),
    Shld(u16),
    Sta(u16),
    Inx(Register),
    Inr(Register),
    Dcr(Register),
    Mvi(Register, u8),
    Rlc,
    Ral,
    Daa,
    Stc,
    Dad(Register),
    Ldax(Register),
    Lhld(u16),
    Lda(u16),
    Dcx(Register),
    Rrc,
    Rar,
    Cma,
    Cmc,
    Mov(Register, Register),
    Hlt,
    Add(Register),
    Adc(Register),
    Sub(Register),
    Sbb(Register),
    Ana(Register),
    Xra(Register),
    Ora(Register),
    Cmp(Register),
    Rnz,
    Rnc,
    Rpo,
    Rp,
    Pop(Register),
    Jnz(u16),
    Jnc(u16),
    Jpo(u16),
    Jp(u16),
    Jmp(u16),
    Out(u8),
    Xthl,
    Di,
    Cnz(u16),
    Cnc(u16),
    Cpo(u16),
    Cp(u16),
    Push(Register),
    Adi(u8),
    Sui(u8),
    Ani(u8),
    Ori(u8),
    Rst(usize),
    Rz,
    Rc,
    Rpe,
    Rm,
    Ret,
    Pchl,
    Sphl,
    Jz(u16),
    Jc(u16),
    Jpe(u16),
    Jm(u16),
    In(u8),
    Xchg,
    Ei,
    Cz(u16),
    Cc(u16),
    Cpe(u16),
    Cm(u16),
    Call(u16),
    Aci(u8),
    Sbi(u8),
    Xri(u8),
    Cpi(u8),
}

pub fn decode_instruction(opcode: u8, cpu: &mut CPU) -> Instruction {
    let result = match opcode {
        0x00 | 0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 => Instruction::Nop,
        0x01 => Instruction::Lxi(Register::B, cpu.read_word()),
        0x11 => Instruction::Lxi(Register::D, cpu.read_word()),
        0x21 => Instruction::Lxi(Register::H, cpu.read_word()),
        0x31 => Instruction::Lxi(Register::SP, cpu.read_word()),
        0x02 => Instruction::Stax(Register::B),
        0x12 => Instruction::Stax(Register::D),
        0x22 => Instruction::Shld(cpu.read_word()),
        0x32 => Instruction::Sta(cpu.read_word()),
        0x03 => Instruction::Inx(Register::B),
        0x13 => Instruction::Inx(Register::D),
        0x23 => Instruction::Inx(Register::H),
        0x33 => Instruction::Inx(Register::SP),
        0x04 => Instruction::Inr(Register::B),
        0x14 => Instruction::Inr(Register::D),
        0x24 => Instruction::Inr(Register::H),
        0x34 => Instruction::Inr(Register::M),
        0x0C => Instruction::Inr(Register::C),
        0x1C => Instruction::Inr(Register::E),
        0x2C => Instruction::Inr(Register::L),
        0x3C => Instruction::Inx(Register::A),
        0x05 => Instruction::Dcr(Register::B),
        0x15 => Instruction::Dcr(Register::D),
        0x25 => Instruction::Dcr(Register::H),
        0x35 => Instruction::Dcr(Register::M),
        0x0D => Instruction::Dcr(Register::C),
        0x1D => Instruction::Dcr(Register::E),
        0x2D => Instruction::Dcr(Register::L),
        0x3D => Instruction::Dcr(Register::A),
        0x06 => Instruction::Mvi(Register::B, cpu.read_byte()),
        0x16 => Instruction::Mvi(Register::D, cpu.read_byte()),
        0x26 => Instruction::Mvi(Register::H, cpu.read_byte()),
        0x36 => Instruction::Mvi(Register::M, cpu.read_byte()),
        0x0E => Instruction::Mvi(Register::C, cpu.read_byte()),
        0x1E => Instruction::Mvi(Register::E, cpu.read_byte()),
        0x2E => Instruction::Mvi(Register::L, cpu.read_byte()),
        0x3E => Instruction::Mvi(Register::A, cpu.read_byte()),
        0x07 => Instruction::Rlc,
        0x17 => Instruction::Ral,
        0x27 => Instruction::Daa,
        0x37 => Instruction::Stc,
        0x09 => Instruction::Dad(Register::B),
        0x19 => Instruction::Dad(Register::D),
        0x29 => Instruction::Dad(Register::H),
        0x39 => Instruction::Dad(Register::SP),
        0x0A => Instruction::Ldax(Register::B),
        0x1A => Instruction::Ldax(Register::D),
        0x2A => Instruction::Lhld(cpu.read_word()),
        0x3A => Instruction::Lda(cpu.read_word()),
        0x0B => Instruction::Dcx(Register::B),
        0x1B => Instruction::Dcx(Register::D),
        0x2B => Instruction::Dcx(Register::H),
        0x3B => Instruction::Dcx(Register::SP),
        0x0F => Instruction::Rrc,
        0x1F => Instruction::Rar,
        0x2F => Instruction::Cma,
        0x3F => Instruction::Cmc,
        0x40 => Instruction::Mov(Register::B, Register::B),
        0x41 => Instruction::Mov(Register::B, Register::C),
        0x42 => Instruction::Mov(Register::B, Register::D),
        0x43 => Instruction::Mov(Register::B, Register::E),
        0x44 => Instruction::Mov(Register::B, Register::H),
        0x45 => Instruction::Mov(Register::B, Register::L),
        0x46 => Instruction::Mov(Register::B, Register::M),
        0x47 => Instruction::Mov(Register::B, Register::A),
        0x48 => Instruction::Mov(Register::C, Register::B),
        0x49 => Instruction::Mov(Register::C, Register::C),
        0x4A => Instruction::Mov(Register::C, Register::D),
        0x4B => Instruction::Mov(Register::C, Register::E),
        0x4C => Instruction::Mov(Register::C, Register::H),
        0x4D => Instruction::Mov(Register::C, Register::L),
        0x4E => Instruction::Mov(Register::C, Register::M),
        0x4F => Instruction::Mov(Register::C, Register::A),
        0x50 => Instruction::Mov(Register::D, Register::B),
        0x51 => Instruction::Mov(Register::D, Register::C),
        0x52 => Instruction::Mov(Register::D, Register::D),
        0x53 => Instruction::Mov(Register::D, Register::E),
        0x54 => Instruction::Mov(Register::D, Register::H),
        0x55 => Instruction::Mov(Register::D, Register::L),
        0x56 => Instruction::Mov(Register::D, Register::M),
        0x57 => Instruction::Mov(Register::D, Register::A),
        0x58 => Instruction::Mov(Register::E, Register::B),
        0x59 => Instruction::Mov(Register::E, Register::C),
        0x5A => Instruction::Mov(Register::E, Register::D),
        0x5B => Instruction::Mov(Register::E, Register::E),
        0x5C => Instruction::Mov(Register::E, Register::H),
        0x5D => Instruction::Mov(Register::E, Register::L),
        0x5E => Instruction::Mov(Register::E, Register::M),
        0x5F => Instruction::Mov(Register::E, Register::A),
        0x60 => Instruction::Mov(Register::H, Register::B),
        0x61 => Instruction::Mov(Register::H, Register::C),
        0x62 => Instruction::Mov(Register::H, Register::D),
        0x63 => Instruction::Mov(Register::H, Register::E),
        0x64 => Instruction::Mov(Register::H, Register::H),
        0x65 => Instruction::Mov(Register::H, Register::L),
        0x66 => Instruction::Mov(Register::H, Register::M),
        0x67 => Instruction::Mov(Register::H, Register::A),
        0x68 => Instruction::Mov(Register::L, Register::B),
        0x69 => Instruction::Mov(Register::L, Register::C),
        0x6A => Instruction::Mov(Register::L, Register::D),
        0x6B => Instruction::Mov(Register::L, Register::E),
        0x6C => Instruction::Mov(Register::L, Register::H),
        0x6D => Instruction::Mov(Register::L, Register::L),
        0x6E => Instruction::Mov(Register::L, Register::M),
        0x6F => Instruction::Mov(Register::L, Register::A),
        0x70 => Instruction::Mov(Register::M, Register::B),
        0x71 => Instruction::Mov(Register::M, Register::C),
        0x72 => Instruction::Mov(Register::M, Register::D),
        0x73 => Instruction::Mov(Register::M, Register::E),
        0x74 => Instruction::Mov(Register::M, Register::H),
        0x75 => Instruction::Mov(Register::M, Register::L),
        0x76 => Instruction::Hlt,
        0x77 => Instruction::Mov(Register::M, Register::A),
        0x78 => Instruction::Mov(Register::A, Register::B),
        0x79 => Instruction::Mov(Register::A, Register::C),
        0x7A => Instruction::Mov(Register::A, Register::D),
        0x7B => Instruction::Mov(Register::A, Register::E),
        0x7C => Instruction::Mov(Register::A, Register::H),
        0x7D => Instruction::Mov(Register::A, Register::L),
        0x7E => Instruction::Mov(Register::A, Register::M),
        0x7F => Instruction::Mov(Register::A, Register::A),
        0x80 => Instruction::Add(Register::B),
        0x81 => Instruction::Add(Register::C),
        0x82 => Instruction::Add(Register::D),
        0x83 => Instruction::Add(Register::E),
        0x84 => Instruction::Add(Register::H),
        0x85 => Instruction::Add(Register::L),
        0x86 => Instruction::Add(Register::M),
        0x87 => Instruction::Add(Register::A),
        0x88 => Instruction::Adc(Register::B),
        0x89 => Instruction::Adc(Register::C),
        0x8A => Instruction::Adc(Register::D),
        0x8B => Instruction::Adc(Register::E),
        0x8C => Instruction::Adc(Register::H),
        0x8D => Instruction::Adc(Register::L),
        0x8E => Instruction::Adc(Register::M),
        0x8F => Instruction::Adc(Register::A),
        0x90 => Instruction::Sub(Register::B),
        0x91 => Instruction::Sub(Register::C),
        0x92 => Instruction::Sub(Register::D),
        0x93 => Instruction::Sub(Register::E),
        0x94 => Instruction::Sub(Register::H),
        0x95 => Instruction::Sub(Register::L),
        0x96 => Instruction::Sub(Register::M),
        0x97 => Instruction::Sub(Register::A),
        0x98 => Instruction::Sbb(Register::B),
        0x99 => Instruction::Sbb(Register::C),
        0x9A => Instruction::Sbb(Register::D),
        0x9B => Instruction::Sbb(Register::E),
        0x9C => Instruction::Sbb(Register::H),
        0x9D => Instruction::Sbb(Register::L),
        0x9E => Instruction::Sbb(Register::M),
        0x9F => Instruction::Sbb(Register::A),
        0xA0 => Instruction::Ana(Register::B),
        0xA1 => Instruction::Ana(Register::C),
        0xA2 => Instruction::Ana(Register::D),
        0xA3 => Instruction::Ana(Register::E),
        0xA4 => Instruction::Ana(Register::H),
        0xA5 => Instruction::Ana(Register::L),
        0xA6 => Instruction::Ana(Register::M),
        0xA7 => Instruction::Ana(Register::A),
        0xA8 => Instruction::Xra(Register::B),
        0xA9 => Instruction::Xra(Register::C),
        0xAA => Instruction::Xra(Register::D),
        0xAB => Instruction::Xra(Register::E),
        0xAC => Instruction::Xra(Register::H),
        0xAD => Instruction::Xra(Register::L),
        0xAE => Instruction::Xra(Register::M),
        0xAF => Instruction::Xra(Register::A),
        0xB0 => Instruction::Ora(Register::B),
        0xB1 => Instruction::Ora(Register::C),
        0xB2 => Instruction::Ora(Register::D),
        0xB3 => Instruction::Ora(Register::E),
        0xB4 => Instruction::Ora(Register::H),
        0xB5 => Instruction::Ora(Register::L),
        0xB6 => Instruction::Ora(Register::M),
        0xB7 => Instruction::Ora(Register::A),
        0xB8 => Instruction::Cmp(Register::B),
        0xB9 => Instruction::Cmp(Register::C),
        0xBA => Instruction::Cmp(Register::D),
        0xBB => Instruction::Cmp(Register::E),
        0xBC => Instruction::Cmp(Register::H),
        0xBD => Instruction::Cmp(Register::L),
        0xBE => Instruction::Cmp(Register::M),
        0xBF => Instruction::Cmp(Register::A),
        0xC0 => Instruction::Rnz,
        0xD0 => Instruction::Rnc,
        0xE0 => Instruction::Rpo,
        0xF0 => Instruction::Rp,
        0xC1 => Instruction::Pop(Register::B),
        0xD1 => Instruction::Pop(Register::D),
        0xE1 => Instruction::Pop(Register::H),
        0xF1 => Instruction::Pop(Register::A),
        0xC2 => Instruction::Jnz(cpu.read_word()),
        0xD2 => Instruction::Jnc(cpu.read_word()),
        0xE2 => Instruction::Jpo(cpu.read_word()),
        0xF2 => Instruction::Jp(cpu.read_word()),
        0xC3 | 0xCB => Instruction::Jmp(cpu.read_word()),
        0xD3 => Instruction::Out(cpu.read_byte()),
        0xE3 => Instruction::Xthl,
        0xF3 => Instruction::Di,
        0xC4 => Instruction::Cnz(cpu.read_word()),
        0xD4 => Instruction::Cnc(cpu.read_word()),
        0xE4 => Instruction::Cpo(cpu.read_word()),
        0xF4 => Instruction::Cp(cpu.read_word()),
        0xC5 => Instruction::Push(Register::B),
        0xD5 => Instruction::Push(Register::D),
        0xE5 => Instruction::Push(Register::H),
        0xF5 => Instruction::Push(Register::A),
        0xC6 => Instruction::Adi(cpu.read_byte()),
        0xD6 => Instruction::Sui(cpu.read_byte()),
        0xE6 => Instruction::Ani(cpu.read_byte()),
        0xF6 => Instruction::Ori(cpu.read_byte()),
        0xC7 => Instruction::Rst(0),
        0xD7 => Instruction::Rst(2),
        0xE7 => Instruction::Rst(4),
        0xF7 => Instruction::Rst(6),
        0xC8 => Instruction::Rz,
        0xD8 => Instruction::Rc,
        0xE8 => Instruction::Rpe,
        0xF8 => Instruction::Rm,
        0xC9 | 0xD9 => Instruction::Ret,
        0xE9 => Instruction::Pchl,
        0xF9 => Instruction::Sphl,
        0xCA => Instruction::Jz(cpu.read_word()),
        0xDA => Instruction::Jc(cpu.read_word()),
        0xEA => Instruction::Jpe(cpu.read_word()),
        0xFA => Instruction::Jm(cpu.read_word()),
        0xDB => Instruction::In(cpu.read_byte()),
        0xEB => Instruction::Xchg,
        0xFB => Instruction::Ei,
        0xCC => Instruction::Cz(cpu.read_word()),
        0xDC => Instruction::Cc(cpu.read_word()),
        0xEC => Instruction::Cpe(cpu.read_word()),
        0xFC => Instruction::Cm(cpu.read_word()),
        0xCD | 0xDD | 0xED | 0xFD => Instruction::Call(cpu.read_word()),
        0xCE => Instruction::Aci(cpu.read_byte()),
        0xDE => Instruction::Sbi(cpu.read_byte()),
        0xEE => Instruction::Xri(cpu.read_byte()),
        0xFE => Instruction::Cpi(cpu.read_byte()),
        0xCF => Instruction::Rst(1),
        0xDF => Instruction::Rst(3),
        0xEF => Instruction::Rst(5),
        0xFF => Instruction::Rst(7),
    };

    println!("[DECODER]: Decoded opcode {:02X}h to {:?}", opcode, result);

    result
}