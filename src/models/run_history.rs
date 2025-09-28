/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;

pub type ConnectionType = diesel::pg::PgConnection;

/// Struct representing a row in table `run_history`
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
    diesel::Identifiable,
)]
#[diesel(table_name=run_history, primary_key(full_name,commit,run))]
pub struct RunHistory {
    /// Field representing column `commit`
    pub commit: String,
    /// Field representing column `full_name`
    pub full_name: String,
    /// Field representing column `run`
    pub run: i32,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
    /// Field representing column `id`
    pub id: Option<String>,
    /// Field representing column `status`
    pub status: Option<String>,
}

impl Default for RunHistory {
    fn default() -> Self {
        Self {
            commit: String::new(),
            full_name: String::new(),
            run: 0,
            created_at: Default::default(),
            id: None,
            status: None,
        }
    }
}

/// Create Struct for a row in table `run_history` for [`RunHistory`]
#[derive(
    utoipa::ToSchema, Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable,
)]
#[diesel(table_name=run_history)]
pub struct CreateRunHistory {
    /// Field representing column `commit`
    pub commit: String,
    /// Field representing column `full_name`
    pub full_name: String,
    /// Field representing column `id`
    pub id: Option<String>,
    /// Field representing column `status`
    pub status: Option<String>,
}

impl Default for CreateRunHistory {
    fn default() -> Self {
        Self {
            commit: String::new(),
            full_name: String::new(),
            id: None,
            status: None,
        }
    }
}

/// Update Struct for a row in table `run_history` for [`RunHistory`]
#[derive(
    utoipa::ToSchema,
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::AsChangeset,
    PartialEq,
)]
#[diesel(table_name=run_history)]
pub struct UpdateRunHistory {
    /// Field representing column `created_at`
    pub created_at: Option<chrono::NaiveDateTime>,
    /// Field representing column `id`
    pub id: Option<Option<String>>,
    /// Field representing column `status`
    pub status: Option<Option<String>>,
}

impl Default for UpdateRunHistory {
    fn default() -> Self {
        Self {
            created_at: None,
            id: None,
            status: None,
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

impl RunHistory {
    /// Insert a new row into `run_history` with a given [`CreateRunHistory`]
    pub fn create(db: &mut ConnectionType, item: &CreateRunHistory) -> diesel::QueryResult<Self> {
        use crate::schema::run_history::dsl::*;

        diesel::insert_into(run_history)
            .values(item)
            .get_result::<Self>(db)
    }

    /// Get a row from `run_history`, identified by the primary keys
    pub fn read(
        db: &mut ConnectionType,
        param_full_name: String,
        param_commit: String,
        param_run: i32,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::run_history::dsl::*;

        run_history
            .filter(full_name.eq(param_full_name))
            .filter(commit.eq(param_commit))
            .filter(run.eq(param_run))
            .first::<Self>(db)
    }

    /// Update a row in `run_history`, identified by the primary keys with [`UpdateRunHistory`]
    pub fn update(
        db: &mut ConnectionType,
        param_full_name: String,
        param_commit: String,
        param_run: i32,
        item: &UpdateRunHistory,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::run_history::dsl::*;

        diesel::update(
            run_history
                .filter(full_name.eq(param_full_name))
                .filter(commit.eq(param_commit))
                .filter(run.eq(param_run)),
        )
        .set(item)
        .get_result(db)
    }

    /// Delete a row in `run_history`, identified by the primary keys
    pub fn delete(
        db: &mut ConnectionType,
        param_full_name: String,
        param_commit: String,
        param_run: i32,
    ) -> diesel::QueryResult<usize> {
        use crate::schema::run_history::dsl::*;

        diesel::delete(
            run_history
                .filter(full_name.eq(param_full_name))
                .filter(commit.eq(param_commit))
                .filter(run.eq(param_run)),
        )
        .execute(db)
    }
}
