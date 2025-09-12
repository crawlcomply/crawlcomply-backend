pub async fn retrieve_token(username: &str, password: &str) -> String {
    rust_actix_diesel_auth_scaffold::establish_connection().unwrap();
    crate::tests::routes::INIT_DB.call_once(|| {
        rust_actix_diesel_auth_scaffold::db_init();
        crate::db_init();
    });

    let token =
        rust_actix_diesel_auth_scaffold::get_token(String::from(username), String::from(password))
            .await;
    token
}
