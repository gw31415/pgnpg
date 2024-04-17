mod usecase;

use std::{collections::HashSet, path::PathBuf, sync::Arc};

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{any, get},
    Router,
};
use reqwest::header;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
};
use usecase::profile;

use crate::usecase::get_last_updated_at;

const DAYS_COUNT: i64 = 28;

static RUNNING_REFRESH: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// charset=utf-8 に対応したJSONレスポンスを生成する
fn json(content: impl Serialize) -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
        serde_json::to_string(&content).unwrap(),
    )
}

/// Spawnされる更新処理タスク
async fn refresh(db: &DatabaseConnection, fetch_url: &str) {
    if RUNNING_REFRESH.load(std::sync::atomic::Ordering::Relaxed) {
        return;
    } else {
        RUNNING_REFRESH.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    let now = chrono::Utc::now();

    let mut active_users_pre: Option<_> = None;

    // 最初に最低限必要な日付を取得
    let end = now.date_naive();
    // 最低一日は取得
    let start = if let Some(last_datetime) = get_last_updated_at(db).await.unwrap() {
        // 30分以上の間隔がない場合は中止
        if now - last_datetime < chrono::Duration::minutes(30) {
            RUNNING_REFRESH.store(false, std::sync::atomic::Ordering::Relaxed);
            return;
        }

        // 更新されているユーザの数を数える
        active_users_pre = usecase::active_users(db).await.unwrap();

        last_datetime
            .date_naive()
            .succ_opt()
            .unwrap()
            .min(end - chrono::Duration::days(1))
    } else {
        end - chrono::Duration::days(DAYS_COUNT - 1)
    };

    // データを取得
    let mut records = usecase::fetch(fetch_url, start, end).await.unwrap();

    // 十分性の確認(新しいユーザがいた場合は再度取得)
    if let Some(active_users_pre) = active_users_pre {
        let active_users_post: HashSet<_> =
            records.iter().map(|r| r.wallet_address.clone()).collect();
        if active_users_pre
            .into_iter()
            .map(|u| u.id)
            .collect::<HashSet<_>>()
            != active_users_post
        {
            records = usecase::fetch(fetch_url, end - chrono::Duration::days(DAYS_COUNT - 1), end)
                .await
                .unwrap();
        }
    }

    usecase::insert(db, now, records).await.unwrap();

    RUNNING_REFRESH.store(false, std::sync::atomic::Ordering::Relaxed);
}

pub struct RunServerConfig {
    pub db: DatabaseConnection,
    pub static_dir: PathBuf,
    pub fetch_url: Arc<str>,
}

/// Start the server
pub async fn run_server(
    RunServerConfig {
        db,
        static_dir,
        fetch_url,
    }: RunServerConfig,
) {
    static NOT_FOUND: (StatusCode, &str) = (StatusCode::NOT_FOUND, "Not found");
    static INTERNAL_SERVER_ERROR: (StatusCode, &str) =
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error");

    let active_users = get({
        let db = db.clone();
        || async move {
            let users = usecase::active_users(&db).await.unwrap();
            if let Some(users) = users {
                Ok(json(users))
            } else {
                Err(NOT_FOUND)
            }
        }
    });
    let profile = get({
        let db = db.clone();
        |Path(pgrit_id): Path<String>| async move {
            let now = chrono::Utc::now();
            match profile(&db, now, &pgrit_id).await {
                Ok(Some(profile)) => Ok(json(profile)),
                Ok(None) => Err(NOT_FOUND),
                Err(e) => {
                    eprintln!("{:?}", e);
                    Err(INTERNAL_SERVER_ERROR)
                }
            }
        }
    });
    let refresh = get({
        let db = db.clone();
        || async move {
            if RUNNING_REFRESH.load(std::sync::atomic::Ordering::Relaxed) {
                (StatusCode::TOO_MANY_REQUESTS, "Already running")
            } else {
                tokio::spawn(async move {
                    refresh(&db.clone(), &fetch_url.clone()).await;
                });
                (StatusCode::OK, "Refresh started")
            }
        }
    });
    let health_check = get("OK");

    let app = Router::new()
        .nest(
            "/api/",
            Router::new()
                .route("/", health_check.clone())
                .route("/actives.json", active_users)
                .route("/profile/pgrit/:pgrit_id/data.json", profile.clone()) // <- 暫定, 本当は /profile/{pgrit_id}.json にしたい
                .route("/refresh/", refresh),
        )
        .nest(
            "/profile/:pgrid_id/",
            Router::new()
                .route_service(
                    "/",
                    ServeFile::new(static_dir.join("profile/name/index.html")),
                )
                .route("/data.json", profile),
        )
        .nest_service(
            "/",
            ServeDir::new(static_dir).not_found_service(any(NOT_FOUND)),
        )
        .layer(CompressionLayer::new())
        .fallback(NOT_FOUND);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3232").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
