mod memory;

fn main() {
    let mut test = memory::Memory::new();
    test.set_value(0,100);
    test.set_value(4095,0xB2);
    println!("{}", test);
    println!("{:04X}",test.get_value(0));
}
