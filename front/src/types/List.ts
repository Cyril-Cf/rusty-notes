import { Item } from './Item';
import { Tag } from './Tag';

export interface List {
    name: string;
    items: Item[];
    tags: Tag[];
}