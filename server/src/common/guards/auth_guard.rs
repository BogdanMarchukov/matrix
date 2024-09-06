use crate::errors::gql_error::GqlError;
use crate::GqlCtx;
use async_graphql::{Error, ErrorExtensions, Guard};

pub struct AuthGuard;

impl Guard for AuthGuard {
    async fn check(&self, ctx: &async_graphql::Context<'_>) -> Result<(), Error> {
        let user = match ctx.data::<GqlCtx>() {
            Ok(data) => &data.user,
            Err(_) => {
                println!("auth error");
                return Err(GqlError::Unauthorized.extend());
            }
        };
        match user {
            Some(_) => Ok(()),
            None => {
                println!("auth error =============");
                Err(GqlError::Unauthorized.extend())
            }
        }
    }
}
