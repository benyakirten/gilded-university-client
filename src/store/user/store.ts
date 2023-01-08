import { ref } from 'vue'
import { defineStore } from 'pinia'

import { User } from '@types'

export const useUserStore = defineStore('config', () => {
  const user = ref<null | User>(null)
  function logout() {
    user.value = null
  }
  function login(newUser: User) {
    user.value = newUser
  }
  return { user, logout, login }
})
