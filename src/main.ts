import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import 'virtual:uno.css'
import { createI18n } from 'vue-i18n'
import { locales } from './locales'

export type SupportedLocales = keyof typeof locales
export type MessageSchema = (typeof locales)['zh-CN']

/** 创建并配置 Vue I18n 实例，用于国际化 */
export const i18n = createI18n<[MessageSchema], SupportedLocales>({
  // 如果浏览器的语言在支持的语言列表中，则使用浏览器语言，否则回退到英语 ('en-US')
  locale: navigator.language in locales ? navigator.language : 'en-US',
  // 缺省语言设为英语
  fallbackLocale: 'en-US',
  // 允许在全局组件中直接使用 $t 等 i18n 属性和方法
  globalInjection: true,
  // 禁用 legacy 模式以支持 Vue 3 的 Composition API
  legacy: false,
  // 导入的语言包数据
  messages: locales,
})

// 创建 Pinia 实例，用于 Vue 的状态管理
const pinia = createPinia()
// 创建 Vue 应用实例
const app = createApp(App)

// 使用 Pinia、I18n 并将应用挂载到 '#app' 的 DOM 节点上
app.use(pinia).use(i18n).mount('#app')
