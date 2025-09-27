pub(crate) const USERNAMES: [&'static str; 4] =
    ["username40", "username41", "username42", "username43"];
pub(crate) const PASSWORD: &'static str = "password";

pub(crate) const ORGS: [&'static str; 4] = ["org40", "org41", "org42", "org43"];
pub(crate) const RUN_HISTORIES: [&'static str; 2] = ["run_history10", "run_history11"];

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
        run_history: &crate::models::run_history::CreateRunHistory,
    ) -> actix_http::Request {
        assert_eq!(
            type_name_of_val(&crate::routes::run_history::upsert),
            "crawlcomply_backend::routes::run_history::upsert"
        );
        actix_web::test::TestRequest::post()
            .uri(&format!("/api/v0/org/{org}/run_history"))
            .append_header(("Authorization", format!("Bearer {}", token)))
            .set_json(run_history)
            .to_request()
    }

    /// `crate::routes::run_history::read_many` test
    pub(crate) fn get_many(org: &str) -> actix_http::Request {
        assert_eq!(
            type_name_of_val(&crate::routes::run_history::read_many),
            "crawlcomply_backend::routes::run_history::read_many"
        );
        actix_web::test::TestRequest::get()
            .uri(&format!("/api/v0_public/org/{org}/run_history"))
            .to_request()
    }

    /*
    ```rs
    crate::routes::run_history::read
    ```
    */
    pub(crate) fn get(org: &str, run_history: &str) -> actix_http::Request {
        assert_eq!(
            type_name_of_val(&crate::routes::run_history::read),
            "crawlcomply_backend::routes::run_history::read"
        );
        actix_web::test::TestRequest::get()
            .uri(&format!(
                "/api/v0_public/org/{org}/run_history/{run_history}"
            ))
            .to_request()
    }

    pub(crate) fn remove(token: &str, org: &str, run_history: &str) -> actix_http::Request {
        assert_eq!(
            type_name_of_val(&crate::routes::run_history::remove),
            "crawlcomply_backend::routes::run_history::remove"
        );
        actix_web::test::TestRequest::delete()
            .uri(&format!("/api/v0/org/{org}/run_history/{run_history}"))
            .append_header(("Authorization", format!("Bearer {}", token)))
            .to_request()
    }
}
