CREATE TABLE repo_history
(
    "commit"         CHAR(40)     NOT NULL CHECK ("commit" ~ '^[A-Za-z0-9]+$'),
    repo_id          INTEGER REFERENCES repo (id),
    full_name        VARCHAR(255) NOT NULL REFERENCES repo (full_name),
    doc_coverage     NUMERIC(4, 2),
    test_coverage    NUMERIC(4, 2),
    hosted_docs_url  VARCHAR(2048),
    security_scanner TEXT,
    git_tag          VARCHAR(244),
    metrics          JSONB,
    notes            TEXT,
    created_at       TIMESTAMP    NOT NULL DEFAULT current_timestamp, -- when this record was first created
    id               TEXT GENERATED ALWAYS AS ("commit" || '@' || full_name) STORED,
    PRIMARY KEY (full_name, "commit")
);
