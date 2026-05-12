use serde_json::Error as JsonError;
use thiserror::Error;
use tower_lsp::jsonrpc::Error as JsonRpcError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] JsonError),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Tree-sitter error")]
    TreeSitter,

    #[error("LSP error: {0}")]
    Lsp(#[from] JsonRpcError),

    #[error("Invalid URI: {0}")]
    InvalidUri(String),
}

impl From<Error> for JsonRpcError {
    fn from(e: Error) -> Self {
        JsonRpcError {
            code: tower_lsp::jsonrpc::ErrorCode::InternalError,
            message: e.to_string().into(),
            data: None,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
