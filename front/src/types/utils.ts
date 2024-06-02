export interface DeleteResult {
    status: DeleteStatus;
}

export enum DeleteStatus {
    ResourceDeleted = 'RESOURCE_DELETED',
    NoDeletion = 'NO_DELETION',
}

export interface UpdateResult {
    status: UpdateStatus;
}

export enum UpdateStatus {
    ResourceUpdated = 'RESOURCE_UPDATED',
    NoUpdate = 'NO_UPDATE',
}

export interface AddFriendToMyListResult {
    status: AddFriendToMyListStatus
}

export enum AddFriendToMyListStatus {
    AddSuccessful = 'ADD_SUCCESSFUL',
    ErrNoFriendshipFound = 'ERR_NO_FRIENDSHIP_FOUND',
    ErrNoListFound = 'ERR_NO_LIST_FOUND',
    ErrNoUserFound = 'ERR_NO_USER_FOUND',
    ErrNoUserFriendFound = 'ERR_NO_USER_FRIEND_FOUND',
    ErrNotFriends = 'ERR_NOT_FRIENDS',
    ErrServerIssue = 'ERR_SERVER_ISSUE',
}

export interface RemoveFriendFromMyListResult {
    status: RemoveFriendFromMyListStatus
}

export enum RemoveFriendFromMyListStatus {
    RemoveSuccessful = 'REMOVE_SUCCESSFUL',
    ErrNoListFound = 'ERR_NO_LIST_FOUND',
    ErrNoUserFriendFound = 'ERR_NO_USER_FRIEND_FOUND',
    ErrServerIssue = 'ERR_SERVER_ISSUE',
}

export interface RefuseListInvitationResult {
    status: RefuseListInvitationStatus
}

export enum RefuseListInvitationStatus {
    REFUSE_SUCCESSFUL = 'REFUSE_SUCCESSFUL',
    ERR_NO_INVITATION_FOUND = 'ERR_NO_INVITATION_FOUND',
}

export interface AcceptListInvitationResult {
    status: AcceptListInvitationStatus
}

export enum AcceptListInvitationStatus {
    ACCEPT_SUCCESSFUL = 'ACCEPT_SUCCESSFUL',
    ERR_NO_INVITATION_FOUND = 'ERR_NO_INVITATION_FOUND',
}