CREATE TABLE users (
    id UUID PRIMARY KEY NOT NULL,
    firstname VARCHAR NOT NULL,
    lastname VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    keycloak_uuid UUID NOT NULL
);