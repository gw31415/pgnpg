//! 日毎・ユーザー毎のPIX取得数を管理するテーブル

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "pix")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub date: Date,
    #[sea_orm(primary_key)]
    pub user_id: String,

    pub amount: u32,
***REMOVED***

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
***REMOVED***]
    User,
***REMOVED***

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
***REMOVED***
***REMOVED***

impl ActiveModelBehavior for ActiveModel {***REMOVED***
