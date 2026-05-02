<script setup lang="ts">
import { useStore } from '../store.ts'
import { commands } from '../bindings.ts'
import { ref, watch, onMounted, onUnmounted } from 'vue'

const store = useStore()
const currentIndex = ref(0)
const currentImage = ref<string | undefined>(undefined)
const loading = ref(false)

const imageCache = new Map<number, Promise<string | undefined>>()

function clearCache() {
  for (const promise of imageCache.values()) {
    promise.then(url => {
      if (url) URL.revokeObjectURL(url)
    }).catch(() => {})
  }
  imageCache.clear()
}

function getOrFetchImage(index: number): Promise<string | undefined> {
  if (!store.pickedComic || index < 0 || index >= store.pickedComic.files.length) {
    return Promise.resolve(undefined)
  }
  if (imageCache.has(index)) {
    return imageCache.get(index)!
  }

  const promise = (async () => {
    const comic = store.pickedComic
    if (!comic) return undefined
    const file = comic.files[index]
    const result = await commands.getImageData(
      comic.id,
      comic.isDownloaded ?? null,
      comic.comicDownloadDir ?? null,
      file
    )
    if (result.status === 'ok') {
      const data = new Uint8Array(result.data)
      const blob = new Blob([data])
      return URL.createObjectURL(blob)
    } else {
      console.error(result.error)
      return undefined
    }
  })()

  imageCache.set(index, promise)
  return promise
}

async function loadCurrentImage() {
  if (!store.pickedComic || store.pickedComic.files.length === 0) return

  loading.value = true
  const index = currentIndex.value

  const url = await getOrFetchImage(index)
  if (currentIndex.value !== index) {
    return
  }

  if (store.currentTabName === 'reader') {
    currentImage.value = url
    loading.value = false
  }

  // Preload next images and previous images
  const preloadIndices = [index + 1, index + 2, index + 3, index - 1]
  for (const i of preloadIndices) {
    if (i >= 0 && i < store.pickedComic.files.length) {
      getOrFetchImage(i).catch(() => {})
    }
  }

  // Cleanup old cache entries to free memory
  for (const key of imageCache.keys()) {
    if (key < index - 3 || key > index + 3) {
      const promise = imageCache.get(key)
      if (promise) {
        promise.then(u => {
          if (u) URL.revokeObjectURL(u)
        }).catch(() => {})
      }
      imageCache.delete(key)
    }
  }
}

watch(
  () => store.pickedComic,
  () => {
    clearCache()
    currentImage.value = undefined
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
  clearCache()
  currentImage.value = undefined
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
