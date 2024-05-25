CREATE TYPE item_type AS ENUM ('Checkbox', 'BulletPoint');
-- Remplacez par les valeurs réelles de votre énumération ItemType
CREATE TABLE items (
    id UUID PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    is_checked BOOLEAN NOT NULL,
    list_id UUID NOT NULL,
    item_type item_type NOT NULL,
    CONSTRAINT fk_item_list FOREIGN KEY (list_id) REFERENCES lists(id) ON DELETE NO ACTION ON UPDATE NO ACTION
);