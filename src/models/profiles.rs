/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::models::users::Users;
use crate::schema::*;

pub type ConnectionType = diesel::pg::PgConnection;

/// Struct representing a row in table `profiles`
#[derive(
    utoipa::ToSchema,
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::QueryableByName,
    PartialEq,
    diesel::Associations,
    diesel::Identifiable,
)]
#[diesel(table_name=profiles, primary_key(alias), belongs_to(Users, foreign_key=username))]
pub struct Profiles {
    /// Field representing column `alias`
    pub alias: String,
    /// Field representing column `username`
    pub username: String,
    /// Field representing column `description`
    pub description: Option<String>,
    /// Field representing column `github_id`
    pub github_id: Option<String>,
    /// Field representing column `avatar_url`
    pub avatar_url: Option<String>,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

impl Default for Profiles {
    fn default() -> Self {
        Self {
            alias: String::new(),
            username: String::new(),
            description: None,
            github_id: None,
            avatar_url: None,
            created_at: Default::default(),
        }
    }
}

/// Create Struct for a row in table `profiles` for [`Profiles`]
#[derive(
    utoipa::ToSchema, Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable,
)]
#[diesel(table_name=profiles)]
pub struct CreateProfiles {
    /// Field representing column `alias`
    pub alias: String,
    /// Field representing column `username`
    pub username: String,
    /// Field representing column `description`
    pub description: Option<String>,
    /// Field representing column `github_id`
    pub github_id: Option<String>,
    /// Field representing column `avatar_url`
    pub avatar_url: Option<String>,
}

impl Default for CreateProfiles {
    fn default() -> Self {
        Self {
            alias: String::new(),
            username: String::new(),
            description: None,
            github_id: None,
            avatar_url: None,
        }
    }
}

/// Update Struct for a row in table `profiles` for [`Profiles`]
#[derive(
    utoipa::ToSchema,
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::AsChangeset,
    PartialEq,
)]
#[diesel(table_name=profiles)]
pub struct UpdateProfiles {
    /// Field representing column `username`
    pub username: Option<String>,
    /// Field representing column `description`
    pub description: Option<Option<String>>,
    /// Field representing column `github_id`
    pub github_id: Option<Option<String>>,
    /// Field representing column `avatar_url`
    pub avatar_url: Option<Option<String>>,
    /// Field representing column `created_at`
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl Default for UpdateProfiles {
    fn default() -> Self {
        Self {
            username: None,
            description: None,
            github_id: None,
            avatar_url: None,
            created_at: None,
        }
    }
}

/// Result of a `.paginate` function
#[derive(Debug, serde::Serialize)]
pub struct PaginationResult<T> {
    /// Resulting items that are from the current page
    pub items: Vec<T>,
    /// The count of total items there are
    pub total_items: i64,
    /// Current page, 0-based index
    pub page: i64,
    /// Size of a page
    pub page_size: i64,
    /// Number of total possible pages, given the `page_size` and `total_items`
    pub num_pages: i64,
}

impl Profiles {
    /// Insert a new row into `profiles` with a given [`CreateProfiles`]
    pub fn create(db: &mut ConnectionType, item: &CreateProfiles) -> diesel::QueryResult<Self> {
        use crate::schema::profiles::dsl::*;

        diesel::insert_into(profiles)
            .values(item)
            .get_result::<Self>(db)
    }

    /// Get a row from `profiles`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_alias: String) -> diesel::QueryResult<Self> {
        use crate::schema::profiles::dsl::*;

        profiles.filter(alias.eq(param_alias)).first::<Self>(db)
    }

    /// Update a row in `profiles`, identified by the primary key with [`UpdateProfiles`]
    pub fn update(
        db: &mut ConnectionType,
        param_alias: String,
        item: &UpdateProfiles,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::profiles::dsl::*;

        diesel::update(profiles.filter(alias.eq(param_alias)))
            .set(item)
            .get_result(db)
    }

    /// Delete a row in `profiles`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_alias: String) -> diesel::QueryResult<usize> {
        use crate::schema::profiles::dsl::*;

        diesel::delete(profiles.filter(alias.eq(param_alias))).execute(db)
    }
}
