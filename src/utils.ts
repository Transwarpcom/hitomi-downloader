import * as i18n from 'vue-i18n'
import { shallowRef } from 'vue'
import { MessageSchema, SupportedLocales } from './main.ts'

export function useI18n() {
  return i18n.useI18n<{ message: MessageSchema }, SupportedLocales>()
}

// Lazy load the large zh-CN tag dictionary (1.7MB+) to reduce initial bundle size
const zhCNTags = shallowRef<Record<string, string> | null>(null)
let isLoadingZhCNTags = false

export function translateTag(tag: string, ns: string, currentLocale: string): string {
  if (currentLocale === 'zh-CN') {
    if (zhCNTags.value !== null) {
      const key = `${ns}:${tag.replace(/ /g, '_')}`
      return zhCNTags.value[key] || tag
    } else if (!isLoadingZhCNTags) {
      isLoadingZhCNTags = true
      import('./locales/tags/zh-CN.json').then((module) => {
        zhCNTags.value = module.default
      }).catch((err) => {
        console.error('Failed to load zh-CN tags', err)
      }).finally(() => {
        isLoadingZhCNTags = false
      })
    }
  }
  return tag
}
