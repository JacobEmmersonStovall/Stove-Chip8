use std::fmt;

pub struct Memory{
    body: [u8; 4096]
}

impl Memory{
    pub fn new()-> Memory{
        Memory{body: [0;4096]}
    }

    pub fn set_value(&mut self, index:usize, value:u8){
        self.body[index] = value;
    }

    pub fn get_value(&self, index:usize) -> u8{
        self.body[index]
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut total_string = String::from("");
        for i in 0..self.body.len() {
            let current_num = self.body[i];
            let current_num_str = format!("{:02X}",current_num);

            let space_or_new_line;
            if i % 20 == 19{
                space_or_new_line = String::from("\n");
            }
            else if i % 20 == 0{
                space_or_new_line = String::from(" ");
                let current_i_str = format!("{:04X}",i);
                let colon = String::from(":");
                let space = String::from(" ");
                total_string = total_string + &current_i_str + &colon + &space;
            }
            else{
                space_or_new_line = String::from(" ");
            }

            total_string = total_string + &current_num_str + &space_or_new_line;
        }
        write!(f,"{}",total_string)
    }
}