import gql from 'graphql-tag'

export const updateNotification = gql`
    mutation updateNotification ($input: UpdateNotificationGQL!) {
        updateNotification(input: $input) {
            status
        }
    }
`