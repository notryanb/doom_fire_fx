extern crate rand;
extern crate sdl2;

use rand::Rng;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::{Rect};
use sdl2::render::{BlendMode, TextureCreator};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::fs::File;


const FIRE_WIDTH: u32 = 320;
const FIRE_HEIGHT: u32 = 168;
const CANVAS_WIDTH: u32 = 640;
const CANVAS_HEIGHT:u32 = 509;

fn main() {
    let color_palette = [
        (0x17, 0x17, 0x17),
        (0x1F, 0x07, 0x07),
        (0x2F, 0x0F, 0x07),
        (0x47, 0x0F, 0x07),
        (0x57, 0x17, 0x07),
        (0x67, 0x1F, 0x07),
        (0x77, 0x1F, 0x07),
        (0x8F, 0x27, 0x07),
        (0x9F, 0x2F, 0x07),
        (0xAF, 0x3F, 0x07),
        (0xBF, 0x47, 0x07),
        (0xC7, 0x47, 0x07),
        (0xDF, 0x4F, 0x07),
        (0xDF, 0x57, 0x07),
        (0xDF, 0x57, 0x07),
        (0xD7, 0x5F, 0x07),
        (0xD7, 0x5F, 0x07),
        (0xD7, 0x67, 0x0F),
        (0xCF, 0x6F, 0x0F),
        (0xCF, 0x77, 0x0F),
        (0xCF, 0x7F, 0x0F),
        (0xCF, 0x87, 0x17),
        (0xC7, 0x87, 0x17),
        (0xC7, 0x8F, 0x17),
        (0xC7, 0x97, 0x1F),
        (0xBF, 0x9F, 0x1F),
        (0xBF, 0x9F, 0x1F),
        (0xBF, 0xA7, 0x27),
        (0xBF, 0xA7, 0x27),
        (0xBF, 0xAF, 0x2F),
        (0xB7, 0xAF, 0x2F),
        (0xB7, 0xB7, 0x2F),
        (0xB7, 0xB7, 0x37),
        (0xCF, 0xCF, 0x6F),
        (0xDF, 0xDF, 0x9F),
        (0xEF, 0xEF, 0xC7),
        (0xFF, 0xFF, 0xFF),
    ];

    /*
        Fire pixel buffer will look like this for a 3x3 grid
        
        [
            0  { x: 0, y: 0 },
            0  { x: 1, y: 0 },
            0  { x: 2, y: 0 },
            0  { x: 0, y: 1 },
            0  { x: 1, y: 1 },
            0  { x: 2, y: 1 },
            36 { x: 0, y: 2 },
            36 { x: 1, y: 2 },
            36 { x: 2, y: 2 }, 
        ]
    */

    // Create the pixel buffer
    let mut pixel_buffer: Vec<u32> = Vec::with_capacity((FIRE_WIDTH * FIRE_HEIGHT) as usize);

    // Set all pixels to black
    for _ in 0..pixel_buffer.capacity() {
        pixel_buffer.push(0);
    }

    // Set bottom row of Pixels to white inside the pixel buffer.
    for i in 0..FIRE_WIDTH {
        let bottom_x_y = ((FIRE_HEIGHT - 1) * FIRE_WIDTH + i) as usize;
        pixel_buffer[bottom_x_y] = 36;
    }

    // Scrolling for keeping track of when fire goes away and logo rising
    let mut y_scrolling = 440;

    // Set Up SDL Windox & Canvas
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("notryanb - Doom Fire FX", CANVAS_WIDTH, CANVAS_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .unwrap();

    let image_texture_creator = canvas.texture_creator();

    let doom_logo = image_texture_creator
        .load_texture("./src/doom_logo.png")
        .unwrap();

    // Ferris Logo:
    // http://enosart.com/animated-crab-9974/
    let logo = image_texture_creator
        .load_texture("./src/ferris_logo.png")
        .unwrap();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    // RGB24 splits each pixel into 3 * 8bit sections. 
    let mut fire_texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA8888, FIRE_WIDTH, FIRE_HEIGHT)
        .map_err(|e| e.to_string())
        .unwrap();

    canvas.clear();
    canvas.set_draw_color(Color::RGBA(18, 18, 18, 255));
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        
        fire_texture
            .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                calculate_fire(&mut pixel_buffer);
                let pixel_vec = convert_to_pixel(&pixel_buffer, &color_palette);

                for (idx, pixel) in pixel_vec.iter().enumerate() {
                    let offset = idx * 4;
                    buffer[offset] = pixel.alpha as u8;
                    buffer[offset + 1] = pixel.blue as u8;
                    buffer[offset + 2] = pixel.green as u8;
                    buffer[offset + 3] = pixel.red as u8;
                }
            })
            .unwrap();

        &fire_texture.set_blend_mode(BlendMode::Blend);


        if y_scrolling != 70 {
            y_scrolling -= 2;
        }
        else {
            for y in (161..168).rev() {
                for x in 0..FIRE_WIDTH {
                    let index = (y * FIRE_WIDTH + x) as usize;
                    if pixel_buffer[index] > 0 {
                        let mut rng = rand::thread_rng();
                        let random_num: f64 = rng.gen(); // generates a float between 0 and 1
                        let random_decrement = random_num.round() as u32 & 3;
                        pixel_buffer[index] -= random_decrement;
                    }
                }

            }
        }

        let rect = Rect::new(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
        let logo_rect = Rect::new(40, y_scrolling, 600, 450);

        canvas.copy(&logo, None, Some(logo_rect)).unwrap();
        canvas.copy(&fire_texture, None, Some(rect)).unwrap();
        canvas.present();
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
    pub fn is_black(self) -> bool {
        self.red <= 0x17 && self.green <= 0x17 && self.blue <= 0x17
    }
}

