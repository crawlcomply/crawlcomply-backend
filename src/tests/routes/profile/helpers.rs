pub(crate) const USERNAMES: [&'static str; 2] = ["username0", "username1"];
pub(crate) const PASSWORD: &'static str = "password";

#[macro_export]
macro_rules! get_profile_app {
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
                        .service(crate::routes::profile::upsert)
                        .service(crate::routes::profile::read)
                        .service(crate::routes::profile::remove),
                ),
        )
    };
}

pub(crate) mod test_profile_api {
    pub(crate) fn post(token: &str, alias: &str, username: &'static str) -> actix_http::Request {
        actix_web::test::TestRequest::post()
            .uri("/api/v0/profile")
            .append_header(("Authorization", format!("Bearer {}", token)))
            .set_json(crate::models::profiles::CreateProfiles {
                alias: String::from(alias),
                username: String::from(username),
                ..crate::models::profiles::CreateProfiles::default()
            })
            .to_request()
    }

    pub(crate) fn get(token: &str) -> actix_http::Request {
        actix_web::test::TestRequest::get()
            .uri("/api/v0/profile")
            .append_header(("Authorization", format!("Bearer {}", token)))
            .to_request()
    }

    pub(crate) fn remove(token: &str) -> actix_http::Request {
        actix_web::test::TestRequest::delete()
            .uri("/api/v0/profile")
            .append_header(("Authorization", format!("Bearer {}", token)))
            .to_request()
    }
}
