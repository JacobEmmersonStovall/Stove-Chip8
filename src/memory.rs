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
        let mut totalString = String::from("");
        for i in 0..self.body.len() {
            let currentNum = self.body[i];
            let currentNumStr = format!("{:02X}",currentNum);

            let spaceOrNewLine;
            if i % 20 == 19{
                spaceOrNewLine = String::from("\n");
            }
            else if i % 20 == 0{
                spaceOrNewLine = String::from(" ");
                let currentIStr = format!("{:04X}",i);
                let colon = String::from(":");
                let space = String::from(" ");
                totalString = totalString + &currentIStr + &colon + &space;
            }
            else{
                spaceOrNewLine = String::from(" ");
            }

            //let currentIStr = i.to_string();
            //let colon = String::from(":");

            //let newLine = String::from("\n");
            totalString = totalString + &currentNumStr + &spaceOrNewLine;
        }
        write!(f,"{}",totalString)
    }
}