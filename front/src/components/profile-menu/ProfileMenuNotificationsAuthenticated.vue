<template>
    <div>
        <v-menu min-width="400px" max-width="400px" rounded>
            <template v-slot:activator="{ props }">
                <v-btn icon v-bind="props">
                    <v-badge :content="unreadNotificationsCount" color="red">
                        <v-icon>mdi-bell</v-icon>
                    </v-badge>
                </v-btn>
            </template>
            <v-card>
                <v-card-text class="p-0">
                    <div class="text-center">
                        <h3 class="py-3">Notifications</h3>
                        <v-divider></v-divider>
                        <v-list dense max-height="200px" min-width="350px" class="overflow-y-auto">
                            <v-list-item v-for="notification in sortedNotifications" :key="notification.id">
                                <template v-slot:prepend>
                                    <v-avatar size="small" :color="getIconColor(notification)">
                                        <v-icon color="white">{{ getIconImage(notification) }}</v-icon>
                                    </v-avatar>
                                </template>
                                <v-list-item-title class="single-line">
                                    {{ getNotificationMessage(notification) }}
                                </v-list-item-title>
                                <template v-slot:append v-if="!notification.hasBeenRead">
                                    <v-tooltip bottom>
                                        <template v-slot:activator="{ props }">
                                            <v-btn icon elevation="0" size="x-small" color="primary"
                                                @click="markAsRead(notification)" v-bind="props">
                                                <v-icon>mdi-check</v-icon>
                                            </v-btn>
                                        </template>
                                        <span>Mark as Read</span>
                                    </v-tooltip>
                                </template>
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

const sortedNotifications = computed(() => {
    return notifications.value.reverse();
});

const unreadNotificationsCount = computed(() => {
    return notifications.value.filter((notif) => !notif.hasBeenRead).length;
});

const getNotificationMessage = (notification: Notification) => {
    switch (notification.notifType) {
        case NotifType.NEW_FRIENDSHIP_DEMAND:
            return "Nouvelle demande d'amitiée";
        case NotifType.NEW_FRIENDSHIP_ACCEPTED:
            return "Amitiée acceptée";
        case NotifType.SHARED_LIST_MODIFIED:
            return "Liste commune modifiée";
        default:
            return "Unknown notification";
    }
};

const getIconColor = (notification: Notification) => {
    switch (notification.notifType) {
        case NotifType.NEW_FRIENDSHIP_DEMAND:
            return "pink";
        case NotifType.NEW_FRIENDSHIP_ACCEPTED:
            return "green";
        case NotifType.SHARED_LIST_MODIFIED:
            return "yellow";
        default:
            return "Unknown notification";
    }
}

const getIconImage = (notification: Notification) => {
    switch (notification.notifType) {
        case NotifType.NEW_FRIENDSHIP_DEMAND:
            return "mdi-heart-box";
        case NotifType.NEW_FRIENDSHIP_ACCEPTED:
            return "mdi-heart-plus";
        case NotifType.SHARED_LIST_MODIFIED:
            return "mdi-playlist-edit";
        default:
            return "Unknown notification";
    }
}

const markAsRead = async (notification: Notification) => {
    const updateNotification: Notification = {
        id: notification.id,
        hasBeenRead: true,
        notifType: notification.notifType,
        userId: notification.userId,
    };
    await notificationStore.updateNotificationBool(
        updateNotification,
        userStore.currentUser!.id
    );
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

.single-line {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}
</style>