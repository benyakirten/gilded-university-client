<script lang="ts" setup>

import { ref, computed } from 'vue';
import { convertObjectToStyleString } from '@/utils';

  const { name, value, style, class: classes, labelClass, labelStyles} = defineProps<{ labelClass?: string, labelStyles?: Record<string, string>, class?: string, style?: Record<string, string>, name: string, value: string }>()
  const emit = defineEmits<{
    (e: "keydown", val: string): void,
  }>()

  const foreignInputClasses = computed(() => classes ?? "")
  const foreignInputStyles = computed(() => convertObjectToStyleString(style))

  const foreignLabelClasses = computed(() => labelClass ?? "")
  const foreignLabelStyles = computed(() => convertObjectToStyleString(labelStyles))

  const inputRef = ref<HTMLInputElement | null>(null)
  function getTargetInputValue(e: Event): string | null {
    if (!e.target || !("value" in e.target) || typeof e.target.value !== "string") {
      return null
    }
    return e.target.value
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && e.shiftKey) {
      return
    }
    const value = getTargetInputValue(e)
    if (!value) {
      return
    }
    emit("keydown", value)
  }
</script>

<template>
  <div class="flex flex-col">
    <label
      :for="name"
      :style="foreignLabelStyles"
      :class="foreignLabelClasses"
    >
      <slot name="label"></slot>
    </label>
    <input
      @keydown="handleKeydown"
      ref="inputRef"
      type="text"
      :value="value"
      :name="name"
      placeholder=""
      :style="foreignInputStyles"
      class="pl-[2px]"
      :class="foreignInputClasses"
    />
  </div>
</template>

