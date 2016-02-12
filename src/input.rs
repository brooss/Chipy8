extern crate sdl2;

use keypad;

pub struct Input {
    mapping: Vec<&'static KeyMapping>
}

impl Input {
    pub fn new(selected: Mappings) -> Input {
        let mut ret = Input {
            mapping: Vec::new()
        };
        match selected {
            Mappings::Tetris => {
                for x in 0..TETRIS.len() {
                    ret.mapping.push(&TETRIS[x]);
                }
            }
            Mappings::Alt => {
                for x in 0..ALT.len() {
                    ret.mapping.push(&ALT[x]);
                }
            }
            Mappings::Default => {
                for x in 0..DEFAULT.len() {
                    ret.mapping.push(&DEFAULT[x]);
                }
            }
        }
        return ret;
    }

    pub fn handle_keys(&self, keypad: &mut keypad::Keypad, event: sdl2::event::Event) {
        match event {
            sdl2::event::Event::KeyDown {keycode, ..} => {
                match keycode {
                    Some(sdlkeycode) => {
                        for x in 0..self.mapping.len() {
                            if sdlkeycode == self.mapping[x].key {
                                keypad.key_down(self.mapping[x].keypad);
                            }
                        }
                    }
                    _ => {}
                }
            }
            sdl2::event::Event::KeyUp {keycode, ..} => {
                match keycode {
                    Some(sdlkeycode) => {
                        for x in 0..self.mapping.len() {
                            if sdlkeycode == self.mapping[x].key {
                                keypad.key_up(self.mapping[x].keypad);
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

pub struct KeyMapping {
    key: sdl2::keyboard::Keycode,
    keypad: u8
}

#[derive(Debug)]
pub enum Mappings {
    Default,
    Tetris,
    Alt
}

pub static TETRIS: [KeyMapping; 15] = [
    KeyMapping{key: sdl2::keyboard::Keycode::Num0,  keypad: 0},
    KeyMapping{key: sdl2::keyboard::Keycode::Num1,  keypad: 1},
    KeyMapping{key: sdl2::keyboard::Keycode::Num2,  keypad: 2},
    KeyMapping{key: sdl2::keyboard::Keycode::Num3,  keypad: 3},
    KeyMapping{key: sdl2::keyboard::Keycode::Num4,  keypad: 4},
    KeyMapping{key: sdl2::keyboard::Keycode::Num5,  keypad: 5},
    KeyMapping{key: sdl2::keyboard::Keycode::Num6,  keypad: 6},
    KeyMapping{key: sdl2::keyboard::Keycode::Num7,  keypad: 7},
    KeyMapping{key: sdl2::keyboard::Keycode::Num8,  keypad: 8},
    KeyMapping{key: sdl2::keyboard::Keycode::Num9,  keypad: 9},
    KeyMapping{key: sdl2::keyboard::Keycode::W,     keypad: 4},
    KeyMapping{key: sdl2::keyboard::Keycode::A,     keypad: 5},
    KeyMapping{key: sdl2::keyboard::Keycode::D,     keypad: 6},
    KeyMapping{key: sdl2::keyboard::Keycode::S,     keypad: 7},
    KeyMapping{key: sdl2::keyboard::Keycode::Space, keypad: 0xF}];

pub static ALT: [KeyMapping; 15] = [
    KeyMapping{key: sdl2::keyboard::Keycode::Num0,  keypad: 0},
    KeyMapping{key: sdl2::keyboard::Keycode::Num1,  keypad: 1},
    KeyMapping{key: sdl2::keyboard::Keycode::Num2,  keypad: 2},
    KeyMapping{key: sdl2::keyboard::Keycode::Num3,  keypad: 3},
    KeyMapping{key: sdl2::keyboard::Keycode::Num4,  keypad: 4},
    KeyMapping{key: sdl2::keyboard::Keycode::Num5,  keypad: 5},
    KeyMapping{key: sdl2::keyboard::Keycode::Num6,  keypad: 6},
    KeyMapping{key: sdl2::keyboard::Keycode::Num7,  keypad: 7},
    KeyMapping{key: sdl2::keyboard::Keycode::Num8,  keypad: 8},
    KeyMapping{key: sdl2::keyboard::Keycode::Num9,  keypad: 9},
    KeyMapping{key: sdl2::keyboard::Keycode::W,     keypad: 8},
    KeyMapping{key: sdl2::keyboard::Keycode::A,     keypad: 4},
    KeyMapping{key: sdl2::keyboard::Keycode::D,     keypad: 6},
    KeyMapping{key: sdl2::keyboard::Keycode::S,     keypad: 2},
    KeyMapping{key: sdl2::keyboard::Keycode::Space, keypad: 0xF}];
pub static DEFAULT: [KeyMapping; 15] = [
    KeyMapping{key: sdl2::keyboard::Keycode::Num0,  keypad: 0},
    KeyMapping{key: sdl2::keyboard::Keycode::Num1,  keypad: 1},
    KeyMapping{key: sdl2::keyboard::Keycode::Num2,  keypad: 2},
    KeyMapping{key: sdl2::keyboard::Keycode::Num3,  keypad: 3},
    KeyMapping{key: sdl2::keyboard::Keycode::Num4,  keypad: 4},
    KeyMapping{key: sdl2::keyboard::Keycode::Num5,  keypad: 5},
    KeyMapping{key: sdl2::keyboard::Keycode::Num6,  keypad: 6},
    KeyMapping{key: sdl2::keyboard::Keycode::Num7,  keypad: 7},
    KeyMapping{key: sdl2::keyboard::Keycode::Num8,  keypad: 8},
    KeyMapping{key: sdl2::keyboard::Keycode::Num9,  keypad: 9},
    KeyMapping{key: sdl2::keyboard::Keycode::W,     keypad: 3},
    KeyMapping{key: sdl2::keyboard::Keycode::A,     keypad: 7},
    KeyMapping{key: sdl2::keyboard::Keycode::D,     keypad: 8},
    KeyMapping{key: sdl2::keyboard::Keycode::S,     keypad: 6},
    KeyMapping{key: sdl2::keyboard::Keycode::Space, keypad: 0xF}];
