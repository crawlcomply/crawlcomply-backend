/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::models::users::Users;
use crate::schema::*;

pub type ConnectionType = diesel::pg::PgConnection;

/// Struct representing a row in table `org`
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
#[diesel(table_name=org, primary_key(name), belongs_to(Users, foreign_key=owner))]
pub struct Org {
    /// Field representing column `name`
    pub name: String,
    /// Field representing column `description`
    pub description: Option<String>,
    /// Field representing column `github_id`
    pub github_id: Option<String>,
    /// Field representing column `avatar_url`
    pub avatar_url: Option<String>,
    /// Field representing column `owner`
    pub owner: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

impl Default for Org {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: None,
            github_id: None,
            avatar_url: None,
            owner: String::new(),
            created_at: Default::default(),
        }
    }
}

/// Create Struct for a row in table `org` for [`Org`]
#[derive(
    utoipa::ToSchema, Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable,
)]
#[diesel(table_name=org)]
pub struct CreateOrg {
    /// Field representing column `name`
    pub name: String,
    /// Field representing column `description`
    pub description: Option<String>,
    /// Field representing column `github_id`
    pub github_id: Option<String>,
    /// Field representing column `avatar_url`
    pub avatar_url: Option<String>,
    /// Field representing column `owner`
    pub owner: String,
}

impl Default for CreateOrg {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: None,
            github_id: None,
            avatar_url: None,
            owner: String::new(),
        }
    }
}

/// Update Struct for a row in table `org` for [`Org`]
#[derive(
    utoipa::ToSchema,
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::AsChangeset,
    PartialEq,
)]
#[diesel(table_name=org)]
pub struct UpdateOrg {
    /// Field representing column `description`
    pub description: Option<Option<String>>,
    /// Field representing column `github_id`
    pub github_id: Option<Option<String>>,
    /// Field representing column `avatar_url`
    pub avatar_url: Option<Option<String>>,
    /// Field representing column `owner`
    pub owner: Option<String>,
    /// Field representing column `created_at`
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl Default for UpdateOrg {
    fn default() -> Self {
        Self {
            description: None,
            github_id: None,
            avatar_url: None,
            owner: None,
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

impl Org {
    /// Insert a new row into `org` with a given [`CreateOrg`]
    pub fn create(db: &mut ConnectionType, item: &CreateOrg) -> diesel::QueryResult<Self> {
        use crate::schema::org::dsl::*;

        diesel::insert_into(org).values(item).get_result::<Self>(db)
    }

    /// Get a row from `org`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_name: String) -> diesel::QueryResult<Self> {
        use crate::schema::org::dsl::*;

        org.filter(name.eq(param_name)).first::<Self>(db)
    }

    /// Update a row in `org`, identified by the primary key with [`UpdateOrg`]
    pub fn update(
        db: &mut ConnectionType,
        param_name: String,
        item: &UpdateOrg,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::org::dsl::*;

        diesel::update(org.filter(name.eq(param_name)))
            .set(item)
            .get_result(db)
    }

    /// Delete a row in `org`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_name: String) -> diesel::QueryResult<usize> {
        use crate::schema::org::dsl::*;

        diesel::delete(org.filter(name.eq(param_name))).execute(db)
    }
}
