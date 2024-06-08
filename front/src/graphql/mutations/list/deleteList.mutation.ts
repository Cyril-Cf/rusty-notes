import gql from 'graphql-tag'

export const deleteList = gql`
    mutation deleteList ($listId: Uuid!) {
        deleteList(listId: $listId) {
            status
        }
    }
`