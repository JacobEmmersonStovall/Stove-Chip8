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
            sp: 0xFF
        }
    }

    //General Register Getters and Setters
    pub fn get_general_register_value(&self, index:usize) -> u8{
        self.general[index]
    }

    pub fn set_general_register_value(&mut self, index:usize, value:u8){
        self.general[index] = value;
    }

    pub fn push_to_stack(&mut self, value:u16){
        if self.sp == 0xFF{
            self.sp = 0;
        }
        else{
            self.sp = self.sp + 1;
        }
        self.stack[(self.sp as usize)] = value;
    }

    pub fn peek_at_stack(&self) -> u16 {
        self.stack[(self.sp as usize)]
    }

    pub fn pop_from_stack(&mut self) -> u16 {
        let returnValue = self.stack[(self.sp as usize)];
        if self.sp == 0{
            self.sp = 0xFF;
        }
        else{
            self.sp = self.sp - 1;
        }
        returnValue
    }

    pub fn set_pc(&mut self, value:u16){
        self.pc = value;
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn set_i(&mut self, value:u16){
        self.i = value;
    }

    pub fn get_i(&self) -> u16 {
        self.i
    }

    pub fn set_delay(&mut self, value:u8){
        self.delay = value;
    }

    pub fn get_delay(&self) -> u8 {
        self.delay
    }

    pub fn decrement_delay(&mut self){
        if self.delay > 0{
            self.delay = self.delay - 1;
        }
    }

    pub fn set_sound(&mut self, value:u8){
        self.sound = value;
    }

    pub fn get_sound(&self) -> u8 {
        self.sound
    }

    pub fn decrement_sound(&mut self){
        if self.sound > 0{
            self.sound = self.sound - 1;
        }
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