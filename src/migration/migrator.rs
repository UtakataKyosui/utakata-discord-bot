pub use sea_orm_migration::prelude::*;
use crate::migration::m20250121_090757_create_table_point;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250121_090757_create_table_point::Migration),
        ]
    }
}
