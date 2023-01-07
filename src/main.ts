import { createApp, provide, h } from "vue";
import { DefaultApolloClient } from "@vue/apollo-composable"
import { createPinia } from "pinia";
import ApolloClient from 'apollo-boost'

import "./style.css";
import App from "./App.vue";


const apolloClient = new ApolloClient({
  // You should use an absolute URL here
  uri: import.meta.env.VITE_GRAPHQL_URL
})

const pinia = createPinia()
const app = createApp({
  setup() {
    provide(DefaultApolloClient, apolloClient)
  },
  render: () => h(App),
})

app.use(pinia)
app.mount("#app");
