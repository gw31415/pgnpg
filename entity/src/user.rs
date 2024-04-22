//! ユーザー情報を管理するモデル

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    /// Ethereumのウォレットアドレス
    pub id: String,
    /// PGrit ID
    pub pgrit_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// 生徒情報, 1対1 (or 1対0)
    #[sea_orm(has_one = "super::student::Entity")]
    Student,

    /// Mastodonトークン, 1対1 (or 1対0)
    #[sea_orm(has_one = "super::mstdn_token::Entity")]
    MstdnToken,
}

impl Related<super::student::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
    }
}

impl Related<super::mstdn_token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MstdnToken.def()
    }
}

impl Related<super::refreshed_users::Entity> for Entity {
    fn to() -> RelationDef {
        super::refreshed_users::Relation::User.def().rev()
    }

    fn via() -> Option<RelationDef> {
        Some(super::refreshed_users::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
