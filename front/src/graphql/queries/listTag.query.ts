import gql from 'graphql-tag'

export const findOneWithItemsAndTags = gql`
    query findOneWithItemsAndTags ($id: Uuid!) {
        findOneWithItemsAndTags(listId: $id) {
            name
            items {
                name
            }
            tags {
                name
            }
        }
    }
`