extern crate sdl2;

pub mod eng;
pub mod error;
pub mod net;
pub mod renderer;

pub use error::{PoppetError, PoppetResult};

pub fn main() -> PoppetResult<()> {
    let mut renderer = renderer::Renderer::new()?;

    loop {
        renderer.eloop()?;
    }
}
