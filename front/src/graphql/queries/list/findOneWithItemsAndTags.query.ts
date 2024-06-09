import gql from 'graphql-tag'

export const findOneWithItemsAndTags = gql`
    query findOneWithItemsAndTags ($listId: Uuid!, $userId: Uuid!) {
        findOneWithItemsAndTags(listId: $listId, userId: $userId) {
            id
            name
            listType {
                id
                name
                description
                allowedItemTypes {
                    id
                    itemTypeVariation
                }
            }
            isOwner
            items {
                createdAt
                id
                content
                isChecked
                mediaUrl
                websiteUrl
                personInCharge
                priorityType
                itemType {
                    itemTypeVariation
                }
                deadlineDate
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