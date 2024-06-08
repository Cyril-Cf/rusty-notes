import gql from 'graphql-tag'

export const getUserFriendships = gql`
    query getUserFriendships ($userId: Uuid!) {
        getUserFriendships(userId: $userId) {
            id
            isValidated
            updatedAt
            friendWhoAsked {
                id
                firstname
                lastname
                email
            }
            friendWhoGotAsked {
                id
                firstname
                lastname
                email
            }
        }
    }
`