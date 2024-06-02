import gql from 'graphql-tag'

export const findAllListForUserWithTags = gql`
    query findAllListForUserWithTags ($userId: Uuid!) {
        findAllListForUserWithTags(userId: $userId) {
            id
            name
            listType
            isOwner
            isValidated
            listPermission
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