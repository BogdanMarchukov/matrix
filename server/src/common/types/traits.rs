use sea_orm::{ConnectionTrait, DbErr, ModelTrait};
use uuid::Uuid;

pub trait OptionFieldsFilter {}
pub trait InsertData {}
pub trait UpdateData {}

pub trait Repository<T, F, D, U>
where
    T: ModelTrait,
    F: OptionFieldsFilter,
    D: InsertData,
    U: UpdateData,
{
    async fn find_by_pk<C>(id: Uuid, db: &C) -> Result<Option<T>, DbErr>
    where
        C: ConnectionTrait;

    async fn find_many<C>(filter: F, db: &C) -> Result<Vec<T>, DbErr>
    where
        C: ConnectionTrait;

    async fn find_one<C>(filter: F, db: &C) -> Result<Option<T>, DbErr>
    where
        C: ConnectionTrait;

    async fn create_one<C>(data: D, db: &C) -> Result<T, DbErr>
    where
        C: ConnectionTrait;

    async fn update_one<C>(id: Uuid, data: U, db: &C) -> Result<T, DbErr>
    where
        C: ConnectionTrait;

    async fn delete_one<C>(id: Uuid, db: &C) -> Result<bool, DbErr>
    where
        C: ConnectionTrait;
}
