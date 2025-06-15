use async_graphql::Error;
use async_graphql::*;

#[derive(Debug, thiserror::Error)]
pub enum GqlError {
    #[error("Could not find resource")]
    NotFound(String),

    #[error("Invalid request")]
    BadRequest(String),

    #[error("Not authorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("ServerError")]
    ServerError(String),
}

impl ErrorExtensions for GqlError {
    fn extend(&self) -> Error {
        Error::new(format!("{}", self)).extend_with(|_, e| match self {
            GqlError::NotFound(reason) => {
                e.set("code", 404);
                e.set("reason", reason.to_owned())
            }
            GqlError::BadRequest(reason) => {
                e.set("reason", reason.to_owned());
                e.set("code", 400)
            }
            GqlError::Unauthorized => e.set("code", 401),
            GqlError::Forbidden => e.set("code", 403),
            GqlError::ServerError(reason) => {
                e.set("reason", reason.to_owned());
                e.set("code", 500)
            }
        })
    }
}
