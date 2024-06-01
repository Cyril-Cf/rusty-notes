import { Item } from './Item';
import { Tag } from './Tag';
import { User } from './User';

export interface List {
    id: string;
    name: string;
    items: Item[];
    tags: Tag[];
    listType: ListType;
    users: User[];
}

export enum ListType {
    TO_DO = "TO_DO",
    TO_BUY = "TO_BUY"
}

export interface NewList {
    name: string;
    listType: string;
    userId: string;
}