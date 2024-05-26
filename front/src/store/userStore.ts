import { ref } from 'vue'
import { defineStore } from 'pinia'
import { KeycloakProfile } from "keycloak-js";
import { apolloClient } from '../apollo'
import { findOneWithItemsAndTags } from '../graphql/queries/listTag.query';
import { findUserWithKeycloakId } from '../graphql/queries/findUserWithKeycloakId.query';
import { createUser } from '@/graphql/mutations/createUser.mutation';
import { User, NewUser } from '../types/User'
import { List } from '../types/List';

export const useUserStore = defineStore('user', () => {
  const currentUser = ref<User | undefined>();
  const userFriends = ref<User[]>([]);

  async function fetchUsers(token: string) {
    // console.log(token)
    // const res = await axios.get(`${import.meta.env.VITE_BACK_API_URL}${URL_FRAGMENT}`, { headers: {"Authorization" : `Bearer ${token}`} });
    // console.log(res);
    const { data } = await apolloClient.query({
      query: findOneWithItemsAndTags,
      variables: { id: "19cd1ab8-c06c-4d34-9e5b-5623c9e04da6" },
      fetchPolicy: 'no-cache'
    });
    if (data && data.findOneWithItemsAndTags) {
      const list: List = data.findOneWithItemsAndTags;
      console.log(list);
    }
  }

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

  return { userFriends, currentUser, fetchUsers, DoesUserExistInDB, createCurrentUser }
})
