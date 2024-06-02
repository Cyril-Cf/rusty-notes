import gql from 'graphql-tag'

export const acceptListInvitation = gql`
    mutation acceptListInvitation ($listId: Uuid!, $userId: Uuid!) {
        acceptListInvitation(listId: $listId, userId: $userId) {
            status
        }
    }
`