import { computed } from "vue"

export function convertObjectToStyleString(record: Record<string, string> = {}): string {
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

export enum CornerEnum {
  TOP_LEFT = "[Corner] Top Left",
  TOP_RIGHT = "[Corner] Top Right",
  BOTTOM_LEFT = "[Corner] Bottom Left",
  BOTTOM_RIGHT = "[Corner] Bottom Right",
}

export function findCorner(e: MouseEvent): CornerEnum | null {
  const { clientX, clientY, target } = e
  if (!target || !("getBoundingClientRect" in target)) {
    return null
  }
  const { x, y, width, height } = (target as HTMLElement).getBoundingClientRect()
  const relX = clientX - x
  const relY = clientY - y

  const right = relX >= width / 2
  const bottom = relY >= height / 2

  if (bottom) {
    if (right) {
      return CornerEnum.BOTTOM_RIGHT
    }
    return CornerEnum.BOTTOM_LEFT
  }

  if (right) {
    return CornerEnum.TOP_RIGHT
  }

  return CornerEnum.TOP_LEFT
}
