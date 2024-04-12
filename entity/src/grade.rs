use std::{fmt::Display, str::FromStr***REMOVED***

use crate::degree::Degree;

/// 大学での学年
#[derive(Debug, Clone)]
pub struct Grade {
    /// 取得を目指している学位
    pub degree_step: Degree,
    /// 何年生か
    pub nth: u8,
***REMOVED***

impl Display for Grade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{***REMOVED***:{***REMOVED***", self.degree_step, self.nth)
***REMOVED***
***REMOVED***

impl FromStr for Grade {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let degree_step = Degree::from_str(&s[0..1]).map_err(|_| "invalid degree step")?;
        let grade = s[1..].parse().map_err(|_| "invalid grade".to_string())?;
        Ok(Grade {
            degree_step,
            nth: grade,
    ***REMOVED***)
***REMOVED***
***REMOVED***
