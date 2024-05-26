import gql from 'graphql-tag'

export const updateItem = gql`
    mutation updateItem ($input: UpdateItem!) {
        updateItem(input: $input) {
            status
        }
    }
`