use std::{fmt::Display, str::FromStr***REMOVED***

***REMOVED***EnumIter, DeriveActiveEnum***REMOVED***
use serde::Serialize;

/// 性別
#[derive(PartialEq, Debug, Clone, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum Sex {
    #[sea_orm(string_value = "male")]
    Male,
    #[sea_orm(string_value = "female")]
    Female,
***REMOVED***

impl FromStr for Sex {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "男性" => Ok(Sex::Male),
            "女性" => Ok(Sex::Female),
            _ => Err("invalid value".to_string()),
    ***REMOVED***
***REMOVED***
***REMOVED***

impl Display for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{***REMOVED***",
            match self {
                Sex::Male => "男性",
                Sex::Female => "女性",
        ***REMOVED***
    ***REMOVED***
***REMOVED***
***REMOVED***
