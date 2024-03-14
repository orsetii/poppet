extern crate sdl2;

pub mod eng;
pub mod error;
pub mod net;
pub mod renderer;

pub use error::{PoppetError, PoppetResult};
use url::Url;

pub fn main() -> PoppetResult<()> {
    tracing_subscriber::fmt::init();
    let mut renderer = renderer::Renderer::new()?;

    let resp = net::request(
        net::Method::GET,
        Url::parse(
            &std::env::args()
                .nth(1)
                .or(Some(String::from("https://example.com/")))
                .unwrap(),
        )
        .unwrap(),
    )?;
    println!("Response: {:#?}", resp);
    renderer.render_response(resp);

    loop {
        renderer.eloop()?;
    }
}
