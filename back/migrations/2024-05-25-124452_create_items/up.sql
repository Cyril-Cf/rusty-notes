CREATE TYPE priority_type AS ENUM ('LOW', 'MIDDLE', 'HIGH');
CREATE TABLE items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    content VARCHAR,
    is_checked BOOLEAN,
    media_url VARCHAR,
    website_url VARCHAR,
    person_in_charge VARCHAR,
    priority_type priority_type,
    list_id UUID NOT NULL,
    item_type_id UUID NOT NULL,
    deadline_date DATE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_item_list FOREIGN KEY (list_id) REFERENCES lists(id) ON DELETE NO ACTION ON UPDATE NO ACTION,
    CONSTRAINT fk_item_item_type FOREIGN KEY (item_type_id) REFERENCES item_types(id) ON DELETE NO ACTION ON UPDATE NO ACTION
);
SELECT diesel_manage_updated_at('items');