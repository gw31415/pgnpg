//! RefreshされたUserを記録するテーブル: ulidが1回のリフレッシュに対応する。
//! リフレッシュ時にどのユーザがリフレッシュされたかを記録する

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "refreshed_users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub ulid: String,
    #[sea_orm(primary_key)]
    pub user_id: String,
}

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl ActiveModelBehavior for ActiveModel {}
