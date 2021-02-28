use druid::piet::{ImageFormat, InterpolationMode};
use druid::widget::prelude::*;
use druid::{AppLauncher, LocalizedString, WindowDesc};
use rand::{prelude::ThreadRng, Rng};
use wasm_bindgen::prelude::*;

pub const FIRE_WIDTH: u32 = 320;
pub const FIRE_HEIGHT: u32 = 168;
pub const CANVAS_WIDTH: u32 = 400;
pub const CANVAS_HEIGHT: u32 = 300;

// State which gets updated every frame. This holds the state of the fire as well as
// the image buffer for the bitmap.
struct FireWidget {
    // Copied over from another example. Can be removed
    // or used to adjust updates via some time delta.
    tick: f64,

    // This buffer is a list of indicies into the color palette
    // It gets updated every frame and can probably be removed if the
    // alpha levels are encoded into the color palette.
    fire_buffer: Vec<u8>,

    // This is the buffer used for making the bitmap image and is red, green, blue, alpha.
    // The algorithm adjusts the alpha level depending on what color the pixel is so an image can be
    // displayed behind it.
    pixel_buffer: Vec<u8>,

    // This is a collection of colors used.
    // Every three items are a group of red, green, blue
    color_palette: Vec<u8>,

    // random number source for fire algorithm.
    rng: ThreadRng,
}

// TODO
//  - might be able to get away without creating this custom widget and leverage an existing one.
//  - The state would still be tracked via a similar FireState struct which would hold the image buffer.
impl Widget<()> for FireWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut (), _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                self.tick = 0.0;
                ctx.request_anim_frame();
            }
            Event::AnimFrame(_) => {
                ctx.request_paint();
                ctx.request_anim_frame();
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &(), _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &(), _data: &(), _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &(),
        _env: &Env,
    ) -> Size {
        // TODO - Learn about druid BoxConstraints. This is taken from example code.

        // BoxConstraints are passed by the parent widget.
        // This method can return any Size within those constraints:
        // bc.constrain(my_size)
        //
        // To check if a dimension is infinite or not (e.g. scrolling):
        // bc.is_width_bounded() / bc.is_height_bounded()
        //
        // bx.max() returns the maximum size of the widget. Be careful
        // using this, since always make sure the widget is bounded.
        // If bx.max() is used in a scrolling widget things will probably
        // not work correctly.
        // if bc.is_width_bounded() | bc.is_height_bounded() {
        let size = Size::new(CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64);
        bc.constrain(size)
        // } else {
        //     bc.max()
        // }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &(), _env: &Env) {
        let size = ctx.size();

        self.calculate_fire();

        for (idx, pixel_cursor) in self.fire_buffer.iter().enumerate() {
            let start = (*pixel_cursor * 3) as usize;
            let end = start + 3;
            match &self.color_palette[start..end] {
                [red, green, blue] => {
                    let mut alpha = 255;

                    if [*red, *green, *blue].iter().all(|color| color <= &0x07) {
                        alpha = 0;
                    }

                    let offset = idx * 4;
                    self.pixel_buffer[offset] = *red;
                    self.pixel_buffer[offset + 1] = *green;
                    self.pixel_buffer[offset + 2] = *blue;
                    self.pixel_buffer[offset + 3] = alpha as u8;
                }
                _ => (),
            }
        }

        let image = ctx
            .make_image(
                FIRE_WIDTH as usize,
                FIRE_HEIGHT as usize,
                &self.pixel_buffer,
                ImageFormat::RgbaSeparate,
            )
            .unwrap();

        // The image is automatically scaled to fit the rect you pass to draw_image
        ctx.draw_image(&image, size.to_rect(), InterpolationMode::Bilinear);
    }
}

