use crate::entity::calculator;
use crate::entity::sea_orm_active_enums::CalculatorType;
use crate::types::traits::{InsertData, OptionFieldsFilter, Repository, UpdateData};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use super::calculator_gql::{CreateOneInput, FindOneFilterInput};

pub struct CalculatorRepository;

impl
    Repository<
        calculator::Model,
        DatabaseConnection,
        CalculatorFilter,
        CalculatorInsertData,
        CalculatorUpdateData,
    > for CalculatorRepository
{
    async fn find_by_pk(
        id: Uuid,
        db: &DatabaseConnection,
    ) -> Result<Option<calculator::Model>, DbErr> {
        calculator::Entity::find_by_id(id).one(db).await
    }

    async fn find_many(
        filter: CalculatorFilter,
        db: &DatabaseConnection,
    ) -> Result<Vec<calculator::Model>, DbErr> {
        let mut query = calculator::Entity::find();
        if let Some(calculator_type) = filter.r#type {
            query = query.filter(calculator::Column::Type.eq(calculator_type));
        }
        query.all(db).await
    }

    async fn find_one(
        filter: CalculatorFilter,
        db: &DatabaseConnection,
    ) -> Result<Option<calculator::Model>, DbErr> {
        let mut query = calculator::Entity::find();
        if let Some(calculator_type) = filter.r#type {
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

    async fn update_one(
        id: Uuid,
        data: CalculatorUpdateData,
        db: &DatabaseConnection,
    ) -> Result<calculator::Model, DbErr> {
        let mut model: calculator::ActiveModel = calculator::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Record not found".to_string()))?
            .into();

        if let Some(calculator_type) = data.r#type {
            model.r#type = Set(calculator_type);
        }
        if let Some(require_params) = data.require_params {
            model.require_params = Set(Some(require_params));
        }
        if let Some(options_params) = data.options_params {
            model.options_params = Set(Some(options_params));
        }

        model.update(db).await
    }

    async fn delete_one(id: Uuid, db: &DatabaseConnection) -> Result<bool, DbErr> {
        let result = calculator::Entity::delete_by_id(id).exec(db).await;
        match result {
            Ok(_) => Ok(true),
            Err(err) => Err(err),
        }
    }
}

pub struct CalculatorFilter {
    pub r#type: Option<CalculatorType>,
}

pub struct CalculatorUpdateData {
    pub r#type: Option<CalculatorType>,
    pub require_params: Option<Vec<String>>,
    pub options_params: Option<Vec<String>>,
}

pub struct CalculatorInsertData {
    pub calculator_id: Uuid,
    pub r#type: CalculatorType,
    pub require_params: Option<Vec<String>>,
    pub options_params: Option<Vec<String>>,
}

impl Default for CalculatorInsertData {
    fn default() -> Self {
        Self {
            calculator_id: Uuid::new_v4(),
            r#type: CalculatorType::MatrixSchema,
            require_params: None,
            options_params: None,
        }
    }
}

impl From<CreateOneInput> for CalculatorInsertData {
    fn from(input: CreateOneInput) -> Self {
        Self {
            r#type: input.r#type.into(),
            require_params: input.require_params,
            options_params: input.options_params,
            ..Default::default()
        }
    }
}

impl InsertData for CalculatorInsertData {}

impl UpdateData for CalculatorUpdateData {}

impl OptionFieldsFilter for CalculatorFilter {}

impl From<FindOneFilterInput> for CalculatorFilter {
    fn from(filter: FindOneFilterInput) -> Self {
        Self {
            r#type: Some(filter.r#type.into()),
        }
    }
}
