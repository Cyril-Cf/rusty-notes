CREATE TABLE friendships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    user_who_asked_id UUID NOT NULL,
    user_who_got_asked_id UUID NOT NULL,
    is_validated BOOLEAN NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
SELECT diesel_manage_updated_at('friendships');