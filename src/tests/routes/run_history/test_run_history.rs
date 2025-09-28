use core::assert_eq;

use actix_http::body::MessageBody as _;

use crate::models::run_history::RunHistory;
use crate::tests::routes::run_history::helpers::{
    test_run_history_api, NUM, ORGS, PASSWORD, REPOS, REPO_HASHES, USERNAMES,
};
use crate::{get_org_app, get_repo_app, get_repo_history_app, get_run_history_app};

#[actix_web::test]
async fn test_upsert_read_delete() {
    const TEST_ID: usize = 0;
    const USERNAME: &'static str = USERNAMES[TEST_ID];
    const ORG: &'static str = ORGS[TEST_ID];
    const REPO: &'static str = REPOS[TEST_ID];
    const HASH: &'static str = REPO_HASHES[TEST_ID];

    let app = get_run_history_app!().await;
    let app_org = get_org_app!().await;
    let app_repo = get_repo_app!().await;
    let app_repo_history = get_repo_history_app!().await;
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
                id: NUM as i32,
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
    let _upserted_repo_history: crate::models::repo_history::RepoHistory =
        actix_web::test::call_and_read_body_json(
            &app_repo_history,
            crate::tests::routes::repo_history::helpers::test_repo_history_api::post(
                &token,
                ORG,
                REPO,
                &crate::models::repo_history::CreateRepoHistory {
                    full_name: format!("{ORG}/{REPO}"),
                    commit: HASH.to_owned(),
                    ..Default::default()
                },
            ),
        )
        .await;

    /* POST run_history */
    let create_run_history = crate::models::run_history::CreateRunHistory {
        full_name: format!("{ORG}/{REPO}"),
        commit: HASH.to_owned(),
        ..Default::default()
    };
    let upserted_run_history_val: serde_json::Value = actix_web::test::call_and_read_body_json(
        &app,
        test_run_history_api::post(&token, ORG, REPO, &create_run_history),
    )
    .await;
    let upserted_run_history: RunHistory = serde_json::from_value(upserted_run_history_val.clone())
        .unwrap_or_else(|_| panic!("{}", upserted_run_history_val));

    /* GET run_history */
    let read_run_history_val: serde_json::Value = actix_web::test::call_and_read_body_json(
        &app,
        test_run_history_api::get(ORG, REPO, &upserted_run_history.run),
    )
    .await;
    let read_run_history: RunHistory = serde_json::from_value(read_run_history_val.clone())
        .unwrap_or_else(|_| panic!("{}", read_run_history_val));

    /* cmp */
    assert_eq!(
        upserted_run_history,
        RunHistory {
            id: upserted_run_history.id.to_owned(),
            full_name: format!("{ORG}/{REPO}"),
            commit: HASH.to_owned(),
            created_at: read_run_history.created_at,
            run: upserted_run_history.run,
            status: upserted_run_history.status.to_owned(),
            ..Default::default()
        }
    );
    assert_eq!(upserted_run_history, read_run_history);

    /* cleanup run_history */
    let resp = actix_web::test::call_service(
        &app,
        test_run_history_api::remove(&token, ORG, REPO, &upserted_run_history.run),
    )
    .await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NO_CONTENT);
    assert_eq!(
        resp.response().body().size(),
        actix_web::body::BodySize::Sized(0)
    );
    /* cleanup repo_history */
    assert_eq!(
        actix_web::test::call_service(
            &app_repo_history,
            crate::tests::routes::repo_history::helpers::test_repo_history_api::remove(
                &token, ORG, REPO, HASH
            )
        )
        .await
        .status(),
        actix_web::http::StatusCode::NO_CONTENT
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

    /* confirm run_history no longer exists */
    let req = test_run_history_api::get(ORG, REPO, &upserted_run_history.run);
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
async fn test_update_run_history_you_do_not_own() {
    const TEST_ID: usize = 1;
    const USERNAME: &'static str = USERNAMES[TEST_ID];
    const USERNAME1: &'static str = USERNAMES[USERNAMES.len() - 1];
    const ORG: &'static str = ORGS[TEST_ID];
    const REPO: &'static str = REPOS[TEST_ID];
    const HASH: &'static str = REPO_HASHES[TEST_ID];

    let app = get_run_history_app!().await;
    let app_org = get_org_app!().await;
    let app_repo = get_repo_app!().await;
    let app_repo_history = get_repo_history_app!().await;
    let token = crate::tests::routes::helpers::retrieve_token(USERNAME, PASSWORD).await;
    let user1_token = crate::tests::routes::helpers::retrieve_token(USERNAME1, PASSWORD).await;

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
                id: NUM as i32,
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
    let _upserted_repo_history: crate::models::repo_history::RepoHistory =
        actix_web::test::call_and_read_body_json(
            &app_repo_history,
            crate::tests::routes::repo_history::helpers::test_repo_history_api::post(
                &token,
                ORG,
                REPO,
                &crate::models::repo_history::CreateRepoHistory {
                    full_name: format!("{ORG}/{REPO}"),
                    commit: HASH.to_owned(),
                    ..Default::default()
                },
            ),
        )
        .await;

    /* POST run_history */
    let create_run_history = crate::models::run_history::CreateRunHistory {
        full_name: format!("{ORG}/{REPO}"),
        commit: String::from(HASH),
        ..Default::default()
    };
    let upserted_error_val: serde_json::Value = actix_web::test::call_and_read_body_json(
        &app,
        test_run_history_api::post(&user1_token, ORG, REPO, &create_run_history),
    )
    .await;
    let unauthorised_error: serde_json::Value = serde_json::from_str(
        r#"{
      "error": "AuthError",
      "error_message": "Unauthorised(\"Owner of run_history org does not match owner of requestor\")"
      }"#,
    )
        .unwrap();
    assert_eq!(unauthorised_error, upserted_error_val);

    /* cleanup repo_history */
    assert_eq!(
        actix_web::test::call_service(
            &app_repo_history,
            crate::tests::routes::repo_history::helpers::test_repo_history_api::remove(
                &token, ORG, REPO, HASH
            )
        )
        .await
        .status(),
        actix_web::http::StatusCode::NO_CONTENT
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
}

#[actix_web::test]
async fn test_get_many_run_history() {
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

    /* GET many run_history */
    let resp = actix_web::test::call_service(
        &get_run_history_app!().await,
        test_run_history_api::get_many(ORG, REPO),
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
