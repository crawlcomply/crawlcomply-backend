pub(crate) const USERNAMES: [&'static str; 4] =
    ["username30", "username31", "username32", "username33"];
pub(crate) const PASSWORD: &'static str = "password";

pub(crate) const ORGS: [&'static str; 4] = ["org30", "org31", "org32", "org33"];
pub(crate) const REPOS: [&'static str; 4] = ["repo20", "repo21", "repo22", "repo23"];

pub(crate) const REPO_HASHES: [&'static str; 4] = [
    "05f0661bd71ce28832f111348153d7b0b701fad0",
    "15df914412468c2191bb99d7bdf5ba7af160ee71",
    "26b93c58ea22bb05d4a5476cd92b339a250c7e92",
    "33cb7621f14face3f47cc03ead2a470f439b8bb3",
];

#[macro_export]
macro_rules! get_repo_history_app {
    () => {
        actix_web::test::init_service(
            actix_web::App::new()
                .app_data(actix_web::web::Data::new(
                    rust_actix_diesel_auth_scaffold::POOL.clone(),
                ))
                .service(
                    actix_web::web::scope("/api/v0")
                        .wrap(actix_web::middleware::Compat::new(
                            actix_web_httpauth::middleware::HttpAuthentication::bearer(
                                rust_actix_diesel_auth_scaffold::middleware::bearer::validator,
                            ),
                        ))
                        .service(crate::routes::repo_history::upsert)
                        .service(crate::routes::repo_history::remove),
                )
                .service(
                    actix_web::web::scope("/api/v0_public")
                        .service(crate::routes::repo_history::read)
                        .service(crate::routes::repo_history::read_many),
                ),
        )
    };
}

pub(crate) mod test_repo_history_api {
    use crate::tests::routes::helpers::type_name_of_val;

    pub(crate) fn post(
        token: &str,
        org: &str,
        repo: &str,
        repo_history: &crate::models::repo_history::CreateRepoHistory,
    ) -> actix_http::Request {
        assert_eq!(
            type_name_of_val(&crate::routes::repo_history::upsert),
            "crawlcomply_backend::routes::repo_history::upsert"
        );
        actix_web::test::TestRequest::post()
            .uri(&format!("/api/v0/org/{org}/repo/{repo}/history"))
            .append_header(("Authorization", format!("Bearer {}", token)))
            .set_json(repo_history)
            .to_request()
    }

    /// `crate::routes::repo_history::read_many` test
    pub(crate) fn get_many(org: &str, repo: &str) -> actix_http::Request {
        assert_eq!(
            type_name_of_val(&crate::routes::repo_history::read_many),
            "crawlcomply_backend::routes::repo_history::read_many"
        );
        actix_web::test::TestRequest::get()
            .uri(&format!("/api/v0_public/org/{org}/repo/{repo}/history"))
            .to_request()
    }

    /*
    ```rs
    crate::routes::repo_history::read
    ```
    */
    pub(crate) fn get(org: &str, repo: &str, hash: &str) -> actix_http::Request {
        assert_eq!(
            type_name_of_val(&crate::routes::repo_history::read),
            "crawlcomply_backend::routes::repo_history::read"
        );
        actix_web::test::TestRequest::get()
            .uri(&format!(
                "/api/v0_public/org/{org}/repo/{repo}/history/{hash}"
            ))
            .to_request()
    }

    pub(crate) fn remove(token: &str, org: &str, repo: &str, hash: &str) -> actix_http::Request {
        assert_eq!(
            type_name_of_val(&crate::routes::repo_history::remove),
            "crawlcomply_backend::routes::repo_history::remove"
        );
        actix_web::test::TestRequest::delete()
            .uri(&format!("/api/v0/org/{org}/repo/{repo}/history/{hash}"))
            .append_header(("Authorization", format!("Bearer {}", token)))
            .to_request()
    }
}
