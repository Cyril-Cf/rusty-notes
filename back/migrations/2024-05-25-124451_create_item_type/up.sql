CREATE TYPE item_type_variation AS ENUM (
    'CONTENT',
    'CHECKBOX',
    'MEDIA_URL',
    'WEBSITE_URL',
    'PERSON_IN_CHARGE',
    'DEADLINE_DATE'
);
CREATE TABLE item_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    item_type_variation item_type_variation NOT NULL
);
SELECT diesel_manage_updated_at('item_types');
INSERT INTO item_types (item_type_variation)
VALUES ('CONTENT'),
    ('CHECKBOX'),
    ('MEDIA_URL'),
    ('WEBSITE_URL'),
    ('PERSON_IN_CHARGE'),
    ('DEADLINE_DATE');