pub use thiserror::Error;

/// Public error type
#[derive(Error, Debug)]
pub enum SlackError {
    /// Errors from hex library or internal hex errors
    #[error("hex color parsing error: {0}")]
    HexColor(String),

    /// Errors from reqwest send/gets or reqwests url TryInto
    #[error("slack service error: {0}")]
    Http(String),

    /// Errors from url's parser
    #[error(transparent)]
    Url(#[from] url::ParseError)
}

/// Error handling convenience type
pub type Result<T> = std::result::Result<T, SlackError>;
