//! 学生情報を表すモデル

use sea_orm::entity::prelude::*;
use serde::Serialize;

use crate::{degree::Degree, level::Level, sex::Sex};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "students")]
pub struct Model {
    #[sea_orm(primary_key)]
    /// Ethereumのウォレットアドレス
    pub user_id: String,
    /// 遂行中の学位
    pub degree_step: Degree,
    /// 学年
    pub grade: u8,
    /// 受講コース
    pub course: String,
    /// レベル
    pub level: Level,
    /// 性別
    pub sex: Sex,
    /// 参加日
    pub join_date: Date,
    /// オフィス
    pub office: String,
    /// メールアドレス
    pub email: String,
    /// 4nonomeメールアドレス
    pub email_of_4nonome: String,
    /// 大学
    pub university: String,
    /// 専攻
    pub major: String,
    /// 脱退日
    pub leave_date: Option<Date>,
    /// アクティブ
    pub active: bool,
    /// Slack ID
    pub slack_id: String,
    /// Discord ID
    pub discord_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::user::Entity")]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
