import { User } from './User'

export interface Friendship {
    id: String;
    friendWhoAsked: User;
    friendWhoGotAsked: User;
    isValidated: boolean;
    updatedAt: Date;
}

export interface AddFriendResult {
    status: AddFriendStatus;
}

export interface RemoveFriendResult {
    status: RemoveFriendStatus;
}

export interface FriendshipAcceptingResult {
    status: FriendshipAcceptingStatus;
}

export enum AddFriendStatus {
    AddSuccessful = 'ADD_SUCCESSFUL',
    ErrNoUserEmail = 'ERR_NO_USER_EMAIL',
    ErrAlreadyFriend = 'ERR_ALREADY_FRIEND',
    ErrAlreadyPendingDemand = 'ERR_ALREADY_PENDING_DEMAND',
}

export enum RemoveFriendStatus {
    RemoveSuccessful = 'REMOVE_SUCCESSFUL',
    ErrNoFriendship = 'ERR_NO_FRIENDSHIP',
}

export enum FriendshipAcceptingStatus {
    AcceptingSuccessful = 'ACCEPTING_SUCCESSFUL',
    ErrCannotAccept = 'ERR_CANNOT_ACCEPT',
}

