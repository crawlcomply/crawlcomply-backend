use core::assert_eq;

use actix_http::body::MessageBody as _;

use crate::get_profile_app;
use crate::models::profiles::Profiles;
use crate::tests::routes::profile::helpers::{test_profile_api, PASSWORD, USERNAMES};

#[actix_web::test]
async fn test_upsert_read_delete() {
    const USERNAME: &'static str = USERNAMES[0];
    let app = get_profile_app!().await;
    let token = crate::tests::routes::helpers::retrieve_token(USERNAME, PASSWORD).await;
    let alias = format!("{}-alias", USERNAME);

    /* POST */
    let upserted_profile: Profiles = actix_web::test::call_and_read_body_json(
        &app,
        test_profile_api::post(&token, &alias, USERNAME),
    )
    .await;

    /* GET */
    let read_profile: Profiles =
        actix_web::test::call_and_read_body_json(&app, test_profile_api::get(&token)).await;

    /* cmp */
    assert_eq!(
        upserted_profile,
        Profiles {
            alias: alias.clone(),
            username: String::from(USERNAME),
            created_at: read_profile.created_at,
            ..Profiles::default()
        }
    );
    assert_eq!(upserted_profile, read_profile);

    /* cleanup profile */
    actix_web::test::call_service(&app, test_profile_api::remove(&token)).await;
    let resp = actix_web::test::call_service(&app, test_profile_api::remove(&token)).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NO_CONTENT);
    assert_eq!(
        resp.response().body().size(),
        actix_web::body::BodySize::Sized(0)
    );

    /* confirm profile no longer exists */
    let req = test_profile_api::get(&token);
    let resp = actix_web::test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(
            "{\"error\":\"AuthError\",\"error_message\":\"`diesel::result::Error` error. NotFound\"}"
        ).unwrap(),
        serde_json::from_slice::<serde_json::Value>(
            &resp.into_body().try_into_bytes().unwrap()
        ).unwrap()
    );
}
