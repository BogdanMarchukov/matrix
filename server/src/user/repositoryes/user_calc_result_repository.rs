use crate::entity::user_calc_result;
use crate::types::traits::{InsertData, OptionFieldsFilter, Repository, UpdateData};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

pub struct UserCalcResultRepository;

impl
    Repository<
        user_calc_result::Model,
        DatabaseConnection,
        UserCalcResultFilter,
        UserCalcResultInsertData,
        UserCalcResultUpdateData,
    > for UserCalcResultRepository
{
    async fn find_by_pk(
        id: Uuid,
        db: &DatabaseConnection,
    ) -> Result<Option<user_calc_result::Model>, DbErr> {
        user_calc_result::Entity::find_by_id(id).one(db).await
    }

    async fn find_many(
        filter: UserCalcResultFilter,
        db: &DatabaseConnection,
    ) -> Result<Vec<user_calc_result::Model>, DbErr> {
        let mut query = user_calc_result::Entity::find();
        if let Some(user_id) = filter.user_id {
            query = query.filter(user_calc_result::Column::UserId.eq(user_id));
        }
        if let Some(calculator_id) = filter.calculator_id {
            query = query.filter(user_calc_result::Column::CalculatorId.eq(calculator_id));
        }
        if let Some(key) = filter.key {
            query = query.filter(user_calc_result::Column::Key.eq(key));
        }
        query.all(db).await
    }

    async fn find_one(
        filter: UserCalcResultFilter,
        db: &DatabaseConnection,
    ) -> Result<Option<user_calc_result::Model>, DbErr> {
        let mut query = user_calc_result::Entity::find();
        if let Some(user_id) = filter.user_id {
            query = query.filter(user_calc_result::Column::UserId.eq(user_id));
        }
        if let Some(calculator_id) = filter.calculator_id {
            query = query.filter(user_calc_result::Column::CalculatorId.eq(calculator_id));
        }
        if let Some(key) = filter.key {
            query = query.filter(user_calc_result::Column::Key.eq(key));
        }
        query.one(db).await
    }

    async fn create_one(
        data: UserCalcResultInsertData,
        db: &DatabaseConnection,
    ) -> Result<user_calc_result::Model, DbErr> {
        let data = user_calc_result::ActiveModel {
            user_calc_result_id: Set(data.user_calc_result_id),
            user_id: Set(data.user_id),
            calculator_id: Set(data.calculator_id),
            result: Set(data.result),
            key: Set(data.key),
        };
        user_calc_result::Entity::insert(data)
            .exec_with_returning(db)
            .await
    }

    async fn update_one(
        id: Uuid,
        data: UserCalcResultUpdateData,
        db: &DatabaseConnection,
    ) -> Result<user_calc_result::Model, DbErr> {
        let mut model: user_calc_result::ActiveModel = user_calc_result::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Record not found".to_string()))?
            .into();

        if let Some(user_id) = data.user_id {
            model.user_id = Set(user_id);
        }
        if let Some(calculator_id) = data.calculator_id {
            model.calculator_id = Set(calculator_id);
        }
        if let Some(key) = data.key {
            model.key = Set(key);
        }
        if let Some(result) = data.result {
            model.result = Set(result);
        }

        model.update(db).await
    }

    async fn delete_one(id: Uuid, db: &DatabaseConnection) -> Result<bool, DbErr> {
        let result = user_calc_result::Entity::delete_by_id(id).exec(db).await;
        match result {
            Ok(result) => Ok(result.rows_affected > 0),
            Err(err) => Err(err),
        }
    }
}

pub struct UserCalcResultFilter {
    pub user_id: Option<Uuid>,
    pub calculator_id: Option<Uuid>,
    pub key: Option<String>,
}

pub struct UserCalcResultUpdateData {
    pub user_id: Option<Uuid>,
    pub calculator_id: Option<Uuid>,
    pub result: Option<serde_json::Value>,
    pub key: Option<String>,
}

pub struct UserCalcResultInsertData {
    pub user_calc_result_id: Uuid,
    pub user_id: Uuid,
    pub calculator_id: Uuid,
    pub result: serde_json::Value,
    pub key: String,
}

impl Default for UserCalcResultInsertData {
    fn default() -> Self {
        Self {
            user_calc_result_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            calculator_id: Uuid::new_v4(),
            result: serde_json::json!({}),
            key: String::new(),
        }
    }
}

impl UpdateData for UserCalcResultUpdateData {}

impl InsertData for UserCalcResultInsertData {}

impl OptionFieldsFilter for UserCalcResultFilter {}
