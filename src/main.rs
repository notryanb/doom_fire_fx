extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
// use sdl2::pixels::PixelFormatEnum;
// use sdl2::surface::Surface;
use sdl2::rect::Point;
use sdl2::render::{Texture, TextureCreator};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use rand::prelude::*;

// use std::time::Duration;

const FIRE_WIDTH: u32 = 640;
const FIRE_HEIGHT: u32 = 480;

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
   
    // Create the pixel buffer
    let mut fire_pixels: Vec<u32> = Vec::with_capacity((FIRE_WIDTH * FIRE_HEIGHT) as usize);
    for _ in 0..fire_pixels.capacity() { 
        fire_pixels.push(0);
    }

    // Set bottom row of Pixels to white inside the pixel buffer.
    for i in 0..FIRE_WIDTH {
        let bottom_x_y = ((FIRE_HEIGHT - 1) * FIRE_WIDTH + i) as usize;
        fire_pixels[bottom_x_y] = 36;
    }
    
    // Set Up SDL Windox & Canvas
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Doom Fire FX", FIRE_WIDTH, FIRE_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .unwrap();

    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
    canvas.clear();
    canvas.present();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let mut fire_texture: Texture = texture_creator
        .create_texture_target(None, FIRE_WIDTH, FIRE_HEIGHT).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // let mut frame: u32 = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        calculate_fire(&mut fire_pixels);
        let pixel_vec = convert_to_pixel(&fire_pixels, &color_palette);

        canvas.with_texture_canvas(&mut fire_texture, |texture_canvas| {
            for y in 0..FIRE_HEIGHT {
                for x in 0..FIRE_WIDTH {

                    let pixel_index = (y * FIRE_HEIGHT + x) as usize;
                    let pixel = pixel_vec[pixel_index];
                    texture_canvas.set_draw_color(
                        Color::RGBA(
                            pixel.red as u8,
                            pixel.blue as u8,
                            pixel.green as u8,
                            255));
                    texture_canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
                }
            }
        }).unwrap();


        canvas.present();
    }




    /*
        I think we want to get a canvas FROM a window.
        - Generate TextureCreator
        - Create one texture (which we'll update on every iteration?)
        - set_draw_color to black, maybe there is Color:RGBA instead of RGB?
        - clear the texture canvas
        - nested iteration through y, then x
            - looks like setting the draw color is for drawing a single pixel to whatever we're drawing
                like a point...
            - set the draw color to the RGB palette we've calculated
            - texture_canvas.draw_point(Point::new(x,y both as i32).unwra();
    */


    
    // 'running: loop {
    //     canvas.clear();

    //     calculate_fire(&mut fire_pixels);
    //     let pixel_vec = convert_to_pixel(&fire_pixels, &color_palette);

    //     for pixel in pixel_vec.iter() {
    //         canvas.set_draw_color(
    //             Color::RGB(
    //                 pixel.red as u8,
    //                 pixel.green as u8,
    //                 pixel.blue as u8
    //             ));
    //     }
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit {..} |
    //             Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
    //                 break 'running
    //             },
    //             _ => {}
    //         }
    //     }

    //     canvas.present();
    //     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    // }
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
