<script setup lang="ts">
import { useQuery } from "@vue/apollo-composable"
import gql from "graphql-tag";

import { UsersResponse } from "@types";
import { watch } from "vue";
import { useUserStore } from "./store/user";
import { BaseButton, CenterItems } from "./components";

const userState = useUserStore()

// const { result } = useQuery<UsersResponse>(gql`
//   query {
//     users {
//       id
//       email
//       name
//       role
//       status
//     }
//   }
// `)

// Check if user is stored by tauri - store encrypted in file
// If user is defined, send request to backend to check if token has expired
// If token has expired, attempt to use refresh token
// If refresh doesn't work, require user to log in
// If login succeeds
</script>

<template>
  <main class="min-h-screen flex">
    <CenterItems>
      <BaseButton>
        Click me
      </BaseButton>
    </CenterItems>
  </main>
</template>