impl FireWidget {
    // algorithm for spreading fire out
    pub fn calculate_fire(&mut self) {
        for x in 0..FIRE_WIDTH {
            for y in 1..FIRE_HEIGHT {
                let fire_pixel_cursor = y * FIRE_WIDTH + x;
                let pixel = self.fire_buffer[fire_pixel_cursor as usize];

                if pixel == 0 {
                    // black pixel
                    let idx = (fire_pixel_cursor - FIRE_WIDTH) as usize;
                    self.fire_buffer[idx] = 0;
                } else {
                    let random_num: f64 = self.rng.gen(); // generates a float between 0 and 1
                    let random_index = (random_num * 3.0).round() as u8 & 3; // 0,1,2
                    let distance = fire_pixel_cursor - (random_index as u32) + 1;
                    let new_index = (distance - FIRE_WIDTH) as usize;
                    self.fire_buffer[new_index] = pixel - (random_index & 1);
                }
            }
        }
    }
}

// Sets the state of the pixel buffer to its initial state.
// The initial state "seeds" the fire by starting off with a "white hot" bottom row.
pub fn init_buffer() -> Vec<u8> {
    // Create the pixel buffer
    let mut pixel_buffer = vec![0; (FIRE_WIDTH * FIRE_HEIGHT) as usize];

    // Set bottom row of Pixels to white inside the pixel buffer.
    for i in 0..FIRE_WIDTH {
        let bottom_x_y = ((FIRE_HEIGHT - 1) * FIRE_WIDTH + i) as usize;
        pixel_buffer[bottom_x_y] = 36;
    }

    pixel_buffer
}

// wasm wrapper around druid app
#[wasm_bindgen]
pub fn wasm_main() {
    // This hook is necessary to get panic messages in the console
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    run_app()
}

// Runs the druid app
pub fn run_app() {
    let color_palette = vec![
        0x07, 0x07, 0x07, 0x1F, 0x07, 0x07, 0x2F, 0x0F, 0x07, 0x47, 0x0F, 0x07, 0x57, 0x17, 0x07,
        0x67, 0x1F, 0x07, 0x77, 0x1F, 0x07, 0x8F, 0x27, 0x07, 0x9F, 0x2F, 0x07, 0xAF, 0x3F, 0x07,
        0xBF, 0x47, 0x07, 0xC7, 0x47, 0x07, 0xDF, 0x4F, 0x07, 0xDF, 0x57, 0x07, 0xDF, 0x57, 0x07,
        0xD7, 0x5F, 0x07, 0xD7, 0x5F, 0x07, 0xD7, 0x67, 0x0F, 0xCF, 0x6F, 0x0F, 0xCF, 0x77, 0x0F,
        0xCF, 0x7F, 0x0F, 0xCF, 0x87, 0x17, 0xC7, 0x87, 0x17, 0xC7, 0x8F, 0x17, 0xC7, 0x97, 0x1F,
        0xBF, 0x9F, 0x1F, 0xBF, 0x9F, 0x1F, 0xBF, 0xA7, 0x27, 0xBF, 0xA7, 0x27, 0xBF, 0xAF, 0x2F,
        0xB7, 0xAF, 0x2F, 0xB7, 0xB7, 0x2F, 0xB7, 0xB7, 0x37, 0xCF, 0xCF, 0x6F, 0xDF, 0xDF, 0x9F,
        0xEF, 0xEF, 0xC7, 0xFF, 0xFF, 0xFF,
    ];

    let fire_widget = FireWidget {
        tick: 0.0,
        fire_buffer: init_buffer(),
        pixel_buffer: vec![0; (FIRE_WIDTH * FIRE_HEIGHT * 4) as usize],
        color_palette: color_palette,
        rng: rand::thread_rng(),
    };

    let window = WindowDesc::new(fire_widget)
        .title(LocalizedString::new("Doom Fire FX").with_placeholder("🔥🔥🔥🔥🔥"));

    AppLauncher::with_window(window)
        .use_env_tracing()
        .launch(())
        .expect("launch failed");
}
