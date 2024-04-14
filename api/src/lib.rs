mod usecase;

use std::{collections::HashSet, path::PathBuf***REMOVED***

use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router***REMOVED***
use reqwest::header;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use tower_http::services::ServeDir;
use usecase::profile;

use crate::usecase::get_last_updated_at;

const DAYS_COUNT: i64 = 28;

static RUNNING_REFRESH: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// charset=utf-8 に対応したJSONレスポンスを生成する
fn json(content: impl Serialize) -> impl IntoResponse {
    (
        StatusCode::OK,
***REMOVED***(header::CONTENT_TYPE, "application/json; charset=utf-8")],
        serde_json::to_string(&content).unwrap(),
***REMOVED***
***REMOVED***

/// Spawnされる更新処理タスク
async fn refresh(db: &DatabaseConnection) {
    if RUNNING_REFRESH.load(std::sync::atomic::Ordering::Relaxed) {
        return;
***REMOVED*** else {
        RUNNING_REFRESH.store(true, std::sync::atomic::Ordering::Relaxed);
***REMOVED***

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
    ***REMOVED***

        // 更新されているユーザの数を数える
        active_users_pre = usecase::active_users(db).await.unwrap();

        last_datetime
            .date_naive()
            .succ_opt()
            .unwrap()
            .min(end - chrono::Duration::days(1))
***REMOVED*** else {
        end - chrono::Duration::days(DAYS_COUNT - 1)
    ***REMOVED***

    // データを取得
    let mut records = usecase::fetch(start, end).await.unwrap();

    // 十分性の確認(新しいユーザがいた場合は再度取得)
    if let Some(active_users_pre) = active_users_pre {
        let active_users_post: HashSet<_> =
            records.iter().map(|r| r.wallet_address.clone()).collect();
        if active_users_pre
***REMOVED***
            .map(|u| u.id)
            .collect::<HashSet<_>>()
            != active_users_post
        {
            records = usecase::fetch(end - chrono::Duration::days(DAYS_COUNT - 1), end)
        ***REMOVED***
            ***REMOVED***
    ***REMOVED***
***REMOVED***

    usecase::insert(db, now, records).await.unwrap();

    RUNNING_REFRESH.store(false, std::sync::atomic::Ordering::Relaxed);
***REMOVED***

pub struct RunServerConfig {
    pub db: DatabaseConnection,
    pub profile_static_dir: PathBuf,
***REMOVED***

/// Start the server
pub async fn run_server(
    RunServerConfig {
        db,
        profile_static_dir,
***REMOVED***: RunServerConfig,
) {
    static NOT_FOUND: (StatusCode, &str) = (StatusCode::NOT_FOUND, "Not found");

    let active_users = get({
        let db = db.clone();
        || async move {
            let users = usecase::active_users(&db).await.unwrap();
            if let Some(users) = users {
                Ok(json(users))
        ***REMOVED*** else {
                Err(NOT_FOUND)
        ***REMOVED***
    ***REMOVED***
***REMOVED***);
    let profile = get({
        let db = db.clone();
        |Path(pgrit_id): Path<String>| async move {
            let now = chrono::Utc::now();
            match profile(&db, now, &pgrit_id).await {
                Ok(Some(profile)) => Ok(json(profile)),
                Ok(None) => Err((StatusCode::NOT_FOUND, "No data")),
                Err(e) => {
                    eprintln!("{:?***REMOVED***", e);
                    Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))
            ***REMOVED***
        ***REMOVED***
    ***REMOVED***
***REMOVED***);
    let refresh = get({
        let db = db.clone();
        || async move {
            if RUNNING_REFRESH.load(std::sync::atomic::Ordering::Relaxed) {
                (StatusCode::TOO_MANY_REQUESTS, "Already running")
        ***REMOVED*** else {
                tokio::spawn(async move {
                    refresh(&db.clone()).await;
            ***REMOVED***);
                (StatusCode::OK, "Refresh started")
        ***REMOVED***
    ***REMOVED***
***REMOVED***);
    let health_check = get("OK");

    let app = Router::new()
        .fallback(NOT_FOUND)
        .nest(
            "/api/",
            Router::new()
                .route("/", health_check.clone())
                .route("/actives.json", active_users)
                .route("/profile/pgrit/:pgrit_id/data.json", profile.clone()) // <- 暫定, 本当は /profile/{pgrit_id***REMOVED***.json にしたい
                .route("/refresh/", refresh),
    ***REMOVED***
        .nest(
            "/profile/:pgrid_id/",
            Router::new()
                .route("/data.json", profile)
                .nest_service("/", ServeDir::new(profile_static_dir)),
    ***REMOVED***;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3232").await.unwrap();
    axum::serve(listener, app).await.unwrap();
***REMOVED***
