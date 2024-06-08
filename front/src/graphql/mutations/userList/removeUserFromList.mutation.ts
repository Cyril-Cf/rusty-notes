import gql from 'graphql-tag'

export const removeUserFromList = gql`
    mutation removeUserFromList ($listId: Uuid!, $friendId: Uuid!) {
        removeUserFromList(listId: $listId, friendId: $friendId) {
            status
        }
    }
`