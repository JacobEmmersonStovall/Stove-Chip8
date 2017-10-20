extern crate minifb;
use self::minifb::{Window, WindowOptions};

pub struct Display{
    display_window: Window
}

fn createDisplay() -> Window{
    let WINDOW_WIDTH = 640;
    let WINDOW_HEIGHT = 360;
    let window = Window::new("Stove Chip8", WINDOW_WIDTH, WINDOW_HEIGHT, WindowOptions::default()).unwrap();
    window
}

impl Display{
    pub fn new()->Display{
        Display{
            display_window: createDisplay()
        }
    }

    pub fn update_display(&mut self,buffer:Vec<u32>){
        self.display_window.update_with_buffer(&buffer);
    }

    pub fn get_is_open(&self) -> bool{
        self.display_window.is_open()
    }
}