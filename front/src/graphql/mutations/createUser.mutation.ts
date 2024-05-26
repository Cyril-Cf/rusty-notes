import gql from 'graphql-tag'

export const createUser = gql`
    mutation createUser ($input: CreateUser!) {
        createUser(input: $input) {
            id
            firstname
            lastname
            email
            keycloakUuid
        }
    }
`