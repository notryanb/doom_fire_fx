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
    
    let mut pixel_buffer: Vec<Pixel> = Vec::with_capacity((FIRE_WIDTH * FIRE_HEIGHT) as usize);

    // Make all Pixels in Pixel Buffer Black and transparent (In case we show the doom.. erm... RUST logo)
    for mut pixel in 0..pixel_buffer.capacity() { 
        pixel_buffer.push(Pixel { red: 0x07, green: 0x07, blue: 0x07, alpha: 0});
    }

    // Set bottom row of Pixels to white.
    for i in 0..FIRE_WIDTH {
        let bottom_x_y = ((FIRE_HEIGHT - 1) * FIRE_WIDTH + i) as usize;
        pixel_buffer[bottom_x_y] = Pixel { red: 0xFF, green: 0xFF, blue: 0xFF, alpha: 255};
    }

    for pixel in pixel_buffer.iter() {
        println!("R: {}, G: {}, B {}", pixel.red, pixel.blue, pixel.green);
    }


}

#[derive(Copy, Clone)]
pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}


// pub fn spread_fire(src: ) {
//     let pixel = fire_pixels[src as usize];
//     if pixel == 0 {
//         // fire_pixels[src as usize - FIRE_WIDTH as usize] = 0;
//     } else {
//         let mut rng = rand::thread_rng();
//         let random_num: f64 = rng.gen(); // generates a float between 0 and 1
//         let random_index = (random_num * 3.0).round() as u32 & 3;
//         let distance = src - random_index + 1;
//         // set fire pixels here

//         // JS Code
//         // var randIdx = Math.round(Math.random() * 3.0) & 3;
//         // var dst = src - randIdx + 1;
//         // firePixels[dst - FIRE_WIDTH ] = pixel - (randIdx & 1);
//     }
// }

// Builds the fire by going through every row over the intial white one.
// pub fn do_fire() {
//     for x in 0..FIRE_WIDTH {
//         for y in 1..FIRE_HEIGHT {
//             let pixel_buffer_position = y * FIRE_WIDTH + x;
//             spread_fire(pixel_buffer_position);
//         }
//     }
// }