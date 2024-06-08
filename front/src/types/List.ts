import { Item, ItemType } from './Item';
import { Tag } from './Tag';
import { User } from './User';

export interface List {
    id: string;
    name: string;
    items: Item[];
    tags: Tag[];
    listType: ListType;
    usersValidated: User[];
    usersAwaitingValidation: User[];
    isOwner: boolean;
    isValidated: boolean;
    listPermission: ListPermission;
}

export interface ListType {
    id: string;
    description: string;
    name: string;
    allowedItemTypes: ItemType[]
}

export interface NewList {
    name: string;
    listTypeId: string;
    userId: string;
}

export enum ListPermission {
    OWNER = "OWNER",
    CAN_SEE_BUT_NOT_MODIFY = "CAN_SEE_BUT_NOT_MODIFY",
    CAN_SEE_AND_MODIFY = "CAN_SEE_AND_MODIFY"
}