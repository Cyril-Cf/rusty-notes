import { ref } from 'vue'
import { defineStore } from 'pinia'
import { apolloClient } from '../apollo'
import { updateItem } from '../graphql/mutations/updateItem.mutation'
import { deleteItem } from '../graphql/mutations/deleteItem.mutation'
import { createItem } from '../graphql/mutations/createItem.mutation'
import { inviteUserToYourList } from '@/graphql/mutations/inviteUserToYourList.mutation'
import { removeUserFromList } from '@/graphql/mutations/removeUserFromList.mutation'
import { findAllListForUserWithTags } from '../graphql/queries/findAllListForUserWithTags.query';
import { findOneWithItemsAndTags } from '../graphql/queries/findOneWithItemsAndTags.query'
import { createList } from '../graphql/mutations/createList.mutation'
import { deleteList } from '../graphql/mutations/deleteList.mutation'
import { List, NewList } from '../types/List';
import { NewItem, Item } from '../types/Item'
import { DeleteStatus, DeleteResult, AddFriendToMyListStatus, AddFriendToMyListResult, RemoveFriendFromMyListResult, RemoveFriendFromMyListStatus } from '@/types/utils';
import { toast } from 'vue3-toastify';

export const useListStore = defineStore('list', () => {
    const lists = ref<List[]>([]);
    const selectedList = ref<List | undefined>();
    const selectedItems = ref<Item[]>([])

    async function fetchLists(userId: String) {
        const { data } = await apolloClient.query({
            query: findAllListForUserWithTags,
            variables: { userId },
            fetchPolicy: 'no-cache'
        });
        if (data && data.findAllListForUserWithTags) {
            lists.value = data.findAllListForUserWithTags;
        }
    }

    async function createNewList(input: NewList) {
        const { data } = await apolloClient.mutate({
            mutation: createList,
            variables: { input },
            fetchPolicy: 'no-cache'
        });
        if (data && data.createList) {
            await fetchLists(input.userId);
        }
    }

    async function fetchOne(listId: String) {
        const { data } = await apolloClient.query({
            query: findOneWithItemsAndTags,
            variables: { listId },
            fetchPolicy: 'no-cache'
        });
        if (data && data.findOneWithItemsAndTags) {
            selectedList.value = data.findOneWithItemsAndTags;
            if (selectedList.value) {
                selectedItems.value = selectedList.value.items;
            }
        }
    }

    async function deleteSelectedList(listId: String, userId: String) {
        const { data } = await apolloClient.mutate({
            mutation: deleteList,
            variables: { listId },
            fetchPolicy: 'no-cache'
        });
        if (data && data.deleteList) {
            const res: DeleteResult = data.deleteList;
            if (res.status == DeleteStatus.ResourceDeleted) {
                toast.success("Liste supprimée !", {
                    position: toast.POSITION.BOTTOM_CENTER,
                });
                await fetchLists(userId);
            } else {
                toast.error("Erreur lors de la suppression !", {
                    position: toast.POSITION.BOTTOM_CENTER,
                });
            }
        }
    }

    async function deleteItemFromList(itemId: string, listId: string) {
        const { data } = await apolloClient.mutate({
            mutation: deleteItem,
            variables: { itemId },
            fetchPolicy: 'no-cache'
        });
        if (data && data.deleteItem) {
            const res: DeleteResult = data.deleteItem;
            if (res.status == DeleteStatus.ResourceDeleted) {
                toast.success("Item supprimé !", {
                    position: toast.POSITION.BOTTOM_CENTER,
                });
                await fetchOne(listId);
            } else {
                toast.error("Erreur lors de la suppression !", {
                    position: toast.POSITION.BOTTOM_CENTER,
                });
            }
        }
    }

    async function addItemToList(listId: string, input: NewItem) {
        const { data } = await apolloClient.mutate({
            mutation: createItem,
            variables: { input },
            fetchPolicy: 'no-cache'
        });
        if (data && data.createItem) {
            toast.success("Item ajouté !", {
                position: toast.POSITION.BOTTOM_CENTER,
            });
            await fetchOne(listId);
        }
    }

    async function updateItemFromList(input: Item, listId: string) {
        const { data } = await apolloClient.mutate({
            mutation: updateItem,
            variables: { input },
            fetchPolicy: 'no-cache'
        });
        if (data && data.updateItem) {
            toast.success("Item mis à jour !", {
                position: toast.POSITION.BOTTOM_CENTER,
            });
            await fetchOne(listId);
        }
    }

    async function inviteUserToMyList(listId: String, userId: String, friendId: String) {
        const { data } = await apolloClient.mutate({
            mutation: inviteUserToYourList,
            variables: { listId, userId, friendId },
            fetchPolicy: 'no-cache'
        });
        if (data && data.inviteUserToYourList) {
            const res: AddFriendToMyListResult = data.inviteUserToYourList;
            if (res.status == AddFriendToMyListStatus.AddSuccessful) {
                toast.success("Votre ami peut voir la liste !", {
                    position: toast.POSITION.BOTTOM_CENTER,
                });
                await fetchLists(userId);
                await fetchOne(listId);
            } else {
                toast.error("Erreur lors du partage !", {
                    position: toast.POSITION.BOTTOM_CENTER,
                });
            }
        }
    }

    async function removeAUserFromList(listId: String, userId: String, friendId: String) {
        const { data } = await apolloClient.mutate({
            mutation: removeUserFromList,
            variables: { listId, friendId },
            fetchPolicy: 'no-cache'
        });
        if (data && data.removeUserFromList) {
            const res: RemoveFriendFromMyListResult = data.removeUserFromList;
            if (res.status == RemoveFriendFromMyListStatus.RemoveSuccessful) {
                toast.success("Utilisateur retiré de la liste.", {
                    position: toast.POSITION.BOTTOM_CENTER,
                });
                await fetchLists(userId);
                await fetchOne(listId);
            } else {
                toast.error("Erreur lors de la suppression !", {
                    position: toast.POSITION.BOTTOM_CENTER,
                });
            }
        }
    }


    return {
        lists,
        selectedList,
        selectedItems,
        fetchLists,
        createNewList,
        deleteSelectedList,
        fetchOne,
        addItemToList,
        deleteItemFromList,
        updateItemFromList,
        inviteUserToMyList,
        removeAUserFromList
    }
})
