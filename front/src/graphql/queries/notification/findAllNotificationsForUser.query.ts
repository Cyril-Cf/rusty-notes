import gql from 'graphql-tag'

export const findAllNotificationsForUser = gql`
    query findAllNotificationsForUser ($userId: Uuid!) {
        findAllNotificationsForUser(userId: $userId) {
            id
            hasBeenRead
            notifType
            userId
        }
    }
`