pub struct Bus {
    memory: [u8; 0xffff],
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: [0; 0xffff],
        }
    }

    pub fn load_bytes(&mut self, starting_address: u16, data: &[u8]) {
        for i in 0..data.len() {
            let idx = starting_address as usize + i;
            self.memory[idx] = data[i];
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        let result = *self.get_mapped_location(address);
        println!("[BUS]: Reading {:02X}h from {:04X}h", result, address);
        result
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        println!("[BUS]: Writing {:02X}h to {:04X}h", value, address);
        *self.get_mapped_location(address) = value;
    }

    fn get_mapped_location(&mut self, address: u16) -> &mut u8 {
        // Map custom memory regions here
        match address {
            _ => &mut self.memory[address as usize],
        }
    }
}
