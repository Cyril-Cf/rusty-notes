CREATE TYPE list_permission AS ENUM (
    'OWNER',
    'CAN_SEE_BUT_NOT_MODIFY',
    'CAN_SEE_AND_MODIFY'
);
CREATE TABLE user_lists (
    user_id UUID NOT NULL REFERENCES users(id),
    list_id UUID NOT NULL REFERENCES lists(id),
    PRIMARY KEY (user_id, list_id),
    is_owner BOOLEAN NOT NULL,
    is_validated BOOLEAN NOT NULL,
    list_permission list_permission NOT NULL
);