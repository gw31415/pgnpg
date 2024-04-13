***REMOVED***

use crate::pgn_level::PgnLevel;

use super::student::Model as Student;
use super::user::Model as User;
***REMOVED***
use sea_orm::prelude::DateTimeUtc;
use serde::Serialize;
use serde_with::serde_as;

/// ユーザプロフィール
#[derive(Debug, Clone, Serialize)]
pub struct UserProfile {
    /// ユーザ情報
    pub user: User,

    /// 学生情報
    pub student: Option<Student>,

    /// 作成日時
    pub created_at: DateTimeUtc,

    /// PGN情報
    pub pgn: PgnSubstract,
***REMOVED***

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct PgnSubstract {
    /// PIXデータの更新日時
    pub updated_at: DateTimeUtc,

    /// 1日ごとのPIX推移
    pub daily: HashMap<NaiveDate, u32>,

    /// 現在のPgnLevel
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub level: PgnLevel,

    /// 最近1ヶ月のPIX
    pub last_month: u32,

    /// 現在のレベルをベースにしたPIX
    pub on_level: u32,

    // グラマスはそれ以上のレベルがないのでNone
    /// 現在のPgnLevelのステップがPIXいくつ分か
    pub level_length: Option<u32>,

    /// 現在のPgnLevelでの進捗
    pub progress: Option<f32>,

    /// 次のレベルの月間総PIX
    pub target: Option<u32>,

    /// 次のレベルまでに必要な残りのPIX
    pub behind_next: Option<u32>,
***REMOVED***
