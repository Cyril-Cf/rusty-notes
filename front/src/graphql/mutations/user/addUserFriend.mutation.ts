import gql from 'graphql-tag'

export const addUserFriend = gql`
    mutation addUserFriend ($userWhoAskedId: Uuid!, $userWhoGetAskedEmail: String!) {
        addUserFriend(userWhoAskedId: $userWhoAskedId, userWhoGetAskedEmail: $userWhoGetAskedEmail) {
            status
        }
    }
`