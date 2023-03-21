use std::net::SocketAddr;

use axum::{
    extract::{Extension, Path},
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod error;
use crate::error::AppError;

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL env variable");
    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // let cors = CorsLayer::new().allow_origin(Any);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("unable to connect to database");

    let app = Router::new()
        .route("/", get(find_all).post(create_employee))
        .route("/:id", get(find_emp_by_id))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn find_all(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let query_result = sqlx::query_as::<_, Employee>("SELECT * FROM employee")
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            dbg!(err);
            AppError::InternalServerError
        })?;

    match query_result {
        employees => Ok(Json(employees)),
        _ => Err(AppError::InternalServerError),
    }
}

async fn find_emp_by_id(
    Path(id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    let query_result = sqlx::query_as::<_, Employee>("SELECT * FROM employee where id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|err| {
            dbg!(err);
            AppError::InternalServerError
        })?;

    match query_result {
        Some(employee) => Ok(Json(employee)),
        None => Err(AppError::EmployeeDoesNotExist),
    }
}

async fn create_employee(
    Json(payload): Json<CreateEmployee>,
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    let query_result = sqlx::query("INSERT INTO employee (name) values ($1)")
        .bind(&payload.name)
        .execute(&pool)
        .await
        .map_err(|err| {
            dbg!(err);
            AppError::InternalServerError
        })?;

    match query_result {
        r if r.rows_affected() > 0 => {
            let employee = sqlx::query_as::<_, Employee>(
                "SELECT * FROM employee WHERE id = (SELECT MAX(id) FROM employee)",
            )
            .fetch_one(&pool)
            .await
            .map_err(|err| {
                dbg!(err);
                AppError::InternalServerError
            })?;

            Ok(Json(employee))
        }
        _ => Err(AppError::InternalServerError),
    }
}

#[derive(Deserialize)]
struct CreateEmployee {
    name: String,
}

#[derive(Serialize, sqlx::FromRow)]
struct Employee {
    id: i32,
    name: String,
}
