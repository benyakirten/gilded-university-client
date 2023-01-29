<script lang="ts" setup>
  import { computed, ref } from "vue"

  import { convertObjectToStyleString, findCorner, CornerEnum } from "@/utils"

  const { class: classes, style, type } = defineProps<{ class?: string, style?: Record<string, string>, type?: "button" | "submit"}>()
  const emit = defineEmits<{ (e: "click", event: MouseEvent ): void}>()
  function handleClick(e: MouseEvent) {
    setCorner(e)
    emit("click", e)
  }

  const foreignStyles = computed(() => convertObjectToStyleString(style ?? {}))
  const buttonType = computed(() => type ?? "button")

  const corner = ref(CornerEnum.TOP_LEFT)
  function setCorner(e: MouseEvent) {
    corner.value = findCorner(e) ?? CornerEnum.TOP_LEFT
  }

  const totalClasses = computed(() => {
    const rotation = (corner.value === CornerEnum.TOP_LEFT || corner.value === CornerEnum.BOTTOM_LEFT) ? 'active:-rotate-1' : 'active:rotate-1'
    return `${rotation} ${classes}`
  })
</script>

<template>
  <button
    class="transition-all duration-300 px-2 py-1 rounded-xl capitalize hover:shadow-lg hover:bg-slate-100 active:shadow-sm active:translate active:translate-y-[2px]"
    :class="totalClasses"
    :style="foreignStyles"
    :type="buttonType"
    @click="handleClick"
  >
    <slot></slot>
  </button>
</template>

