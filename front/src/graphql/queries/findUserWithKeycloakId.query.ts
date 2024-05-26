import gql from 'graphql-tag'

export const findUserWithKeycloakId = gql`
    query findUserWithKeycloakId ($keycloakId: Uuid!) {
        findUserWithKeycloakId(keycloakId: $keycloakId) {
            id
            firstname
            lastname
            email
            keycloakUuid
        }
    }
`