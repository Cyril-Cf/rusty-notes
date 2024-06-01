import gql from 'graphql-tag'

export const findAllListForUserWithTags = gql`
    query findAllListForUserWithTags ($userId: Uuid!) {
        findAllListForUserWithTags(userId: $userId) {
            id
            name
            items {
                name
            }
            tags {
                name
            }
            users {
                id
                firstname
                lastname
                email
            }
        }
    }
`