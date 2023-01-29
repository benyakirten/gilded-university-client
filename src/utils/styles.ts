import { computed } from "vue"

export function convertObjectToStyleString(record: Record<string, string>): string {
  return Object.entries(record).reduce<string>((acc, [key, value], idx) => {
    const styleString = `${key}: ${value};`
    if (idx > 0) {
      acc += " "
    }
    return `${acc}${styleString}`
  }, "")
}

export function useForeignStyles(baseClasses: string = "", foreignClasses: string = "", foreignStyles: Record<string, string> = {}) {
  const combinedClasses = computed(() => `${baseClasses} ${foreignClasses}`)
  const convertedStyles = computed(() => convertObjectToStyleString(foreignStyles))

  return { classes: combinedClasses, style: convertedStyles }
}
