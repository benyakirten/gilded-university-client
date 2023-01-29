<script lang="ts" setup>
  import { useStyleProps } from "@/utils"
  import { ForeignStyles } from '@/types'

  const { class: classes, style } = useStyleProps<ForeignStyles>()
  const emit = defineEmits<{ (e: "click", event: MouseEvent ): void}>()
</script>

<template>
  <button :class="classes" :style="style" @click="(e) => emit("click", e)">
    <slot></slot>
  </button>
</template>
