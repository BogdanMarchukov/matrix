use async_graphql::{ErrorExtensions, FieldResult, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::entity::user_tariff_plan;
use crate::errors::gql_error::GqlError;

use super::user_gql_model::{User, UserRoleGqlType};

#[derive(SimpleObject, Clone, Debug)]
#[graphql(name = "UserTariffPlan")]
pub struct UserTariffPlanGqlModel {
    pub user_tariff_plan_id: Uuid,
    pub tariff_plan_id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub tariff_plan_payment_id: Option<Uuid>,
}

impl UserTariffPlanGqlModel {
    pub fn new(user_tariff_plan: user_tariff_plan::Model) -> Self {
        Self {
            user_tariff_plan_id: user_tariff_plan.user_tariff_plan_id,
            tariff_plan_id: user_tariff_plan.tariff_plan_id,
            user_id: user_tariff_plan.user_id,
            created_at: DateTime::<Utc>::from_naive_utc_and_offset(
                user_tariff_plan.created_at,
                Utc,
            ),
            expires_at: DateTime::<Utc>::from_naive_utc_and_offset(
                user_tariff_plan.expires_at,
                Utc,
            ),
            tariff_plan_payment_id: user_tariff_plan.tariff_plan_payment_id,
        }
    }

    pub fn check_role(&self, user: &User) -> FieldResult<&Self> {
        let allowed = match user.0.role {
            UserRoleGqlType::Owner => true,
            UserRoleGqlType::Admin => true,
            UserRoleGqlType::Member => self.user_id == user.0.user_id,
        };
        if allowed {
            return Ok(self);
        }
        Err(GqlError::Forbidden.extend())
    }
}
