use entity::{pix, refresh_log, refreshed_users, student, user};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(user::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(user::Column::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(user::Column::PgritId)
                            .unique_key()
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(student::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(student::Column::UserId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(student::Column::DegreeStep)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(student::Column::Grade)
                            .small_unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(student::Column::Course).string().not_null())
                    .col(ColumnDef::new(student::Column::Level).string().not_null())
                    .col(ColumnDef::new(student::Column::Sex).string().not_null())
                    .col(ColumnDef::new(student::Column::JoinDate).date().not_null())
                    .col(ColumnDef::new(student::Column::Office).string().not_null())
                    .col(ColumnDef::new(student::Column::Email).string().not_null())
                    .col(
                        ColumnDef::new(student::Column::EmailOf4nonome)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(student::Column::University)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(student::Column::Major).string().not_null())
                    .col(ColumnDef::new(student::Column::LeaveDate).date().null())
                    .col(ColumnDef::new(student::Column::Active).boolean().not_null())
                    .col(ColumnDef::new(student::Column::SlackId).string().not_null())
                    .col(ColumnDef::new(student::Column::DiscordId).string().null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(refreshed_users::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(refreshed_users::Column::UserId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(refreshed_users::Column::RefreshLogId).not_null())
                    .primary_key(
                        Index::create()
                            .col(refreshed_users::Column::UserId)
                            .col(refreshed_users::Column::RefreshLogId),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(refresh_log::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(refresh_log::Column::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(refresh_log::Column::UpdatedAt)
                            .date_time()
                            .unique_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(pix::Entity)
                    .if_not_exists()
                    .col(ColumnDef::new(pix::Column::UserId).string().not_null())
                    .col(ColumnDef::new(pix::Column::Date).date().not_null())
                    .col(ColumnDef::new(pix::Column::Amount).unsigned().not_null())
                    .primary_key(
                        Index::create()
                            .col(pix::Column::UserId)
                            .col(pix::Column::Date),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(user::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(student::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(refreshed_users::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(refresh_log::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(pix::Entity).to_owned())
            .await?;
        Ok(())
    }
}
