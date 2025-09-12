CREATE TABLE profiles
(
    alias       VARCHAR(50) PRIMARY KEY,                                   -- user-modifiable alias
    username    VARCHAR(50) NOT NULL UNIQUE REFERENCES "users" (username), -- (basically) immutable username
    description TEXT,
    github_id   VARCHAR(39),                                               -- github id (username's can be modified)
    avatar_url  VARCHAR(2048),                                             -- profile image URL
    created_at  TIMESTAMP   NOT NULL DEFAULT current_timestamp             -- when this record was first created
);
CREATE UNIQUE INDEX profiles_alias_idx ON profiles (alias, username);
