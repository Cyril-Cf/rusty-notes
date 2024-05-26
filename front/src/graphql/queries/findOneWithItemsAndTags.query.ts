import gql from 'graphql-tag'

export const findOneWithItemsAndTags = gql`
    query findOneWithItemsAndTags ($listId: Uuid!) {
        findOneWithItemsAndTags(listId: $listId) {
            id
            name
            listType
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
        }
    }
`