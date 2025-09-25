-- inspired by https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#get-a-repository
CREATE TABLE repo
(
    "id"           INTEGER PRIMARY KEY,
    node_id        VARCHAR(255),
    "name"         VARCHAR(39),
    full_name      VARCHAR(255) UNIQUE NOT NULL,
    private        BOOLEAN,
    html_url       VARCHAR(2048),
    description    TEXT,
    fork           BOOLEAN,
    default_branch VARCHAR(244),
    pulls_url      VARCHAR(2048),
    comments_url   VARCHAR(2048),
    languages      TEXT[],
    spdx           TEXT,
    visibility     VARCHAR(8) CHECK (visibility in ('public', 'private')),
    org            VARCHAR(50)         NOT NULL REFERENCES org ("name"),
    is_monorepo    BOOLEAN,                                                -- whether repo contains more than one "package"
    last_commit    CHAR(40),
    created_at     TIMESTAMP           NOT NULL DEFAULT current_timestamp, -- when this record was first created
    updated_at     TIMESTAMP           NOT NULL DEFAULT current_timestamp  -- when this record was last updated
);

CREATE TRIGGER set_timestamp
    BEFORE UPDATE
    ON repo
    FOR EACH ROW
EXECUTE PROCEDURE diesel_set_updated_at();
