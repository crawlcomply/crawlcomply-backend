use diesel_migrations::MigrationHarness;

pub mod diesel {
    pub use diesel::*;
}

pub mod models;

pub mod routes;

pub mod schema;

#[cfg(test)]
mod tests;

pub const CARGO_PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const CARGO_PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations =
    diesel_migrations::embed_migrations!("./migrations");

pub fn db_init() {
    log::info!("Initializing DB");
    lazy_static::initialize(&rust_actix_diesel_auth_scaffold::POOL);
    let mut connection = rust_actix_diesel_auth_scaffold::establish_connection()
        .expect("Failed to create connection");
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
}

/* #[derive(utoipa::ToSchema, serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[schema(as = std::time::SystemTime, value_type = std::time::SystemTime)]
pub struct SchemaSystemTime(std::time::SystemTime); */

pub fn utoipa_for_system_time() -> utoipa::openapi::Object {
    utoipa::openapi::ObjectBuilder::new()
        .schema_type(utoipa::openapi::Type::String)
        .format(Some(utoipa::openapi::SchemaFormat::Custom(String::from(
            "SystemTime",
        ))))
        .build()
}
