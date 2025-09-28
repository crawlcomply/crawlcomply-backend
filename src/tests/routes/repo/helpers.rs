pub(crate) const NUM: u8 = 20;
pub(crate) const USERNAMES: [&'static str; 4] =
    ["username20", "username21", "username22", "username23"];
pub(crate) const PASSWORD: &'static str = "password";

pub(crate) const ORGS: [&'static str; 4] = ["org20", "org21", "org22", "org23"];
pub(crate) const REPOS: [&'static str; 2] = ["repo20", "repo21"];

#[macro_export]
macro_rules! get_repo_app {
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
                        .service(crate::routes::repo::upsert)
                        .service(crate::routes::repo::remove),
                )
                .service(
                    actix_web::web::scope("/api/v0_public")
                        .service(crate::routes::repo::read)
                        .service(crate::routes::repo::read_many),
                ),
        )
    };
}

pub(crate) mod test_repo_api {
    use crate::tests::routes::helpers::type_name_of_val;

    pub(crate) fn post(
        token: &str,
        org: &str,
        repo: &crate::models::repo::CreateRepo,
    ) -> actix_http::Request {
        assert_eq!(
            type_name_of_val(&crate::routes::repo::upsert),
            "crawlcomply_backend::routes::repo::upsert"
        );
        actix_web::test::TestRequest::post()
            .uri(&format!("/api/v0/org/{org}/repo"))
            .append_header(("Authorization", format!("Bearer {}", token)))
            .set_json(repo)
            .to_request()
    }

    /// `crate::routes::repo::read_many` test
    pub(crate) fn get_many(org: &str) -> actix_http::Request {
        assert_eq!(
            type_name_of_val(&crate::routes::repo::read_many),
            "crawlcomply_backend::routes::repo::read_many"
        );
        actix_web::test::TestRequest::get()
            .uri(&format!("/api/v0_public/org/{org}/repo"))
            .to_request()
    }

    /*
    ```rs
    crate::routes::repo::read
    ```
    */
    pub(crate) fn get(org: &str, repo: &str) -> actix_http::Request {
        assert_eq!(
            type_name_of_val(&crate::routes::repo::read),
            "crawlcomply_backend::routes::repo::read"
        );
        actix_web::test::TestRequest::get()
            .uri(&format!("/api/v0_public/org/{org}/repo/{repo}"))
            .to_request()
    }

    pub(crate) fn remove(token: &str, org: &str, repo: &str) -> actix_http::Request {
        assert_eq!(
            type_name_of_val(&crate::routes::repo::remove),
            "crawlcomply_backend::routes::repo::remove"
        );
        actix_web::test::TestRequest::delete()
            .uri(&format!("/api/v0/org/{org}/repo/{repo}"))
            .append_header(("Authorization", format!("Bearer {}", token)))
            .to_request()
    }
}
