use actix_web::{delete, get, post};

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use rust_actix_diesel_auth_scaffold::errors::AuthError;
use rust_actix_diesel_auth_scaffold::routes::token::helpers::parse_bearer_token;
use rust_actix_diesel_auth_scaffold::DbPool;

use crate::models::profiles::{CreateProfiles, Profiles};
use crate::schema::profiles::dsl::profiles;
use crate::schema::profiles::dsl::username as profile_username;

const PROFILE: &'static str = "profile";

#[derive(serde::Deserialize, serde::Serialize)]
struct ProfilesVecObj {
    profiles: Vec<Profiles>,
}

/// Get Profiles
#[utoipa::path(
    tag = PROFILE,
    responses(
        (status = 200, description = "Profiles found in database"),
        (status = 404, description = "Not found")
    )
)]
#[get("/profiles")]
pub async fn read_many(
    pool: actix_web::web::Data<DbPool>,
) -> Result<actix_web::web::Json<ProfilesVecObj>, AuthError> {
    let mut conn = pool.get()?;
    let profiles_vec: Vec<Profiles> = profiles.select(Profiles::as_select()).load(&mut conn)?;
    Ok(actix_web::web::Json(ProfilesVecObj {
        profiles: profiles_vec,
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
    form: actix_web::web::Json<CreateProfiles>,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
) -> Result<actix_web::web::Json<Profiles>, AuthError> {
    let mut conn = pool.get()?;

    let token_username = parse_bearer_token(credentials.token())?.username;
    let new_profile = form.into_inner();
    if new_profile.username != token_username {
        return Err(AuthError::Unauthorised(
            "Trying to upsert profile of someone else",
        ));
    }
    let profile = diesel::insert_into(profiles)
        .values(&new_profile)
        .returning(Profiles::as_returning())
        .get_result(&mut conn)?;
    Ok(actix_web::web::Json(profile))
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
) -> Result<actix_web::web::Json<Profiles>, AuthError> {
    let mut conn = pool.get()?;
    let token_username = parse_bearer_token(credentials.token())?.username;
    Ok(actix_web::web::Json(
        crate::schema::profiles::table
            .filter(profile_username.eq(token_username))
            .select(Profiles::as_select())
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
    diesel::delete(crate::schema::profiles::table.filter(profile_username.eq(token_username)))
        .execute(&mut conn)?;
    Ok(actix_web::HttpResponse::new(
        actix_web::http::StatusCode::NO_CONTENT,
    ))
}
