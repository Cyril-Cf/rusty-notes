import { ref } from 'vue'
import { defineStore } from 'pinia'
import { apolloClient } from '../apollo'
import { findUserWithKeycloakId } from '../graphql/queries/findUserWithKeycloakId.query';
import { createUser } from '@/graphql/mutations/createUser.mutation';
import { User, NewUser } from '../types/User'

export const useUserStore = defineStore('user', () => {
  const currentUser = ref<User | undefined>();
  const userFriends = ref<User[]>([]);

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

  return { userFriends, currentUser, DoesUserExistInDB, createCurrentUser }
})
