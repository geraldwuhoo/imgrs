use actix_web::error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImagineError {
    #[error("uri parse error\n{0}")]
    ImgurParse(#[from] http::uri::InvalidUri),

    #[error("imgur request error\n{0}")]
    ImgurRequest(#[from] hyper::Error),

    #[error("request construction error\n{0}")]
    RequestBuildError(#[from] http::Error),
    
    #[error("request construction error\n{0}")]
    ResponseError(#[from] std::string::FromUtf8Error),

    #[error("json error\n{0}")]
    JsonError(#[from] serde_json::Error),

    #[error("askama templating error\n{0}")]
    AskamaError(#[from] askama::Error),

    #[error("content type parsing error\n{0}")]
    ContentTypeError(#[from] http::header::ToStrError),
}

impl error::ResponseError for ImagineError {}
