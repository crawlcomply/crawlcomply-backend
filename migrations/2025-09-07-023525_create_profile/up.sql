CREATE TABLE profile
(
    alias       VARCHAR(50) PRIMARY KEY CHECK (alias ~ '^[A-Za-z0-9._~@-]+$'), -- user-modifiable alias
    username    VARCHAR(50) NOT NULL UNIQUE REFERENCES "users" (username),     -- (basically) immutable username
    description TEXT,
    github_id   VARCHAR(39),                                                   -- github id (username's can be modified)
    avatar_url  VARCHAR(2048),                                                 -- profile image URL
    created_at  TIMESTAMP   NOT NULL DEFAULT current_timestamp                 -- when this record was first created
);
CREATE UNIQUE INDEX IF NOT EXISTS profiles_alias_idx ON profile (alias, username);

CREATE FUNCTION create_personal_org_for_new_profile()
    RETURNS TRIGGER AS
$$
BEGIN
    INSERT INTO org (name, description, owner)
    VALUES (NEW.alias, 'Personal org of ' || NEW.alias, NEW.username);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION drop_personal_org_for_new_profile()
    RETURNS TRIGGER AS
$$
BEGIN
    DELETE
    FROM org_username
    WHERE group_name = OLD.alias;
    DELETE
    FROM org
    WHERE org.name = OLD.alias
      AND org.owner = OLD.username;
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER new_profile_trigger
    AFTER INSERT
    ON profile
    FOR EACH ROW
EXECUTE FUNCTION create_personal_org_for_new_profile();

CREATE TRIGGER drop_profile_org_trigger
    BEFORE DELETE
    ON profile
    FOR EACH ROW
EXECUTE FUNCTION drop_personal_org_for_new_profile();
