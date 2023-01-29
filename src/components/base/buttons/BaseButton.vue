<script lang="ts" setup>
  import { useStyleProps } from "@/utils"
  import { ForeignStyles } from '@/types'

  const { class: classes, style } = useStyleProps<ForeignStyles>()
  const emit = defineEmits<{ (e: "click", event: MouseEvent ): void}>()

  function handleClick(e: MouseEvent) {
    emit("click", e)
  }
</script>

<template>
  <button :class="classes" :style="style" @click="handleClick">
    <slot></slot>
  </button>
</template>
