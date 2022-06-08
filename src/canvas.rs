use std::time::{
    self,
    Duration,
};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render,
    video,
    EventPump,
};

use crate::{
    algoritms::sorting,
    utils,
};

pub struct Canvas {
    canvas: render::Canvas<video::Window>,
    frame_rate: u32,
    event_pump: EventPump,
    running: bool,
    bg_color: Color,
    data_color: Color,
}

impl Canvas {
    pub fn new(frame_rate: u32, bg_color: Color, data_color: Color) -> Self {
        let sdl_context = sdl2::init().unwrap();
        // Event handler
        let event_pump = sdl_context.event_pump().unwrap();

        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Algorithms visualizer", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let running = false;

        Self {
            canvas,
            frame_rate,
            event_pump,
            running,
            bg_color,
            data_color,
        }
    }

    pub fn run(&mut self) {
        self.running = true;

        let vec = utils::generate_vec(100, 0, self.height() as i32);

        let mut data_w = self.width() / (vec.len() as u32);

        if data_w == 0 {
            data_w = 1;
        }

        utils::time_execution!("Comb sort", {
            let mut vec_clone = vec.clone();
            sorting::exchange::cocktail_shaker::sort(&mut vec_clone, self, data_w)
        });

        while self.running {
            self.handle_events();
            self.control_frame_rate();
        }
    }

    fn control_frame_rate(&self) {
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / self.frame_rate));
    }

    pub fn handle_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.running = false;
                }
                _ => (),
            }
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }
}

impl Canvas {
    pub fn clear(&mut self) {
        let prev_color = self.canvas.draw_color();

        self.canvas.set_draw_color(self.bg_color);
        self.canvas.clear();

        self.canvas.set_draw_color(prev_color);
    }

    pub fn draw_filled_rect(&mut self, rect: sdl2::rect::Rect, color: Color) {
        let prev_color = self.canvas.draw_color();

        self.canvas.set_draw_color(color);
        self.canvas.draw_rect(rect).unwrap();
        self.canvas.fill_rect(rect).unwrap();

        self.canvas.set_draw_color(prev_color);
    }

    pub fn draw_vec(&mut self, vec: &Vec<i32>, width: u32, hl: Vec<usize>) {
        let y = self.height() as i32;

        for (i, v) in vec.iter().enumerate() {
            let x = (i as i32) * (width as i32);

            let height = v.clone() as u32;

            if hl.contains(&i) {
                let rect = sdl2::rect::Rect::new(x, y - (height as i32), width, height);
                self.draw_filled_rect(rect, Color::RGB(0, 0, 255));
            } else {
                let rect = sdl2::rect::Rect::new(x, y - (height as i32), width, height);
                self.draw_filled_rect(rect, Color::RGB(0, 0, 0));
            }
        }
    }

    pub fn present(&mut self) {
        self.canvas.present();
        // Control framerate
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

// Getters
impl Canvas {
    pub fn height(&self) -> u32 {
        self.canvas.window().size().1
    }
    pub fn width(&self) -> u32 {
        self.canvas.window().size().0
    }
}
