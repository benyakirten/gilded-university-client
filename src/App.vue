<script setup lang="ts">
import { useQuery } from "@vue/apollo-composable"
import gql from "graphql-tag";

import { UsersResponse } from "@types";
import { watch } from "vue";

const { result } = useQuery<UsersResponse>(gql`
  query {
    users {
      id
      email
      name
      role
      status
    }
  }
`)

watch(result, value => {
  value?.users.forEach(u => Object.keys(u).forEach(console.log))
})
</script>

<template>
  <div class="container">
    <div class="text-xl" v-if="result">
      <ul>
        <li v-for="user of result?.users">
          {{ user.email }} - {{ user.id }} - {{ user.name }} - {{ user.role }} - {{ user.status }}
        </li>
      </ul>
    </div>
    <div v-else>
      I'm sorry but no result!
    </div>
  </div>
</template>

