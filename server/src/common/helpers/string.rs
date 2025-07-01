use crate::errors::gql_error::GqlError;
use async_graphql::{ErrorExtensions, FieldResult};
use uuid::Uuid;

pub fn _get_uuid_from_string(id: &String) -> FieldResult<Uuid> {
    let uuid = match Uuid::parse_str(&id[..]) {
        Ok(id) => id,
        Err(_) => {
            return Err(GqlError::BadRequest("pars id error, mast by uuid".to_string()).extend())
        }
    };
    Ok(uuid)
}
