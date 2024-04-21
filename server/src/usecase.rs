use std::collections::HashMap;

use anyhow::Context;
use axum::http::HeaderValue;
use chrono::NaiveDate;
use entity::{
    error::Error,
    grade::Grade,
    mstdn_token,
    pgn_level::{self, PgnLevel},
    pix,
    record::Record,
    refreshed_users, student, user,
    user_profile::{PgnInfo, UserProfile},
};
use itertools::Itertools;
use reqwest::{header, Url};
use sea_orm::{
    prelude::DateTimeUtc, sea_query::OnConflict, ActiveValue, ColumnTrait, DatabaseConnection,
    EntityTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, TransactionTrait,
};
use serde_json::Value;
use ulid::Ulid;
use valq::query_value;

const CHUNK_SIZE: usize = 512;

pub async fn fetch(url: &str, start: NaiveDate, end: NaiveDate) -> Result<Vec<Record>, Error> {
    if start >= end {
        return Err(Error::InvalidDateRange);
    }
    let url = Url::parse_with_params(
        url,
        [
            ("start", &start.format("%Y-%m-%d").to_string()),
            ("end", &end.format("%Y-%m-%d").to_string()),
        ],
    )
    .unwrap();

    let response = reqwest::get(url).await?.text().await?;
    let records = serde_json::from_str(&response)?;
    Ok(records)
}

pub async fn profile(
    db: &DatabaseConnection,
    now: DateTimeUtc,
    pgrit_id: &str,
) -> Result<Option<UserProfile>, Error> {
    let joined_table = pix::Entity::find()
        .select_also(user::Entity)
        .join(sea_orm::JoinType::InnerJoin, pix::Relation::User.def())
        .filter(user::Column::PgritId.eq(pgrit_id))
        .order_by(pix::Column::Date, sea_orm::Order::Desc)
        .limit(28)
        .all(db)
        .await
        .unwrap();

    // 全て同じUser
    let user: user::Model = {
        let Some(m) = joined_table.first() else {
            // 指定されたPgrit IDのユーザが存在しない
            return Ok(None);
        };
        m.1.clone().unwrap()
    };
    let student: Option<student::Model> = student::Entity::find_by_id(&user.id.clone())
        .one(db)
        .await?;

    let pgn: PgnInfo = {
        let daily: HashMap<NaiveDate, u32> = joined_table
            .into_iter()
            .map(|(pix, _)| (pix.date, pix.amount))
            .collect();

        let last_month: u32 = daily.values().sum();
        let level = pgn_level::PgnLevel::from(last_month);

        let base_pix = level.min_pix();
        let on_level = last_month - base_pix;

        let mut level_length = None;
        let mut progress = None;
        let mut target = None;
        let mut behind_next = None;
        if level != PgnLevel::GrandMaster {
            let t = (level + 1).min_pix();
            let ll = t - base_pix;
            target = Some(t);
            level_length = Some(ll);

            progress = 'pgs: {
                if level == PgnLevel::GrandMaster {
                    break 'pgs None;
                }

                Some(on_level as f32 / ll as f32)
            };

            behind_next = Some(t - last_month);
        };
        PgnInfo {
            level,
            level_length,
            last_month,
            on_level,
            progress,
            target,
            behind_next,
            daily,
            updated_at: get_last_updated_at(db).await?.unwrap(),
        }
    };

    Ok(Some(UserProfile {
        user,
        student,
        created_at: now,
        pgn,
    }))
}

pub async fn active_users(db: &DatabaseConnection) -> Result<Option<Vec<user::Model>>, Error> {
    let Some(refresh_log_item) = refreshed_users::Entity::find()
        .column(refreshed_users::Column::Ulid)
        .order_by_desc(refreshed_users::Column::Ulid)
        .one(db)
        .await?
    else {
        return Ok(None);
    };

    let log_id = refresh_log_item.ulid;

    let users = user::Entity::find()
        .join(
            sea_orm::JoinType::LeftJoin,
            refreshed_users::Relation::User.def().rev(),
        )
        .filter(refreshed_users::Column::Ulid.eq(log_id))
        .all(db)
        .await?
        .into_iter()
        .collect_vec();

    Ok(Some(users))
}

