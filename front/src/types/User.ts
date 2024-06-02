import { ListPermission } from "./List";

export interface User {
    id: String;
    firstname: String;
    lastname: String;
    email: String;
    keycloakUuid: String;
    listPermission: ListPermission;
}

export interface NewUser {
    firstname: String;
    lastname: String;
    email: String;
    keycloakUuid: String;
}