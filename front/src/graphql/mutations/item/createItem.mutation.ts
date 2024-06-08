import gql from 'graphql-tag'

export const createItem = gql`
    mutation createItem ($input: CreateItem!) {
        createItem(input: $input) {
            status
        }
    }
`