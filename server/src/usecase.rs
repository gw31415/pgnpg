use std::collections::HashMap;

use anyhow::Context;
use chrono::NaiveDate;
use entity::{
    error::Error,
    grade::Grade,
    pgn_level::{self, PgnLevel},
    pix,
    record::Record,
    refresh_log, refreshed_users, student, user,
    user_profile::{PgnInfo, UserProfile},
};
use itertools::Itertools;
use reqwest::Url;
use sea_orm::{
    prelude::DateTimeUtc, sea_query::OnConflict, ActiveValue, ColumnTrait, DatabaseConnection,
    EntityTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, TransactionTrait,
};

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
    let Some(refresh_log_item) = refresh_log::Entity::find()
        .order_by_desc(refresh_log::Column::UpdatedAt)
        .limit(1)
        .one(db)
        .await?
    else {
        return Ok(None);
    };

    let users = user::Entity::find()
        .find_with_related(refresh_log::Entity)
        .filter(refreshed_users::Column::RefreshLogId.eq(refresh_log_item.id))
        .all(db)
        .await?
        .into_iter()
        .map(|i| i.0)
        .collect_vec();

    Ok(Some(users))
}

pub async fn get_last_updated_at(db: &DatabaseConnection) -> Result<Option<DateTimeUtc>, Error> {
    let model = refresh_log::Entity::find()
        .select_only()
        .order_by_desc(refresh_log::Column::UpdatedAt)
        .columns([refresh_log::Column::Id, refresh_log::Column::UpdatedAt])
        .one(db)
        .await?;
    Ok(model.map(|m| m.updated_at))
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

pub async fn insert(
    db: &DatabaseConnection,
    now: DateTimeUtc,
    records: impl IntoIterator<Item = Record>,
) -> Result<(), Error> {
    let log = refresh_log::ActiveModel {
        updated_at: ActiveValue::Set(now),
        ..Default::default()
    };

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
            ..Default::default()
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
            let log_insert_res = refresh_log::Entity::insert(log).exec(db).await?;
            {
                let log_id = log_insert_res.last_insert_id;
                for item in refreshed_user_item.iter_mut() {
                    item.refresh_log_id = ActiveValue::Set(log_id);
                }
            }
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
                        refreshed_users::Column::RefreshLogId,
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
