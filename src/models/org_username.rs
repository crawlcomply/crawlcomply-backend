/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::models::org::Org;
use crate::models::users::Users;
use crate::schema::*;

pub type ConnectionType = diesel::pg::PgConnection;

/// Struct representing a row in table `org_username`
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
#[diesel(table_name=org_username, primary_key(group_name,username), belongs_to(Org, foreign_key=group_name) , belongs_to(Users, foreign_key=username))]
pub struct OrgUsername {
    /// Field representing column `group_name`
    pub group_name: String,
    /// Field representing column `username`
    pub username: String,
    /// Field representing column `role`
    pub role: Option<String>,
}

impl Default for OrgUsername {
    fn default() -> Self {
        Self {
            group_name: String::new(),
            username: String::new(),
            role: None,
        }
    }
}

/// Create Struct for a row in table `org_username` for [`OrgUsername`]
#[derive(
    utoipa::ToSchema, Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable,
)]
#[diesel(table_name=org_username)]
pub struct CreateOrgUsername {
    /// Field representing column `group_name`
    pub group_name: String,
    /// Field representing column `username`
    pub username: String,
    /// Field representing column `role`
    pub role: Option<String>,
}

impl Default for CreateOrgUsername {
    fn default() -> Self {
        Self {
            group_name: String::new(),
            username: String::new(),
            role: None,
        }
    }
}

/// Update Struct for a row in table `org_username` for [`OrgUsername`]
#[derive(
    utoipa::ToSchema,
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::AsChangeset,
    PartialEq,
)]
#[diesel(table_name=org_username)]
pub struct UpdateOrgUsername {
    /// Field representing column `role`
    pub role: Option<Option<String>>,
}

impl Default for UpdateOrgUsername {
    fn default() -> Self {
        Self { role: None }
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

impl OrgUsername {
    /// Insert a new row into `org_username` with a given [`CreateOrgUsername`]
    pub fn create(db: &mut ConnectionType, item: &CreateOrgUsername) -> diesel::QueryResult<Self> {
        use crate::schema::org_username::dsl::*;

        diesel::insert_into(org_username)
            .values(item)
            .get_result::<Self>(db)
    }

    /// Get a row from `org_username`, identified by the primary keys
    pub fn read(
        db: &mut ConnectionType,
        param_group_name: String,
        param_username: String,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::org_username::dsl::*;

        org_username
            .filter(group_name.eq(param_group_name))
            .filter(username.eq(param_username))
            .first::<Self>(db)
    }

    /// Update a row in `org_username`, identified by the primary keys with [`UpdateOrgUsername`]
    pub fn update(
        db: &mut ConnectionType,
        param_group_name: String,
        param_username: String,
        item: &UpdateOrgUsername,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::org_username::dsl::*;

        diesel::update(
            org_username
                .filter(group_name.eq(param_group_name))
                .filter(username.eq(param_username)),
        )
        .set(item)
        .get_result(db)
    }

    /// Delete a row in `org_username`, identified by the primary keys
    pub fn delete(
        db: &mut ConnectionType,
        param_group_name: String,
        param_username: String,
    ) -> diesel::QueryResult<usize> {
        use crate::schema::org_username::dsl::*;

        diesel::delete(
            org_username
                .filter(group_name.eq(param_group_name))
                .filter(username.eq(param_username)),
        )
        .execute(db)
    }
}
