use actix_web::{get, post, web, Result};

use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use rust_actix_diesel_auth_scaffold::errors::AuthError;
use rust_actix_diesel_auth_scaffold::DbPool;

use crate::models::run_history::{CreateRunHistory, RunHistory};
use crate::schema::run_history::dsl::run_history as run_history_table;

const RUN_HISTORY: &'static str = "run_history";

#[derive(serde::Deserialize, serde::Serialize)]
struct RepoVecObj {
    models: Vec<RunHistory>,
}

/// Get RunHistory
#[utoipa::path(
    tag = RUN_HISTORY,
    responses(
        (status = 200, description = "RunHistory found in database"),
        (status = 404, description = "Not found")
    ),
)]
#[get("/run_history")]
pub async fn read_many(pool: web::Data<DbPool>) -> Result<web::Json<RepoVecObj>, AuthError> {
    let mut conn = pool.get()?;

    let models_vec: Vec<RunHistory> = run_history_table
        .select(RunHistory::as_select())
        .load(&mut conn)?;

    Ok(web::Json(RepoVecObj { models: models_vec }))
}

/// Upsert RunHistory
#[utoipa::path(
    tag = RUN_HISTORY,
    responses(
        (status = 200, description = "RunHistory created"),
        (status = 500, description = "Internal Server Error")
    ),
    security(("password"=[]))
)]
#[post("/run_history")]
pub async fn upsert(
    pool: web::Data<DbPool>,
    form: web::Json<CreateRunHistory>,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
) -> Result<web::Json<RunHistory>, AuthError> {
    let mut conn = pool.get()?;

    // 0. Check token username vs username in request
    if let Some((_username_s, _)) = credentials.token().split_once(":") {
        // 1. Upsert model
        let new_model_vals = form.into_inner();
        let model = diesel::insert_into(run_history_table)
            .values(&new_model_vals)
            .returning(RunHistory::as_returning())
            .get_result(&mut conn)?;
        return Ok(web::Json(model));
    }

    Err(AuthError::HttpError(500))
}

/// Get RunHistory by id
#[utoipa::path(
    tag = RUN_HISTORY,
    responses(
        (status = 200, description = "RunHistory found from database"),
        (status = 404, description = "Not found")
    ),
    params(
        ("id", description = "RunHistory id, like \"SamuelMarks@crawlcomply-backend\""),
    )
)]
#[get("/run_history/{id}")]
pub async fn read(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
) -> Result<web::Json<RunHistory>, AuthError> {
    let mut conn = pool.get()?;
    match id.split_once('@') {
        Some((owner, repo_name)) => Ok(web::Json(
            crate::schema::run_history::table
                .filter(
                    crate::schema::run_history::dsl::full_name
                        .eq(format!("{}/{}", owner, repo_name)),
                )
                .select(RunHistory::as_select())
                .first(&mut conn)?,
        )),
        None => Err(AuthError::HttpError(500)),
    }
}
