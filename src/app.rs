
extern crate sdl2;

use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct App {
    sdl: Sdl,
    canvas: Canvas<Window>,
    gfx: [u8;2048],
    keystate: [u8;16]
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
            sdl: sdl,
            canvas: canvas,
            gfx: [0;2048],
            keystate: [0;16]
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

    pub fn get_keystate(&mut self) -> [u8;16] {
        let mut event_pump = self.sdl.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            // clean up this ugly code later
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Num0), ..
                } => {
                    self.keystate[0] = 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Num1), ..
                } => {
                    self.keystate[1] = 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Num2), ..
                } => {
                    self.keystate[2] = 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Num3), ..
                } => {
                    self.keystate[3] = 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Num4), ..
                } => {
                    self.keystate[4] = 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Num5), ..
                } => {
                    self.keystate[5] = 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Num6), ..
                } => {
                    self.keystate[6] = 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Num7), ..
                } => {
                    self.keystate[7] = 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Num8), ..
                } => {
                    self.keystate[8] = 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Num9), ..
                } => {
                    self.keystate[9] = 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::A), ..
                } => {
                    self.keystate[10] = 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::B), ..
                } => {
                    self.keystate[11] = 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::C), ..
                } => {
                    self.keystate[12] = 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::D), ..
                } => {
                    self.keystate[13] = 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::E), ..
                } => {
                    self.keystate[14] = 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::F), ..
                } => {
                    self.keystate[15] = 1;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Num0), ..
                } => {
                    self.keystate[0] = 0;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Num1), ..
                } => {
                    self.keystate[1] = 0;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Num2), ..
                } => {
                    self.keystate[2] = 0;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Num3), ..
                } => {
                    self.keystate[3] = 0;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Num4), ..
                } => {
                    self.keystate[4] = 0;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Num5), ..
                } => {
                    self.keystate[5] = 0;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Num6), ..
                } => {
                    self.keystate[6] = 0;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Num7), ..
                } => {
                    self.keystate[7] = 0;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Num8), ..
                } => {
                    self.keystate[8] = 0;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Num9), ..
                } => {
                    self.keystate[9] = 0;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::A), ..
                } => {
                    self.keystate[10] = 0;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::B), ..
                } => {
                    self.keystate[11] = 0;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::C), ..
                } => {
                    self.keystate[12] = 0;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::D), ..
                } => {
                    self.keystate[13] = 0;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::E), ..
                } => {
                    self.keystate[14] = 0;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::F), ..
                } => {
                    self.keystate[15] = 0;
                }


                _=> {}
            }
        }

        // for i in 0..keystate.len() {
        //     print!("{} ", keystate[i]);
        // }
        // println!();

        self.keystate
    }

    pub fn await_keypress(&mut self) -> u8 {
        let mut event_pump = self.sdl.event_pump().unwrap();
        loop {
            for event in event_pump.poll_iter() {
                // clean up this ugly code later
                match event {
                    Event::KeyDown {
                        keycode: Some(Keycode::Num0), ..
                    } => {
                        return 0x30;
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::Num1), ..
                    } => {
                        return 0x31;
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::Num2), ..
                    } => {
                        return 0x32;
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::Num3), ..
                    } => {
                        return 0x33;
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::Num4), ..
                    } => {
                        return 0x34;
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::Num5), ..
                    } => {
                        return 0x35;
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::Num6), ..
                    } => {
                        return 0x36;
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::Num7), ..
                    } => {
                        return 0x37;
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::Num8), ..
                    } => {
                        return 0x38;
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::Num9), ..
                    } => {
                        return 0x39;
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::A), ..
                    } => {
                        return 0x61;
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::B), ..
                    } => {
                        return 0x62;
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::C), ..
                    } => {
                        return 0x63;
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::D), ..
                    } => {
                        return 0x64;
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::E), ..
                    } => {
                        return 0x65;
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::F), ..
                    } => {
                        return 0x66;
                    }
    
                    _=> {}
                }
            }
        }
    }
}
