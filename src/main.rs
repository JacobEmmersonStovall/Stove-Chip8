mod memory;
mod registers;
mod display;

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
    test_registers.push_to_stack(0x0AFB);
    println!("{}",test_registers);
    
    let mut display = display::Display::new();

    while display.get_is_open(){
        display.update_display(vec![1; 640 * 400]);
    }
}
