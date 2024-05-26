use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let result = manager
            .create_table(
                Table::create()
                    .table(NodeInfo::Table)
                    .col(
                        ColumnDef::new(NodeInfo::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(NodeInfo::NeighborNodes).string().not_null())
                    .col(ColumnDef::new(NodeInfo::IsAlive).bool())
                    .col(ColumnDef::new(NodeInfo::ClockInfoIndex).integer())
                    .col(ColumnDef::new(NodeInfo::MergeLogIndex).integer())
                    .col(ColumnDef::new(NodeInfo::ZMessageIndex).integer())
                    .to_owned(),
            )
            .await;

        if let Err(err) = result {
            return Err(err);
        }
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NodeInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum NodeInfo {
    Table,
    Id,
    NeighborNodes,
    IsAlive,
    ClockInfoIndex,
    MergeLogIndex,
    ZMessageIndex
}