pub async fn get_last_updated_at(db: &DatabaseConnection) -> Result<Option<DateTimeUtc>, Error> {
    let model = refreshed_users::Entity::find()
        .column(refreshed_users::Column::Ulid)
        .order_by_desc(refreshed_users::Column::Ulid)
        .one(db)
        .await?;
    let Some(model) = model else {
        return Ok(None);
    };
    let systemtime = Ulid::from_string(&model.ulid)
        .context("parse error")?
        .datetime();
    let datetime = DateTimeUtc::from(systemtime);
    Ok(Some(datetime))
}

fn create_student_activemodel(record: Record) -> Option<student::ActiveModel> {
    let Grade { degree_step, nth } = record.grade?;
    Some(student::ActiveModel {
        user_id: ActiveValue::Set(record.wallet_address.clone()),
        degree_step: ActiveValue::Set(degree_step),
        grade: ActiveValue::Set(nth),
        course: ActiveValue::Set(record.course?),
        level: ActiveValue::Set(record.level?),
        sex: ActiveValue::Set(record.sex?),
        join_date: ActiveValue::Set(record.join_date?),
        office: ActiveValue::Set(record.office?),
        email: ActiveValue::Set(record.email?),
        email_of_4nonome: ActiveValue::Set(record.email_of_4nonome?),
        university: ActiveValue::Set(record.university?),
        major: ActiveValue::Set(record.major?),
        leave_date: ActiveValue::Set(record.leave_date),
        active: ActiveValue::Set(record.active?),
        slack_id: ActiveValue::Set(record.slack_id?),
        discord_id: ActiveValue::Set(record.discord_id),
    })
}

