// @generated automatically by Diesel CLI.

diesel::table! {
    clients (id) {
        id -> Int4,
        client_id -> Varchar,
        client_secret -> Varchar,
        redirect_uri -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    org (name) {
        #[max_length = 50]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 39]
        github_id -> Nullable<Varchar>,
        #[max_length = 2048]
        avatar_url -> Nullable<Varchar>,
        #[max_length = 50]
        owner -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    org_username (group_name, username) {
        #[max_length = 50]
        group_name -> Varchar,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 15]
        role -> Nullable<Varchar>,
    }
}

diesel::table! {
    profile (alias) {
        #[max_length = 50]
        alias -> Varchar,
        #[max_length = 50]
        username -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 39]
        github_id -> Nullable<Varchar>,
        #[max_length = 2048]
        avatar_url -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    repo (id) {
        id -> Int4,
        #[max_length = 255]
        node_id -> Nullable<Varchar>,
        #[max_length = 39]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        full_name -> Varchar,
        private -> Nullable<Bool>,
        #[max_length = 2048]
        html_url -> Nullable<Varchar>,
        description -> Nullable<Text>,
        fork -> Nullable<Bool>,
        #[max_length = 244]
        default_branch -> Nullable<Varchar>,
        #[max_length = 2048]
        pulls_url -> Nullable<Varchar>,
        #[max_length = 2048]
        comments_url -> Nullable<Varchar>,
        languages -> Nullable<Array<Nullable<Text>>>,
        spdx -> Nullable<Text>,
        #[max_length = 8]
        visibility -> Nullable<Varchar>,
        #[max_length = 50]
        org -> Varchar,
        is_monorepo -> Nullable<Bool>,
        #[max_length = 40]
        last_commit -> Nullable<Bpchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    repo_history (full_name, commit) {
        #[max_length = 40]
        commit -> Bpchar,
        repo_id -> Nullable<Int4>,
        #[max_length = 255]
        full_name -> Varchar,
        doc_coverage -> Nullable<Numeric>,
        test_coverage -> Nullable<Numeric>,
        #[max_length = 2048]
        hosted_docs_url -> Nullable<Varchar>,
        security_scanner -> Nullable<Text>,
        #[max_length = 244]
        git_tag -> Nullable<Varchar>,
        metrics -> Nullable<Jsonb>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        id -> Nullable<Text>,
    }
}

diesel::table! {
    run_history (full_name, commit, run) {
        #[max_length = 40]
        commit -> Bpchar,
        #[max_length = 255]
        full_name -> Varchar,
        run -> Int4,
        created_at -> Timestamp,
        id -> Nullable<Text>,
    }
}

diesel::table! {
    users (username) {
        #[max_length = 50]
        username -> Varchar,
        password_hash -> Text,
        role -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(org -> users (owner));
diesel::joinable!(org_username -> org (group_name));
diesel::joinable!(org_username -> users (username));
diesel::joinable!(profile -> users (username));
diesel::joinable!(repo -> org (org));

diesel::allow_tables_to_appear_in_same_query!(
    clients,
    org,
    org_username,
    profile,
    repo,
    repo_history,
    run_history,
    users,
);
