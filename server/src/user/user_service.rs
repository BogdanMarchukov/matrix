use super::user_gql_model::User;
use crate::errors::gql_error::GqlError;
use crate::GqlCtx;
use async_graphql::{Context, ErrorExtensions, FieldResult};
use sea_orm::DatabaseConnection;

pub fn get_auth_user_from_ctx(ctx: &Context) -> FieldResult<(User, DatabaseConnection)> {
    let ctx_data = match ctx.data::<GqlCtx>() {
        Ok(data) => data,
        Err(_) => return Err(GqlError::ServerError("get cxt error".to_string()).into()),
    };
    let user = match ctx_data.user.to_owned() {
        Some(user) => user,
        None => return Err(GqlError::Unauthorized.extend()),
    };
    Ok((user, ctx_data.db.to_owned()))
}
