use actix_web::{delete, get, post};

use diesel::dsl::exists;
use diesel::sql_types::Integer;
use diesel::{
    BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};

use rust_actix_diesel_auth_scaffold::errors::AuthError;
use rust_actix_diesel_auth_scaffold::routes::token::helpers::parse_bearer_token;
use rust_actix_diesel_auth_scaffold::DbPool;

use crate::models::run_history::{CreateRunHistory, RunHistory, UpdateRunHistory};
use crate::schema::org as org_tbl;
use crate::schema::run_history as run_history_tbl;
use crate::schema::run_history::dsl::run_history;

const REPO_HISTORY: &'static str = "run_history";

#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct RunHistoryVecObj {
    pub(crate) runs: Vec<RunHistory>,
}

#[derive(Debug, serde::Deserialize)]
pub struct OrgRepoPath {
    pub org: String,
    pub repo: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct OrgRepoRunPath {
    pub org: String,
    pub repo: String,
    pub run: i32,
}

/// Get RunHistory
#[utoipa::path(
    tag = REPO_HISTORY,
    responses(
        (status = 200, description = "RunHistory found in database"),
        (status = 404, description = "Not found")
    ),
    params(
        ("org", description = "Org name"),
        ("repo", description = "Repo name"),
    )
)]
#[get("/org/{org}/repo/{repo}/run")]
pub async fn read_many(
    pool: actix_web::web::Data<DbPool>,
    path: actix_web::web::Path<OrgRepoPath>,
) -> Result<actix_web::web::Json<RunHistoryVecObj>, AuthError> {
    let OrgRepoPath { org, repo } = path.into_inner();
    let mut conn = pool.get()?;

    let run_history_vec: Vec<RunHistory> = run_history
        .filter(run_history_tbl::full_name.eq(format!("{org}/{repo}")))
        .select(RunHistory::as_select())
        .load(&mut conn)?;

    Ok(actix_web::web::Json(RunHistoryVecObj {
        runs: run_history_vec,
    }))
}

/// Upsert RunHistory
#[utoipa::path(
    tag = REPO_HISTORY,
    responses(
        (status = 200, description = "RunHistory created"),
        (status = 500, description = "Internal Server Error")
    ),
    params(
        ("org", description = "Org name"),
        ("repo", description = "Repo name"),
    ),
    security(("password"=[]))
)]
#[post("/org/{org}/repo/{repo}/run")]
pub async fn upsert(
    pool: actix_web::web::Data<DbPool>,
    path: actix_web::web::Path<OrgRepoPath>,
    form: actix_web::web::Json<CreateRunHistory>,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
) -> Result<actix_web::web::Json<RunHistory>, AuthError> {
    let OrgRepoPath { org, repo } = path.into_inner();
    let mut conn = pool.get()?;

    let token_username = parse_bearer_token(credentials.token())?.username;

    let create_run_history = {
        let mut _form = form.into_inner();
        _form.full_name = format!("{org}/{repo}");
        _form
    };

    let run_history_upserted: RunHistory = conn
        .transaction(|trans_con| {
            org_tbl::table
                .filter(
                    org_tbl::name
                        .eq(&org)
                        .and(org_tbl::owner.eq(token_username)),
                )
                .select(diesel::dsl::sql::<Integer>("1 / COUNT(*)"))
                .execute(trans_con)?;

            diesel::insert_into(run_history_tbl::table)
                .values(&create_run_history)
                .on_conflict((
                    run_history_tbl::full_name,
                    run_history_tbl::commit,
                    run_history_tbl::run,
                ))
                .do_update()
                .set(UpdateRunHistory {
                    created_at: None,
                    id: if create_run_history.id.is_some() {
                        Some(create_run_history.id.clone())
                    } else {
                        None
                    },
                    status: if create_run_history.status.is_some() {
                        Some(create_run_history.status.clone())
                    } else {
                        None
                    },
                })
                .returning(RunHistory::as_returning())
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
                        "Owner of run_history org does not match owner of requestor",
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

    Ok(actix_web::web::Json(run_history_upserted))
}

/// Get RunHistory by org name & repo name & run number
#[utoipa::path(
    tag = REPO_HISTORY,
    responses(
        (status = 200, description = "RunHistory found from database"),
        (status = 404, description = "Not found")
    ),
    params(
        ("org", description = "Org name"),
        ("repo", description = "Repo name"),
        ("run", description = "Run number")
    )
)]
#[get("/org/{org}/repo/{repo}/run/{run}")]
pub async fn read(
    pool: actix_web::web::Data<DbPool>,
    path: actix_web::web::Path<OrgRepoRunPath>,
) -> Result<actix_web::web::Json<RunHistory>, AuthError> {
    let OrgRepoRunPath { org, repo, run } = path.into_inner();
    let mut conn = pool.get()?;

    Ok(actix_web::web::Json(
        run_history_tbl::table
            .filter(
                run_history_tbl::full_name
                    .eq(format!("{org}/{repo}"))
                    .and(run_history_tbl::run.eq(run)),
            )
            .select(RunHistory::as_select())
            .first(&mut conn)?,
    ))
}

/// Delete RunHistory by org & repo & commit
#[utoipa::path(
    tag = REPO_HISTORY,
    responses(
        (status = 204, description = "RunHistory deleted"),
        (status = 404, description = "Not found")
    ),
    params(
        ("org", description = "Org name"),
        ("repo", description = "Repo name"),
        ("run", description = "Run number")
    ),
    security(("password"=[]))
)]
#[delete("/org/{org}/repo/{repo}/run/{run}")]
pub async fn remove(
    pool: actix_web::web::Data<DbPool>,
    path: actix_web::web::Path<OrgRepoRunPath>,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
) -> actix_web::Result<impl actix_web::Responder, AuthError> {
    let OrgRepoRunPath { org, repo, run } = path.into_inner();
    let mut conn = pool.get()?;
    let token_username = parse_bearer_token(credentials.token())?.username;
    let _rows_deleted = diesel::delete(
        run_history_tbl::table.filter(
            run_history_tbl::full_name
                .eq(format!("{org}/{repo}"))
                .and(run_history_tbl::run.eq(run))
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
