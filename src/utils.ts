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
    // 如果字典还没加载，触发异步加载
    if (!zhCNTags.value) {
      if (!isLoadingZhCNTags) {
        isLoadingZhCNTags = true
        import('./locales/tags/zh-CN.json')
          .then((module) => {
            zhCNTags.value = module.default
          })
          .catch((err) => {
            console.error('Failed to load zh-CN tags', err)
            isLoadingZhCNTags = false
          })
      }
      // 加载完成前，暂时返回原始 tag
      return tag
    }

    // 字典已加载完毕，进行翻译
    const key = `${ns}:${tag.replace(/ /g, '_')}`
    return zhCNTags.value[key] || tag
  }
  
  return tag
}