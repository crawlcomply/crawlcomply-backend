use core::assert_eq;

use actix_http::body::MessageBody as _;

use crate::models::repo_history::RepoHistory;
use crate::tests::routes::repo_history::helpers::{
    test_repo_history_api, ORGS, PASSWORD, REPOS, REPO_HASHES, USERNAMES,
};
use crate::{get_org_app, get_repo_app, get_repo_history_app};

#[actix_web::test]
async fn test_upsert_read_delete() {
    const TEST_ID: usize = 0;
    const USERNAME: &'static str = USERNAMES[TEST_ID];
    const ORG: &'static str = ORGS[TEST_ID];
    const REPO: &'static str = REPOS[TEST_ID];
    const HASH: &'static str = REPO_HASHES[TEST_ID];

    let app = get_repo_history_app!().await;
    let app_org = get_org_app!().await;
    let app_repo = get_repo_app!().await;
    let token = crate::tests::routes::helpers::retrieve_token(USERNAME, PASSWORD).await;

    /* POST */

    /* POST org */
    let upserted_org_val: serde_json::Value = actix_web::test::call_and_read_body_json(
        &app_org,
        crate::tests::routes::org::helpers::test_org_api::post(
            &token,
            &crate::models::org::CreateOrg {
                name: String::from(ORG),
                owner: String::from(USERNAME),
                ..Default::default()
            },
        ),
    )
    .await;
    let _upserted_org: crate::models::org::Org = serde_json::from_value(upserted_org_val.clone())
        .unwrap_or_else(|_| panic!("{}", upserted_org_val));

    /* POST repo */
    let upserted_repo_val: serde_json::Value = actix_web::test::call_and_read_body_json(
        &app_repo,
        crate::tests::routes::repo::helpers::test_repo_api::post(
            &token,
            ORG,
            &crate::models::repo::CreateRepo {
                name: REPO.to_owned(),
                org: ORG.to_owned(),
                ..Default::default()
            },
        ),
    )
    .await;
    let _upserted_repo: crate::models::repo::Repo =
        serde_json::from_value(upserted_repo_val.clone())
            .unwrap_or_else(|_| panic!("{}", upserted_repo_val));

    /* POST repo_history */
    let create_repo_history = crate::models::repo_history::CreateRepoHistory {
        full_name: format!("{ORG}/{REPO}"),
        commit: HASH.to_owned(),
        ..Default::default()
    };
    let upserted_repo_history_val: serde_json::Value = actix_web::test::call_and_read_body_json(
        &app,
        test_repo_history_api::post(&token, ORG, REPO, &create_repo_history),
    )
    .await;
    let upserted_repo_history: RepoHistory =
        serde_json::from_value(upserted_repo_history_val.clone())
            .unwrap_or_else(|_| panic!("{}", upserted_repo_history_val));

    /* GET repo_history */
    let read_repo_history_val: serde_json::Value = actix_web::test::call_and_read_body_json(
        &app,
        test_repo_history_api::get(ORG, REPO, &upserted_repo_history.commit),
    )
    .await;
    let read_repo_history: RepoHistory = serde_json::from_value(read_repo_history_val.clone())
        .unwrap_or_else(|_| panic!("{}", read_repo_history_val));

    /* cmp */
    assert_eq!(
        upserted_repo_history,
        RepoHistory {
            id: upserted_repo_history.id.to_owned(),
            full_name: format!("{ORG}/{REPO}"),
            commit: HASH.to_owned(),
            created_at: read_repo_history.created_at,
            ..Default::default()
        }
    );
    assert_eq!(upserted_repo_history, read_repo_history);

    /* cleanup repo_history */
    let resp =
        actix_web::test::call_service(&app, test_repo_history_api::remove(&token, ORG, REPO, HASH))
            .await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NO_CONTENT);
    assert_eq!(
        resp.response().body().size(),
        actix_web::body::BodySize::Sized(0)
    );
    /* cleanup repo */
    assert_eq!(
        actix_web::test::call_service(
            &app_repo,
            crate::tests::routes::repo::helpers::test_repo_api::remove(&token, ORG, REPO)
        )
        .await
        .status(),
        actix_web::http::StatusCode::NO_CONTENT
    );
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

    /* confirm repo_history no longer exists */
    let req = test_repo_history_api::get(ORG, REPO, &upserted_repo_history.commit);
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
async fn test_update_repo_history_you_do_not_own() {
    const TEST_ID: usize = 1;
    const USERNAME: &'static str = USERNAMES[TEST_ID];
    const USERNAME1: &'static str = USERNAMES[USERNAMES.len() - 1];
    const ORG: &'static str = ORGS[TEST_ID];
    const REPO: &'static str = REPOS[TEST_ID];
    const HASH: &'static str = REPO_HASHES[TEST_ID];

    let app = get_repo_history_app!().await;
    let token = crate::tests::routes::helpers::retrieve_token(USERNAME, PASSWORD).await;
    let user1_token = crate::tests::routes::helpers::retrieve_token(USERNAME1, PASSWORD).await;

    let app_org = get_org_app!().await;
    let app_repo = get_repo_app!().await;

    /* POST */
    let upserted_org_val: serde_json::Value = actix_web::test::call_and_read_body_json(
        &app_org,
        crate::tests::routes::org::helpers::test_org_api::post(
            &token,
            &crate::models::org::CreateOrg {
                name: ORG.to_owned(),
                owner: USERNAME.to_owned(),
                ..Default::default()
            },
        ),
    )
    .await;
    let _upserted_org: crate::models::org::Org = serde_json::from_value(upserted_org_val.clone())
        .unwrap_or_else(|_| panic!("{}", upserted_org_val));

    let upserted_repo_val: serde_json::Value = actix_web::test::call_and_read_body_json(
        &app_repo,
        crate::tests::routes::repo::helpers::test_repo_api::post(
            &token,
            ORG,
            &crate::models::repo::CreateRepo {
                name: String::from(REPO),
                ..Default::default()
            },
        ),
    )
    .await;
    let _upserted_repo: crate::models::repo::Repo =
        serde_json::from_value(upserted_repo_val.clone())
            .unwrap_or_else(|_| panic!("{}", upserted_repo_val));

    let create_repo_history = crate::models::repo_history::CreateRepoHistory {
        full_name: format!("{ORG}/{REPO}"),
        commit: String::from(HASH),
        ..Default::default()
    };
    let upserted_error_val: serde_json::Value = actix_web::test::call_and_read_body_json(
        &app,
        test_repo_history_api::post(&user1_token, ORG, REPO, &create_repo_history),
    )
    .await;
    let unauthorised_error: serde_json::Value = serde_json::from_str(
        r#"{
      "error": "AuthError",
      "error_message": "Unauthorised(\"Owner of repo_history org does not match owner of requestor\")"
      }"#,
    )
    .unwrap();
    assert_eq!(unauthorised_error, upserted_error_val);

    /* cleanup repo */
    assert_eq!(
        actix_web::test::call_service(
            &app_repo,
            crate::tests::routes::repo::helpers::test_repo_api::remove(&token, ORG, REPO)
        )
        .await
        .status(),
        actix_web::http::StatusCode::NO_CONTENT
    );

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

    /* confirm repo_history never exists */
    let req = test_repo_history_api::get(ORG, REPO, HASH);
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
async fn test_get_many_repo_history() {
    const TEST_ID: usize = 2;
    const ORG: &'static str = ORGS[TEST_ID];
    const USERNAME: &'static str = USERNAMES[TEST_ID];
    const REPO: &'static str = REPOS[TEST_ID];

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

    /* GET many repo_history */
    let resp = actix_web::test::call_service(
        &get_repo_history_app!().await,
        test_repo_history_api::get_many(ORG, REPO),
    )
    .await;
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
