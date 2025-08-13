use sea_orm::{ConnectionTrait, DbErr, ModelTrait};
use uuid::Uuid;

pub trait OptionFieldsFilter {}

pub trait Repository<T, C, F>
where
    T: ModelTrait,
    C: ConnectionTrait,
    F: OptionFieldsFilter,
{
    async fn find_by_pk(id: Uuid, db: &C) -> Result<T, DbErr>;

    async fn find_many(filter: F, db: &C) -> Result<Vec<T>, DbErr>;

    async fn find_one(filter: F, db: &C) -> Result<Option<T>, DbErr>;

    async fn create_one(data: T, db: &C) -> Result<T, DbErr>;

    async fn update_one(id: Uuid, data: T, db: &C) -> Result<T, DbErr>;

    async fn delete_one(id: Uuid, db: &C) -> Result<bool, DbErr>;
}
