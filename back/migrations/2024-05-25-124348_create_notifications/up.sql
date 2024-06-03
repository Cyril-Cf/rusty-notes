CREATE TYPE notif_type AS ENUM (
    'NEW_FRIENDSHIP_DEMAND',
    'NEW_FRIENDSHIP_ACCEPTED',
    'SHARED_LIST_MODIFIED'
);
CREATE TABLE notifications (
    id UUID PRIMARY KEY NOT NULL,
    has_been_read BOOLEAN NOT NULL,
    notif_type notif_type NOT NULL,
    user_id UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_user_notifications FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE NO ACTION ON UPDATE NO ACTION
);
SELECT diesel_manage_updated_at('notifications');