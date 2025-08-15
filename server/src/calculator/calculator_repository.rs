use crate::entity::sea_orm_active_enums::CalculatorType;
use crate::entity::{calculator, offer};
use crate::types::traits::{InsertData, OptionFieldsFilter, Repository, UpdateData};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter, Set,
};
use uuid::Uuid;

use super::calculator_gql::{CreateOneInput, FindOneFilterInput};

pub struct CalculatorRepository;

impl Repository<calculator::Model, CalculatorFilter, CalculatorInsertData, CalculatorUpdateData>
    for CalculatorRepository
{
    async fn find_by_pk<C>(id: Uuid, db: &C) -> Result<Option<calculator::Model>, DbErr>
    where
        C: ConnectionTrait,
    {
        calculator::Entity::find_by_id(id).one(db).await
    }

    async fn find_many<C>(filter: CalculatorFilter, db: &C) -> Result<Vec<calculator::Model>, DbErr>
    where
        C: ConnectionTrait,
    {
        let mut query = calculator::Entity::find();
        if let Some(calculator_type) = filter.r#type {
            query = query.filter(calculator::Column::Type.eq(calculator_type));
        }
        query.all(db).await
    }

    async fn find_one<C>(
        filter: CalculatorFilter,
        db: &C,
    ) -> Result<Option<calculator::Model>, DbErr>
    where
        C: ConnectionTrait,
    {
        let mut query = calculator::Entity::find();
        if let Some(calculator_type) = filter.r#type {
            query = query.filter(calculator::Column::Type.eq(calculator_type));
        }
        query.one(db).await
    }

    async fn create_one<C>(data: CalculatorInsertData, db: &C) -> Result<calculator::Model, DbErr>
    where
        C: ConnectionTrait,
    {
        let data = calculator::ActiveModel {
            calculator_id: Set(data.calculator_id),
            r#type: Set(data.r#type),
            require_params: Set(data.require_params),
            options_params: Set(data.options_params),
            offer_id: Set(data.offer_id),
        };
        calculator::Entity::insert(data)
            .exec_with_returning(db)
            .await
    }

    async fn update_one<C>(
        id: Uuid,
        data: CalculatorUpdateData,
        db: &C,
    ) -> Result<calculator::Model, DbErr>
    where
        C: ConnectionTrait,
    {
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

    async fn delete_one<C>(id: Uuid, db: &C) -> Result<bool, DbErr>
    where
        C: ConnectionTrait,
    {
        let result = calculator::Entity::delete_by_id(id).exec(db).await;
        match result {
            Ok(_) => Ok(true),
            Err(err) => Err(err),
        }
    }
}

impl CalculatorRepository {
    pub async fn find_by_pk_with_offer<C>(
        id: Uuid,
        db: &C,
    ) -> Result<Option<(calculator::Model, Option<offer::Model>)>, DbErr>
    where
        C: ConnectionTrait,
    {
        calculator::Entity::find_by_id(id)
            .find_also_related(offer::Entity)
            .one(db)
            .await
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
    pub offer_id: Uuid,
    pub r#type: CalculatorType,
    pub require_params: Option<Vec<String>>,
    pub options_params: Option<Vec<String>>,
}

impl Default for CalculatorInsertData {
    fn default() -> Self {
        Self {
            offer_id: Uuid::new_v4(),
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
            offer_id: input.offer_id,
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
