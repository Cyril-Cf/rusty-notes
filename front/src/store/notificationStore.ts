import { ref } from 'vue'
import { defineStore } from 'pinia'
import { apolloClient } from '../apollo'
import { Notification, NotifType } from '../types/Notification';
import { UpdateResult, UpdateStatus } from '@/types/Utils';
import { toast } from 'vue3-toastify';
import { findAllNotificationsForUser } from '@/graphql/queries/notification/findAllNotificationsForUser.query';
import { updateNotification } from '@/graphql/mutations/notification/updateNotification.mutation';

export const useNotificationStore = defineStore('notification', () => {
    const notifications = ref<Notification[]>([]);

    async function fetchNotifications(userId: String) {
        const { data } = await apolloClient.query({
            query: findAllNotificationsForUser,
            variables: { userId },
            fetchPolicy: 'no-cache'
        });
        if (data && data.findAllNotificationsForUser) {
            notifications.value = data.findAllNotificationsForUser;
        }
    }

    async function updateNotificationBool(input: Notification, userId: String) {
        const { data } = await apolloClient.mutate({
            mutation: updateNotification,
            variables: { input },
            fetchPolicy: 'no-cache'
        });
        if (data && data.updateNotification) {
            toast.success("Notif mise à jour !", {
                position: toast.POSITION.BOTTOM_CENTER,
            });
            await fetchNotifications(userId);
        }
    }

    return {
        notifications,
        fetchNotifications,
        updateNotificationBool
    }
})
