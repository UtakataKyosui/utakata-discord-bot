use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Record::Table)
                    .if_not_exists()
                    // String 型のカラムを主キーとして設定
                    .col(
                        ColumnDef::new(Record::Key)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    // Int 型のカラムを追加
                    .col(ColumnDef::new(Record::Value).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Record::Table).to_owned())
            .await
    }
}

// テーブルとカラムの識別子
#[derive(DeriveIden)]
enum Record {
    Table,
    Key,
    Value,
}
