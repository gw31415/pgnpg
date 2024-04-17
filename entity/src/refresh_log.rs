//! リフレッシュ履歴

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "refresh_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::refreshed_users::Relation::User.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::refreshed_users::Relation::RefreshLog.def())
    }
}

impl ActiveModelBehavior for ActiveModel {}
