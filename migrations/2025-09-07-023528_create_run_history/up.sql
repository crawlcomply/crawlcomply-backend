CREATE TABLE run_history
(
    "commit"   CHAR(40)     NOT NULL CHECK ("commit" ~ '^[A-Za-z0-9]+$'),
    full_name  VARCHAR(255) NOT NULL CHECK (full_name ~ '^[A-Za-z0-9._~/-]+$'),
    run        INTEGER      NOT NULL DEFAULT 0,
    created_at TIMESTAMP    NOT NULL DEFAULT current_timestamp, -- when this record was first created
    id         TEXT GENERATED ALWAYS AS ("commit" || '@' || full_name || '#' || run) STORED,
    PRIMARY KEY (full_name, "commit", run),
    FOREIGN KEY (full_name, "commit") REFERENCES repo_history (full_name, "commit")
);
