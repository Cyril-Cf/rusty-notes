import gql from 'graphql-tag'

export const createList = gql`
    mutation createList ($input: CreateList!) {
        createList(input: $input) {
            id
            name
            listType
        }
    }
`