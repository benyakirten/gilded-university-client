import { defineStore } from "pinia"
import { ref } from "vue"
import { PageEnum } from "./types"

export const usePageStore = defineStore('config', () => {
  const page = ref<PageEnum>(PageEnum.HOME)

  return { page }
})
