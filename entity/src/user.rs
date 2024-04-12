//! ユーザー情報を管理するモデル

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    /// Ethereumのウォレットアドレス
    pub id: String,
    /// PGrit ID
    pub pgrit_id: String,
***REMOVED***

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// 生徒情報, 1対1 (or 1対0)
    #[sea_orm(
        belongs_to = "super::student::Entity",
        from = "Column::Id",
        to = "super::student::Column::UserId"
***REMOVED***]
    Student,
***REMOVED***

impl Related<super::refresh_log::Entity> for Entity {
    fn to() -> RelationDef {
        super::refreshed_users::Relation::RefreshLog.def()
***REMOVED***

    fn via() -> Option<RelationDef> {
        Some(super::refreshed_users::Relation::User.def().rev())
***REMOVED***
***REMOVED***

impl Related<super::student::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
***REMOVED***
***REMOVED***

impl ActiveModelBehavior for ActiveModel {***REMOVED***
