extern crate sdl2;
extern crate rand;

use std::fmt;
use super::super::screen::Screen;
use super::super::keypad::Keypad;
use cpu::instruction::Instruction;
use cpu::instruction::Opcode;
use cpu::instruction::Operands;
use rand::Rng;

pub struct Cpu {
    pc: u16,
    ram: [u8;0x1000],
    screen: Screen,
    pub keypad: Keypad,
    gpr: [u8;16],
    i: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    rng: rand::ThreadRng,
    waiting_for_key: bool
}

impl Cpu {
    pub fn new (rom: &Vec<u8>) -> Cpu {
        let mut ret = Cpu {
            pc: 0x200,
            ram: [0;0x1000],
            screen: Screen::new(),
            keypad: Keypad::new(),
            gpr: [0;16],
            i: 0,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            rng: rand::thread_rng(),
            waiting_for_key: false
        };
        for x in 0..rom.len() {
            //load ROM file to RAM at 0x0200
            ret.ram[0x200+x] = rom[x];
        }
        Cpu::load_sprites(&mut ret.ram);
        return ret;
    }
    
    fn load_sprites(ram: &mut [u8;0x1000]) {
        let sprites = [
            [0xF0,0x90,0x90,0x90,0xF0], //0
            [0x20,0x60,0x20,0x20,0x70], //1
            [0xF0,0x10,0xF0,0x80,0xF0], //2
            [0xF0,0x10,0xF0,0x10,0xF0], //3
            [0x90,0x90,0xF0,0x10,0x10], //4
            [0xF0,0x80,0xF0,0x10,0xF0], //5
            [0xF0,0x80,0xF0,0x90,0xF0], //6
            [0xF0,0x10,0x20,0x40,0x40], //7
            [0xF0,0x90,0xF0,0x90,0xF0], //8
            [0xF0,0x90,0xF0,0x10,0xF0], //9
            [0xF0,0x90,0xF0,0x90,0x90], //A
            [0xE0,0x90,0xE0,0x90,0xE0], //B
            [0xE0,0x90,0xE0,0x90,0xE0], //C
            [0xE0,0x90,0x90,0x90,0xE0], //D
            [0xF0,0x80,0xF0,0x80,0xF0], //E
            [0xF0,0x80,0xF0,0x80,0x80]];//F
        for x in 0..16 {
            for y in 0..5 {
                //Load sprites to ram starting at 0x0000
                ram[((x*5)+y) as usize] = sprites[x as usize][y as usize];
            }
        }
    }

