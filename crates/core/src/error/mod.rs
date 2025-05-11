use crate::model::ModelValidation;
use async_trait::async_trait;
use lettre::{address::AddressError, transport::smtp};
use oauth2::url::ParseError;
use salvo::http::ParseError as SalvoHttpParseError;
use salvo::http::header::{InvalidHeaderName, InvalidHeaderValue};
use salvo::http::method::InvalidMethod;
use salvo::jwt_auth::JwtError;
use salvo::prelude::{StatusCode, StatusError};
use salvo::{Depot, Request, Response, Writer};

impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        Self::JSON(val).bt()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{inner}\n{backtrace}")]
    WithBacktrace { inner: Box<Self>, backtrace: Box<std::backtrace::Backtrace> },

    #[error("{0}")]
    Message(String),

    #[error("task not found: '{0}'")]
    TaskNotFound(String),

    #[error(transparent)]
    JSON(serde_json::Error),

    #[error(transparent)]
    EnvVar(#[from] std::env::VarError),

    #[error(transparent)]
    Smtp(#[from] smtp::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    SqlxError(#[from] sea_orm::SqlxError),

    #[error(transparent)]
    ParseAddress(#[from] AddressError),

    #[error("{0}")]
    Hash(String),

    // API
    #[error("{0}")]
    Unauthorized(String),

    // API
    #[error("not found")]
    NotFound,

    #[error("{0}")]
    BadRequest(String),

    #[error("internal server error")]
    InternalServerError,

    #[error(transparent)]
    InvalidHeaderValue(#[from] InvalidHeaderValue),

    #[error(transparent)]
    InvalidHeaderName(#[from] InvalidHeaderName),

    #[error(transparent)]
    InvalidMethod(#[from] InvalidMethod),

    #[error(transparent)]
    StatusError(#[from] StatusError),

    #[error(transparent)]
    OpendalError(#[from] opendal::Error),

    #[error(transparent)]
    PasswordHashError(#[from] argon2::password_hash::Error),

    #[error(transparent)]
    JwtError(#[from] JwtError),

    #[error(transparent)]
    Any(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("Entity already exists")]
    EntityAlreadyExists,

    #[error("Entity not found")]
    EntityNotFound,

    #[error("{errors:?}")]
    ModelValidation { errors: ModelValidation },

    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),

    #[error("CsrfToken supplied does not match")]
    CSRFTokenMismatch,
    #[error("Request Error: `{0}`")]
    Reqwest(String),
    #[error("CsrfToken is missing")]
    MissingCsrfToken,
    /// This happens when user cancels authorization
    #[error("Authorization Code is missing")]
    MissingAuthorizationCode,
    #[error("Authorization Code and CsrfToken are missing")]
    MissingAuthorizationCodeAndCsrfToken,
    #[error("Parse Error: {0}")]
    ParseError(#[from] ParseError),
    #[error("Parse Error: {0}")]
    SalvoHttpParseError(#[from] SalvoHttpParseError),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

impl Error {
    pub fn wrap(err: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Any(Box::new(err)) //.bt()
    }

    pub fn msg(err: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Message(err.to_string()) //.bt()
    }
    #[must_use]
    pub fn string(s: &str) -> Self {
        Self::Message(s.to_string())
    }
    #[must_use]
    pub fn bt(self) -> Self {
        let backtrace = std::backtrace::Backtrace::capture();
        match backtrace.status() {
            std::backtrace::BacktraceStatus::Disabled
            | std::backtrace::BacktraceStatus::Unsupported => self,
            _ => Self::WithBacktrace { inner: Box::new(self), backtrace: Box::new(backtrace) },
        }
    }
}

#[async_trait]
impl Writer for Error {
    async fn write(mut self, _req: &mut Request, _: &mut Depot, res: &mut Response) {
        let code = match self {
            Error::WithBacktrace { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Message(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::TaskNotFound(_) => StatusCode::NOT_FOUND,
            Error::JSON(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::EnvVar(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Smtp(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::IO(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ParseAddress(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Hash(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::InvalidHeaderValue(_) => StatusCode::BAD_REQUEST,
            Error::InvalidHeaderName(_) => StatusCode::BAD_REQUEST,
            Error::InvalidMethod(_) => StatusCode::BAD_REQUEST,
            Error::StatusError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::JwtError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Any(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::EntityAlreadyExists => StatusCode::INTERNAL_SERVER_ERROR,
            Error::EntityNotFound => StatusCode::NOT_FOUND,
            Error::ModelValidation { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DbErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::OpendalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        res.render(
            StatusError::from_code(code)
                .unwrap_or(StatusError::internal_server_error())
                .brief(self.to_string())
                .cause(self.bt()),
        )
    }
}
