use std::time::Duration;

use nom::error::ParseError;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window, Sdl};
use thiserror::Error;
use tracing::info;

use crate::{net::Response, renderer::parsers::html, PoppetResult};

pub mod page;
pub mod parsers;

pub struct Renderer {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    current_page: Option<page::Page>,
}

impl Renderer {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Poppet Browser", 800, 600)
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
            current_page: None,
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
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        Ok(())
    }

    fn current_page_raw(&self) -> String {
        self.current_page.clone().unwrap().html_content
    }

    pub fn render_response(&mut self, resp: Response) {
        info!("Rendering new {}-bytes long page", &resp.body.len());
        self.current_page = Some(page::Page::new(resp));
        let parsed_html = html::parse(&self.current_page_raw());
        info!("Parsed HTML: {:#?}", parsed_html);
    }
}

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("User requested exit")]
    Exit,
    #[error("Error parsing HTML: {0}")]
    HTMLParseError(String),
}
