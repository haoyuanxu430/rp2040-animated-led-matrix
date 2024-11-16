extern crate alloc;
use smart_leds::RGB8;
use micromath::F32Ext;  // For sin, sqrt

// Define PI since we can't use std
const PI: f32 = 3.14159265359;

pub struct RightTiltAnimation {
    position: usize,
    up: bool,
}

impl RightTiltAnimation {
    pub fn new() -> Self {
        Self {
            position: 0,
            up: true,
        }
    }

    pub fn next(&mut self) {
        if self.up {
            if self.position < 7 {
                self.position += 1;
            } else {
                self.up = false;
                self.position -= 1;
            }
        } else {
            if self.position > 0 {
                self.position -= 1;
            } else {
                self.up = true;
                self.position += 1;
            }
        }
    }

    pub fn to_list(&self) -> [RGB8; 64] {
        let mut pixels = [RGB8::default(); 64];
        for i in 0..8 {
            pixels[i * 8 + self.position] = RGB8 { r: 0, g: 64, b: 0 };
        }
        pixels
    }
}

pub struct LeftTiltAnimation {
    wave_position: u8,
}

impl LeftTiltAnimation {
    pub fn new() -> Self {
        Self {
            wave_position: 0,
        }
    }

    pub fn next(&mut self) {
        self.wave_position = (self.wave_position + 1) % 8;
    }

    pub fn to_list(&self) -> [RGB8; 64] {
        let mut pixels = [RGB8::default(); 64];
        for col in 0..8 {
            let wave = (self.wave_position as f32 + col as f32 * 0.8) * 0.8;
            let row = ((wave.sin() * 3.5 + 3.5) as usize).min(7);
            pixels[row * 8 + col] = RGB8 { r: 64, g: 0, b: 64 };
        }
        pixels
    }
}

pub struct ForwardTiltAnimation {
    frame: u8,
}

impl ForwardTiltAnimation {
    pub fn new() -> Self {
        Self { frame: 0 }
    }

    pub fn next(&mut self) {
        self.frame = (self.frame + 1) % 16;
    }

    pub fn to_list(&self) -> [RGB8; 64] {
        let mut pixels = [RGB8::default(); 64];
        let center_x = 3.5;
        let center_y = 3.5;
        let radius = ((self.frame as f32 * PI / 8.0).sin() * 3.0 + 1.0).abs();
        
        for y in 0..8 {
            for x in 0..8 {
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let distance = (dx * dx + dy * dy).sqrt();
                if (distance - radius).abs() < 0.8 {
                    pixels[y * 8 + x] = RGB8 { r: 0, g: 0, b: 64 };
                }
            }
        }
        pixels
    }
}

pub struct BackwardTiltAnimation {
    frame: u8,
}

impl BackwardTiltAnimation {
    pub fn new() -> Self {
        Self {
            frame: 0,
        }
    }

    pub fn next(&mut self) {
        self.frame = self.frame.wrapping_add(1);
    }

    pub fn to_list(&self) -> [RGB8; 64] {
        let mut pixels = [RGB8::default(); 64];
        // Create a simpler pattern that doesn't require random numbers
        for y in 0..8 {
            for x in 0..8 {
                if ((x + y + (self.frame as usize / 2)) % 4) == 0 {
                    pixels[y * 8 + x] = RGB8 { r: 64, g: 64, b: 0 };
                }
            }
        }
        pixels
    }
}