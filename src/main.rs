mod memory;
mod registers;
mod display;

fn main() {
    let mut test_mem = memory::Memory::new();
    test_mem.set_value(0,0x25);
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

   // while display.get_is_open(){
        execute_instruction(&test_mem,&test_registers,&display);
        //display.update_display(vec![1; 640 * 400]);
    //}
}

fn split_mem_chunk_into_nibbles(value:u8) -> [u8;2]{
    let mut x: [u8;2] = [0,0];
    x[0] = value & 0xF0;
    x[1] = value & 0x0F;
    x
}

fn execute_instruction(mem:&memory::Memory, reg:&registers::Registers, disp:&display::Display){
    let pc = reg.get_pc();
    let value1 = split_mem_chunk_into_nibbles(mem.get_value((pc as usize)));
    let value2 = split_mem_chunk_into_nibbles(mem.get_value(((pc+1) as usize)));
    //println!("{}",value1);
    let instruct_tuple: (u8,u8,u8,u8) = (value1[0], value1[1],value2[0], value2[1]);
    match instruct_tuple{
        (0x00,0x00,0xE0,0x00) => println!("CLS"),
        (0x00,0x00,0xE0,0x0E) => println!("RET"),
        (0x00,_,_,_) => println!("SYS"),
        (0x10,_,_,_) => println!("JP"),
        (0x20,_,_,_) => println!("CALL"),
        (_,_,_,_) => println!("DEFAULT")
    }
}