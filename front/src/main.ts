import App from './App.vue'
import { DefaultApolloClient } from '@vue/apollo-composable'
import { apolloClient } from './apollo'
import { createApp } from 'vue'
import { registerPlugins } from '@/plugins'

const app = createApp(App).provide(DefaultApolloClient, apolloClient)

registerPlugins(app)

app.mount('#app')
