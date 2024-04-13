use chrono::{DateTime, NaiveDate***REMOVED***
use serde::{Deserialize, Deserializer***REMOVED***
use serde_with::{serde_as, DisplayFromStr, NoneAsEmptyString***REMOVED***
***REMOVED***

use crate::{grade::Grade, level::Level, sex::Sex***REMOVED***

/// APIから取得したレコード
#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct Record {
    /// Pgrit ID
    pub id: String,

    /// Ethereumのウォレットアドレス
    #[serde(rename = "walletAddress")]
    pub wallet_address: String,

    /// 学年と遂行中学位
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub grade: Option<Grade>,

    /// 受講コース
    pub course: Option<String>,

    /// 生徒レベル; 新人, アシスタント, ...
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub level: Option<Level>,

    /// 性別
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub sex: Option<Sex>,

    /// 加入日
    #[serde(
        default,
        rename = "joinDate",
        deserialize_with = "deserialize_optional_naive_date"
***REMOVED***]
    pub join_date: Option<NaiveDate>,

    /// 加入月
    #[serde(rename = "joinMonth")]
    pub join_month: Option<String>,

    /// オフィス
    pub office: Option<String>,

    /// メールアドレス
    pub email: Option<String>,

    /// 4nonomeメールアドレス
    #[serde(rename = "emailOf4nonome")]
    pub email_of_4nonome: Option<String>,

    /// 所属大学
    pub university: Option<String>,

    /// 専攻
    pub major: Option<String>,

    /// 脱退日
    #[serde(
        default,
        rename = "leaveDate",
        deserialize_with = "deserialize_optional_naive_date"
***REMOVED***]
    pub leave_date: Option<NaiveDate>,

    /// アクティブかどうか
    pub active: Option<bool>,

    /// Slack ID
    #[serde(rename = "slackId")]
    pub slack_id: Option<String>,

    /// Discord ID
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(default, rename = "discordId")]
    pub discord_id: Option<String>,

    /// 月間のPIX
    pub total: u32,

    /// 月間のPgritにおけるPIX
    #[serde(rename = "total_pgrit")]
    pub total_pgrit: u32,

    /// 月間のDawnにおけるPIX
    #[serde(rename = "total_dawn")]
    pub total_dawn: u32,

    /// 月間のその他のPIX
    #[serde(rename = "total_other")]
    pub total_other: u32,

    /// 日毎のPIXの内訳
    #[serde(flatten, deserialize_with = "deserialize_date_map")]
    pub daily_totals: HashMap<NaiveDate, u32>,
***REMOVED***

fn deserialize_optional_naive_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Result<Option<String>, _> = Option::deserialize(deserializer);
    match s? {
        Some(s) if !s.is_empty() => {
            let res = {
                let s = s.as_str();
                s.len() >= 10
                    && &s[4..5] == "-"
                    && &s[7..8] == "-"
                    && [&s[0..4], &s[5..7], &s[8..10]]
                        .iter()
                        .all(|s| s.chars().all(|c| c.is_ascii_digit()))
            ***REMOVED***
            Ok(Some(
                if res {
                    NaiveDate::parse_from_str(&s[0..10], "%Y-%m-%d")
            ***REMOVED*** else {
                    DateTime::parse_from_rfc3339(&s).map(|d| d.date_naive())
            ***REMOVED***
                .map_err(serde::de::Error::custom)?,
        ***REMOVED***)
    ***REMOVED***
        _ => Ok(None),
***REMOVED***
***REMOVED***

fn deserialize_date_map<'de, D>(deserializer: D) -> Result<HashMap<NaiveDate, u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let map: HashMap<String, u32> = HashMap::deserialize(deserializer)?;
    map.into_iter()
        .map(|(k, v)| {
            NaiveDate::parse_from_str(&k, "%Y-%m-%d")
                .map_err(serde::de::Error::custom)
                .map(|date| (date, v))
    ***REMOVED***)
        .collect()
***REMOVED***