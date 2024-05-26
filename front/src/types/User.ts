export interface User {
    id: String;
    firstname: String;
    lastname: String;
    email: String;
    keycloakUuid: String;
}

export interface NewUser {
    firstname: String;
    lastname: String;
    email: String;
    keycloakUuid: String;
}