use actix_web::{get, post, web, Result};

use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use rust_actix_diesel_auth_scaffold::errors::AuthError;
use rust_actix_diesel_auth_scaffold::DbPool;

use crate::models::repo::{CreateRepo, Repo};
use crate::schema::repo::dsl::{name as repo_name, repo as repo_table};

const REPO: &str = "repo";

#[derive(serde::Deserialize, serde::Serialize)]
struct RepoVecObj {
    models: Vec<Repo>,
}

/// Get Repo
#[utoipa::path(
    tag = REPO,
    responses(
        (status = 200, description = "Repo found in database"),
        (status = 404, description = "Not found")
    ),
)]
#[get("/repo")]
pub async fn read_many(pool: web::Data<DbPool>) -> Result<web::Json<RepoVecObj>, AuthError> {
    let mut conn = pool.get()?;

    let models_vec: Vec<Repo> = repo_table.select(Repo::as_select()).load(&mut conn)?;

    Ok(web::Json(RepoVecObj { models: models_vec }))
}

/// Upsert Repo
#[utoipa::path(
    tag = REPO,
    responses(
        (status = 200, description = "Repo created"),
        (status = 500, description = "Internal Server Error")
    ),
    security(("password"=[]))
)]
#[post("/repo")]
pub async fn upsert(
    pool: web::Data<DbPool>,
    form: web::Json<CreateRepo>,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
) -> Result<web::Json<Repo>, AuthError> {
    let mut conn = pool.get()?;

    // 0. Check token username vs username in request
    if let Some((_username_s, _)) = credentials.token().split_once(":") {
        // 1. Upsert model
        let new_model_vals = form.into_inner();
        let model = diesel::insert_into(repo_table)
            .values(&new_model_vals)
            .returning(Repo::as_returning())
            .get_result(&mut conn)?;
        return Ok(web::Json(model));
    }

    Err(AuthError::HttpError(500))
}

/// Get Repo by id
#[utoipa::path(
    tag = REPO,
    responses(
        (status = 200, description = "Repo found from database"),
        (status = 404, description = "Not found")
    ),
    params(
        ("id", description = "Repo id"),
    )
)]
#[get("/repo/{name}")]
pub async fn read(
    pool: web::Data<DbPool>,
    name: web::Path<String>,
) -> Result<web::Json<Repo>, AuthError> {
    let mut conn = pool.get()?;

    Ok(actix_web::web::Json(
        crate::schema::repo::table
            .filter(repo_name.eq(name.into_inner()))
            .select(Repo::as_select())
            .first(&mut conn)?,
    ))
}
