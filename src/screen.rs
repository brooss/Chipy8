extern crate sdl2;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Screen {
    vram: [[bool;HEIGHT];WIDTH],
    updated: bool
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            vram: [[false;HEIGHT];WIDTH],
            updated: true
        }
    }

    pub fn clear(&mut self) {
        for y in 0 .. HEIGHT {
            for x in 0 .. WIDTH {
                self.vram[x][y] = false;
            }
        }
    }

    pub fn draw_sprire(&mut self, mut pos_x: u8, mut pos_y: u8, sprite: Vec<bool>) -> bool{
        if sprite.len()<=0 || sprite.len()>15*8 || (sprite.len()%8 !=0) {
            panic!("Invalid sprite size");
        }
        let mut cleared = false;
        pos_x = pos_x % WIDTH as u8;
        pos_y = pos_y % HEIGHT as u8;
        for row in 0..((sprite.len())/8) {
            for pxl in 0..8 {
                let vram_x = (pos_x+pxl as u8) as usize;
                let vram_y = (pos_y+row as u8) as usize;
                if vram_y<HEIGHT && vram_x<WIDTH{
                    let old_bit = self.vram[vram_x][vram_y];
                    let set_bit = sprite[((row*8)+pxl) as usize];
                    self.vram[vram_x][vram_y] ^= set_bit;
                    if old_bit & set_bit {
                        cleared = true;
                    }
                }

            }
        }
        self.updated=true;
        return cleared;
    }

    pub fn draw(&mut self, renderer: &mut sdl2::render::Renderer) {
        if !self.updated {
            return;
        }
        self.updated=false;
        renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        let output_size = renderer.output_size().expect("Renderer output_size error");

        let x_scale = (output_size.0 as f64 / WIDTH as f64) as i32;
        let y_scale = (output_size.1 as f64 / HEIGHT as f64) as i32;
        for y in 0 .. HEIGHT {
            for x in 0 .. WIDTH {
                if self.vram[x as usize][y as usize] == true {
                    renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
                    let rect = sdl2::rect::Rect::new(x as i32 * x_scale,(y as i32 * y_scale), x_scale as u32, y_scale as u32).unwrap().unwrap();
                    renderer.fill_rect(rect);
                }
            }
        }
        renderer.present();
    }
}
