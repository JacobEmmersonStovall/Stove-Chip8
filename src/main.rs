mod memory;
mod registers;

fn main() {
    let mut test_mem = memory::Memory::new();
    test_mem.set_value(0,100);
    test_mem.set_value(4095,0xB2);
    println!("{}", test_mem);
    println!("");
    println!("{:04X}",test_mem.get_value(0));
    println!("");

    let mut test_registers = registers::Registers::new();
    test_registers.set_general_register_value(0xA,0xBB);
    println!("{}",test_registers);
}
