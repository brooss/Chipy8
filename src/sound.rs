extern crate sdl2;
pub struct Sound {
    device: sdl2::audio::AudioDevice<SquareWave>,
    running: bool
}

impl Sound {
    pub fn new (audio_subsystem: &sdl2::AudioSubsystem) -> Sound {
        Sound {
            device: setup_audio(audio_subsystem),
            running: false
        }
    }
    pub fn start(&mut self) {
        if !self.running {
            self.running = true;
            self.device.resume();
        }
    }

    pub fn stop(&mut self) {
        if self.running {
            self.running = false;
            self.device.pause();
        }
    }

    pub fn set_state(&mut self, run: bool) {
        if run {
            self.start();
        } else {
            self.stop();
        }
    }
}
fn setup_audio(audio_subsystem: &sdl2::AudioSubsystem) -> sdl2::audio::AudioDevice<SquareWave>{
    let desired_spec = sdl2::audio::AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),  // mono
        samples: None       // default sample size
    };

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
    // initialize the audio callback
        SquareWave {
            phase_inc: 44.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.01,
        }}).unwrap();
    return device;
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}


impl sdl2::audio::AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = match self.phase {
                0.0...0.5 => self.volume,
                _ => -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}
