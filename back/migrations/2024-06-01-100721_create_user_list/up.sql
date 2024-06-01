CREATE TABLE user_lists (
    user_id UUID NOT NULL REFERENCES users(id),
    list_id UUID NOT NULL REFERENCES lists(id),
    PRIMARY KEY (user_id, list_id)
);