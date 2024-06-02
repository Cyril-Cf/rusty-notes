import { ref } from 'vue'
import { defineStore } from 'pinia'
import { apolloClient } from '../apollo'
import { acceptListInvitation } from '@/graphql/mutations/list/acceptListInvitation.mutation'
import { refuseListInvitation } from '@/graphql/mutations/user/refuseListInvitation.mutation'
import { updateItem } from '../graphql/mutations/item/updateItem.mutation'
import { deleteItem } from '../graphql/mutations/item/deleteItem.mutation'
import { createItem } from '../graphql/mutations/item/createItem.mutation'
import { inviteUserToYourList } from '@/graphql/mutations/userList/inviteUserToYourList.mutation'
import { removeUserFromList } from '@/graphql/mutations/userList/removeUserFromList.mutation'
import { findAllListForUserWithTags } from '../graphql/queries/list/findAllListForUserWithTags.query';
import { findOneWithItemsAndTags } from '../graphql/queries/list/findOneWithItemsAndTags.query'
import { createList } from '../graphql/mutations/list/createList.mutation'
import { deleteList } from '../graphql/mutations/list/deleteList.mutation'
import { List, ListPermission, NewList } from '../types/List';
import { NewItem, Item } from '../types/Item'
import { DeleteStatus, DeleteResult, AddFriendToMyListStatus, AddFriendToMyListResult, RemoveFriendFromMyListResult, RemoveFriendFromMyListStatus, RefuseListInvitationResult, RefuseListInvitationStatus, AcceptListInvitationResult, AcceptListInvitationStatus } from '@/types/Utils';
import { toast } from 'vue3-toastify';
import { useUserStore } from './userStore'
import { User } from '@/types/User'

export const useListStore = defineStore('list', () => {
    const userStore = useUserStore();
    const lists = ref<List[]>([]);
    const ownedLists = ref<List[]>([]);
    const sharedListsValidated = ref<List[]>([]);
    const sharedListToValidate = ref<List[]>([]);
    const selectedList = ref<List | undefined>();
    const selectedItems = ref<Item[]>([])
    const usersValidated = ref<User[]>([]);
    const usersAwaitingValidation = ref<User[]>([]);

    async function fetchLists(userId: String) {
        const { data } = await apolloClient.query({
            query: findAllListForUserWithTags,
            variables: { userId },
            fetchPolicy: 'no-cache'
        });
        if (data && data.findAllListForUserWithTags) {
            lists.value = data.findAllListForUserWithTags;
            ownedLists.value = [];
            sharedListToValidate.value = [];
            sharedListsValidated.value = [];
            lists.value.forEach((list) => {
                if (list.isOwner) {
                    ownedLists.value.push(list);
                } else if (list.isValidated) {
                    sharedListsValidated.value.push(list);
                } else {
                    sharedListToValidate.value.push(list)
                }
            })
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
        let userId = userStore.currentUser?.id;
        const { data } = await apolloClient.query({
            query: findOneWithItemsAndTags,
            variables: { listId, userId },
            fetchPolicy: 'no-cache'
        });
        if (data && data.findOneWithItemsAndTags) {
            selectedList.value = data.findOneWithItemsAndTags;
            if (selectedList.value) {
                selectedItems.value = selectedList.value.items;
                usersValidated.value = selectedList.value.usersValidated;
                usersAwaitingValidation.value = selectedList.value.usersAwaitingValidation;
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

    async function inviteUserToMyList(listId: String, userId: String, friendId: String, permission: ListPermission) {
        const { data } = await apolloClient.mutate({
            mutation: inviteUserToYourList,
            variables: { listId, userId, friendId, permission },
            fetchPolicy: 'no-cache'
        });
        if (data && data.inviteUserToYourList) {
            const res: AddFriendToMyListResult = data.inviteUserToYourList;
            if (res.status == AddFriendToMyListStatus.AddSuccessful) {
                toast.success("Invitation envoyée !", {
                    position: toast.POSITION.BOTTOM_CENTER,
                });
                await fetchLists(userId);
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
            } else {
                toast.error("Erreur lors de la suppression !", {
                    position: toast.POSITION.BOTTOM_CENTER,
                });
            }
        }
    }

    async function acceptListInvitationStore(listId: String, userId: String) {
        const { data } = await apolloClient.mutate({
            mutation: acceptListInvitation,
            variables: { listId, userId },
            fetchPolicy: 'no-cache'
        });
        if (data && data.acceptListInvitation) {
            const res: AcceptListInvitationResult = data.acceptListInvitation;
            if (res.status == AcceptListInvitationStatus.ACCEPT_SUCCESSFUL) {
                toast.success("Liste ajoutée aux votres !", {
                    position: toast.POSITION.BOTTOM_CENTER,
                });
                await fetchLists(userId);
            } else {
                toast.error("Erreur lors du partage !", {
                    position: toast.POSITION.BOTTOM_CENTER,
                });
            }
        }
    }

    async function refuseListInvitationStore(listId: String, userId: String) {
        const { data } = await apolloClient.mutate({
            mutation: refuseListInvitation,
            variables: { listId, userId },
            fetchPolicy: 'no-cache'
        });
        if (data && data.refuseListInvitation) {
            const res: RefuseListInvitationResult = data.refuseListInvitation;
            if (res.status == RefuseListInvitationStatus.REFUSE_SUCCESSFUL) {
                toast.success("Refus prise en compte", {
                    position: toast.POSITION.BOTTOM_CENTER,
                });
                await fetchLists(userId);
            } else {
                toast.error("Erreur lors du partage !", {
                    position: toast.POSITION.BOTTOM_CENTER,
                });
            }
        }
    }


    return {
        ownedLists,
        sharedListToValidate,
        sharedListsValidated,
        selectedList,
        selectedItems,
        usersValidated,
        usersAwaitingValidation,
        fetchLists,
        createNewList,
        deleteSelectedList,
        fetchOne,
        addItemToList,
        deleteItemFromList,
        updateItemFromList,
        inviteUserToMyList,
        removeAUserFromList,
        acceptListInvitationStore,
        refuseListInvitationStore
    }
})
