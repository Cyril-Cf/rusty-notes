export interface Item {
    id: string;
    name: string;
    isChecked: boolean;
    listId: String;
    itemType: ItemType;
}

export enum ItemType {
    CHECKBOX = "CHECKBOX",
    BULLET_POINT = "BULLET_POINT"
}

export interface NewItem {
    name: String;
    isChecked: boolean;
    listId: String;
    itemType: ItemType;
}