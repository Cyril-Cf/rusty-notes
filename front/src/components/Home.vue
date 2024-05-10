<script>
import api from '../services/api';

export default {
  data() {
    return {
      loading: false,
      result: '',
    };
  },
  created() {},
  methods: {
    async validateToken() {
      try {
        this.loading = true;
        const apiURL = '/auth/validate-token';
        const res = await api.post(apiURL);
        this.result = JSON.stringify(res.data, null, 2);

        return;
      } catch (error) {
        console.error(error);
        return;
      } finally {
        this.loading = false;
      }
    },
    async getMenuHandler() {
      try {
        this.loading = true;
        const apiURL = '/menu/menus';
        const res = await api.get(apiURL);
        this.result = res.data;

        return;
      } catch (error) {
        console.error(error);
        return;
      } finally {
        this.loading = false;
      }
    },
  },
};
</script>

<template>
  <div>
    <a href="https://keycloak.org/" target="_blank">
      <img
        src="@/assets/keycloak.svg"
        class="logo keycloak"
        alt="Keycloak logo"
      />
    </a>
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo" alt="Vite logo" />
    </a>
    <a href="https://vuejs.org/" target="_blank">
      <img src="@/assets/vue.svg" class="logo vue" alt="Vue logo" />
    </a>
    <a href="https://pinia.vuejs.org/" target="_blank">
      <img src="@/assets/pinia.svg" class="logo pinia" alt="Pinia logo" />
    </a>
  </div>

  <p class="read-the-docs">
    Keycloak Authentication and Authorization with Vue 3 + Pinia + Express
  </p>

  <div class="card">
    <!-- getMenus validation button -->
    <button
      class="mr-15"
      type="button"
      title="Check Console with Dev Tools"
      :disabled="loading"
      @click="getMenuHandler"
    >
      Get Menus
    </button>

    <!-- Refresh token button -->
    <button
      class="mr-15"
      type="button"
      title="Refreshes user token"
      @click="$store.refreshUserToken"
    >
      Refresh Token
    </button>

    <!-- Token validation button -->
    <button
      class="mr-15"
      type="button"
      title="Check Console with Dev Tools"
      :disabled="loading"
      @click="validateToken"
    >
      Validate Token
    </button>

    <!-- Logout button -->
    <button type="button" title="Logout Keycloak user" @click="$store.logout">
      Logout
    </button>
  </div>

  <div v-if="result" class="card">
    {{ result }}
  </div>

  <div class="card py-10">
    <h2>Info</h2>
    <p class="my-5">Username: {{ $store.user.username }}</p>
    <p class="my-5">
      Name: {{ $store.user.family_name }} {{ $store.user.given_name }}
    </p>
    <p class="my-5">Role: {{ $store.user.roles }}</p>
    <p class="my-5">Token:</p>
    <p class="wrap-text font-small my-5">{{ $store.user.token }}</p>
    <p class="my-5">Refresh Token:</p>
    <p class="wrap-text font-small my-5">{{ $store.user.refToken }}</p>
  </div>
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
.logo.keycloak:hover {
  filter: drop-shadow(0 0 2em #00b8e2aa);
}
.logo.pinia:hover {
  filter: drop-shadow(0 0 2em #ffe56c);
}

.read-the-docs {
  color: #888;
}

.my-5 {
  margin-top: 5px;
  margin-bottom: 5px;
}

.mr-15 {
  margin-right: 15px;
}

.py-10 {
  padding-top: 10px;
  padding-bottom: 10px;
}

.wrap-text {
  word-wrap: break-word;
}

.font-small {
  font-size: 10px;
}
</style>
