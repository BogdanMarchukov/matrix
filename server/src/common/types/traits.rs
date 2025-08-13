use sea_orm::{ConnectionTrait, DbErr, ModelTrait};

trait Repository {
    async fn find_by_pk<T, C>(id: String, db: &C) -> Result<T, DbErr>
    where
        T: ModelTrait,
        C: ConnectionTrait;

    async fn find_many<T, C, F>(filter: F, db: &C) -> Result<Vec<T>, DbErr>
    where
        T: ModelTrait;
}
