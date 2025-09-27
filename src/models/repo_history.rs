/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;

pub type ConnectionType = diesel::pg::PgConnection;

/// Struct representing a row in table `repo_history`
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
#[diesel(table_name=repo_history, primary_key(full_name,commit))]
pub struct RepoHistory {
    /// Field representing column `commit`
    pub commit: String,
    /// Field representing column `repo_id`
    pub repo_id: Option<i32>,
    /// Field representing column `full_name`
    pub full_name: String,
    /// Field representing column `doc_coverage`
    pub doc_coverage: Option<bigdecimal::BigDecimal>,
    /// Field representing column `test_coverage`
    pub test_coverage: Option<bigdecimal::BigDecimal>,
    /// Field representing column `hosted_docs_url`
    pub hosted_docs_url: Option<String>,
    /// Field representing column `security_scanner`
    pub security_scanner: Option<String>,
    /// Field representing column `git_tag`
    pub git_tag: Option<String>,
    /// Field representing column `git_branch`
    pub git_branch: Option<String>,
    /// Field representing column `github_pr`
    pub github_pr: Option<i32>,
    /// Field representing column `metrics`
    pub metrics: Option<serde_json::Value>,
    /// Field representing column `notes`
    pub notes: Option<String>,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
    /// Field representing column `id`
    pub id: Option<String>,
}

impl Default for RepoHistory {
    fn default() -> Self {
        Self {
            commit: String::new(),
            repo_id: None,
            full_name: String::new(),
            doc_coverage: None,
            test_coverage: None,
            hosted_docs_url: None,
            security_scanner: None,
            git_tag: None,
            git_branch: None,
            github_pr: None,
            metrics: None,
            notes: None,
            created_at: Default::default(),
            id: None,
        }
    }
}

/// Create Struct for a row in table `repo_history` for [`RepoHistory`]
#[derive(
    utoipa::ToSchema, Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable,
)]
#[diesel(table_name=repo_history)]
pub struct CreateRepoHistory {
    /// Field representing column `commit`
    pub commit: String,
    /// Field representing column `repo_id`
    pub repo_id: Option<i32>,
    /// Field representing column `full_name`
    pub full_name: String,
    /// Field representing column `doc_coverage`
    pub doc_coverage: Option<bigdecimal::BigDecimal>,
    /// Field representing column `test_coverage`
    pub test_coverage: Option<bigdecimal::BigDecimal>,
    /// Field representing column `hosted_docs_url`
    pub hosted_docs_url: Option<String>,
    /// Field representing column `security_scanner`
    pub security_scanner: Option<String>,
    /// Field representing column `git_tag`
    pub git_tag: Option<String>,
    /// Field representing column `git_branch`
    pub git_branch: Option<String>,
    /// Field representing column `github_pr`
    pub github_pr: Option<i32>,
    /// Field representing column `metrics`
    pub metrics: Option<serde_json::Value>,
    /// Field representing column `notes`
    pub notes: Option<String>,
    /// Field representing column `id`
    pub id: Option<String>,
}

impl Default for CreateRepoHistory {
    fn default() -> Self {
        Self {
            commit: String::new(),
            repo_id: None,
            full_name: String::new(),
            doc_coverage: None,
            test_coverage: None,
            hosted_docs_url: None,
            security_scanner: None,
            git_tag: None,
            git_branch: None,
            github_pr: None,
            metrics: None,
            notes: None,
            id: None,
        }
    }
}

/// Update Struct for a row in table `repo_history` for [`RepoHistory`]
#[derive(
    utoipa::ToSchema,
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::AsChangeset,
    PartialEq,
)]
#[diesel(table_name=repo_history)]
pub struct UpdateRepoHistory {
    /// Field representing column `repo_id`
    pub repo_id: Option<Option<i32>>,
    /// Field representing column `doc_coverage`
    pub doc_coverage: Option<Option<bigdecimal::BigDecimal>>,
    /// Field representing column `test_coverage`
    pub test_coverage: Option<Option<bigdecimal::BigDecimal>>,
    /// Field representing column `hosted_docs_url`
    pub hosted_docs_url: Option<Option<String>>,
    /// Field representing column `security_scanner`
    pub security_scanner: Option<Option<String>>,
    /// Field representing column `git_tag`
    pub git_tag: Option<Option<String>>,
    /// Field representing column `git_branch`
    pub git_branch: Option<Option<String>>,
    /// Field representing column `github_pr`
    pub github_pr: Option<Option<i32>>,
    /// Field representing column `metrics`
    pub metrics: Option<Option<serde_json::Value>>,
    /// Field representing column `notes`
    pub notes: Option<Option<String>>,
    /// Field representing column `created_at`
    pub created_at: Option<chrono::NaiveDateTime>,
    /// Field representing column `id`
    pub id: Option<Option<String>>,
}

impl Default for UpdateRepoHistory {
    fn default() -> Self {
        Self {
            repo_id: None,
            doc_coverage: None,
            test_coverage: None,
            hosted_docs_url: None,
            security_scanner: None,
            git_tag: None,
            git_branch: None,
            github_pr: None,
            metrics: None,
            notes: None,
            created_at: None,
            id: None,
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

impl RepoHistory {
    /// Insert a new row into `repo_history` with a given [`CreateRepoHistory`]
    pub fn create(db: &mut ConnectionType, item: &CreateRepoHistory) -> diesel::QueryResult<Self> {
        use crate::schema::repo_history::dsl::*;

        diesel::insert_into(repo_history)
            .values(item)
            .get_result::<Self>(db)
    }

    /// Get a row from `repo_history`, identified by the primary keys
    pub fn read(
        db: &mut ConnectionType,
        param_full_name: String,
        param_commit: String,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::repo_history::dsl::*;

        repo_history
            .filter(full_name.eq(param_full_name))
            .filter(commit.eq(param_commit))
            .first::<Self>(db)
    }

    /// Update a row in `repo_history`, identified by the primary keys with [`UpdateRepoHistory`]
    pub fn update(
        db: &mut ConnectionType,
        param_full_name: String,
        param_commit: String,
        item: &UpdateRepoHistory,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::repo_history::dsl::*;

        diesel::update(
            repo_history
                .filter(full_name.eq(param_full_name))
                .filter(commit.eq(param_commit)),
        )
        .set(item)
        .get_result(db)
    }

    /// Delete a row in `repo_history`, identified by the primary keys
    pub fn delete(
        db: &mut ConnectionType,
        param_full_name: String,
        param_commit: String,
    ) -> diesel::QueryResult<usize> {
        use crate::schema::repo_history::dsl::*;

        diesel::delete(
            repo_history
                .filter(full_name.eq(param_full_name))
                .filter(commit.eq(param_commit)),
        )
        .execute(db)
    }
}
