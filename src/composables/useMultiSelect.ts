import { ref } from 'vue'
import { SelectionArea, SelectionEvent } from '@viselect/vue'

export function useMultiSelect() {
  const selectedIds = ref<Set<number>>(new Set())
  const selectionAreaRef = ref<InstanceType<typeof SelectionArea>>()
  const selectableRefs = ref<HTMLDivElement[]>([])

  function extractIds(elements: Element[]): number[] {
    return elements
      .map((element) => element.getAttribute('data-key'))
      .filter(Boolean)
      .map(Number)
  }

  function updateSelectedIds({
    store: {
      changed: { added, removed },
    },
  }: SelectionEvent) {
    extractIds(added).forEach((id) => selectedIds.value.add(id))
    extractIds(removed).forEach((id) => selectedIds.value.delete(id))
  }

  function unselectAll({ event, selection }: SelectionEvent) {
    if (!event?.ctrlKey && !event?.metaKey) {
      selection.clearSelection()
      selectedIds.value.clear()
    }
  }

  function onContextMenu(id: number) {
    if (selectedIds.value.has(id)) {
      return
    }
    selectedIds.value.clear()
    selectedIds.value.add(id)
  }

  return {
    selectedIds,
    selectionAreaRef,
    selectableRefs,
    updateSelectedIds,
    unselectAll,
    onContextMenu,
  }
}
