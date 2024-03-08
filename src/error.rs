use thiserror::Error;

pub type PoppetResult<T> = core::result::Result<T, PoppetError>;

#[derive(Error, Debug)]
pub enum PoppetError {
    #[error("I/O Error")]
    Disconnect(#[from] std::io::Error),

    #[error("Error from SDL2")]
    SdlError(String),

    #[error("Error from Poppet Renderer")]
    RenderError(#[from] crate::renderer::RenderError),

    #[error("unknown error")]
    Unknown,
}

impl From<String> for PoppetError {
    fn from(value: String) -> Self {
        Self::SdlError(value)
    }
}

