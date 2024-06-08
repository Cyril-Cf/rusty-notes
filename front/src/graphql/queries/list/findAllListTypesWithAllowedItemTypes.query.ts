import gql from 'graphql-tag'

export const findAllListTypesWithAllowedItemTypes = gql`
    query findAllListTypesWithAllowedItemTypes {
        findAllListTypesWithAllowedItemTypes {
            id
            name
            description
            allowedItemTypes {
                id
                itemTypeVariation
            }
        }
    }
`