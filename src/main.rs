mod algoritms;
mod canvas;
mod utils;

use {
    canvas::Canvas,
    sdl2::pixels::Color,
};

fn main() {
    let frame_rate = 60u32;
    let bg_color = Color::RGB(255, 0, 255); // #ff00ff
    let data_color = Color::RGB(0, 0, 0); // #000000
    let mut canvas = Canvas::new(frame_rate, bg_color, data_color);

    canvas.run();
}
