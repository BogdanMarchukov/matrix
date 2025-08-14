use crate::guards::{auth_guard::AuthGuard, system_guard::SystemGuard};
use crate::{calculator::calculator_gql_model::CalculatorGqlModel, db_utils};
use async_graphql::{Context, FieldResult, InputObject, Object};

use super::{calculator_gql_model::CalculatorGqlType, calculator_service};

pub struct CalculatorQuery;
pub struct CalculatorMutation;

#[derive(InputObject, Clone)]
pub struct FindOneFilterInput {
    pub r#type: CalculatorGqlType,
}

#[derive(InputObject, Clone)]
pub struct CreateOneInput {
    pub r#type: CalculatorGqlType,
    pub require_params: Option<Vec<String>>,
    pub options_params: Option<Vec<String>>,
}

#[Object]
impl CalculatorQuery {
    #[graphql(guard = "AuthGuard")]
    async fn find_one<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        filter: FindOneFilterInput,
    ) -> FieldResult<Option<CalculatorGqlModel>> {
        let db = db_utils::get_connection_from_gql_ctx(ctx)?;
        calculator_service::find_one(filter, &db).await
    }
}

#[Object]
impl CalculatorMutation {
    #[graphql(guard = "SystemGuard")]
    async fn crete_one<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: CreateOneInput,
    ) -> FieldResult<CalculatorGqlModel> {
        let db = db_utils::get_connection_from_gql_ctx(ctx)?;
        calculator_service::create_one(data, &db).await
    }
}
