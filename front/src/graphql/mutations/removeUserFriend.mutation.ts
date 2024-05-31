import gql from 'graphql-tag'

export const removeUserFriend = gql`
    mutation removeUserFriend ($userId: Uuid!, $userFriendId: Uuid!) {
        removeUserFriend(userId: $userId, userFriendId: $userFriendId) {
            status
        }
    }
`