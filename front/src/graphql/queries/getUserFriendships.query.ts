import gql from 'graphql-tag'

export const getUserFriendships = gql`
    query getUserFriendships ($userId: Uuid!) {
        getUserFriendships(userId: $userId) {
            id
            isValidated
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