pub use sea_orm_migration::prelude::*;

mod m20240731_110230_create_user;
mod m20240807_135202_telegram_uniqu;
mod m20240809_100140_user_add_role;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240731_110230_create_user::Migration),
            Box::new(m20240807_135202_telegram_uniqu::Migration),
            Box::new(m20240809_100140_user_add_role::Migration),
        ]
    }
}
