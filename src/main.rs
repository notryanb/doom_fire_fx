extern crate rand;

use rand::prelude::*;

const FIRE_WIDTH: u32 = 64;
const FIRE_HEIGHT: u32 = 128;

/*
Source: https://github.com/fabiensanglard/DoomFirePSX/blob/master/flames.html
    Overall Logic for pixel computation
    - Set RGB palette, which can be a Vec<Color>
    - The entire screen should be set to black, but the bottom line set to white.
    - Every frame, we need to update the palette buffer
    - Spreading the Fire involves getting one of the pixels in the pixel buffer
    - If it's the bottom row, don't do anything
    - If not - get a random nu,ber between 1 - 3, then & operator it with 3.
    - Take that number, and add it to a random index to get the distance.
    - We then index into the firePixel buffer that distance from the end and subtract
        a pixel - the random index & 1

    We then need to take the pixel buffer and convert it to RGB then pipe to the output.
    - Get the pixel we're iterating over and index into the palette.
    - Set the reg/green/blue of the converted pixel
    - if it's all black, calculate the next one to be black
    - 

*/
fn main() {

    let color_palette = [
                (0x07,0x07,0x07),
                (0x1F,0x07,0x07),
                (0x2F,0x0F,0x07),
                (0x47,0x0F,0x07),
                (0x57,0x17,0x07),
                (0x67,0x1F,0x07),
                (0x77,0x1F,0x07),
                (0x8F,0x27,0x07),
                (0x9F,0x2F,0x07),
                (0xAF,0x3F,0x07),
                (0xBF,0x47,0x07),
                (0xC7,0x47,0x07),
                (0xDF,0x4F,0x07),
                (0xDF,0x57,0x07),
                (0xDF,0x57,0x07),
                (0xD7,0x5F,0x07),
                (0xD7,0x5F,0x07),
                (0xD7,0x67,0x0F),
                (0xCF,0x6F,0x0F),
                (0xCF,0x77,0x0F),
                (0xCF,0x7F,0x0F),
                (0xCF,0x87,0x17),
                (0xC7,0x87,0x17),
                (0xC7,0x8F,0x17),
                (0xC7,0x97,0x1F),
                (0xBF,0x9F,0x1F),
                (0xBF,0x9F,0x1F),
                (0xBF,0xA7,0x27),
                (0xBF,0xA7,0x27),
                (0xBF,0xAF,0x2F),
                (0xB7,0xAF,0x2F),
                (0xB7,0xB7,0x2F),
                (0xB7,0xB7,0x37),
                (0xCF,0xCF,0x6F),
                (0xDF,0xDF,0x9F),
                (0xEF,0xEF,0xC7),
                (0xFF,0xFF,0xFF)
            ];
    
    // let mut pixel_buffer: Vec<Pixel> = Vec::with_capacity((FIRE_WIDTH * FIRE_HEIGHT) as usize);

    let mut fire_pixels: Vec<u32> = Vec::with_capacity((FIRE_WIDTH * FIRE_HEIGHT) as usize);
    for mut pixel in 0..fire_pixels.capacity() { 
        fire_pixels.push(0);
    }

    // Set bottom row of Pixels to white.
    for i in 0..FIRE_WIDTH {
        let bottom_x_y = ((FIRE_HEIGHT - 1) * FIRE_WIDTH + i) as usize;
        fire_pixels[bottom_x_y] = 36;
    }

    let pixel_vec = convert_to_pixel(&fire_pixels, &color_palette);

    for num in 0..100 {
        calculate_fire(&mut fire_pixels);
        println!("{:?}", &pixel_vec);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    red: u32,
    green: u32,
    blue: u32,
    alpha: u32,
}

impl Pixel {
    pub fn is_white(self) -> bool {
        self.red == 0x07 &&
        self.green == 0x07 && 
        self.blue == 0x07
    }
}

pub fn spread_fire(src: u32, pixel_buffer: &mut Vec<u32>) {
    let pixel = pixel_buffer[src as usize];

    if pixel == 0 {
        let idx = (src - FIRE_WIDTH) as usize;
        pixel_buffer[idx] = 0;
    } else {
        let mut rng = rand::thread_rng();
        let random_num: f64 = rng.gen(); // generates a float between 0 and 1
        let random_index = (random_num * 3.0).round() as u32 & 3; // 0,1,2
        let distance = src - random_index + 1;
        let new_index = (distance - FIRE_WIDTH) as usize;
        pixel_buffer[new_index] = pixel - (random_index & 1);
    }
}

pub fn calculate_fire(pixel_buffer: &mut Vec<u32>) {
    for x in 0..FIRE_WIDTH {
        for y in 1..FIRE_HEIGHT {
            let fire_pixel_cursor = y * FIRE_WIDTH + x;
            spread_fire(fire_pixel_cursor, pixel_buffer);
        }
    }
}

pub fn convert_to_pixel(pixel_buffer: &Vec<u32>, color_palette: &[(u32, u32, u32)]) -> Vec<Pixel> {
    let mut pixel_vector: Vec<Pixel> = Vec::with_capacity(0);

    for y in 0..FIRE_HEIGHT {
        for x in 0..FIRE_WIDTH {
            let cursor = (y * FIRE_WIDTH + x) as usize;
            let pixel_index = pixel_buffer[cursor] as usize;
            let color = color_palette[pixel_index];

            let mut pixel = Pixel {
                red: color.0,
                green: color.1,
                blue: color.2,
                alpha: 0,
            };

            if pixel.is_white() {
                pixel.alpha = 0;
            }
            else {
                pixel.alpha = 255;
            }

            pixel_vector.push(pixel);
        }
    }

    pixel_vector
}