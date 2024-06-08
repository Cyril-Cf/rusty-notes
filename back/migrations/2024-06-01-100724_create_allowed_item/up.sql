CREATE TABLE allowed_items (
    list_type_id UUID NOT NULL REFERENCES list_types(id),
    item_type_id UUID NOT NULL REFERENCES item_types(id),
    PRIMARY KEY (list_type_id, item_type_id)
);
SELECT diesel_manage_updated_at('allowed_items');
INSERT INTO allowed_items (list_type_id, item_type_id)
VALUES (
        (
            SELECT id
            FROM list_types
            WHERE name = 'Notes personnelles'
        ),
        (
            SELECT id
            FROM item_types
            WHERE item_type_variation = 'CONTENT'
        )
    ),
    (
        (
            SELECT id
            FROM list_types
            WHERE name = 'Notes personnelles'
        ),
        (
            SELECT id
            FROM item_types
            WHERE item_type_variation = 'CHECKBOX'
        )
    ),
    (
        (
            SELECT id
            FROM list_types
            WHERE name = 'Notes personnelles'
        ),
        (
            SELECT id
            FROM item_types
            WHERE item_type_variation = 'MEDIA_URL'
        )
    ),
    (
        (
            SELECT id
            FROM list_types
            WHERE name = 'Notes personnelles'
        ),
        (
            SELECT id
            FROM item_types
            WHERE item_type_variation = 'WEBSITE_URL'
        )
    ),
    (
        (
            SELECT id
            FROM list_types
            WHERE name = 'Notes professionnelles'
        ),
        (
            SELECT id
            FROM item_types
            WHERE item_type_variation = 'CONTENT'
        )
    ),
    (
        (
            SELECT id
            FROM list_types
            WHERE name = 'Notes professionnelles'
        ),
        (
            SELECT id
            FROM item_types
            WHERE item_type_variation = 'CHECKBOX'
        )
    ),
    (
        (
            SELECT id
            FROM list_types
            WHERE name = 'Notes professionnelles'
        ),
        (
            SELECT id
            FROM item_types
            WHERE item_type_variation = 'MEDIA_URL'
        )
    ),
    (
        (
            SELECT id
            FROM list_types
            WHERE name = 'Notes professionnelles'
        ),
        (
            SELECT id
            FROM item_types
            WHERE item_type_variation = 'WEBSITE_URL'
        )
    ),
    (
        (
            SELECT id
            FROM list_types
            WHERE name = 'Notes académiques'
        ),
        (
            SELECT id
            FROM item_types
            WHERE item_type_variation = 'CONTENT'
        )
    ),
    (
        (
            SELECT id
            FROM list_types
            WHERE name = 'Notes académiques'
        ),
        (
            SELECT id
            FROM item_types
            WHERE item_type_variation = 'WEBSITE_URL'
        )
    ),
    (
        (
            SELECT id
            FROM list_types
            WHERE name = 'Notes académiques'
        ),
        (
            SELECT id
            FROM item_types
            WHERE item_type_variation = 'MEDIA_URL'
        )
    ),
    (
        (
            SELECT id
            FROM list_types
            WHERE name = 'Notes de gestion de projet'
        ),
        (
            SELECT id
            FROM item_types
            WHERE item_type_variation = 'CONTENT'
        )
    ),
    (
        (
            SELECT id
            FROM list_types
            WHERE name = 'Notes de gestion de projet'
        ),
        (
            SELECT id
            FROM item_types
            WHERE item_type_variation = 'CHECKBOX'
        )
    ),
    (
        (
            SELECT id
            FROM list_types
            WHERE name = 'Notes de gestion de projet'
        ),
        (
            SELECT id
            FROM item_types
            WHERE item_type_variation = 'DEADLINE_DATE'
        )
    ),
    (
        (
            SELECT id
            FROM list_types
            WHERE name = 'Notes de gestion de projet'
        ),
        (
            SELECT id
            FROM item_types
            WHERE item_type_variation = 'PERSON_IN_CHARGE'
        )
    ),
    (
        (
            SELECT id
            FROM list_types
            WHERE name = 'Notes de vie quotidienne'
        ),
        (
            SELECT id
            FROM item_types
            WHERE item_type_variation = 'CONTENT'
        )
    ),
    (
        (
            SELECT id
            FROM list_types
            WHERE name = 'Notes de vie quotidienne'
        ),
        (
            SELECT id
            FROM item_types
            WHERE item_type_variation = 'CHECKBOX'
        )
    ),
    (
        (
            SELECT id
            FROM list_types
            WHERE name = 'Notes de vie quotidienne'
        ),
        (
            SELECT id
            FROM item_types
            WHERE item_type_variation = 'WEBSITE_URL'
        )
    ),
    (
        (
            SELECT id
            FROM list_types
            WHERE name = 'Notes de vie quotidienne'
        ),
        (
            SELECT id
            FROM item_types
            WHERE item_type_variation = 'MEDIA_URL'
        )
    );