//! RefreshLogとUserの中間テーブル: そのリフレッシュ時にどのユーザがリフレッシュされたかを記録する

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "refreshed_users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: String,
    #[sea_orm(primary_key)]
    pub refresh_log_id: i64,
}

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::refresh_log::Entity",
        from = "Column::RefreshLogId",
        to = "super::refresh_log::Column::Id"
    )]
    RefreshLog,
}

impl ActiveModelBehavior for ActiveModel {}
