mod usecase;

use entity::user;
use std::{collections::HashSet, path::PathBuf, sync::Arc};
use time::Duration;

use axum::{
    extract::{Path, Query, Request},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Redirect, Response},
    routing::{any, get},
    Router,
};
use chrono::Local;
use reqwest::{header, Url};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
};
use tower_sessions::{Expiry, Session, SessionManagerLayer};
use tower_sessions_sqlx_store::SqliteStore;
use usecase::profile;

use crate::usecase::{get_last_updated_at, signup};

const DAYS_COUNT: i64 = 30;

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
    let end = now.with_timezone(&Local).date_naive();
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
            .with_timezone(&Local)
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

    usecase::insert(db, chrono::Utc::now(), records)
        .await
        .unwrap();

    RUNNING_REFRESH.store(false, std::sync::atomic::Ordering::Relaxed);
}

#[derive(Deserialize)]
pub struct Config {
    pub static_dir: PathBuf,
    pub fetch_url: Arc<str>,
    pub origin: String,
    pub pgrit_origin: String,
    pub pgrit_client_key: Arc<str>,
    pub pgrit_client_secret: Arc<str>,
}

#[derive(serde::Deserialize)]
struct PgritOauthQuery {
    code: String,
}

/// Start the server
pub async fn run(
    db: DatabaseConnection,
    Config {
        static_dir,
        fetch_url,
        origin,
        pgrit_origin,
        pgrit_client_key,
        pgrit_client_secret,
    }: Config,
) {
    const NOT_FOUND: (StatusCode, &str) = (StatusCode::NOT_FOUND, "Not found");
    const INTERNAL_SERVER_ERROR: (StatusCode, &str) =
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error");
    const UNAUTHORIZED: (StatusCode, &str) = (StatusCode::UNAUTHORIZED, "Unauthorized");
    const USER_KEY: &str = "user";

    let callback_url_ours: Arc<str> = format!("{}/api/auth/pgrit/confirm/", origin).into();

    let pgrit_auth_url: Arc<str> = Url::parse_with_params(
        &format!("{}/oauth/authorize", pgrit_origin),
        &[
            ("client_id", pgrit_client_key.as_ref()),
            ("response_type", "code"),
            ("redirect_uri", callback_url_ours.as_ref()),
            ("scope", "read:accounts"),
        ],
    )
    .unwrap()
    .as_str()
    .into();

    let account_verify_url: Arc<str> = Url::parse(&format!(
        "{}/api/v1/accounts/verify_credentials",
        pgrit_origin
    ))
    .unwrap()
    .as_str()
    .into();

    let session_layer = SessionManagerLayer::new({
        let store = SqliteStore::new(db.get_sqlite_connection_pool().clone());
        store.migrate().await.unwrap();
        store
    })
    .with_expiry(Expiry::OnInactivity(Duration::days(7)))
    .with_secure(origin.starts_with("https://"));

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
    let me = get({
        |session: Session| async move { json(session.get::<user::Model>(USER_KEY).await.ok().flatten()) }
    });
    let health_check = get("OK");

    let pgrit_oauth_router = {
        const INITIATED_KEY: &str = "initiated";
        Router::new()
            .route(
                "/initiate/",
                get({
                    let pgrit_login_url = pgrit_auth_url.clone();
                    |session: Session| async move {
                        if let Ok(Some(user)) = session.get::<user::Model>(USER_KEY).await {
                            return Redirect::permanent(&format!("/profile/{}/", user.pgrit_id))
                                .into_response();
                        }
                        if session.insert(INITIATED_KEY, true).await.is_err() {
                            INTERNAL_SERVER_ERROR.into_response()
                        } else {
                            Redirect::permanent(&pgrit_login_url).into_response()
                        }
                    }
                }),
            )
            .route(
                "/confirm/",
                get({
                    let db = db.clone();
                    |Query(query): Query<PgritOauthQuery>, session: Session| async move {
                        if session
                            .get::<bool>(INITIATED_KEY)
                            .await
                            .unwrap_or(Some(false))
                            .unwrap_or(false)
                        {
                            return UNAUTHORIZED.into_response();
                        }
                        session.remove::<bool>(INITIATED_KEY).await.unwrap();

                        let code = query.code;

                        // 一時ユーザが正常に認証された場合

                        match signup(
                            &db,
                            &pgrit_origin,
                            &callback_url_ours,
                            &account_verify_url,
                            &pgrit_client_key,
                            &pgrit_client_secret,
                            &code,
                        )
                        .await
                        {
                            Ok(user) => {
                                if session.insert(USER_KEY, user.clone()).await.is_err() {
                                    return UNAUTHORIZED.into_response();
                                }
                                Redirect::permanent("/").into_response()
                            }
                            Err(usecase::SignupError::UserNotFound) => {
                                (StatusCode::NOT_FOUND, "User not found").into_response()
                            }
                            Err(_) => INTERNAL_SERVER_ERROR.into_response(),
                        }
                    }
                }),
            )
    };

    let block_unauthorized = {
        async fn func(
            session: Session,
            req: Request,
            next: Next,
        ) -> Result<Response, (StatusCode, impl IntoResponse)> {
            if let Ok(Some(_)) = session.get::<user::Model>(USER_KEY).await {
                Ok(next.run(req).await)
            } else {
                Err(UNAUTHORIZED)
            }
        }
        middleware::from_fn(func)
    };

    let app = Router::new()
        .nest(
            "/api/",
            Router::new()
                .route("/", health_check)
                .route("/actives.json", active_users)
                .route("/profile/pgrit/:pgrit_id/data.json", profile.clone()) // <- 暫定, 本当は /profile/{pgrit_id}.json にしたい
                .route(
                    "/auth/logout/",
                    get({
                        |session: Session| async move {
                            session.remove::<user::Model>(USER_KEY).await.unwrap();
                            Redirect::permanent("/").into_response()
                        }
                    }),
                )
                .layer(block_unauthorized.clone())
                .route("/me.json", me)
                .route("/refresh/", refresh)
                .nest("/auth/pgrit/", pgrit_oauth_router),
        )
        .nest(
            "/profile/:pgrid_id/",
            Router::new()
                .route_service(
                    "/",
                    ServeFile::new(static_dir.join("profile/name/index.html")),
                )
                .route("/data.json", profile)
                .layer(block_unauthorized),
        )
        .nest_service(
            "/",
            ServeDir::new(static_dir).not_found_service(any(NOT_FOUND)),
        )
        .layer(CompressionLayer::new())
        .layer(session_layer)
        .fallback(NOT_FOUND);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3232").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
