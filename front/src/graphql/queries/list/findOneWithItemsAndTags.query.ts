import gql from 'graphql-tag'

export const findOneWithItemsAndTags = gql`
    query findOneWithItemsAndTags ($listId: Uuid!, $userId: Uuid!) {
        findOneWithItemsAndTags(listId: $listId, userId: $userId) {
            id
            name
            listType
            isOwner
            isValidated
            listPermission
            items {
                id
                name
                createdAt
                isChecked
                itemType
                listId
            }
            tags {
                id
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