use crate::errors::gql_error::GqlError;
use crate::{entity::users, GqlCtx};
use async_graphql::{Context, FieldResult};
use sea_orm::DatabaseConnection;

pub fn get_auth_user_from_ctx<'ctx>(
    ctx: &Context<'ctx>,
) -> FieldResult<(users::Model, DatabaseConnection)> {
    let ctx_data = match ctx.data::<GqlCtx>() {
        Ok(data) => data,
        Err(_) => return Err(GqlError::ServerError("get cxt error".to_string()).into()),
    };
    let user = match ctx_data.user.clone() {
        Some(user) => user,
        None => return Err(GqlError::Unauthorized.into()),
    };
    Ok((user, ctx_data.db.clone()))
}
