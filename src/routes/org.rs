use actix_web::{delete, get, post};

use diesel::query_dsl::methods::FilterDsl;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
    SelectableHelper,
};

use rust_actix_diesel_auth_scaffold::errors::{AuthError, AuthErrorSchema};
use rust_actix_diesel_auth_scaffold::routes::token::helpers::parse_bearer_token;
use rust_actix_diesel_auth_scaffold::DbPool;

use crate::models::org::{CreateOrg, Org, UpdateOrg};
use crate::schema::org as org_tbl;
use crate::schema::org::dsl::org;

const ORG: &'static str = "org";

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
struct OrgVecObj {
    orgs: Vec<Org>,
}

/// Get all `Org`s
#[utoipa::path(
    tag = ORG,
    responses(
        (status = 200, description = "`Org`s", body = OrgVecObj),
        (status = 404, description = "Not found", body = AuthErrorSchema)
    )
)]
#[get("/org")]
pub async fn read_many(
    pool: actix_web::web::Data<DbPool>,
) -> Result<actix_web::web::Json<OrgVecObj>, AuthError> {
    let mut conn = pool.get()?;

    let org_vec: Vec<Org> = org.select(Org::as_select()).load(&mut conn)?;

    Ok(actix_web::web::Json(OrgVecObj { orgs: org_vec }))
}

/// Upsert `Org`
#[utoipa::path(
    tag = ORG,
    responses(
        (status = 200, description = "Org created/updated", body = Org),
        (status = 500, description = "Internal Server Error", body = AuthErrorSchema)
    ),
    security(("password"=[]))
)]
#[post("/org")]
pub async fn upsert(
    pool: actix_web::web::Data<DbPool>,
    form: actix_web::web::Json<CreateOrg>,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
) -> Result<actix_web::web::Json<Org>, AuthError> {
    let mut conn = pool.get()?;
    let token_username = parse_bearer_token(credentials.token())?.username;
    let new_org_vals: CreateOrg = form.into_inner();
    if new_org_vals.owner != token_username {
        // ADMIN: can skip this check
        return Err(AuthError::Unauthorised(
            "Trying to make someone else owner of org",
        ));
    }

    let org_upserted: Option<Org> = diesel::insert_into(org_tbl::table)
        .values(&new_org_vals)
        .on_conflict(org_tbl::name)
        .do_update()
        .set(UpdateOrg {
            description: Some(new_org_vals.description.clone()),
            github_id: Some(new_org_vals.github_id.clone()),
            avatar_url: Some(new_org_vals.avatar_url.clone()),
            owner: None,
            created_at: None,
        })
        .filter(org_tbl::owner.eq(&token_username))
        .returning(Org::as_returning())
        .get_result(&mut conn)
        .optional()?;
    match org_upserted {
        Some(o) => Ok(actix_web::web::Json(o)),
        None => Err(AuthError::Unauthorised(
            "Owner of org does not match owner of requestor",
        )),
    }
}

/// Get `Org` by name
#[utoipa::path(
    tag = ORG,
    responses(
        (status = 200, description = "Org found from database", body = Org),
        (status = 404, description = "Not found", body = AuthErrorSchema)
    ),
    params(
        ("name", description = "Org name"),
    )
)]
#[get("/org/{name}")]
pub async fn read(
    pool: actix_web::web::Data<DbPool>,
    name: actix_web::web::Path<String>,
) -> Result<actix_web::web::Json<Org>, AuthError> {
    let mut conn = pool.get()?;
    Ok(actix_web::web::Json(
        org.find(name.into_inner()).first(&mut conn)?,
    ))
}

/// Delete `Org` by name
#[utoipa::path(
    tag = ORG,
    responses(
        (status = 204, description = "Org deleted"),
        (status = 404, description = "Not found", body = AuthErrorSchema)
    ),
    params(
        ("name", description = "Org name"),
    ),
    security(("password"=[]))
)]
#[delete("/org/{name}")]
pub async fn remove(
    pool: actix_web::web::Data<DbPool>,
    name: actix_web::web::Path<String>,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
) -> actix_web::Result<impl actix_web::Responder, AuthError> {
    let mut conn = pool.get()?;
    let token_username = parse_bearer_token(credentials.token())?.username;
    let _rows_deleted = diesel::delete(QueryDsl::filter(
        org_tbl::table,
        org_tbl::owner
            .eq(token_username)
            .and(org_tbl::name.eq(name.into_inner())),
    ))
    .execute(&mut conn)
    .unwrap_or_else(|_| 0usize);
    Ok(actix_web::HttpResponse::new(
        actix_web::http::StatusCode::NO_CONTENT,
    ))
}
