use crate::entity::calculator;
use crate::entity::sea_orm_active_enums::CalculatorType;
use async_graphql::{Enum, SimpleObject};
use uuid::Uuid;

#[derive(Clone, Debug, SimpleObject)]
#[graphql(name = "Calculator")]
pub struct CalculatorGqlModel {
    pub calculator_id: Uuid,
    pub r#type: CalculatorGqlType,
    pub require_params: Option<Vec<String>>,
    pub options_params: Option<Vec<String>>,
    pub offer_id: Uuid,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
#[graphql(remote = "CalculatorType", name = "CalculatorType")]
pub enum CalculatorGqlType {
    MatrixSchema,
}

impl From<calculator::Model> for CalculatorGqlModel {
    fn from(model: calculator::Model) -> Self {
        Self {
            calculator_id: model.calculator_id,
            r#type: model.r#type.into(),
            require_params: model.require_params,
            options_params: model.options_params,
            offer_id: model.offer_id,
        }
    }
}
