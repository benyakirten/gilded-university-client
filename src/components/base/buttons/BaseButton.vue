<script lang="ts" setup>
  import { useForeignStyles } from "@/utils"

  const props = defineProps<{ class?: string, style?: Record<string, string>}>()
  const baseClasses = "transition-all duration-500 px-2 py-1 rounded-xl capitalize hover:shadow-lg hover:bg-slate-100 active:shadow-sm active:translate-y-[2px]"
  const { classes, style } = useForeignStyles(baseClasses, props.class, props.style)
  const emit = defineEmits<{ (e: "click", event: MouseEvent ): void}>()

  function handleClick(e: MouseEvent) {
    emit("click", e)
  }
</script>

<template>
  <button class="" :class="classes" :style="style" @click="handleClick">
    <slot></slot>
  </button>
</template>
