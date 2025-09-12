/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::models::org::Org;
use crate::schema::*;

pub type ConnectionType = diesel::pg::PgConnection;

/// Struct representing a row in table `repo`
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
#[diesel(table_name=repo, primary_key(id), belongs_to(Org, foreign_key=org))]
pub struct Repo {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `node_id`
    pub node_id: Option<String>,
    /// Field representing column `name`
    pub name: Option<String>,
    /// Field representing column `full_name`
    pub full_name: String,
    /// Field representing column `private`
    pub private: Option<bool>,
    /// Field representing column `html_url`
    pub html_url: Option<String>,
    /// Field representing column `description`
    pub description: Option<String>,
    /// Field representing column `fork`
    pub fork: Option<bool>,
    /// Field representing column `default_branch`
    pub default_branch: Option<String>,
    /// Field representing column `pulls_url`
    pub pulls_url: Option<String>,
    /// Field representing column `comments_url`
    pub comments_url: Option<String>,
    /// Field representing column `languages`
    pub languages: Option<Vec<Option<String>>>,
    /// Field representing column `spdx`
    pub spdx: Option<String>,
    /// Field representing column `visibility`
    pub visibility: Option<String>,
    /// Field representing column `org`
    pub org: Option<String>,
    /// Field representing column `is_monorepo`
    pub is_monorepo: Option<bool>,
    /// Field representing column `last_commit`
    pub last_commit: Option<String>,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
    /// Field representing column `updated_at`
    pub updated_at: chrono::NaiveDateTime,
}

impl Default for Repo {
    fn default() -> Self {
        Self {
            id: 0,
            node_id: None,
            name: None,
            full_name: String::new(),
            private: None,
            html_url: None,
            description: None,
            fork: None,
            default_branch: None,
            pulls_url: None,
            comments_url: None,
            languages: None,
            spdx: None,
            visibility: None,
            org: None,
            is_monorepo: None,
            last_commit: None,
            created_at: Default::default(),
            updated_at: Default::default(),
        }
    }
}

/// Create Struct for a row in table `repo` for [`Repo`]
#[derive(
    utoipa::ToSchema, Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable,
)]
#[diesel(table_name=repo)]
pub struct CreateRepo {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `node_id`
    pub node_id: Option<String>,
    /// Field representing column `name`
    pub name: Option<String>,
    /// Field representing column `full_name`
    pub full_name: String,
    /// Field representing column `private`
    pub private: Option<bool>,
    /// Field representing column `html_url`
    pub html_url: Option<String>,
    /// Field representing column `description`
    pub description: Option<String>,
    /// Field representing column `fork`
    pub fork: Option<bool>,
    /// Field representing column `default_branch`
    pub default_branch: Option<String>,
    /// Field representing column `pulls_url`
    pub pulls_url: Option<String>,
    /// Field representing column `comments_url`
    pub comments_url: Option<String>,
    /// Field representing column `languages`
    pub languages: Option<Vec<Option<String>>>,
    /// Field representing column `spdx`
    pub spdx: Option<String>,
    /// Field representing column `visibility`
    pub visibility: Option<String>,
    /// Field representing column `org`
    pub org: Option<String>,
    /// Field representing column `is_monorepo`
    pub is_monorepo: Option<bool>,
    /// Field representing column `last_commit`
    pub last_commit: Option<String>,
}

impl Default for CreateRepo {
    fn default() -> Self {
        Self {
            id: 0,
            node_id: None,
            name: None,
            full_name: String::new(),
            private: None,
            html_url: None,
            description: None,
            fork: None,
            default_branch: None,
            pulls_url: None,
            comments_url: None,
            languages: None,
            spdx: None,
            visibility: None,
            org: None,
            is_monorepo: None,
            last_commit: None,
        }
    }
}

/// Update Struct for a row in table `repo` for [`Repo`]
#[derive(
    utoipa::ToSchema,
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::AsChangeset,
    PartialEq,
)]
#[diesel(table_name=repo)]
pub struct UpdateRepo {
    /// Field representing column `node_id`
    pub node_id: Option<Option<String>>,
    /// Field representing column `name`
    pub name: Option<Option<String>>,
    /// Field representing column `full_name`
    pub full_name: Option<String>,
    /// Field representing column `private`
    pub private: Option<Option<bool>>,
    /// Field representing column `html_url`
    pub html_url: Option<Option<String>>,
    /// Field representing column `description`
    pub description: Option<Option<String>>,
    /// Field representing column `fork`
    pub fork: Option<Option<bool>>,
    /// Field representing column `default_branch`
    pub default_branch: Option<Option<String>>,
    /// Field representing column `pulls_url`
    pub pulls_url: Option<Option<String>>,
    /// Field representing column `comments_url`
    pub comments_url: Option<Option<String>>,
    /// Field representing column `languages`
    pub languages: Option<Option<Vec<Option<String>>>>,
    /// Field representing column `spdx`
    pub spdx: Option<Option<String>>,
    /// Field representing column `visibility`
    pub visibility: Option<Option<String>>,
    /// Field representing column `org`
    pub org: Option<Option<String>>,
    /// Field representing column `is_monorepo`
    pub is_monorepo: Option<Option<bool>>,
    /// Field representing column `last_commit`
    pub last_commit: Option<Option<String>>,
    /// Field representing column `created_at`
    pub created_at: Option<chrono::NaiveDateTime>,
    /// Field representing column `updated_at`
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Default for UpdateRepo {
    fn default() -> Self {
        Self {
            node_id: None,
            name: None,
            full_name: None,
            private: None,
            html_url: None,
            description: None,
            fork: None,
            default_branch: None,
            pulls_url: None,
            comments_url: None,
            languages: None,
            spdx: None,
            visibility: None,
            org: None,
            is_monorepo: None,
            last_commit: None,
            created_at: None,
            updated_at: None,
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

impl Repo {
    /// Insert a new row into `repo` with a given [`CreateRepo`]
    pub fn create(db: &mut ConnectionType, item: &CreateRepo) -> diesel::QueryResult<Self> {
        use crate::schema::repo::dsl::*;

        diesel::insert_into(repo)
            .values(item)
            .get_result::<Self>(db)
    }

    /// Get a row from `repo`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<Self> {
        use crate::schema::repo::dsl::*;

        repo.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Update a row in `repo`, identified by the primary key with [`UpdateRepo`]
    pub fn update(
        db: &mut ConnectionType,
        param_id: i32,
        item: &UpdateRepo,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::repo::dsl::*;

        diesel::update(repo.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
    }

    /// Delete a row in `repo`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<usize> {
        use crate::schema::repo::dsl::*;

        diesel::delete(repo.filter(id.eq(param_id))).execute(db)
    }
}
