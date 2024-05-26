import gql from 'graphql-tag'

export const deleteItem = gql`
    mutation deleteItem ($itemId: Uuid!) {
        deleteItem(itemId: $itemId) {
            status
        }
    }
`