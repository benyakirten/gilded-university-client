<script setup lang="ts">
import { useQuery } from "@vue/apollo-composable"
import gql from "graphql-tag";

import Greet from "./components/Greet.vue";
import { UsersResponse } from "@types";
import { watch } from "vue";

const { result } = useQuery<UsersResponse>(gql`
  query users {
    id
    email
    name
  }
`)

watch(result, value => {
  alert(value)
})
</script>

<template>
  <div class="container">
    <h1 v-if="!result">NO RESULT</h1>
    <ul v-if="result">
      <li :v-for="user in result.users">
        {user}
      </li>
    </ul>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
