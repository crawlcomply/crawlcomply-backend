use actix_web::{get, post, web, Result};

use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use rust_actix_diesel_auth_scaffold::errors::AuthError;
use rust_actix_diesel_auth_scaffold::DbPool;

use crate::models::repo_history::{CreateRepoHistory, RepoHistory};
use crate::schema::repo_history::dsl::repo_history as repo_history_table;

const REPO_HISTORY: &str = "repo_history";

#[derive(serde::Deserialize, serde::Serialize)]
struct RepoVecObj {
    models: Vec<RepoHistory>,
}

/// Get RepoHistory
#[utoipa::path(
    tag = REPO_HISTORY,
    responses(
        (status = 200, description = "RepoHistory found in database"),
        (status = 404, description = "Not found")
    ),
)]
#[get("/repo_history")]
pub async fn read_many(pool: web::Data<DbPool>) -> Result<web::Json<RepoVecObj>, AuthError> {
    let mut conn = pool.get()?;

    let models_vec: Vec<RepoHistory> = repo_history_table
        .select(RepoHistory::as_select())
        .load(&mut conn)?;

    Ok(web::Json(RepoVecObj { models: models_vec }))
}

/// Upsert RepoHistory
#[utoipa::path(
    tag = REPO_HISTORY,
    responses(
        (status = 200, description = "RepoHistory created"),
        (status = 500, description = "Internal Server Error")
    ),
    security(("password"=[]))
)]
#[post("/repo_history")]
pub async fn upsert(
    pool: web::Data<DbPool>,
    form: web::Json<CreateRepoHistory>,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
) -> Result<web::Json<RepoHistory>, AuthError> {
    let mut conn = pool.get()?;

    // 0. Check token username vs username in request
    if let Some((_username_s, _)) = credentials.token().split_once(":") {
        // 1. Upsert model
        let new_model_vals = form.into_inner();
        let model = diesel::insert_into(repo_history_table)
            .values(&new_model_vals)
            .returning(RepoHistory::as_returning())
            .get_result(&mut conn)?;
        return Ok(web::Json(model));
    }

    Err(AuthError::HttpError(500))
}

/// Get RepoHistory by id
#[utoipa::path(
    tag = REPO_HISTORY,
    responses(
        (status = 200, description = "RepoHistory found from database"),
        (status = 404, description = "Not found")
    ),
    params(
        ("id", description = "RepoHistory id, like \"SamuelMarks@crawlcomply-backend\""),
    )
)]
#[get("/repo_history/{id}")]
pub async fn read(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
) -> Result<web::Json<RepoHistory>, AuthError> {
    let mut conn = pool.get()?;
    match id.split_once('@') {
        Some((owner, repo_name)) => Ok(web::Json(
            crate::schema::repo_history::table
                .filter(
                    crate::schema::repo_history::dsl::full_name
                        .eq(format!("{}/{}", owner, repo_name)),
                )
                .select(RepoHistory::as_select())
                .first(&mut conn)?,
        )),
        None => Err(AuthError::HttpError(500)),
    }
}
