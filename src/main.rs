use rand::Rng;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, TextureCreator};

const FIRE_WIDTH: u32 = 320;
const FIRE_HEIGHT: u32 = 168;
const CANVAS_WIDTH: u32 = 800;
const CANVAS_HEIGHT: u32 = 600;

fn main() {
    let color_palette = [
        0x07, 0x07, 0x07, 0x1F, 0x07, 0x07, 0x2F, 0x0F, 0x07, 0x47, 0x0F, 0x07, 0x57, 0x17, 0x07,
        0x67, 0x1F, 0x07, 0x77, 0x1F, 0x07, 0x8F, 0x27, 0x07, 0x9F, 0x2F, 0x07, 0xAF, 0x3F, 0x07,
        0xBF, 0x47, 0x07, 0xC7, 0x47, 0x07, 0xDF, 0x4F, 0x07, 0xDF, 0x57, 0x07, 0xDF, 0x57, 0x07,
        0xD7, 0x5F, 0x07, 0xD7, 0x5F, 0x07, 0xD7, 0x67, 0x0F, 0xCF, 0x6F, 0x0F, 0xCF, 0x77, 0x0F,
        0xCF, 0x7F, 0x0F, 0xCF, 0x87, 0x17, 0xC7, 0x87, 0x17, 0xC7, 0x8F, 0x17, 0xC7, 0x97, 0x1F,
        0xBF, 0x9F, 0x1F, 0xBF, 0x9F, 0x1F, 0xBF, 0xA7, 0x27, 0xBF, 0xA7, 0x27, 0xBF, 0xAF, 0x2F,
        0xB7, 0xAF, 0x2F, 0xB7, 0xB7, 0x2F, 0xB7, 0xB7, 0x37, 0xCF, 0xCF, 0x6F, 0xDF, 0xDF, 0x9F,
        0xEF, 0xEF, 0xC7, 0xFF, 0xFF, 0xFF,
    ];

    // Create the pixel buffer
    let mut pixel_buffer = vec![0; (FIRE_WIDTH * FIRE_HEIGHT) as usize];

    // Set bottom row of Pixels to white inside the pixel buffer.
    for i in 0..FIRE_WIDTH {
        let bottom_x_y = ((FIRE_HEIGHT - 1) * FIRE_WIDTH + i) as usize;
        pixel_buffer[bottom_x_y] = 36;
    }

    // Scrolling for keeping track of when fire goes away and logo rising
    let mut y_scrolling = 540;

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

    let _doom_logo = image_texture_creator
        .load_texture("./src/doom_logo.png")
        .unwrap();

    // Ferris Logo:
    // http://enosart.com/animated-crab-9974/
    let logo = image_texture_creator
        .load_texture("./src/ferris_logo.png")
        .unwrap();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    // RGBA8888 splits each pixel into 4 * 8 bit sections
    let mut fire_texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA8888, FIRE_WIDTH, FIRE_HEIGHT)
        .map_err(|e| e.to_string())
        .unwrap();

    canvas.clear();
    canvas.set_draw_color(Color::RGBA(0x07, 0x07, 0x07, 255));
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

                for (idx, pixel_cursor) in pixel_buffer.iter().enumerate() {
                    let start = (*pixel_cursor * 3) as usize;
                    let end = start + 3;
                    match &color_palette[start..end] {
                        [red, green, blue] => {
                            let mut alpha = 255;

                            if [*red, *green, *blue].iter().all(|color| color <= &0x07) {
                                alpha = 0;
                            }

                            let offset = idx * 4;
                            buffer[offset] = alpha as u8;
                            buffer[offset + 1] = *blue;
                            buffer[offset + 2] = *green;
                            buffer[offset + 3] = *red;
                        }
                        _ => (),
                    }
                }
            })
            .unwrap();

        &fire_texture.set_blend_mode(BlendMode::Blend);

        if y_scrolling != 70 {
            y_scrolling -= 2;
        } else {
            for y in (161..168).rev() {
                for x in 0..FIRE_WIDTH {
                    let index = (y * FIRE_WIDTH + x) as usize;
                    if pixel_buffer[index] > 0 {
                        let mut rng = rand::thread_rng();
                        let random_num: f64 = rng.gen(); // generates a float between 0 and 1
                        let random_decrement = random_num.round() as u8 & 3;
                        pixel_buffer[index] -= random_decrement;
                    }
                }
            }
        }

        let rect = Rect::new(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
        let logo_rect = Rect::new(40, y_scrolling, CANVAS_WIDTH - 75, 450);

        canvas.copy(&logo, None, Some(logo_rect)).unwrap();
        canvas.copy(&fire_texture, None, Some(rect)).unwrap();
        canvas.present();
    }
}

// This function will be called by iterating down columns left to right.
pub fn spread_fire(cursor: u32, pixel_buffer: &mut [u8]) {
    let pixel = pixel_buffer[cursor as usize];

    if pixel == 0 {
        // black pixel
        let idx = (cursor - FIRE_WIDTH) as usize;
        pixel_buffer[idx] = 0;
    } else {
        let mut rng = rand::thread_rng();
        let random_num: f64 = rng.gen(); // generates a float between 0 and 1
        let random_index = (random_num * 3.0).round() as u8 & 3; // 0,1,2
        let distance = cursor - (random_index as u32) + 1;
        let new_index = (distance - FIRE_WIDTH) as usize;
        pixel_buffer[new_index] = pixel - (random_index & 1);
    }
}

pub fn calculate_fire(pixel_buffer: &mut [u8]) {
    for x in 0..FIRE_WIDTH {
        for y in 1..FIRE_HEIGHT {
            let fire_pixel_cursor = y * FIRE_WIDTH + x;
            spread_fire(fire_pixel_cursor, pixel_buffer);
        }
    }
}
