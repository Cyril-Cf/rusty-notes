CREATE TYPE notification_type AS ENUM ('NewFriend', 'NewList');
CREATE TABLE notifications (
    id UUID PRIMARY KEY NOT NULL,
    content VARCHAR NOT NULL,
    has_been_read BOOLEAN NOT NULL,
    notification_type notification_type NOT NULL,
    user_id UUID NOT NULL,
    CONSTRAINT fk_user_notifications FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE NO ACTION ON UPDATE NO ACTION
);