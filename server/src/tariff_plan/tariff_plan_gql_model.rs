use async_graphql::SimpleObject;
use num_traits::ToPrimitive;
use uuid::Uuid;

use async_graphql::*;

use crate::entity::tariff_plan;

#[derive(Clone, SimpleObject)]
#[graphql(name = "TariffPlan")]
pub struct TariffPlanGqlModel {
    pub tariff_plan_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub price: f64,
    pub expiry_days: i32,
}

impl TariffPlanGqlModel {
    pub fn new(tariff_plan: tariff_plan::Model) -> Self {
        Self {
            tariff_plan_id: tariff_plan.tariff_plan_id,
            title: tariff_plan.title,
            description: tariff_plan.description,
            price: tariff_plan.price.to_f64().unwrap_or(0.00),
            expiry_days: tariff_plan.expiry_days,
        }
    }
}
