CREATE TABLE org
(
    "name"      VARCHAR(50) PRIMARY KEY,
    description TEXT,
    github_id   VARCHAR(39),                                   -- github id (username's can be modified)
    avatar_url  VARCHAR(2048),                                 -- profile image URL
    owner       VARCHAR(50) NOT NULL REFERENCES users (username),
    created_at  TIMESTAMP   NOT NULL DEFAULT current_timestamp -- when this record was first created
);

CREATE TABLE org_username
(
    group_name VARCHAR(50) REFERENCES org ("name") ON DELETE CASCADE,
    username   VARCHAR(50) NOT NULL REFERENCES users (username), -- (basically) immutable username
    role       VARCHAR(15),
    PRIMARY KEY (group_name, username)
);

CREATE INDEX IF NOT EXISTS org_profile_name_idx
    ON org_username USING HASH (group_name);
CREATE INDEX IF NOT EXISTS org_profile_username_idx
    ON org_username USING HASH (username);
