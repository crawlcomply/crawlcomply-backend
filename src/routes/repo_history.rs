use actix_web::{delete, get, post};

use diesel::dsl::exists;
use diesel::sql_types::Integer;
use diesel::{
    BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};

use rust_actix_diesel_auth_scaffold::errors::{AuthError, AuthErrorSchema};
use rust_actix_diesel_auth_scaffold::routes::token::helpers::parse_bearer_token;
use rust_actix_diesel_auth_scaffold::DbPool;

use crate::models::repo_history::{CreateRepoHistory, RepoHistory, UpdateRepoHistory};
use crate::schema::org as org_tbl;
use crate::schema::repo_history as repo_history_tbl;
use crate::schema::repo_history::dsl::repo_history;

const REPO_HISTORY: &'static str = "repo_history";

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
struct RepoHistoryVecObj {
    repo_histories: Vec<RepoHistory>,
}

#[derive(Debug, serde::Deserialize)]
pub struct OrgRepoPath {
    pub org: String,
    pub repo: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct OrgRepoHashPath {
    pub org: String,
    pub repo: String,
    pub hash: String,
}

/// Get `RepoHistory`-ies
#[utoipa::path(
    tag = REPO_HISTORY,
    responses(
        (status = 200, description = "`RepoHistory`-ies", body = RepoHistoryVecObj),
        (status = 404, description = "Not found", body = AuthErrorSchema)
    ),
    params(
        ("org", description = "Org name"),
        ("repo", description = "Repo name"),
    )
)]
#[get("/org/{org}/repo/{repo}/history")]
pub async fn read_many(
    pool: actix_web::web::Data<DbPool>,
    path: actix_web::web::Path<OrgRepoPath>,
) -> Result<actix_web::web::Json<RepoHistoryVecObj>, AuthError> {
    let OrgRepoPath { org, repo } = path.into_inner();
    let mut conn = pool.get()?;

    let repo_history_vec: Vec<RepoHistory> = repo_history
        .filter(repo_history_tbl::full_name.eq(format!("{org}/{repo}")))
        .select(RepoHistory::as_select())
        .load(&mut conn)?;

    Ok(actix_web::web::Json(RepoHistoryVecObj {
        repo_histories: repo_history_vec,
    }))
}

