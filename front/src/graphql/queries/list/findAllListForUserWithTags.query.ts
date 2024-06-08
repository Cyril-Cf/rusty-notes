import gql from 'graphql-tag'

export const findAllListForUserWithTags = gql`
    query findAllListForUserWithTags ($userId: Uuid!) {
        findAllListForUserWithTags(userId: $userId) {
            id
            name
            listType {
                id
                name
                description
            }
            isOwner
            isValidated
            listPermission
            tags {
                name
            }
            usersValidated {
                id
                firstname
                lastname
                email
                listPermission
            }
            usersAwaitingValidation {
                id
                firstname
                lastname
                email
                listPermission
            }
        }
    }
`