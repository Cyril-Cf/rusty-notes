import gql from 'graphql-tag'

export const inviteUserToYourList = gql`
    mutation inviteUserToYourList ($listId: Uuid!, $userId: Uuid!, $friendId: Uuid!, $permission: ListPermission!) {
        inviteUserToYourList(listId: $listId, userId: $userId, friendId: $friendId, permission: $permission) {
            status
        }
    }
`