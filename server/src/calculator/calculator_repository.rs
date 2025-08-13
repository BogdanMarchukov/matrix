use crate::entity::calculator;
use crate::entity::sea_orm_active_enums::CalculatorType;
use crate::types::traits::{InsertData, OptionFieldsFilter, Repository};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub struct CalculatorRepository;

impl Repository<calculator::Model, DatabaseConnection, CalculatorFilter, CalculatorInsertData>
    for CalculatorRepository
{
    async fn find_by_pk(id: Uuid, db: &DatabaseConnection) -> Result<calculator::Model, DbErr> {
        calculator::Entity::find_by_id(id)
            .one(db)
            .await
            .map(|opt| opt.unwrap())
    }

    async fn find_many(
        filter: CalculatorFilter,
        db: &DatabaseConnection,
    ) -> Result<Vec<calculator::Model>, DbErr> {
        let mut query = calculator::Entity::find();
        if let Some(calculator_type) = filter.calculator_type {
            query = query.filter(calculator::Column::Type.eq(calculator_type));
        }
        query.all(db).await
    }

    async fn find_one(
        filter: CalculatorFilter,
        db: &DatabaseConnection,
    ) -> Result<Option<calculator::Model>, DbErr> {
        let mut query = calculator::Entity::find();
        if let Some(calculator_type) = filter.calculator_type {
            query = query.filter(calculator::Column::Type.eq(calculator_type));
        }
        query.one(db).await
    }

    async fn create_one(
        data: CalculatorInsertData,
        db: &DatabaseConnection,
    ) -> Result<calculator::Model, DbErr> {
        let data = calculator::ActiveModel {
            calculator_id: Set(data.calculator_id),
            r#type: Set(data.r#type),
            require_params: Set(data.require_params),
            options_params: Set(data.options_params),
        };
        calculator::Entity::insert(data)
            .exec_with_returning(db)
            .await
    }
}

pub struct CalculatorFilter {
    pub calculator_type: Option<String>,
}

pub struct CalculatorInsertData {
    pub calculator_id: Uuid,
    pub r#type: CalculatorType,
    pub require_params: Option<Vec<String>>,
    pub options_params: Option<Vec<String>>,
}

impl InsertData for CalculatorInsertData {}

impl OptionFieldsFilter for CalculatorFilter {}
