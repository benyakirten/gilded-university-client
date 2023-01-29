<script lang="ts" setup>
  import { useStyleProps } from "@/utils"
  import { ForeignStyles } from '@/types'

  const props = useStyleProps<ForeignStyles>()
</script>

<template>
    <slot :props="props"></slot>
</template>
