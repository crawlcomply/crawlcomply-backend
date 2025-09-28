pub(crate) const NUM: u8 = 40;
pub(crate) const USERNAMES: [&'static str; 5] = [
    "username40",
    "username41",
    "username42",
    "username43",
    "username44",
];
pub(crate) const PASSWORD: &'static str = "password";

pub(crate) const ORGS: [&'static str; 4] = ["org40", "org41", "org42", "org43"];
pub(crate) const REPOS: [&'static str; 4] = ["repo40", "repo41", "repo42", "repo43"];

pub(crate) const REPO_HASHES: [&'static str; 4] = [
    "45f0661bd71ce28832f111348153d7b0b701fad4",
    "55df914412468c2191bb99d7bdf5ba7af160ee75",
    "66b93c58ea22bb05d4a5476cd92b339a250c7e96",
    "73cb7621f14face3f47cc03ead2a470f439b8bb7",
];

#[macro_export]
macro_rules! get_run_history_app {
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
                        .service(crate::routes::run_history::upsert)
                        .service(crate::routes::run_history::remove),
                )
                .service(
                    actix_web::web::scope("/api/v0_public")
                        .service(crate::routes::run_history::read)
                        .service(crate::routes::run_history::read_many),
                ),
        )
    };
}

pub(crate) mod test_run_history_api {
    use crate::tests::routes::helpers::type_name_of_val;

    pub(crate) fn post(
        token: &str,
        org: &str,
        repo: &str,
        run_history: &crate::models::run_history::CreateRunHistory,
    ) -> actix_http::Request {
        assert_eq!(
            type_name_of_val(&crate::routes::run_history::upsert),
            "crawlcomply_backend::routes::run_history::upsert"
        );
        actix_web::test::TestRequest::post()
            .uri(&format!("/api/v0/org/{org}/repo/{repo}/run"))
            .append_header(("Authorization", format!("Bearer {}", token)))
            .set_json(run_history)
            .to_request()
    }

    /// `crate::routes::run_history::read_many` test
    pub(crate) fn get_many(org: &str, repo: &str) -> actix_http::Request {
        assert_eq!(
            type_name_of_val(&crate::routes::run_history::read_many),
            "crawlcomply_backend::routes::run_history::read_many"
        );
        actix_web::test::TestRequest::get()
            .uri(&format!("/api/v0_public/org/{org}/repo/{repo}/run"))
            .to_request()
    }

    /*
    ```rs
    crate::routes::run_history::read
    ```
    */
    pub(crate) fn get(org: &str, repo: &str, run: &i32) -> actix_http::Request {
        assert_eq!(
            type_name_of_val(&crate::routes::run_history::read),
            "crawlcomply_backend::routes::run_history::read"
        );
        actix_web::test::TestRequest::get()
            .uri(&format!("/api/v0_public/org/{org}/repo/{repo}/run/{run}"))
            .to_request()
    }

    pub(crate) fn remove(token: &str, org: &str, repo: &str, run: &i32) -> actix_http::Request {
        assert_eq!(
            type_name_of_val(&crate::routes::run_history::remove),
            "crawlcomply_backend::routes::run_history::remove"
        );
        actix_web::test::TestRequest::delete()
            .uri(&format!("/api/v0/org/{org}/repo/{repo}/run/{run}"))
            .append_header(("Authorization", format!("Bearer {}", token)))
            .to_request()
    }
}
