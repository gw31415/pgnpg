use std::{fmt::Display, str::FromStr};

use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::Serialize;

/// 学生レベル
#[derive(PartialEq, Debug, Clone, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum Level {
    #[sea_orm(string_value = "newbie")]
    Newbie,
    #[sea_orm(string_value = "assistant")]
    Assistant,
    #[sea_orm(string_value = "normal")]
    Normal,
    #[sea_orm(string_value = "lead")]
    Lead,
}

impl FromStr for Level {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "新人" => Ok(Level::Newbie),
            "アシスタント" => Ok(Level::Assistant),
            "ノーマル" => Ok(Level::Normal),
            "リード" => Ok(Level::Lead),
            _ => Err("invalid value".to_string()),
        }
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Level::Newbie => "新人",
                Level::Assistant => "アシスタント",
                Level::Normal => "ノーマル",
                Level::Lead => "リード",
            }
        )
    }
}