// This function will be called by iterating down columns left to right.
pub fn spread_fire(cursor: u32, pixel_buffer: &mut Vec<u32>) {

    let pixel = pixel_buffer[cursor as usize];

    /*
        First iteration will be 0
        in a 3x3 grid, the idx calculation will be 3 - 3 = 0
        the very first pixel 0,0 will be = 0

        Second iteration pixel won't be 0, so 
        calculation = 8 - random number, which will be close to original number, as this corresponds to the shade
        then set the cursor position in the pixel_buffer to that close shade. The shades are actually indexes into the 
        color_palette.
    */
    if pixel == 0 { // black pixel
        let idx = (cursor - FIRE_WIDTH) as usize;
        pixel_buffer[idx] = 0;
    } else {
        let mut rng = rand::thread_rng();
        let random_num: f64 = rng.gen(); // generates a float between 0 and 1
        let random_index = (random_num * 3.0).round() as u32 & 3; // 0,1,2
        let distance = cursor - random_index + 1;
        let new_index = (distance - FIRE_WIDTH) as usize;
        pixel_buffer[new_index] = pixel - (random_index & 1);
    }
}


/*
    Fire pixel buffer will look like this for a 3x3 grid.
    The buffer is ordered by row then column. 
    ie. every FIRE_WIDTH indexes represents one ROW, starting at the top of the image.
    The last row represents the bottom of the image, AKA the entire white row.

    This function iterates down by column.
    ie. starts at the top of the first column, works it's way down,
    then moves into the next column to the right.
    
    [
        0  { x: 0, y: 0 }, never touched, this is the top of the fire where it doesn't go
        0  { x: 1, y: 0 }, never touched, this is the top of the fire where it doesn't go
        0  { x: 2, y: 0 }, never touched, this is the top of the fire where it doesn't go
        0  { x: 0, y: 1 }, <- 1. cursor first iteration
        0  { x: 1, y: 1 }, <- 3. cursor third iteration
        0  { x: 2, y: 1 }, <- 3. cursor fifth iteration
        36 { x: 0, y: 2 }, <- 2. cursor second iteration
        36 { x: 1, y: 2 }, <- 4. cursor fourth iteration
        36 { x: 2, y: 2 }, <- 6. cursor sixth iteration
    ]
*/
pub fn calculate_fire(pixel_buffer: &mut Vec<u32>) {
    for x in 0..FIRE_WIDTH {
        for y in 1..FIRE_HEIGHT {
            /*  
                3x3 Grid
                - when x = 0, y = 1, cursor = 3
                - when x = 0, y = 2, cursor = 6
                - when x = 1, y = 1, cursor = 4
                - when x = 1, y = 2, cursor = 7
                - when x = 2, y = 1, cursor = 5
                - when x = 2, y = 2, cursor = 8

            */
            let fire_pixel_cursor = y * FIRE_WIDTH + x;
            spread_fire(fire_pixel_cursor, pixel_buffer);
        }
    }
}


pub fn convert_to_pixel(pixel_buffer: &Vec<u32>, color_palette: &[(u32, u32, u32)]) -> Vec<Pixel> {
    // The Pixel vector should end up being the same length as pixel_buffer.
    let mut pixel_vector: Vec<Pixel> = Vec::with_capacity(0);

    for color_cursor in pixel_buffer.iter() {
        let color = color_palette[*color_cursor as usize];

        let mut pixel = Pixel {
                red: color.0,
                green: color.1,
                blue: color.2,
                alpha: 0,
            };

            if !pixel.is_black() {
                pixel.alpha = 255;
            }

            pixel_vector.push(pixel);
    } 

    pixel_vector
}
