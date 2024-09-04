use crate::{errors::gql_error::GqlError, GqlCtx};
use async_graphql::{Context, ErrorExtensions, FieldResult};

pub fn get_ctx_data<'a>(ctx: &Context<'a>) -> FieldResult<&'a GqlCtx> {
    match ctx.data::<GqlCtx>() {
        Ok(data) => Ok(data),
        Err(_) => return Err(GqlError::ServerError("get ctx data errors".to_string()).extend()),
    }
}
