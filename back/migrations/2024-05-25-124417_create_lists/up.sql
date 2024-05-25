-- Remplacez 'Type1', 'Type2', 'Type3' par les valeurs réelles de votre énumération ListType
CREATE TYPE list_type AS ENUM ('ToDo', 'ToBuy');
CREATE TABLE lists (
    id UUID PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    list_type list_type NOT NULL,
    user_id UUID NOT NULL,
    CONSTRAINT fk_user_list FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE NO ACTION ON UPDATE NO ACTION
);