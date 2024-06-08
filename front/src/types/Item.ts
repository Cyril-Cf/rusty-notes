export interface Item {
    id: string;
    content?: string;
    isChecked?: boolean;
    mediaUrl?: string;
    websiteUrl?: string;
    personInCharge?: string;
    priorityType?: PriorityType;
    itemType: ItemType;
    deadlineDate?: Date;
    createdAt: Date;
    updatedAt: Date;
}

export interface ItemType {
    id: string;
    itemTypeVariation: ItemTypeVariation;
}

export enum ItemTypeVariation {
    CONTENT = "CONTENT",
    CHECKBOX = "CHECKBOX",
    MEDIA_URL = "MEDIA_URL",
    WEBSITE_URL = "WEBSITE_URL",
    PERSON_IN_CHARGE = "PERSON_IN_CHARGE",
    DEADLINE_DATE = "DEADLINE_DATE"
}

export enum PriorityType {
    LOW = "LOW",
    MIDDLE = "MIDDLE",
    HIGH = "HIGH"
}

export interface NewItem {
    content?: string;
    isChecked?: boolean;
    mediaUrl?: string;
    websiteUrl?: string;
    personInCharge?: string;
    priorityType?: PriorityType;
    deadlineDate?: Date;
    itemTypeId: string;
    listId: string;
}