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