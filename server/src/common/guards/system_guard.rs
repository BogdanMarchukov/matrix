use crate::config;
use crate::errors::gql_error::GqlError;
use crate::GqlCtx;
use async_graphql::{Error, ErrorExtensions, Guard};

pub struct SystemGuard;

impl Guard for SystemGuard {
    async fn check(&self, ctx: &async_graphql::Context<'_>) -> Result<(), Error> {
        let header = match ctx.data::<GqlCtx>() {
            Ok(data) => &data.headers,
            Err(_) => return Err(GqlError::Unauthorized.extend()),
        };
        let api_key = config::get_api_key();
        let header_key = match header.get("x-api-key") {
            Some(key) => key,
            None => return Err(GqlError::Unauthorized.extend()),
        };
        if &api_key == header_key {
            return Ok(());
        }
        Err(GqlError::Unauthorized.extend())
    }
}
