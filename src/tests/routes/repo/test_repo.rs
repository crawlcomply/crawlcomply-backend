use core::assert_eq;

use std::borrow::ToOwned;

use actix_http::body::MessageBody as _;

use bigdecimal::ToPrimitive;

use crate::models::repo::Repo;
use crate::tests::routes::repo::helpers::{test_repo_api, NUM, ORGS, PASSWORD, REPOS, USERNAMES};
use crate::{get_org_app, get_repo_app};

#[actix_web::test]
async fn test_upsert_read_delete() {
    const TEST_ID: usize = 0;
    const USERNAME: &'static str = USERNAMES[TEST_ID];
    const ORG: &'static str = ORGS[TEST_ID];
    const REPO: &'static str = REPOS[TEST_ID];

    let app = get_repo_app!().await;
    let app_org = get_org_app!().await;
    let token = crate::tests::routes::helpers::retrieve_token(USERNAME, PASSWORD).await;

    /* POST */
    let upserted_org_val: serde_json::Value = actix_web::test::call_and_read_body_json(
        &app_org,
        crate::tests::routes::org::helpers::test_org_api::post(
            &token,
            &crate::models::org::CreateOrg {
                name: String::from(ORG),
                description: Some(String::from("Test description")),
                owner: String::from(USERNAME),
                ..Default::default()
            },
        ),
    )
    .await;
    let upserted_org: crate::models::org::Org = serde_json::from_value(upserted_org_val.clone())
        .unwrap_or_else(|_| panic!("{}", upserted_org_val));

    let create_repo = crate::models::repo::CreateRepo {
        id: TEST_ID.to_i32().unwrap() + &15i32 + NUM as i32,
        name: REPO.to_owned(),
        description: Some(format!(
            "Repo made by {} with org {}",
            USERNAME, upserted_org.owner
        )),
        org: String::from(upserted_org.name),
        ..Default::default()
    };
    let upserted_repo_val: serde_json::Value = actix_web::test::call_and_read_body_json(
        &app,
        test_repo_api::post(&token, ORG, &create_repo),
    )
    .await;
    let upserted_repo: Repo = serde_json::from_value(upserted_repo_val.clone())
        .unwrap_or_else(|_| panic!("{}", upserted_repo_val));

    /* GET */
    let read_repo_val: serde_json::Value =
        actix_web::test::call_and_read_body_json(&app, test_repo_api::get(ORG, &create_repo.name))
            .await;
    let read_repo: Repo = serde_json::from_value(read_repo_val.clone())
        .unwrap_or_else(|_| panic!("{}", read_repo_val));

    /* cmp */
    assert_eq!(
        upserted_repo,
        Repo {
            id: create_repo.id,
            name: REPO.to_owned(),
            full_name: Some(format!("{ORG}/{REPO}")),
            description: create_repo.description,
            org: create_repo.org.clone(),
            created_at: read_repo.created_at,
            updated_at: read_repo.updated_at,
            ..Default::default()
        }
    );
    assert_eq!(upserted_repo, read_repo);

    /* cleanup repo */
    let resp = actix_web::test::call_service(&app, test_repo_api::remove(&token, ORG, REPO)).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NO_CONTENT);
    assert_eq!(
        resp.response().body().size(),
        actix_web::body::BodySize::Sized(0)
    );
    assert_eq!(
        actix_web::test::call_service(
            &app_org,
            crate::tests::routes::org::helpers::test_org_api::remove(&token, ORG)
        )
        .await
        .status(),
        actix_web::http::StatusCode::NO_CONTENT
    );

    /* confirm repo no longer exists */
    let req = test_repo_api::get(ORG, REPO);
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
async fn test_update_repo_you_do_not_own() {
    const TEST_ID: usize = 1;
    const USERNAME: &'static str = USERNAMES[TEST_ID];
    const USERNAME1: &'static str = USERNAMES[USERNAMES.len() - 1];
    const ORG: &'static str = ORGS[TEST_ID];
    const REPO: &'static str = REPOS[TEST_ID];

    let app = get_repo_app!().await;
    let token = crate::tests::routes::helpers::retrieve_token(USERNAME, PASSWORD).await;
    let user1_token = crate::tests::routes::helpers::retrieve_token(USERNAME1, PASSWORD).await;

    let app_org = get_org_app!().await;

    let repo_name = REPO;

    /* POST */
    let upserted_org_val: serde_json::Value = actix_web::test::call_and_read_body_json(
        &app_org,
        crate::tests::routes::org::helpers::test_org_api::post(
            &token,
            &crate::models::org::CreateOrg {
                name: String::from(ORG),
                description: Some(String::from("Test description")),
                owner: String::from(USERNAME),
                ..Default::default()
            },
        ),
    )
    .await;
    let upserted_org: crate::models::org::Org = serde_json::from_value(upserted_org_val.clone())
        .unwrap_or_else(|_| panic!("{}", upserted_org_val));

    let create_repo = crate::models::repo::CreateRepo {
        id: TEST_ID.to_i32().unwrap() + &15i32 + NUM as i32,
        name: repo_name.to_owned(),
        description: Some(format!(
            "Repo made by {} with org {}",
            USERNAME, upserted_org.owner
        )),
        org: String::from(upserted_org.name),
        ..Default::default()
    };
    let upserted_error_val: serde_json::Value = actix_web::test::call_and_read_body_json(
        &app,
        test_repo_api::post(&user1_token, &create_repo.org, &create_repo),
    )
    .await;
    let unauthorised_error: serde_json::Value = serde_json::from_str(
        r#"{
      "error": "AuthError",
      "error_message": "Unauthorised(\"Owner of repo does not match owner of requestor\")"
      }"#,
    )
    .unwrap();
    assert_eq!(unauthorised_error, upserted_error_val);

    /* cleanup org */
    assert_eq!(
        actix_web::test::call_service(
            &app_org,
            crate::tests::routes::org::helpers::test_org_api::remove(&token, ORG)
        )
        .await
        .status(),
        actix_web::http::StatusCode::NO_CONTENT
    );

    /* confirm repo never exists */
    let req = test_repo_api::get(ORG, &repo_name);
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
async fn test_get_many_repo() {
    const TEST_ID: usize = 2;
    const ORG: &'static str = ORGS[TEST_ID];
    const USERNAME: &'static str = USERNAMES[TEST_ID];

    let app_org = get_org_app!().await;
    let token = crate::tests::routes::helpers::retrieve_token(USERNAME, PASSWORD).await;

    /* Create org */
    let upserted_org_val: serde_json::Value = actix_web::test::call_and_read_body_json(
        &app_org,
        crate::tests::routes::org::helpers::test_org_api::post(
            &token,
            &crate::models::org::CreateOrg {
                name: String::from(ORG),
                description: Some(String::from("Test description")),
                owner: String::from(USERNAME),
                ..Default::default()
            },
        ),
    )
    .await;
    let _upserted_org: crate::models::org::Org = serde_json::from_value(upserted_org_val.clone())
        .unwrap_or_else(|_| panic!("{}", upserted_org_val));

    /* GET many repo */
    let resp =
        actix_web::test::call_service(&get_repo_app!().await, test_repo_api::get_many(ORG)).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    /* cleanup org */
    assert_eq!(
        actix_web::test::call_service(
            &app_org,
            crate::tests::routes::org::helpers::test_org_api::remove(&token, ORG)
        )
        .await
        .status(),
        actix_web::http::StatusCode::NO_CONTENT
    );
}
