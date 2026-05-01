import zhCNTags from './locales/tags/zh-CN.json'
import * as i18n from 'vue-i18n'
import { MessageSchema, SupportedLocales } from './main.ts'

export function useI18n() {
  return i18n.useI18n<{ message: MessageSchema }, SupportedLocales>()
}

export function translateTag(tag: string, ns: string): string {
  const { locale } = useI18n()
  if (locale.value === 'zh-CN') {
    const key = `${ns}:${tag.replace(/ /g, '_')}`
    const translations = zhCNTags as Record<string, string>
    return translations[key] || tag
  }
  return tag
}
