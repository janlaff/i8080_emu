pub fn set_bit_enabled(value: &mut u8, bit: usize, enabled: bool) {
    if enabled {
        set_bit(value, bit);
    } else {
        clear_bit(value, bit);
    }
}

pub fn set_bit(value: &mut u8, bit: usize) {
    *value |= 1 << bit;
}

pub fn clear_bit(value: &mut u8, bit: usize) {
    *value &= !(1 << bit)
}

pub fn get_bit(value: u8, bit: usize) -> bool {
    value & (1 << bit) > 0
}

pub fn get_low_byte(reg: u16) -> u8 {
    (reg & 0xff) as u8
}

pub fn get_high_byte(reg: u16) -> u8 {
    (reg >> 8) as u8
}

pub fn set_low_byte(reg: &mut u16, val: u8) {
    *reg &= 0xff00;
    *reg |= val as u16;
}

pub fn set_high_byte(reg: &mut u16, val: u8) {
    *reg &= 0x00ff;
    *reg |= (val as u16) << 8
}

pub fn join_bytes(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

pub fn get_enabled_bits(value: u8) -> u8 {
    let mut n = 0;
    for i in 0..8 {
        if (value >> i) & 0x1 == 0x1 {
            n += 1;
        }
    }
    n
}
