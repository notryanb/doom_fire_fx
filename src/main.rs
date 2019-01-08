extern crate rand;

use rand::prelude::*;

const FIRE_WIDTH: u32 = 64;
const FIRE_HEIGHT: u32 = 128;

fn main() {

    let fire_rgb = [
                0x07,0x07,0x07,
                0x1F,0x07,0x07,
                0x2F,0x0F,0x07,
                0x47,0x0F,0x07,
                0x57,0x17,0x07,
                0x67,0x1F,0x07,
                0x77,0x1F,0x07,
                0x8F,0x27,0x07,
                0x9F,0x2F,0x07,
                0xAF,0x3F,0x07,
                0xBF,0x47,0x07,
                0xC7,0x47,0x07,
                0xDF,0x4F,0x07,
                0xDF,0x57,0x07,
                0xDF,0x57,0x07,
                0xD7,0x5F,0x07,
                0xD7,0x5F,0x07,
                0xD7,0x67,0x0F,
                0xCF,0x6F,0x0F,
                0xCF,0x77,0x0F,
                0xCF,0x7F,0x0F,
                0xCF,0x87,0x17,
                0xC7,0x87,0x17,
                0xC7,0x8F,0x17,
                0xC7,0x97,0x1F,
                0xBF,0x9F,0x1F,
                0xBF,0x9F,0x1F,
                0xBF,0xA7,0x27,
                0xBF,0xA7,0x27,
                0xBF,0xAF,0x2F,
                0xB7,0xAF,0x2F,
                0xB7,0xB7,0x2F,
                0xB7,0xB7,0x37,
                0xCF,0xCF,0x6F,
                0xDF,0xDF,0x9F,
                0xEF,0xEF,0xC7,
                0xFF,0xFF,0xFF
            ];
    
    const palette_size: usize = 9;
    let fire_pixels: [u32; palette_size] = [0; palette_size];
    let mut palette: [Color; palette_size] = [Color { red: 0, green: 0, blue: 0 }; palette_size];

    for idx in 0..palette.len() {
        palette[idx] = Color {
            red: fire_rgb[idx * 3 + 0],
            green: fire_rgb[idx * 3 + 1],
            blue: fire_rgb[idx * 3 + 2],
        };
    }
}

#[derive(Copy, Clone)]
pub struct Color {
    red: u32,
    green: u32,
    blue: u32,
}


pub fn spread_fire(src: u32, fire_pixels: &[u32]) {
    let pixel = fire_pixels[src as usize];
    if pixel == 0 {
        println!("It's zero")
        // fire_pixels[src as usize - FIRE_WIDTH as usize] = 0;
    } else {
        let mut rng = rand::thread_rng();
        let random_num: f64 = rng.gen(); // generates a float between 0 and 1


        // var randIdx = Math.round(Math.random() * 3.0) & 3;
        // var dst = src - randIdx + 1;
        // firePixels[dst - FIRE_WIDTH ] = pixel - (randIdx & 1);
    }
}

pub fn doFire(fire_pixels: &[u32]) {
    for x in 0..FIRE_WIDTH {
        for y in 1..FIRE_HEIGHT {
            let source = y * FIRE_WIDTH + x;
            spread_fire(source, fire_pixels);
        }
    }
}