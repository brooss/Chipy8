extern crate sdl2;

#[derive(Debug)]
pub struct Keypad {
    pub keys: [bool;16],
    pub wait: bool,
    pub new_key: bool,
    pub last_key: u8
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            keys: [false;16],
            wait: false,
            new_key: false,
            last_key: 0
        }
    }
    pub fn key_down(&mut self, key: u8) {
        self.keys[key as usize]=true;
        self.wait = false;
        self.new_key = true;
        self.last_key = key;
    }
    pub fn key_up(&mut self, key: u8) {
        self.keys[key as usize]=false;
    }
    pub fn set_wait(&mut self) {
        self.wait=true;
        self.new_key=false;
    }
    pub fn get_last_key(&self) -> u8{
        self.last_key
    }
}
