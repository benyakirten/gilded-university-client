import { ForeignStyles } from "@/types"
import { ref, watch, type Ref } from "vue"

export function convertObjectToStyleString(record: Record<string, string>): string {
  return Object.entries(record).reduce<string>((acc, [key, value], idx) => {
    const styleString = `${key}: ${value};`
    if (idx > 0) {
      acc += " "
    }
    return `${acc}${styleString}`
  }, "")
}

export function useStyleProps<T extends ForeignStyles>(baseClasses: string = "") {
  const props = defineProps<T>()
  const combinedClasses = ref("")
  watch({ classes: props.class }, ({ classes }) => {
    combinedClasses.value = `${baseClasses} ${classes}`
  })

  const convertedStyles = ref("")
  watch({ style: props.style }, ({ style }) => {
    convertedStyles.value = convertObjectToStyleString(style)
  })

  return { ...props, classes: combinedClasses, style: convertedStyles }
}
