use actix_web::{delete, get, post};

use diesel::dsl::exists;
use diesel::sql_types::Integer;
use diesel::{
    BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};

use rust_actix_diesel_auth_scaffold::errors::AuthError;
use rust_actix_diesel_auth_scaffold::routes::token::helpers::parse_bearer_token;
use rust_actix_diesel_auth_scaffold::DbPool;

use crate::models::repo::{CreateRepo, Repo, UpdateRepo};
use crate::schema::org as org_tbl;
use crate::schema::repo as repo_tbl;
use crate::schema::repo::dsl::repo;

const REPO: &'static str = "repo";

#[derive(serde::Deserialize, serde::Serialize)]
struct RepoVecObj {
    repos: Vec<Repo>,
}

#[derive(Debug, serde::Deserialize)]
pub struct OrgRepoPath {
    pub org: String,
    pub name: String,
}

/// Get Repo
#[utoipa::path(
    tag = REPO,
    responses(
        (status = 200, description = "Repo found in database"),
        (status = 404, description = "Not found")
    ),
    params(
        ("org", description = "Org name")
    )
)]
#[get("/org/{org}/repo")]
pub async fn read_many(
    pool: actix_web::web::Data<DbPool>,
    org: actix_web::web::Path<String>,
) -> Result<actix_web::web::Json<RepoVecObj>, AuthError> {
    let mut conn = pool.get()?;

    let repo_vec: Vec<Repo> = repo
        .filter(repo_tbl::org.eq(org.into_inner()))
        .select(Repo::as_select())
        .load(&mut conn)?;

    Ok(actix_web::web::Json(RepoVecObj { repos: repo_vec }))
}

/// Upsert Repo
#[utoipa::path(
    tag = REPO,
    responses(
        (status = 200, description = "Repo created"),
        (status = 500, description = "Internal Server Error")
    ),
    params(
        ("org", description = "Org name")
    ),
    security(("password"=[]))
)]
#[post("/org/{org}/repo")]
pub async fn upsert(
    pool: actix_web::web::Data<DbPool>,
    org: actix_web::web::Path<String>,
    form: actix_web::web::Json<CreateRepo>,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
) -> Result<actix_web::web::Json<Repo>, AuthError> {
    let mut conn = pool.get()?;

    let token_username = parse_bearer_token(credentials.token())?.username;

    let org_str = org.into_inner(); // ADMIN: make this `val.org` for admin
    let new_repo_vals: CreateRepo = {
        let mut val = form.into_inner();
        if val.full_name.is_some() {
            val.full_name = Some(format!("{}/{}", &org_str, &val.name));
        }
        val.org = org_str;
        val
    };

    println!(
        "new_repo_vals.org: {}, token_username: {}",
        &new_repo_vals.org, &token_username
    );

    let repo_upserted: Repo = conn
        .transaction(|trans_con| {
            org_tbl::table
                .filter(
                    org_tbl::name
                        .eq(&new_repo_vals.org)
                        .and(org_tbl::owner.eq(token_username)),
                )
                .select(diesel::dsl::sql::<Integer>("1 / COUNT(*)"))
                .execute(trans_con)?;

            diesel::insert_into(repo_tbl::table)
                .values(&new_repo_vals)
                .on_conflict(repo_tbl::full_name)
                .do_update()
                .set(UpdateRepo {
                    // id: new_repo_vals.id,
                    node_id: if new_repo_vals.node_id.is_some() {
                        Some(new_repo_vals.node_id.clone())
                    } else {
                        None
                    },
                    name: Some(new_repo_vals.name.clone()),
                    full_name: if new_repo_vals.full_name.is_some() {
                        Some(new_repo_vals.full_name.clone())
                    } else {
                        None
                    },
                    private: if new_repo_vals.private.is_some() {
                        Some(new_repo_vals.private.clone())
                    } else {
                        None
                    },
                    html_url: if new_repo_vals.html_url.is_some() {
                        Some(new_repo_vals.html_url.clone())
                    } else {
                        None
                    },
                    description: if new_repo_vals.description.is_some() {
                        Some(new_repo_vals.description.clone())
                    } else {
                        None
                    },
                    fork: if new_repo_vals.fork.is_some() {
                        Some(new_repo_vals.fork.clone())
                    } else {
                        None
                    },
                    default_branch: if new_repo_vals.default_branch.is_some() {
                        Some(new_repo_vals.default_branch.clone())
                    } else {
                        None
                    },
                    pulls_url: if new_repo_vals.pulls_url.is_some() {
                        Some(new_repo_vals.pulls_url.clone())
                    } else {
                        None
                    },
                    comments_url: if new_repo_vals.comments_url.is_some() {
                        Some(new_repo_vals.comments_url.clone())
                    } else {
                        None
                    },
                    languages: if new_repo_vals.languages.is_some() {
                        Some(new_repo_vals.languages.clone())
                    } else {
                        None
                    },
                    spdx: if new_repo_vals.spdx.is_some() {
                        Some(new_repo_vals.spdx.clone())
                    } else {
                        None
                    },
                    visibility: if new_repo_vals.visibility.is_some() {
                        Some(new_repo_vals.visibility.clone())
                    } else {
                        None
                    },
                    org: Some(new_repo_vals.org.clone()),
                    is_monorepo: if new_repo_vals.is_monorepo.is_some() {
                        Some(new_repo_vals.is_monorepo.clone())
                    } else {
                        None
                    },
                    last_commit: if new_repo_vals.last_commit.is_some() {
                        Some(new_repo_vals.last_commit.clone())
                    } else {
                        None
                    },
                    created_at: None,
                    updated_at: None,
                })
                .returning(Repo::as_returning())
                .get_result(trans_con)
        })
        .map_err(|e| -> AuthError {
            if let diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                r,
            ) = e
            {
                if r.message() == "division by zero" {
                    AuthError::Unauthorised("Owner of repo does not match owner of requestor")
                } else {
                    AuthError::from(diesel::result::Error::DatabaseError(
                        diesel::result::DatabaseErrorKind::Unknown,
                        r,
                    ))
                }
            } else {
                e.into()
            }
        })?;

    Ok(actix_web::web::Json(repo_upserted))
}

