pub(crate) const USERNAMES: [&'static str; 4] =
    ["username10", "username11", "username12", "username13"];
pub(crate) const PASSWORD: &'static str = "password";

pub(crate) const ORGS: [&'static str; 2] = ["org10", "org11"];

#[macro_export]
macro_rules! get_org_app {
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
                        .service(crate::routes::org::upsert)
                        .service(crate::routes::org::read)
                        .service(crate::routes::org::remove),
                ),
        )
    };
}

pub(crate) mod test_org_api {
    pub(crate) fn post(token: &str, org: &crate::models::org::CreateOrg) -> actix_http::Request {
        actix_web::test::TestRequest::post()
            .uri("/api/v0/org")
            .append_header(("Authorization", format!("Bearer {}", token)))
            .set_json(org)
            .to_request()
    }

    pub(crate) fn get(token: &str, org_name: &str) -> actix_http::Request {
        actix_web::test::TestRequest::get()
            .uri(format!("/api/v0/org/{}", org_name).as_str())
            .append_header(("Authorization", format!("Bearer {}", token)))
            .to_request()
    }

    pub(crate) fn remove(token: &str, org_name: &str) -> actix_http::Request {
        actix_web::test::TestRequest::delete()
            .uri(format!("/api/v0/org/{}", org_name).as_str())
            .append_header(("Authorization", format!("Bearer {}", token)))
            .to_request()
    }
}
