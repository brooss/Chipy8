extern crate rand;
extern crate sdl2;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

mod cpu;
mod screen;
mod keypad;
mod sound;
mod input;

fn main() {
    let sdl_context = sdl2::init().expect("Failed to init SDL2");
    let mut event_pump = sdl_context.event_pump().expect("Failed to init SDL2 event_pump");
    let video_subsystem = sdl_context.video().expect("Failed to init SDL2 video");
    let audio_subsystem = sdl_context.audio().expect("Failed to init SDL2 audio");
    let timer = sdl_context.timer().expect("Failed to init SDL2 timer subsystem");

    //Load the rom file
    let rom_file_name;
    match env::args().nth(1) {
        Some(r) => {
            rom_file_name = r;
        }
        _ => {
            println!("Usage: chipy romfile.ch8");
            return;
        }
    }
    let path = Path::new(&rom_file_name);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };
    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer);
    println!("\"{}\" read successfully", display);

    //Create the window
    let window = video_subsystem.window(&format!("Chipy8 - Current key mappings: {:?}", input::Mappings::Default), 1024, 512).resizable().build().expect("Failed to create window");

    //Setup timers
    let mut last_tick = 0;
    let mut cycles = 0;

    //let audio_device = sound::setup_audio(&audio_subsystem);
    let mut sound = sound::Sound::new(&audio_subsystem);
    let current_mapping = input::Mappings::Default;
    let mut input = input::Input::new(current_mapping);
    let mut renderer = window.renderer().present_vsync().build().expect("Failed to create SDL2 renderer");

    let mut cpu = cpu::cpu::Cpu::new(&buffer);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    break 'running
                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F1), .. } => {
                    let mut window = renderer.window_mut().unwrap();
                    window.set_title(&format!("Chipy8 - Current key mappings: {:?}", input::Mappings::Default));
                    input = input::Input::new(input::Mappings::Default);
                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F2), .. } => {
                    let mut window = renderer.window_mut().unwrap();
                    window.set_title(&format!("Chipy8 - Current key mappings: {:?}", input::Mappings::Alt));
                    input = input::Input::new(input::Mappings::Alt);
                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F3), .. } => {
                    let mut window = renderer.window_mut().unwrap();
                    window.set_title(&format!("Chipy8 - Current key mappings: {:?}", input::Mappings::Tetris));
                    input = input::Input::new(input::Mappings::Tetris);
                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F10), .. } |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Period), .. } => {
                    // Reset key pressed
                    cpu = cpu::cpu::Cpu::new(&buffer);
                },
            _ => {}
            }
            input.handle_keys(&mut cpu.keypad, event);
        }
        let now = timer.performance_counter();

        //No set speed for the chip8 CPU
        //This seems about right
        if now - cycles > ((timer.performance_frequency()/1000/1000)*555) {
            cpu.execute_next_instruction();
            cycles = timer.performance_counter();
        }

        //Cap at 120FPS
        if ((now - last_tick)*1000)/timer.performance_frequency() > (1000/120) {
            last_tick = now;
            cpu.draw(&mut renderer);
            cpu.tick_timers();
        }
        sound.set_state(cpu.get_sound_state());
    }
}
