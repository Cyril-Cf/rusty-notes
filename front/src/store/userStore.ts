// Utilities
import { ref } from 'vue'
import { User } from '@/models/user'
import { defineStore } from 'pinia'
import axios from 'axios'
import { KeycloakProfile } from "keycloak-js";

export const useUserStore = defineStore('user', () => {
  const URL_FRAGMENT = "users"
  const currentUser = ref<User | undefined>();
  const userFriends = ref<User[]>([]);

  async function fetchUsers(token: string) {
    console.log(token)
    const res = await axios.get(`${import.meta.env.VITE_BACK_API_URL}${URL_FRAGMENT}`, { headers: {"Authorization" : `Bearer ${token}`} });
    console.log(res);
  }

  function setCurrentUser(keycloak: KeycloakProfile) {
    console.log(keycloak);
    currentUser.value = new User("");
  }
  return {userFriends, currentUser, fetchUsers}
})
