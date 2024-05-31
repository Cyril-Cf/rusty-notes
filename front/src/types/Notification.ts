export interface Notification {
    id: string;
    hasBeenRead: boolean;
    notifType: NotifType;
    userId: string
}

export enum NotifType {
    NEW_FRIENDSHIP_DEMAND = "NEW_FRIENDSHIP_DEMAND",
    NEW_FRIENDSHIP_ACCEPTED = "NEW_FRIENDSHIP_ACCEPTED",
    SHARED_LIST_MODIFIED = "SHARED_LIST_MODIFIED"
}