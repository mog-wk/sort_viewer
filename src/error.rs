
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    WindowBuildError(#[from] sdl2::video::WindowBuildError),
    #[error(transparent)]
    CanvasError(#[from] sdl2::IntegerOrSdlError),
}

impl std::convert::From<String> for Error {
    fn from(st: String) -> Error {
        Error::Generic(st)
    }
}


