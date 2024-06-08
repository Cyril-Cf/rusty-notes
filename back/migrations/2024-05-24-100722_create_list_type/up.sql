CREATE TABLE list_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL
);
SELECT diesel_manage_updated_at('list_types');
INSERT INTO list_types (name, description)
VALUES (
        'Notes personnelles',
        'Notes pour un usage personnel, incluant les idées, les pensées et les listes de tâches personnelles'
    ),
    (
        'Notes professionnelles',
        'Notes liées au travail, incluant les réunions, les projets et les tâches professionnelles'
    ),
    (
        'Notes académiques',
        'Notes pour les études, incluant les cours, les travaux de recherche et les projets académiques'
    ),
    (
        'Notes de gestion de projet',
        'Notes pour la gestion de projets, incluant les planifications, les suivis et les rapports de projet'
    ),
    (
        'Notes de vie quotidienne',
        'Notes pour la gestion de la vie quotidienne, incluant les courses, les rendez-vous et les tâches ménagères'
    );