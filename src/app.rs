
extern crate sdl2;

use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct App {
    canvas: Canvas<Window>,
    gfx: [u8;2048]
}

impl App {

    pub fn new() -> App {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window("chip-8-window", 640, 320)
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().present_vsync().build().unwrap();
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        App {
            canvas: canvas,
            gfx: [0;2048]
        }

    }

    pub fn render(&mut self) {
        // use graphics::*;
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::WHITE);

        for y_coord in 0..32 {
            for x_coord in 0..64 {
                if self.gfx[x_coord + (y_coord * 64)] == 1 {
                    match self.canvas.fill_rect(Rect::new((x_coord * 10) as i32, (y_coord * 10) as i32, 10, 10)) {
                        Err(e) => panic!("Error drawing to canvas: {}", e),
                        _=>()
                    }
                }
            }
        }

        self.canvas.present();

    }

    pub fn update(&mut self, gfx: &[u8;2048]) {
        self.gfx = *gfx;
    }
}
