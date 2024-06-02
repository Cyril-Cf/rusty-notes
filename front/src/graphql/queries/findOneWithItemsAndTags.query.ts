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
                isChecked
                itemType
                listId
            }
            tags {
                id
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