#[derive(Debug, thiserror::Error)]
pub enum SignupError {
    #[error("User not found")]
    UserNotFound,
    #[error(transparent)]
    InternalServerError(#[from] anyhow::Error),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    ConvertError(#[from] std::convert::Infallible),
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    SeaOrmError(#[from] sea_orm::error::DbErr),
}

pub async fn signup(
    db: &DatabaseConnection,
    pgrit_origin: &str,
    callback_url_ours: &str,
    account_verify_url: &str,
    pgrit_client_key: &str,
    pgrit_client_secret: &str,
    code: &str,
) -> Result<String, SignupError> {
    // auhtorization codeを使ってtokenを取得
    let data = reqwest::Client::new()
        .post(&format!("{}/oauth/token", pgrit_origin))
        .form(&[
            ("grant_type", "authorization_code"),
            ("redirect_uri", callback_url_ours),
            ("client_id", pgrit_client_key),
            ("client_secret", pgrit_client_secret),
            ("code", code),
            ("scope", "read:accounts"),
        ])
        .send()
        .await?
        .text()
        .await?;

    let json: Value = serde_json::from_str(&data)?;
    let token = query_value!(json.access_token -> str).context("token not found")?;

    // tokenを使ってユーザ情報を取得
    let client = reqwest::Client::builder()
        .default_headers({
            let mut headers = header::HeaderMap::new();
            headers.insert(
                header::AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
            );
            headers
        })
        .build()?;

    // retrieve response
    let res = client.get(account_verify_url).send().await?;

    // get response body
    let data: Value = serde_json::from_str(&res.text().await?)?;

    // parse response body to get username
    let username = query_value!(data.username -> str).context("username not found")?;

    let id = {
        let Ok(Some(u)) = user::Entity::find()
            .column(user::Column::Id)
            .filter(user::Column::PgritId.eq(username))
            .one(db)
            .await
        else {
            return Err(SignupError::UserNotFound);
        };
        u.id
    };

    mstdn_token::Entity::insert(mstdn_token::ActiveModel {
        user_id: ActiveValue::Set(id.clone()),
        access_token: ActiveValue::Set(token.to_string()),
        authorization_code: ActiveValue::Set(code.to_string()),
    })
    .on_conflict(
        OnConflict::column(mstdn_token::Column::UserId)
            .update_columns([
                mstdn_token::Column::AccessToken,
                mstdn_token::Column::AuthorizationCode,
            ])
            .to_owned(),
    )
    .exec(db)
    .await?;
    Ok(username.to_string())
}

pub async fn insert(
    db: &DatabaseConnection,
    now: DateTimeUtc,
    records: impl IntoIterator<Item = Record>,
) -> Result<(), Error> {
    let log_id = Ulid::from_datetime(now.into()).to_string();
    let mut users = Vec::new();
    let mut refreshed_user_item = Vec::new();
    let mut pixes = Vec::new();
    let mut students = Vec::new();
    for record in records {
        if let Some(student) = create_student_activemodel(record.clone()) {
            students.push(student);
        }
        let user = user::ActiveModel {
            id: ActiveValue::Set(record.wallet_address.clone()),
            pgrit_id: ActiveValue::Set(record.id.clone()),
        };
        let tb = refreshed_users::ActiveModel {
            user_id: ActiveValue::Set(record.wallet_address.clone()),
            ulid: ActiveValue::Set(log_id.clone()),
        };
        let pix = record
            .daily_totals
            .into_iter()
            .map(|(date, amount)| pix::ActiveModel {
                user_id: ActiveValue::Set(record.wallet_address.clone()),
                date: ActiveValue::Set(date),
                amount: ActiveValue::Set(amount),
            });
        users.push(user);
        refreshed_user_item.push(tb);
        pixes.extend(pix);
    }

    db.transaction(|db| {
        Box::pin(async move {
            user::Entity::insert_many(users)
                .on_conflict(
                    OnConflict::column(user::Column::Id)
                        .update_column(user::Column::PgritId)
                        .to_owned(),
                )
                .do_nothing()
                .exec(db)
                .await?;
            student::Entity::insert_many(students)
                .on_conflict(
                    OnConflict::column(student::Column::UserId)
                        .update_columns([
                            student::Column::DegreeStep,
                            student::Column::Grade,
                            student::Column::Course,
                            student::Column::Level,
                            student::Column::Sex,
                            student::Column::JoinDate,
                            student::Column::Office,
                            student::Column::Email,
                            student::Column::EmailOf4nonome,
                            student::Column::University,
                            student::Column::Major,
                            student::Column::LeaveDate,
                            student::Column::Active,
                            student::Column::SlackId,
                            student::Column::DiscordId,
                        ])
                        .to_owned(),
                )
                .do_nothing()
                .exec(db)
                .await?;
            refreshed_users::Entity::insert_many(refreshed_user_item)
                .on_conflict(
                    OnConflict::columns([
                        refreshed_users::Column::UserId,
                        refreshed_users::Column::Ulid,
                    ])
                    .do_nothing()
                    .to_owned(),
                )
                .do_nothing()
                .exec(db)
                .await?;

            // 多すぎてトークン制限に引っかかるので分割
            while !pixes.is_empty() {
                let mut items = Vec::new();
                for _ in 0..CHUNK_SIZE {
                    if let Some(item) = pixes.pop() {
                        items.push(item);
                    } else {
                        break;
                    }
                }
                pix::Entity::insert_many(items)
                    .on_conflict(
                        OnConflict::columns([pix::Column::UserId, pix::Column::Date])
                            .update_column(pix::Column::Amount)
                            .to_owned(),
                    )
                    .do_nothing()
                    .exec(db)
                    .await?;
            }
            Ok::<(), Error>(())
        })
    })
    .await
    .context("Failed to insert records into the database")?;
    Ok(())
}