    pub fn execute_next_instruction(&mut self) {
        let instruction_bytes = ((self.ram[self.pc as usize] as u16) << 8 as u16) + (self.ram[(self.pc+1) as usize]) as u16;
        let instr = Instruction::decode(instruction_bytes);

        self.pc+=2;
        let Operands {reg_x, reg_y, address, imm} = instr.get_operands(instruction_bytes);

        match instr.opcode {
            Opcode::Add => {
                //The values of Vx and Vy are added together.
                //If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
                //Only the lowest 8 bits of the result are kept, and stored in Vx.
                let sum = self.gpr[reg_x as usize] as u16 + self.gpr[reg_y as usize] as u16;
                if sum > 0xFF {
                    self.gpr[0xF] = 1;
                } else {
                    self.gpr[0xF] = 0;
                }
                self.gpr[reg_x as usize] = (sum & 0x00FF) as u8
            }
            Opcode::Addir => {
                //The values of I and Vx are added, and the results are stored in I.
                self.i = self.i.wrapping_add(self.gpr[reg_x as usize] as u16);
                if(self.i as u32) + self.gpr[reg_x as usize] as u32 > 0xffff {
                    panic!("I overflow");
                }
            }
            Opcode::Addi => {
                //Add imm
                if self.gpr[reg_x as usize] as u16 + imm as u16 > 255 {
                    self.gpr[reg_x as usize] = self.gpr[reg_x as usize].wrapping_add(imm);
                } else {
                    self.gpr[reg_x as usize] = self.gpr[reg_x as usize].wrapping_add(imm);
                }
            }
            Opcode::And => {
                //And
                self.gpr[reg_x as usize]=self.gpr[reg_x as usize] & self.gpr[reg_y as usize];
            }
            Opcode::Call => {
                //Call
                self.stack.push(self.pc);
                self.pc=address;
            }
            Opcode::Cls => {
                //Clear screen
                self.screen.clear();
            }
            Opcode::Drw => {
                //Draw sprite
                let mut sprite = vec![];
                for offset in 0..imm {
                    let byte = self.ram[(self.i+(offset as u16)) as usize];
                    sprite.push((byte & 0b1000_0000) != 0);
                    sprite.push((byte & 0b0100_0000) != 0);
                    sprite.push((byte & 0b0010_0000) != 0);
                    sprite.push((byte & 0b0001_0000) != 0);
                    sprite.push((byte & 0b0000_1000) != 0);
                    sprite.push((byte & 0b0000_0100) != 0);
                    sprite.push((byte & 0b0000_0010) != 0);
                    sprite.push((byte & 0b0000_0001) != 0);
                }
                let pos_x = self.gpr[reg_x as usize];
                let pos_y = self.gpr[reg_y as usize];
                let vf = self.screen.draw_sprire(pos_x, pos_y, sprite);
                if vf {
                    self.gpr[0xF]=1;
                } else {
                    self.gpr[0xF]=0;
                }
            }
            Opcode::Fetch => {
                //Read registers V0 through Vx from memory starting at location I.
                for x in 0..reg_x+1 {
                    self.gpr[x as usize] = self.ram[(self.i + (x as u16)) as usize];
                }
            }
            Opcode::Ibcd => {
                // Store Vx as a binary coded decimal at location I
                let x = self.gpr[reg_x as usize];
                let h = x/100;
                let t = (x%100)/10;
                let o = x%10;
                self.ram[self.i as usize] = h;
                self.ram[(self.i+1) as usize] = t;
                self.ram[(self.i+2) as usize] = o;
            }
            Opcode::Jp => {
                //Jump
                self.pc=address;

            }
            Opcode::Ld => {
                //Stores the value of register Vy in register Vx.
                self.gpr[reg_x as usize]=self.gpr[reg_y as usize];
            }
            Opcode::Lddt => {
                //Set Vx = delay timer value.
                self.gpr[reg_x as usize] = self.delay_timer;
            }
            Opcode::Ldi => {
                //Load x with imm
                self.gpr[reg_x as usize] = imm;
            }
            Opcode::Ldkp => {
                //Wait for a key press, store the value of the key in Vx.
                if self.waiting_for_key {
                    if self.keypad.new_key {
                        self.waiting_for_key=false;
                        self.gpr[reg_x as usize] = self.keypad.get_last_key();
                    } else {
                        self.pc-=2;
                    }
                } else {
                    self.waiting_for_key=true;
                    self.keypad.set_wait();
                    self.pc-=2;
                }
            }
            Opcode::Or => {
                //Or
                self.gpr[reg_x as usize]=self.gpr[reg_x as usize] | self.gpr[reg_y as usize];
            }
            Opcode::Ret => {
                //Return
                match self.stack.pop() {
                    Some(x) => {
                        self.pc = x;
                    }
                    None => {
                        panic!("Ret called with empty stack");
                    }
                }
            }
            Opcode::Rnd => {
                //Random
                self.gpr[reg_x as usize] = (self.rng.gen::<f64>()*(255) as f64) as u8 & imm;
            }
            Opcode::Se => {
                //Skip next instruction if Vx == Vy.
                if self.gpr[reg_x as usize] == self.gpr[reg_y as usize] {
                    self.pc+=2;
                }
            }
            Opcode::Sei => {
                //Skip next instruction if Vx == kk.
                if self.gpr[reg_x as usize] == imm {
                    self.pc+=2;
                }
            }
            Opcode::Setdt => {
                //Set delay timer = Vx.
                self.delay_timer = self.gpr[reg_x as usize];
            }
            Opcode::Seti => {
                //load i with address
                self.i=address;
            }
            Opcode::Setis => {
                //Set I = location of sprite for digit Vx.
                self.i = (self.gpr[reg_x as usize] as u16)*0x5;
            }
            Opcode::Setst => {
                //Sets the sound timer to VX.
                self.sound_timer = self.gpr[reg_x as usize];
            }
            Opcode::Shl => {
                //Left shift.
                self.gpr[0xf] = (self.gpr[reg_x as usize] & (0x80) != 0) as u8;
                self.gpr[reg_x as usize] = self.gpr[reg_x as usize].wrapping_shl(1);
            }
            Opcode::Shr => {
                //Right Shift
                self.gpr[0xf] = (self.gpr[reg_x as usize] &1 ) as u8;
                self.gpr[reg_x as usize] = self.gpr[reg_x as usize].wrapping_shr(1);
            }
            Opcode::Skp => {
                //Skip next instruction if key with the value of Vx is pressed.
                if self.keypad.keys[self.gpr[reg_x as usize] as usize] == true {
                    self.pc+=2;
                }
            }
            Opcode::Sknp => {
                //Skip next instruction if key with the value of Vx is not pressed.
                if self.keypad.keys[self.gpr[reg_x as usize] as usize] == false {
                    self.pc+=2;
                }
            }
            Opcode::Sne => {
                //Skip next instruction if Vx != Vy.
                if self.gpr[reg_x as usize] != self.gpr[reg_y as usize] {
                    self.pc+=2;
                }
            }
            Opcode::Snei => {
                //Skip next instruction if Vx != kk.
                if self.gpr[reg_x as usize] != imm {
                    self.pc+=2;
                }
            }
            Opcode::Stri => {
                //Stores V0 to VX in memory starting at address I
                for x in 0..reg_x+1 {
                    self.ram[(self.i + (x as u16)) as usize]=self.gpr[x as usize];
                }
            }
            Opcode::Sub => {
                //Set Vx = Vx - Vy, set VF = NOT borrow.
                if self.gpr[reg_y as usize] > self.gpr[reg_x as usize] {
                    self.gpr[0xf] = 0;
                } else {
                    self.gpr[0xf] = 1;
                }
                self.gpr[reg_x as usize] = self.gpr[reg_x as usize].wrapping_sub(self.gpr[reg_y as usize]);
            }
            Opcode::Subn => {
                //Set Vx = Vy - Vx, set VF = NOT borrow.
                if self.gpr[reg_y as usize] > self.gpr[reg_x as usize] {
                    self.gpr[0xf] = 0;
                } else {
                    self.gpr[0xf] = 1;
                }
                self.gpr[reg_x as usize] = self.gpr[reg_y as usize].wrapping_sub(self.gpr[reg_x as usize]);
            }
            Opcode::Xor => {
                //Xor
                self.gpr[reg_x as usize]=self.gpr[reg_x as usize] ^ self.gpr[reg_y as usize];
            }
            _ => {
                panic!("Unhandled opcode {:?} {:#04X}", instr, instruction_bytes)
            }
        }
    }

    pub fn tick_timers(&mut self){
        if self.delay_timer > 0 {
            self.delay_timer-=1;
        }
        if self.sound_timer > 0 {
            self.sound_timer-=1;
        }
    }

    pub fn get_sound_state(&self) -> bool {
        if self.sound_timer > 0 {
            return true;
        } else {
            return false;
        }
    }
    pub fn draw(&mut self, renderer: &mut sdl2::render::Renderer) {
        self.screen.draw(renderer);
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "pc: 0x{:04X} i:0x{:02X} stack:{} delay_timer{:#02X}", self.pc, self.i, self.stack.len(), self.delay_timer);

        for x in 0..16 {
            let _ = write!(f, "v{}=0x{:02X} ", x, self.gpr[x]);
        }
        write!(f, "")
    }
}
