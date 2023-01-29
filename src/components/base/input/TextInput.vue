<script lang="ts" setup>
import { ref, computed } from 'vue';

  const { name, value } = defineProps<{ class?: string, style?: string, name: string, value: string }>()
  const emit = defineEmits<{ (e: "change", val: string): void, (e: "enter", val: string): void }>()

  const inputRef = ref<HTMLInputElement | null>(null)
  const height = computed(() => inputRef.value?.getBoundingClientRect().height)

  const inputFocused = ref(false)
  const placeholderShown = computed(() => !inputFocused.value && value === "")

  function handleChange(e: Event) {
    if (!e.target || !("value" in e.target) || typeof e.target.value !== "string") {
      return
    }
    emit("change", e.target.value)
  }

  function handleEnter(e: KeyboardEvent) {
    if (e.shiftKey || !e.target || !("value" in e.target) || typeof e.target.value !== "string") {
      return
    }
    emit("enter", e.target.value)
  }

  function handleInput(e: Event) {
    if (!e.target || !("value" in e.target) || typeof e.target.value !== "string") {
      return
    }
  }

  function focusInput() {
    inputRef.value?.focus()
  }

  function setInputFocus() {
    inputFocused.value = true
  }

  function removeInputFocus() {
    inputFocused.value = false
  }

  const computedStyle = computed(() => placeholderShown.value ? "" : `font-size: 1rem; line-height: 1.5rem; transform: translateY(-${height.value}px);`)
</script>

<template>
  <div class="grid relative">
    <input @change="handleChange" @keydown.enter="handleEnter" @input="handleInput" @focusin="setInputFocus" @focusout="removeInputFocus" ref="inputRef" type="text" :value="value" :name="name" placeholder="" />
    <label :style="computedStyle" :for="name" class="transition-all duration-500 text-sm cursor-text absolute" @click="focusInput">
      <slot name="label"></slot>
    </label>
  </div>
</template>

