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
                )
                .service(
                    actix_web::web::scope("/api/v0_public")
                        .service(crate::routes::profile::read_many),
                ),
        )
    };
}

pub(crate) mod test_profile_api {
    use crate::tests::routes::helpers::type_name_of_val;

    pub(crate) fn post(token: &str, alias: &str, username: &'static str) -> actix_http::Request {
        assert_eq!(
            type_name_of_val(&crate::routes::profile::upsert),
            "crawlcomply_backend::routes::profile::upsert"
        );
        actix_web::test::TestRequest::post()
            .uri("/api/v0/profile")
            .append_header(("Authorization", format!("Bearer {}", token)))
            .set_json(crate::models::profile::CreateProfile {
                alias: String::from(alias),
                username: String::from(username),
                ..crate::models::profile::CreateProfile::default()
            })
            .to_request()
    }

    pub(crate) fn get_many() -> actix_http::Request {
        assert_eq!(
            type_name_of_val(&crate::routes::profile::read_many),
            "crawlcomply_backend::routes::profile::read_many"
        );
        actix_web::test::TestRequest::get()
            .uri("/api/v0_public/profiles")
            .to_request()
    }

    pub(crate) fn get(token: &str) -> actix_http::Request {
        assert_eq!(
            type_name_of_val(&crate::routes::profile::read),
            "crawlcomply_backend::routes::profile::read"
        );
        actix_web::test::TestRequest::get()
            .uri("/api/v0/profile")
            .append_header(("Authorization", format!("Bearer {}", token)))
            .to_request()
    }

    pub(crate) fn remove(token: &str) -> actix_http::Request {
        assert_eq!(
            type_name_of_val(&crate::routes::profile::remove),
            "crawlcomply_backend::routes::profile::remove"
        );
        actix_web::test::TestRequest::delete()
            .uri("/api/v0/profile")
            .append_header(("Authorization", format!("Bearer {}", token)))
            .to_request()
    }
}
