/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::models::users::Users;
use crate::schema::*;

pub type ConnectionType = diesel::pg::PgConnection;

/// Struct representing a row in table `profile`
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
#[diesel(table_name=profile, primary_key(alias), belongs_to(Users, foreign_key=username))]
pub struct Profile {
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

impl Default for Profile {
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

/// Create Struct for a row in table `profile` for [`Profile`]
#[derive(
    utoipa::ToSchema, Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable,
)]
#[diesel(table_name=profile)]
pub struct CreateProfile {
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

impl Default for CreateProfile {
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

/// Update Struct for a row in table `profile` for [`Profile`]
#[derive(
    utoipa::ToSchema,
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::AsChangeset,
    PartialEq,
)]
#[diesel(table_name=profile)]
pub struct UpdateProfile {
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

impl Default for UpdateProfile {
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

impl Profile {
    /// Insert a new row into `profile` with a given [`CreateProfile`]
    pub fn create(db: &mut ConnectionType, item: &CreateProfile) -> diesel::QueryResult<Self> {
        use crate::schema::profile::dsl::*;

        diesel::insert_into(profile)
            .values(item)
            .get_result::<Self>(db)
    }

    /// Get a row from `profile`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_alias: String) -> diesel::QueryResult<Self> {
        use crate::schema::profile::dsl::*;

        profile.filter(alias.eq(param_alias)).first::<Self>(db)
    }

    /// Update a row in `profile`, identified by the primary key with [`UpdateProfile`]
    pub fn update(
        db: &mut ConnectionType,
        param_alias: String,
        item: &UpdateProfile,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::profile::dsl::*;

        diesel::update(profile.filter(alias.eq(param_alias)))
            .set(item)
            .get_result(db)
    }

    /// Delete a row in `profile`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_alias: String) -> diesel::QueryResult<usize> {
        use crate::schema::profile::dsl::*;

        diesel::delete(profile.filter(alias.eq(param_alias))).execute(db)
    }
}
