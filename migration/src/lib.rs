pub use sea_orm_migration::prelude::*;

mod create_user_table;

mod create_app_table;
mod create_app_version_table;
mod create_dump_table;
mod create_pay_record_table;
mod create_pay_table;
mod create_user_dump_table;
mod create_pay_menu_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(create_user_table::Migration),
            Box::new(create_app_table::Migration),
            Box::new(create_pay_table::Migration),
            Box::new(create_pay_record_table::Migration),
            Box::new(create_app_version_table::Migration),
            Box::new(create_dump_table::Migration),
            Box::new(create_user_dump_table::Migration),
            Box::new(create_pay_menu_table::Migration),
        ]
    }
}
