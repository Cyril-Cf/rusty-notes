CREATE TABLE list_tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    name VARCHAR NOT NULL,
    list_id UUID NOT NULL,
    CONSTRAINT fk_list_list_tag FOREIGN KEY (list_id) REFERENCES lists(id) ON DELETE NO ACTION ON UPDATE NO ACTION,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
SELECT diesel_manage_updated_at('list_tags');