/// Get Repo by name
#[utoipa::path(
    tag = REPO,
    responses(
        (status = 200, description = "Repo found from database"),
        (status = 404, description = "Not found")
    ),
    params(
        ("org", description = "Org name"),
        ("name", description = "Repo name"),
    )
)]
#[get("/org/{org}/repo/{name}")]
pub async fn read(
    pool: actix_web::web::Data<DbPool>,
    path: actix_web::web::Path<OrgRepoPath>,
) -> Result<actix_web::web::Json<Repo>, AuthError> {
    let OrgRepoPath { org, name } = path.into_inner();
    let mut conn = pool.get()?;

    Ok(actix_web::web::Json(
        repo_tbl::table
            .filter(repo_tbl::org.eq(org).and(repo_tbl::name.eq(name)))
            .select(Repo::as_select())
            .first(&mut conn)?,
    ))
}

/// Delete Repo by name
#[utoipa::path(
    tag = REPO,
    responses(
        (status = 204, description = "Repo deleted"),
        (status = 404, description = "Not found")
    ),
    params(
        ("org", description = "Org name"),
        ("name", description = "Repo name"),
    ),
    security(("password"=[]))
)]
#[delete("/org/{org}/repo/{name}")]
pub async fn remove(
    pool: actix_web::web::Data<DbPool>,
    path: actix_web::web::Path<OrgRepoPath>,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
) -> actix_web::Result<impl actix_web::Responder, AuthError> {
    let OrgRepoPath { org, name } = path.into_inner();
    let mut conn = pool.get()?;
    let token_username = parse_bearer_token(credentials.token())?.username;
    let _rows_deleted = diesel::delete(
        repo_tbl::table.filter(
            repo_tbl::name
                .eq(name)
                .and(repo_tbl::org.eq(org))
                .and(exists(
                    org_tbl::table.filter(
                        org_tbl::name
                            .eq(repo_tbl::org)
                            .and(org_tbl::owner.eq(&token_username)),
                    ),
                )),
        ),
    )
    .execute(&mut conn)
    .unwrap_or_else(|_| 0usize);
    Ok(actix_web::HttpResponse::new(
        actix_web::http::StatusCode::NO_CONTENT,
    ))
}
