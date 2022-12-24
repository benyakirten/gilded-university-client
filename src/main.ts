import { createApp } from "vue";
import { createPinia } from "pinia";
import ApolloClient from 'apollo-boost'
import VueApollo from 'vue-apollo'

import "./style.css";
import App from "./App.vue";


const apolloClient = new ApolloClient({
  // You should use an absolute URL here
  uri: import.meta.env.VITE_GRAPHQL_URI
})

export const apolloProvider = new VueApollo({
  defaultClient: apolloClient,
})



const pinia = createPinia()
const app = createApp(App)

app.use(pinia)
app.use(apolloProvider)
app.mount("#app");
