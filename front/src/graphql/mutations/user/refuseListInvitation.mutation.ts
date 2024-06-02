import gql from 'graphql-tag'

export const refuseListInvitation = gql`
    mutation refuseListInvitation ($listId: Uuid!, $userId: Uuid!) {
        refuseListInvitation(listId: $listId, userId: $userId) {
            status
        }
    }
`