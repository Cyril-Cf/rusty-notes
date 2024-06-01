import { ref } from 'vue'
import { defineStore } from 'pinia'
import { apolloClient } from '../apollo'
import { findUserWithKeycloakId } from '../graphql/queries/findUserWithKeycloakId.query';
import { addUserFriend } from '@/graphql/mutations/addUserFriend.mutation';
import { getUserFriendships } from '@/graphql/queries/getUserFriendships.query';
import { confirmFriendship } from '@/graphql/mutations/confirmFriendship.mutation';
import { removeUserFriend } from '@/graphql/mutations/removeUserFriend.mutation';
import { createUser } from '@/graphql/mutations/createUser.mutation';
import { User, NewUser } from '../types/User'
import { AddFriendResult, AddFriendStatus, Friendship, FriendshipAcceptingResult, FriendshipAcceptingStatus, RemoveFriendResult, RemoveFriendStatus } from '@/types/Friendship';
import { toast } from 'vue3-toastify';

export const useUserStore = defineStore('user', () => {
  const currentUser = ref<User | undefined>();
  const userFriends = ref<User[]>([]);
  const friendships = ref<Friendship[]>([]);

  async function DoesUserExistInDB(keycloakId: String): Promise<boolean> {
    const { data } = await apolloClient.query({
      query: findUserWithKeycloakId,
      variables: { keycloakId },
      fetchPolicy: 'no-cache'
    });
    if (data && data.findUserWithKeycloakId) {
      currentUser.value = data.findUserWithKeycloakId;
      return true;
    }
    return false;
  }

  async function createCurrentUser(input: NewUser) {
    const { data } = await apolloClient.mutate({
      mutation: createUser,
      variables: { input },
      fetchPolicy: 'no-cache'
    });
    if (data && data.createUser) {
      currentUser.value = data.createUser;
    }
  }

  async function getFriendships(userId: String) {
    const { data } = await apolloClient.query({
      query: getUserFriendships,
      variables: { userId },
      fetchPolicy: 'no-cache'
    });
    if (data && data.getUserFriendships) {
      friendships.value = data.getUserFriendships;
      let friends: User[] = [];
      friendships.value.forEach((friendship) => {
        if (friendship.friendWhoAsked.id === userId) {
          friends.push(friendship.friendWhoGotAsked);
        } else {
          friends.push(friendship.friendWhoAsked)
        }
      });
      userFriends.value = friends;
    }
  }

  async function addUserFriendship(userWhoAskedId: String, userWhoGetAskedEmail: String) {
    const { data } = await apolloClient.mutate({
      mutation: addUserFriend,
      variables: { userWhoAskedId, userWhoGetAskedEmail },
      fetchPolicy: 'no-cache'
    });
    if (data && data.addUserFriend) {
      const res: AddFriendResult = data.addUserFriend;
      if (res.status == AddFriendStatus.AddSuccessful) {
        toast.success("Demande envoyée !", {
          position: toast.POSITION.BOTTOM_CENTER,
        });
        await getFriendships(userWhoAskedId);
      } else if (res.status == AddFriendStatus.ErrAlreadyFriend) {
        toast.error("Vous êtes déjà amis !", {
          position: toast.POSITION.BOTTOM_CENTER,
        });
      } else if (res.status == AddFriendStatus.ErrAlreadyPendingDemand) {
        toast.error("Il existe déjà une demande en attente !", {
          position: toast.POSITION.BOTTOM_CENTER,
        });
      } else if (res.status == AddFriendStatus.ErrNoUserEmail) {
        toast.error("Aucun utilisateur n'emploie cet email !", {
          position: toast.POSITION.BOTTOM_CENTER,
        });
      }
    }
  }

  async function acceptFriendship(userAskedId: String, userAskingId: String, userId: String) {
    const { data } = await apolloClient.mutate({
      mutation: confirmFriendship,
      variables: { userAskedId, userAskingId },
      fetchPolicy: 'no-cache'
    });
    if (data && data.confirmFriendship) {
      const res: FriendshipAcceptingResult = data.confirmFriendship;
      if (res.status == FriendshipAcceptingStatus.AcceptingSuccessful) {
        toast.success("Une amitiée est née !", {
          position: toast.POSITION.BOTTOM_CENTER,
        });
        await getFriendships(userId);
      } else if (res.status == FriendshipAcceptingStatus.ErrCannotAccept) {
        toast.error("Un soucis s'est produit, contactez nos équipes !", {
          position: toast.POSITION.BOTTOM_CENTER,
        });
      }
    }
  }

  async function removeFriendship(userId: String, userFriendId: String) {
    const { data } = await apolloClient.mutate({
      mutation: removeUserFriend,
      variables: { userId, userFriendId },
      fetchPolicy: 'no-cache'
    });
    if (data && data.removeUserFriend) {
      const res: RemoveFriendResult = data.removeUserFriend;
      if (res.status == RemoveFriendStatus.RemoveSuccessful) {
        toast.success("Une amitiée se fâne...", {
          position: toast.POSITION.BOTTOM_CENTER,
        });
        await getFriendships(userId);
      } else if (res.status == RemoveFriendStatus.ErrNoFriendship) {
        toast.error("Un soucis s'est produit, contactez nos équipes !", {
          position: toast.POSITION.BOTTOM_CENTER,
        });
      }
    }
  }

  return {
    userFriends,
    currentUser,
    friendships,
    DoesUserExistInDB,
    createCurrentUser,
    getFriendships,
    addUserFriendship,
    acceptFriendship,
    removeFriendship
  }
})
