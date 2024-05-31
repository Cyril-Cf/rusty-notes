<template>
    <div>
        <v-menu min-width="300px" rounded>
            <template v-slot:activator="{ props }">
                <v-btn icon v-bind="props">
                    <v-badge :content="unreadNotificationsCount" color="red">
                        <v-icon>mdi-bell</v-icon>
                    </v-badge>
                </v-btn>
            </template>
            <v-card class="px-5">
                <v-card-text>
                    <div class="mx-auto text-center">
                        <h3>Notifications</h3>
                        <v-list dense>
                            <v-list-item v-for="notification in notifications" :key="notification.id">
                                <v-list-item-title>
                                    {{ getNotificationMessage(notification) }}
                                </v-list-item-title>
                                <v-list-item-action>
                                    <v-btn v-if="!notification.hasBeenRead" small color="primary"
                                        @click="markAsRead(notification)">
                                        Mark as Read
                                    </v-btn>
                                </v-list-item-action>
                            </v-list-item>
                        </v-list>
                    </div>
                </v-card-text>
            </v-card>
        </v-menu>
    </div>
</template>

<script lang="ts" setup>
import { computed, onMounted } from "vue";
import { useNotificationStore } from "@/store/notificationStore";
import { useUserStore } from "@/store/userStore";
import { Notification, NotifType } from "@/types/Notification";
import authPromise from "@/plugins/keycloak";

const notificationStore = useNotificationStore();
const userStore = useUserStore();

const notifications = computed(() => notificationStore.notifications);

const unreadNotificationsCount = computed(() => {
    return notifications.value.filter((notif) => !notif.hasBeenRead).length;
});

const getNotificationMessage = (notification: Notification) => {
    switch (notification.notifType) {
        case NotifType.NEW_FRIENDSHIP_DEMAND:
            return "New friendship demand";
        case NotifType.NEW_FRIENDSHIP_ACCEPTED:
            return "Friendship accepted";
        case NotifType.SHARED_LIST_MODIFIED:
            return "Shared list modified";
        default:
            return "Unknown notification";
    }
};

const markAsRead = async (notification: Notification) => {
    const updateNotification: Notification = { id: notification.id, hasBeenRead: true, notifType: notification.notifType, userId: notification.userId };
    await notificationStore.updateNotificationBool(updateNotification, userStore.currentUser!.id);
};

onMounted(async () => {
    authPromise.then(async (auth) => {
        if (auth.isAuthenticated()) {
            const userStore = useUserStore();
            const notificationStore = useNotificationStore();
            let userIsInDB = await userStore.DoesUserExistInDB(auth.userId()!);
            if (userIsInDB && userStore.currentUser) {
                await notificationStore.fetchNotifications(userStore.currentUser.id);
            }
        }
    });
});
</script>

<style scoped>
.v-badge {
    display: inline-block;
    margin-right: 12px;
}
</style>