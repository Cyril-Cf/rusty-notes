CREATE TABLE lists (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    name VARCHAR NOT NULL,
    list_type_id UUID NOT NULL,
    CONSTRAINT fk_list_list_type FOREIGN KEY (list_type_id) REFERENCES list_types(id) ON DELETE NO ACTION ON UPDATE NO ACTION,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
SELECT diesel_manage_updated_at('lists');