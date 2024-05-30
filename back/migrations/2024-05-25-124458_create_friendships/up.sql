CREATE TABLE friendships (
    id UUID PRIMARY KEY NOT NULL,
    user_who_asked_id UUID NOT NULL,
    user_who_got_asked_id UUID NOT NULL,
    is_validated BOOLEAN NOT NULL
);