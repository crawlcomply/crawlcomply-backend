pub(crate) const USERNAMES: [&'static str; 4] =
    ["username20", "username21", "username22", "username23"];
pub(crate) const PASSWORD: &'static str = "password";

pub(crate) const ORGS: [&'static str; 4] = ["org20", "org21", "org22", "org23"];
pub(crate) const REPOS: [&'static str; 2] = ["repo10", "repo11"];

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
                        .service(crate::routes::repo::read)
                        .service(crate::routes::repo::remove),
                ),
        )
    };
}

pub(crate) mod test_repo_api {
    pub(crate) fn post(token: &str, repo: &crate::models::repo::CreateRepo) -> actix_http::Request {
        actix_web::test::TestRequest::post()
            .uri("/api/v0/repo")
            .append_header(("Authorization", format!("Bearer {}", token)))
            .set_json(repo)
            .to_request()
    }

    pub(crate) fn get(token: &str, repo_name: &str) -> actix_http::Request {
        actix_web::test::TestRequest::get()
            .uri(format!("/api/v0/repo/{}", repo_name).as_str())
            .append_header(("Authorization", format!("Bearer {}", token)))
            .to_request()
    }

    pub(crate) fn remove(token: &str, repo_name: &str) -> actix_http::Request {
        actix_web::test::TestRequest::delete()
            .uri(format!("/api/v0/repo/{}", repo_name).as_str())
            .append_header(("Authorization", format!("Bearer {}", token)))
            .to_request()
    }
}
