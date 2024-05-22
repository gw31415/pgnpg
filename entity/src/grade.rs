use std::{borrow::Cow, fmt::Display, str::FromStr};

use crate::degree::Degree;

/// 大学での学年
#[derive(Debug, Clone)]
pub struct Grade {
    /// 取得を目指している学位
    pub degree_step: Degree,
    /// 何年生か
    pub nth: u16,
}

impl Display for Grade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.degree_step, self.nth)
    }
}

fn split_alpha_numeric(s: &str) -> (&str, &str) {
    let split_index = s
        .chars()
        .position(|c| c.is_ascii_digit())
        .unwrap_or(s.len());

    let alpha = &s[..split_index];
    let numeric = &s[split_index..];
    (alpha, numeric)
}

impl FromStr for Grade {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut source = Cow::Borrowed(s);

        // 例外処理
        if s.starts_with('高') {
            let mut string = s.replace("高校", "");
            if string.ends_with("年生") {
                string = string.replace("年生", "");
            }
            match string.as_str() {
                "1" | "一" => string = "H1".to_string(),
                "2" | "二" => string = "H2".to_string(),
                "3" | "三" => string = "H3".to_string(),
                _ => return Err("invalid grade".to_string()),
            }
            source = Cow::Owned(string);
        }
        if s.starts_with("OB - ") && s.ends_with('卒') {
            let string = s.replace(" - ", "").replace('卒', "");
            source = Cow::Owned(string);
        }

        let (degree_step, grade) = split_alpha_numeric(&source);
        let degree_step = Degree::from_str(degree_step).map_err(|_| "invalid degree step")?;
        let grade = grade.parse().map_err(|_| "invalid grade".to_string())?;
        Ok(Grade {
            degree_step,
            nth: grade,
        })
    }
}
