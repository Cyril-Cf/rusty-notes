import gql from 'graphql-tag'

export const findOneWithItemsAndTags = gql`
    query findOneWithItemsAndTags ($id: Uuid!) {
        findOneWithItemsAndTags(listId: $id) {
            id
            name
            listType
            items {
                name
            }
            tags {
                name
            }
        }
    }
`