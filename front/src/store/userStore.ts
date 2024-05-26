import { ref } from 'vue'
import { User } from '@/models/user'
import { defineStore } from 'pinia'
import { KeycloakProfile } from "keycloak-js";
import { apolloClient } from '../apollo'
import { findOneWithItemsAndTags } from '../graphql/queries/listTag.query';
import { List } from '../types/List';

export const useUserStore = defineStore('user', () => {
  const URL_FRAGMENT = "users"
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

  function setCurrentUser(keycloak: KeycloakProfile) {
    console.log(keycloak);
    currentUser.value = new User("");
  }
  return { userFriends, currentUser, fetchUsers }
})
