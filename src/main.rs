#[allow(dead_code)]
mod i8080;
// CPU Frequency:      2 MHZ
// Data Bus:           8 Bit
// Address Bus:        16 Bit
// Addressable memory: 64 KB
// Addressable IO:     256 B

fn main() {
    let mut cpu = i8080::CPU::new();

    println!("[*] Welcome to the i8080_emu Emulator Project");

    cpu.set_bc(0xDEAD);
    cpu.a = 0xFF;
    cpu.bus.write_byte(0x00, 0x02);

    {
        let mut executor = i8080::Executor::new(&mut cpu);
        executor.execute();
    }

    assert_eq!(cpu.bus.read_byte(0xDEAD), 0xFF);
}
