use core::assert_eq;

use actix_http::body::MessageBody as _;

use crate::get_org_app;
use crate::models::org::Org;
use crate::tests::routes::org::helpers::{test_org_api, PASSWORD, USERNAMES};

#[actix_web::test]
async fn test_upsert_read_delete() {
    const TEST_ID: usize = 0;
    const USERNAME: &'static str = USERNAMES[TEST_ID];
    const ORG: &'static str = crate::tests::routes::org::helpers::ORGS[TEST_ID];

    let app = get_org_app!().await;
    let token = crate::tests::routes::helpers::retrieve_token(USERNAME, PASSWORD).await;

    let create_org = crate::models::org::CreateOrg {
        name: String::from(ORG),
        description: Some(String::from("Test description")),
        owner: String::from(USERNAME),
        ..Default::default()
    };

    /* POST */
    let upserted_org: Org =
        actix_web::test::call_and_read_body_json(&app, test_org_api::post(&token, &create_org))
            .await;

    /* GET */
    let read_org: Org =
        actix_web::test::call_and_read_body_json(&app, test_org_api::get(&create_org.name)).await;

    /* cmp */
    assert_eq!(
        upserted_org,
        Org {
            name: create_org.name.clone(),
            description: create_org.description,
            github_id: create_org.github_id,
            avatar_url: create_org.avatar_url,
            owner: create_org.owner,
            created_at: read_org.created_at
        }
    );
    assert_eq!(upserted_org, read_org);

    /* cleanup org */
    actix_web::test::call_service(&app, test_org_api::remove(&token, &create_org.name)).await;
    let resp =
        actix_web::test::call_service(&app, test_org_api::remove(&token, &create_org.name)).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NO_CONTENT);
    assert_eq!(
        resp.response().body().size(),
        actix_web::body::BodySize::Sized(0)
    );

    /* confirm org no longer exists */
    let req = test_org_api::get(&create_org.name);
    let resp = actix_web::test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(
            r#"{
            "error":"AuthError",
            "error_message":"`diesel::result::Error` error. NotFound"
        }"#
        )
        .unwrap(),
        serde_json::from_slice::<serde_json::Value>(&resp.into_body().try_into_bytes().unwrap())
            .unwrap()
    );
}

#[actix_web::test]
async fn test_update_org_you_do_not_own() {
    const TEST_ID: usize = 1;
    const USERNAME: &'static str = USERNAMES[TEST_ID];
    const USERNAME1: &'static str = USERNAMES[USERNAMES.len() - 1];
    const ORG: &'static str = crate::tests::routes::org::helpers::ORGS[TEST_ID];

    let app = get_org_app!().await;
    let token = crate::tests::routes::helpers::retrieve_token(USERNAME, PASSWORD).await;
    let user1_token = crate::tests::routes::helpers::retrieve_token(USERNAME1, PASSWORD).await;

    let create_org = crate::models::org::CreateOrg {
        name: String::from(ORG),
        description: Some(String::from("Test description")),
        owner: String::from(USERNAME),
        ..Default::default()
    };

    /* POST, creating new org */
    actix_web::test::call_service(&app, test_org_api::post(&token, &create_org)).await;

    /* POST, attempt to update org with non-owner user */
    let req = test_org_api::post(
        &user1_token,
        &crate::models::org::CreateOrg {
            description: Some(format!("New description from {}", USERNAME1)),
            ..create_org.clone()
        },
    );
    let resp = actix_web::test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::UNAUTHORIZED);

    /* cleanup org */
    actix_web::test::call_service(&app, test_org_api::remove(&token, &create_org.name)).await;
    let resp =
        actix_web::test::call_service(&app, test_org_api::remove(&token, &create_org.name)).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NO_CONTENT);
    assert_eq!(
        resp.response().body().size(),
        actix_web::body::BodySize::Sized(0)
    );

    /* confirm org no longer exists */
    let req = test_org_api::get(&create_org.name);
    let resp = actix_web::test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(
            r#"{
            "error":"AuthError",
            "error_message":"`diesel::result::Error` error. NotFound"
        }"#
        )
        .unwrap(),
        serde_json::from_slice::<serde_json::Value>(&resp.into_body().try_into_bytes().unwrap())
            .unwrap()
    );
}

#[actix_web::test]
async fn test_get_many_org() {
    let resp = actix_web::test::call_service(&get_org_app!().await, test_org_api::get_many()).await;
    let status = resp.status();
    println!(
        "body from test_get_many_org: {}",
        serde_json::from_slice::<serde_json::Value>(&resp.into_body().try_into_bytes().unwrap())
            .unwrap()
    );
    assert_eq!(status, actix_web::http::StatusCode::OK);
}
