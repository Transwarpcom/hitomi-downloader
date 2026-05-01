<script setup lang="ts">
import { useStore } from '../store.ts'
import { commands } from '../bindings.ts'
import { ref, watch, onMounted, onUnmounted } from 'vue'

const store = useStore()
const currentIndex = ref(0)
const currentImage = ref<string | undefined>(undefined)
const loading = ref(false)

async function loadCurrentImage() {
  if (!store.pickedComic || store.pickedComic.files.length === 0) return

  loading.value = true
  const file = store.pickedComic.files[currentIndex.value]
  const result = await commands.getImageData(store.pickedComic, file)

  if (result.status === 'ok') {
    const data = new Uint8Array(result.data)
    const blob = new Blob([data])
    if (currentImage.value) {
      URL.revokeObjectURL(currentImage.value)
    }
    currentImage.value = URL.createObjectURL(blob)
  } else {
    console.error(result.error)
  }
  loading.value = false
}

watch(
  () => store.pickedComic,
  () => {
    currentIndex.value = 0
    if (store.currentTabName === 'reader') {
      loadCurrentImage()
    }
  }
)

watch(
  () => store.currentTabName,
  (name) => {
    if (name === 'reader') {
      loadCurrentImage()
    }
  }
)

watch(currentIndex, loadCurrentImage)

function handleKeydown(e: KeyboardEvent) {
  if (store.currentTabName !== 'reader') return

  if (e.key === 'ArrowRight' || e.key === 'ArrowDown') {
    e.preventDefault()
    if (store.pickedComic && currentIndex.value < store.pickedComic.files.length - 1) {
      currentIndex.value++
    }
  } else if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') {
    e.preventDefault()
    if (currentIndex.value > 0) {
      currentIndex.value--
    }
  } else if (e.key === ' ') {
    e.preventDefault()
    store.currentTabName = 'comic'
  } else if (e.key === 'Enter') {
    e.preventDefault()
    store.currentTabName = 'search' // Can be downloaded based on previous context, but search is fine
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
  if (currentImage.value) {
    URL.revokeObjectURL(currentImage.value)
  }
})
</script>

<template>
  <div class="h-full flex flex-col items-center justify-center bg-gray-100 p-2 relative outline-none" tabindex="0">
    <n-spin v-if="loading" class="absolute z-10" />
    <img
      v-if="currentImage"
      :src="currentImage"
      class="max-w-full max-h-full object-contain"
    />
    <div v-if="store.pickedComic" class="absolute bottom-2 bg-black/50 text-white px-2 py-1 rounded text-sm">
      {{ currentIndex + 1 }} / {{ store.pickedComic.files.length }}
    </div>
  </div>
</template>
