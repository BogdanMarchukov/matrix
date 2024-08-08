use crate::GqlCtx;
use async_graphql::{Error, Guard};

pub struct AuthGuard;

impl Guard for AuthGuard {
    async fn check(&self, ctx: &async_graphql::Context<'_>) -> Result<(), Error> {
        let user = match ctx.data::<GqlCtx>() {
            Ok(data) => &data.user,
            Err(_) => return Err("Unautoraize".into()),
        };
        match user {
            Some(_) => Ok(()),
            None => Err("Unautoraize".into()),
        }
    }
}
