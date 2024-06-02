import gql from 'graphql-tag'

export const confirmFriendship = gql`
    mutation confirmFriendship ($userAskedId: Uuid!, $userAskingId: Uuid!) {
        confirmFriendship(userAskedId: $userAskedId, userAskingId: $userAskingId) {
            status
        }
    }
`