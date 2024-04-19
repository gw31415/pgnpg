//! 学生情報を表すモデル

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "mstdn_token")]
pub struct Model {
    #[sea_orm(primary_key)]
    /// Ethereumのウォレットアドレス
    pub user_id: String,
    /// Authorization Code
    pub authorization_code: String,
    /// Mastodonトークン
    pub access_token: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
