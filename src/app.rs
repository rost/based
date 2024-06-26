use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use serde_json::{json, Value};

use crate::state::AppState;

mod collection_repo;
mod user_repo;

pub fn router() -> Router<AppState> {
    let users = Router::new()
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user).get(get_users));

    let collections = Router::new()
        .route(
            "/collections",
            get(list_collections).post(create_collection),
        )
        .route(
            "/collections/:collection",
            get(view_collection)
                .patch(update_collection)
                .delete(delete_collection),
        )
        .route(
            "/collections/:collection/records",
            get(list_collection_records).post(create_collection_record),
        )
        .route(
            "/collections/:collection/records/:id",
            get(view_collection_record)
                .patch(update_collection_record)
                .delete(delete_collection_record),
        );

    Router::new()
        .route("/_status", get(status))
        .nest("/api", Router::new().merge(users).merge(collections))
}

async fn status() -> impl IntoResponse {
    let v: Value = json!({"status":"ok"});
    axum::Json(v)
}

// users
async fn create_user(
    State(state): State<AppState>,
    axum::Json(payload): axum::Json<user_repo::User>,
) -> impl IntoResponse {
    match user_repo::create_user(&state.db, &payload).await {
        Ok(user) => (StatusCode::CREATED, axum::Json(json!(user))),
        Err(err) => handle_error(err),
    }
}

async fn get_user(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    match user_repo::get_user(&state.db, id).await {
        Ok(user) => (StatusCode::OK, axum::Json(json!(user))),
        Err(err) => handle_error(err),
    }
}

async fn get_users(State(state): State<AppState>) -> impl IntoResponse {
    match user_repo::get_users(&state.db).await {
        Ok(users) => (StatusCode::OK, axum::Json(json!(users))),
        Err(err) => handle_error(err),
    }
}

// collections
async fn list_collections(State(state): State<AppState>) -> impl IntoResponse {
    match collection_repo::list_collections(&state.db).await {
        Ok(collections) => (StatusCode::OK, axum::Json(json!(collections))),
        Err(err) => handle_error(err),
    }
}

async fn create_collection(
    State(state): State<AppState>,
    axum::Json(payload): axum::Json<collection_repo::Collection>,
) -> impl IntoResponse {
    match collection_repo::create_collection(&state.db, payload).await {
        Ok(collection) => (StatusCode::CREATED, axum::Json(json!(collection))),
        Err(err) => handle_error(err),
    }
}

async fn view_collection(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match collection_repo::view_collection(&state.db, name).await {
        Ok(collection) => (StatusCode::OK, axum::Json(json!(collection))),
        Err(err) => handle_error(err),
    }
}

async fn update_collection(
    State(state): State<AppState>,
    Path(name): Path<String>,
    axum::Json(payload): axum::Json<collection_repo::Collection>,
) -> impl IntoResponse {
    // unimplemented!("update collection")
    match collection_repo::update_collection(&state.db, name, payload).await {
        Ok(collection) => (StatusCode::OK, axum::Json(json!(collection))),
        Err(err) => handle_error(err),
    }
}

async fn delete_collection(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    // unimplemented!("delete collection")
    match collection_repo::delete_collection(&state.db, name).await {
        Ok(collection) => (StatusCode::OK, axum::Json(json!(collection))),
        Err(err) => handle_error(err),
    }
}

// collection records
async fn list_collection_records(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match collection_repo::list_collection_records(&state.db, name).await {
        Ok(collection) => (StatusCode::OK, axum::Json(json!(collection))),
        Err(err) => handle_error(err),
    }
}

async fn create_collection_record(
    State(state): State<AppState>,
    Path(name): Path<String>,
    axum::Json(payload): axum::Json<collection_repo::CollectionEntry>,
) -> impl IntoResponse {
    match collection_repo::create_collection_record(&state.db, name, payload).await {
        Ok(entry) => (StatusCode::CREATED, axum::Json(json!(entry))),
        Err(err) => handle_error(err),
    }
}

async fn view_collection_record(
    State(state): State<AppState>,
    Path((name, id)): Path<(String, i64)>,
) -> impl IntoResponse {
    match collection_repo::view_collection_record(&state.db, name, id).await {
        Ok(entry) => (StatusCode::OK, axum::Json(json!(entry))),
        Err(err) => handle_error(err),
    }
}

async fn update_collection_record(
    State(state): State<AppState>,
    Path((name, id)): Path<(String, i64)>,
    axum::Json(payload): axum::Json<collection_repo::CollectionEntry>,
) -> impl IntoResponse {
    match collection_repo::update_collection_record(&state.db, name, id, payload).await {
        Ok(entry) => (StatusCode::OK, axum::Json(json!(entry))),
        Err(err) => handle_error(err),
    }
}

async fn delete_collection_record(
    State(state): State<AppState>,
    Path((name, id)): Path<(String, i64)>,
) -> impl IntoResponse {
    match collection_repo::delete_collection_record(&state.db, name, id).await {
        Ok(entry) => (StatusCode::OK, axum::Json(json!(entry))),
        Err(err) => handle_error(err),
    }
}

// utils
fn handle_error(err: sqlx::error::Error) -> (StatusCode, axum::Json<serde_json::Value>) {
    let msg = axum::Json(json!({ "error": format!("{}", &err) }));
    (StatusCode::INTERNAL_SERVER_ERROR, msg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt; // for `collect`
    use serde_json::{json, Value};
    use tokio::net::TcpListener;
    use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

    #[tokio::test]
    async fn unit_check_status() {
        let state = AppState::new().await.unwrap();
        let app = router().with_state(state);

        let response = app
            .oneshot(Request::builder().uri("/_status").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({"status": "ok"}));
    }

    #[tokio::test]
    async fn integration_check_status() {
        let state = AppState::new().await.unwrap();
        let app = router().with_state(state);

        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build_http();

        let response = client
            .request(
                Request::builder()
                    .uri(format!("http://{addr}/_status"))
                    .header("Host", "localhost")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({"status": "ok"}));
    }

    #[tokio::test]
    async fn create_user() {
        let state = AppState::new().await.unwrap();
        let app = router().with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/users")
                    .method("POST")
                    .header("Content-Type", "application/json")
                    .body(Body::from(r#"{"name":"test"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({"id": 1, "name": "test"}));
    }
}
