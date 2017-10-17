use std::fmt;

pub struct Registers{
    general: [u8; 16],
    stack: [u16; 16],
    i: u16,
    delay: u8,
    sound: u8,
    pc: u16,
    sp: u8
}

impl Registers{
    pub fn new()->Registers{
        Registers{
            general: [0;16],
            stack: [0;16],
            i: 0,
            delay: 0,
            sound: 0,
            pc: 0,
            sp: 0
        }
    }

    //General Register Getters and Setters
    pub fn get_general_register_value(&self, index:usize) -> u8{
        self.general[index]
    }

    pub fn set_general_register_value(&mut self, index:usize, value:u8){
        self.general[index] = value;
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut total_string = String::from("");
        let new_line = String::from("\n");
        
        //General Register Display
        total_string += &String::from("General Registers\n--------------------------------\n");
        for i in 0..self.general.len() {
            let current_reg_str = format!("V{:X}: {:02X}\n",i,self.general[i]);
            total_string += &current_reg_str;
        }
        total_string += &new_line;

        //Stack Display
        total_string += &String::from("Stack Addresses\n--------------------------------\n");
        for i in 0..self.stack.len() {
            let current_stack_str = format!("{:X}: {:04X}\n",i,self.stack[i]);
            total_string += &current_stack_str;
        }
        total_string += &new_line;

        //Other Variables Dispaly
        total_string += &String::from("Others\n--------------------------------\n");
        //I
        total_string += &format!("I: {:04X}\n",self.i);
        //Delay
        total_string += &format!("Delay: {:02X}\n",self.delay);
        //Sound
        total_string += &format!("Sound: {:02X}\n",self.sound);
        //PC
        total_string += &format!("PC: {:04X}\n",self.pc);
        //SP
        total_string += &format!("SP: {:02X}\n",self.sp);
        total_string += &new_line;

        write!(f,"{}",total_string)
    }
}