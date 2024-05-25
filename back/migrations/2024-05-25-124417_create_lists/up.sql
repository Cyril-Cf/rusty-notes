CREATE TYPE list_type AS ENUM ('TO_DO', 'TO_BUY');
CREATE TABLE lists (
    id UUID PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    list_type list_type NOT NULL,
    user_id UUID NOT NULL,
    CONSTRAINT fk_user_list FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE NO ACTION ON UPDATE NO ACTION
);