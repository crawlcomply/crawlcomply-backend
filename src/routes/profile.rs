use actix_web::{delete, get, post};

use diesel::query_dsl::methods::FilterDsl;
use diesel::OptionalExtension;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use rust_actix_diesel_auth_scaffold::errors::AuthError;
use rust_actix_diesel_auth_scaffold::routes::token::helpers::parse_bearer_token;
use rust_actix_diesel_auth_scaffold::DbPool;

use crate::models::profile::{CreateProfile, Profile, UpdateProfile};
use crate::schema::profile as profile_tbl;
use crate::schema::profile::dsl::profile;

const PROFILE: &'static str = "profile";

#[derive(serde::Deserialize, serde::Serialize)]
struct ProfileVecObj {
    profiles: Vec<Profile>,
}

/// Get Profile
#[utoipa::path(
    tag = PROFILE,
    responses(
        (status = 200, description = "Profile found in database"),
        (status = 404, description = "Not found")
    )
)]
#[get("/profiles")]
pub async fn read_many(
    pool: actix_web::web::Data<DbPool>,
) -> Result<actix_web::web::Json<ProfileVecObj>, AuthError> {
    let mut conn = pool.get()?;
    let profile_vec: Vec<Profile> = profile.select(Profile::as_select()).load(&mut conn)?;
    Ok(actix_web::web::Json(ProfileVecObj {
        profiles: profile_vec,
    }))
}

/// Upsert Model
#[utoipa::path(
    tag = PROFILE,
    responses(
        (status = 200, description = "Profile created"),
        (status = 401, description = "Unauthorised"),
        (status = 500, description = "Internal Server Error")
    ),
    security(("password"=[]))
)]
#[post("/profile")]
pub async fn upsert(
    pool: actix_web::web::Data<DbPool>,
    form: actix_web::web::Json<CreateProfile>,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
) -> Result<actix_web::web::Json<Profile>, AuthError> {
    let mut conn = pool.get()?;

    let token_username = parse_bearer_token(credentials.token())?.username;
    let new_profile = form.into_inner();
    if new_profile.username != token_username {
        return Err(AuthError::Unauthorised(
            "username of profile does not match username of requestor",
        ));
    }
    let profile_upserted: Option<Profile> = diesel::insert_into(profile)
        .values(&new_profile)
        .on_conflict(profile_tbl::username)
        .do_update()
        .set(UpdateProfile {
            username: Some(new_profile.username.clone()),
            description: if new_profile.description.is_some() {
                Some(new_profile.description.clone())
            } else {
                None
            },
            github_id: if new_profile.github_id.is_some() {
                Some(new_profile.github_id.clone())
            } else {
                None
            },
            avatar_url: if new_profile.avatar_url.is_some() {
                Some(new_profile.avatar_url.clone())
            } else {
                None
            },
            created_at: None,
        })
        .filter(profile_tbl::username.eq(&token_username))
        .returning(Profile::as_returning())
        .get_result(&mut conn)
        .optional()?;
    match profile_upserted {
        Some(o) => Ok(actix_web::web::Json(o)),
        None => Err(AuthError::Unauthorised(
            "username of profile does not match username of requestor",
        )),
    }
}

/// Get Profile
#[utoipa::path(
    tag = PROFILE,
    responses(
        (status = 200, description = "Profile found from database"),
        (status = 404, description = "Not found")
    )
)]
#[get("/profile")]
pub async fn read(
    pool: actix_web::web::Data<DbPool>,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
) -> Result<actix_web::web::Json<Profile>, AuthError> {
    let mut conn = pool.get()?;
    let token_username = parse_bearer_token(credentials.token())?.username;
    Ok(actix_web::web::Json(
        diesel::QueryDsl::filter(
            crate::schema::profile::table,
            profile_tbl::username.eq(token_username),
        )
        .select(Profile::as_select())
        .first(&mut conn)?,
    ))
}

/// Delete Profile
#[utoipa::path(
    tag = PROFILE,
    responses(
        (status = 204, description = "Profile deleted"),
        (status = 404, description = "Not found")
    )
)]
#[delete("/profile")]
pub async fn remove(
    pool: actix_web::web::Data<DbPool>,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
) -> actix_web::Result<impl actix_web::Responder, AuthError> {
    let mut conn = pool.get()?;
    let token_username = parse_bearer_token(credentials.token())?.username;
    diesel::delete(diesel::QueryDsl::filter(
        crate::schema::profile::table,
        profile_tbl::username.eq(token_username),
    ))
    .execute(&mut conn)?;
    Ok(actix_web::HttpResponse::new(
        actix_web::http::StatusCode::NO_CONTENT,
    ))
}
