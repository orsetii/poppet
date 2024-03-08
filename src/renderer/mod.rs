use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window, Sdl};
use thiserror::Error;

use crate::{error, PoppetError, PoppetResult};

pub struct Renderer {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
}

impl Renderer {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("rust-sdl2 demo: Events", 800, 600)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();

        Ok(Self {
            sdl_context,
            canvas,
        })
    }

    pub fn eloop(&mut self) -> PoppetResult<()> {
        let mut event_pump = self.sdl_context.event_pump()?;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Err(RenderError::Exit.into()),
                // skip mouse motion intentionally because of the verbose it might cause.
                Event::MouseMotion { .. } => {}
                _e => {}
            }
        }

        self.canvas.clear();
        self.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("User requested exit")]
    Exit,
}
