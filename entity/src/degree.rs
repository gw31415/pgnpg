use std::{fmt::Display, str::FromStr***REMOVED***

***REMOVED***DeriveActiveEnum, EnumIter***REMOVED***
use serde::Serialize;

/// 遂行中の学位
#[derive(PartialEq, Debug, Clone, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum Degree {
    /// 学士
    #[sea_orm(string_value = "bachelor")]
    Bachelor,
    /// 修士
    #[sea_orm(string_value = "master")]
    Master,
    /// 博士
    #[sea_orm(string_value = "doctor")]
    Doctor,
***REMOVED***

impl FromStr for Degree {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "B" => Ok(Degree::Bachelor),
            "M" => Ok(Degree::Master),
            "D" => Ok(Degree::Doctor),
            _ => Err("invalid degree step".to_string()),
    ***REMOVED***
***REMOVED***
***REMOVED***

impl Display for Degree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{***REMOVED***",
            match self {
                Degree::Bachelor => "B",
                Degree::Master => "M",
                Degree::Doctor => "D",
        ***REMOVED***
    ***REMOVED***
***REMOVED***
***REMOVED***