/// Upsert `RepoHistory`
#[utoipa::path(
    tag = REPO_HISTORY,
    responses(
        (status = 200, description = "RepoHistory created", body = RepoHistory),
        (status = 500, description = "Internal Server Error", body = AuthErrorSchema)
    ),
    params(
        ("org", description = "Org name"),
        ("repo", description = "Repo name"),
    ),
    security(("password"=[]))
)]
#[post("/org/{org}/repo/{repo}/history")]
pub async fn upsert(
    pool: actix_web::web::Data<DbPool>,
    path: actix_web::web::Path<OrgRepoPath>,
    form: actix_web::web::Json<CreateRepoHistory>,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
) -> Result<actix_web::web::Json<RepoHistory>, AuthError> {
    let OrgRepoPath { org, repo } = path.into_inner();
    let mut conn = pool.get()?;

    let token_username = parse_bearer_token(credentials.token())?.username;

    let create_repo_history = {
        let mut _form = form.into_inner();
        _form.full_name = format!("{org}/{repo}");
        _form
    };

    let repo_history_upserted: RepoHistory = conn
        .transaction(|trans_con| {
            org_tbl::table
                .filter(
                    org_tbl::name
                        .eq(&org)
                        .and(org_tbl::owner.eq(token_username)),
                )
                .select(diesel::dsl::sql::<Integer>("1 / COUNT(*)"))
                .execute(trans_con)?;

            diesel::insert_into(repo_history_tbl::table)
                .values(&create_repo_history)
                .on_conflict((repo_history_tbl::full_name, repo_history_tbl::commit))
                .do_update()
                .set(UpdateRepoHistory {
                    repo_id: if create_repo_history.repo_id.is_some() {
                        Some(create_repo_history.repo_id)
                    } else {
                        None
                    },
                    doc_coverage: if create_repo_history.doc_coverage.is_some() {
                        Some(create_repo_history.doc_coverage.clone())
                    } else {
                        None
                    },
                    test_coverage: if create_repo_history.test_coverage.is_some() {
                        Some(create_repo_history.test_coverage.clone())
                    } else {
                        None
                    },
                    hosted_docs_url: if create_repo_history.hosted_docs_url.is_some() {
                        Some(create_repo_history.hosted_docs_url.clone())
                    } else {
                        None
                    },
                    security_scanner: if create_repo_history.security_scanner.is_some() {
                        Some(create_repo_history.security_scanner.clone())
                    } else {
                        None
                    },
                    git_tag: if create_repo_history.git_tag.is_some() {
                        Some(create_repo_history.git_tag.clone())
                    } else {
                        None
                    },
                    git_branch: if create_repo_history.git_branch.is_some() {
                        Some(create_repo_history.git_branch.clone())
                    } else {
                        None
                    },
                    github_pr: if create_repo_history.github_pr.is_some() {
                        Some(create_repo_history.github_pr)
                    } else {
                        None
                    },
                    metrics: if create_repo_history.metrics.is_some() {
                        Some(create_repo_history.metrics.clone())
                    } else {
                        None
                    },
                    notes: if create_repo_history.notes.is_some() {
                        Some(create_repo_history.notes.clone())
                    } else {
                        None
                    },
                    created_at: None,
                    id: if create_repo_history.id.is_some() {
                        Some(create_repo_history.id.clone())
                    } else {
                        None
                    },
                })
                .returning(RepoHistory::as_returning())
                .get_result(trans_con)
        })
        .map_err(|e| -> AuthError {
            if let diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                r,
            ) = e
            {
                if r.message() == "division by zero" {
                    AuthError::Unauthorised(
                        "Owner of repo_history org does not match owner of requestor",
                    )
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

    Ok(actix_web::web::Json(repo_history_upserted))
}

/// Get `RepoHistory` by org & repo & commit
#[utoipa::path(
    tag = REPO_HISTORY,
    responses(
        (status = 200, description = "RepoHistory found from database", body = RepoHistory),
        (status = 404, description = "Not found", body = AuthErrorSchema)
    ),
    params(
        ("org", description = "Org name"),
        ("repo", description = "Repo name"),
        ("hash", description = "Commit hash")
    )
)]
#[get("/org/{org}/repo/{repo}/history/{hash}")]
pub async fn read(
    pool: actix_web::web::Data<DbPool>,
    path: actix_web::web::Path<OrgRepoHashPath>,
) -> Result<actix_web::web::Json<RepoHistory>, AuthError> {
    let OrgRepoHashPath { org, repo, hash } = path.into_inner();
    let mut conn = pool.get()?;

    Ok(actix_web::web::Json(
        repo_history_tbl::table
            .filter(
                repo_history_tbl::full_name
                    .eq(format!("{org}/{repo}"))
                    .and(repo_history_tbl::commit.eq(hash)),
            )
            .select(RepoHistory::as_select())
            .first(&mut conn)?,
    ))
}

/// Delete `RepoHistory` by org & repo & commit
#[utoipa::path(
    tag = REPO_HISTORY,
    responses(
        (status = 204, description = "RepoHistory deleted"),
        (status = 404, description = "Not found", body = AuthErrorSchema)
    ),
    params(
        ("org", description = "Org name"),
        ("repo", description = "Repo name"),
        ("hash", description = "Commit hash")
    ),
    security(("password"=[]))
)]
#[delete("/org/{org}/repo/{repo}/history/{hash}")]
pub async fn remove(
    pool: actix_web::web::Data<DbPool>,
    path: actix_web::web::Path<OrgRepoHashPath>,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
) -> actix_web::Result<impl actix_web::Responder, AuthError> {
    let OrgRepoHashPath { org, repo, hash } = path.into_inner();
    let mut conn = pool.get()?;
    let token_username = parse_bearer_token(credentials.token())?.username;
    let _rows_deleted = diesel::delete(
        repo_history_tbl::table.filter(
            repo_history_tbl::full_name
                .eq(format!("{org}/{repo}"))
                .and(repo_history_tbl::commit.eq(hash))
                .and(exists(
                    org_tbl::table.filter(
                        org_tbl::name
                            .eq(org)
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
