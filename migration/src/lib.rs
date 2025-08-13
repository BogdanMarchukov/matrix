pub use sea_orm_migration::prelude::*;

mod m20240731_110230_create_user;
mod m20240807_135202_telegram_uniqu;
mod m20240809_100140_user_add_role;
mod m20240819_115128_create_notify;
mod m20240820_072759_create_newslatter;
mod m20240904_151724_add_notify_type;
mod m20241103_104246_user_info_create;
mod m20241103_182645_user_info_data_create;
mod m20250610_173639_tariff_plan_create;
mod m20250610_181858_user_tariff_plan_create;
mod m20250611_191338_offer_create;
mod m20250612_095148_payment_crete;
mod m20250612_100633_payment_tariff_plan_create;
mod m20250612_101751_alter_table_user_tariff_plan;
mod m20250612_110702_create_free_tariff_plan;
mod m20250702_135213_news;
mod m20250702_141344_user_news;
mod m20250707_200845_create_user_news_index;
mod m20250806_000000_create_offer_like_table;
mod m20250806_000001_create_news_like_table;
mod m20250813_143047_create_calculator_table;
mod m20250813_185426_calculate_insert_data;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240731_110230_create_user::Migration),
            Box::new(m20240807_135202_telegram_uniqu::Migration),
            Box::new(m20240809_100140_user_add_role::Migration),
            Box::new(m20240819_115128_create_notify::Migration),
            Box::new(m20240820_072759_create_newslatter::Migration),
            Box::new(m20240904_151724_add_notify_type::Migration),
            Box::new(m20241103_104246_user_info_create::Migration),
            Box::new(m20241103_182645_user_info_data_create::Migration),
            Box::new(m20250610_173639_tariff_plan_create::Migration),
            Box::new(m20250610_181858_user_tariff_plan_create::Migration),
            Box::new(m20250611_191338_offer_create::Migration),
            Box::new(m20250612_095148_payment_crete::Migration),
            Box::new(m20250612_100633_payment_tariff_plan_create::Migration),
            Box::new(m20250612_101751_alter_table_user_tariff_plan::Migration),
            Box::new(m20250612_110702_create_free_tariff_plan::Migration),
            Box::new(m20250702_135213_news::Migration),
            Box::new(m20250702_141344_user_news::Migration),
            Box::new(m20250707_200845_create_user_news_index::Migration),
            Box::new(m20250806_000000_create_offer_like_table::Migration),
            Box::new(m20250806_000001_create_news_like_table::Migration),
            Box::new(m20250813_143047_create_calculator_table::Migration),
            Box::new(m20250813_185426_calculate_insert_data::Migration),
        ]
    }
}
