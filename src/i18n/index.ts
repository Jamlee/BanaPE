import { ref, computed } from 'vue'
import en from './en'
import zhCN from './zh-CN'
import ja from './ja'
import ko from './ko'
import ru from './ru'

export type Locale = 'en' | 'zh-CN' | 'ja' | 'ko' | 'ru'

const messages: Record<Locale, typeof en> = {
  en,
  'zh-CN': zhCN,
  ja,
  ko,
  ru,
}

export const localeNames: Record<Locale, string> = {
  en: 'English',
  'zh-CN': '简体中文',
  ja: '日本語',
  ko: '한국어',
  ru: 'Русский',
}

const currentLocale = ref<Locale>('zh-CN')

export function useI18n() {
  const t = computed(() => messages[currentLocale.value])

  function setLocale(loc: Locale) {
    currentLocale.value = loc
    localStorage.setItem('banape-locale', loc)
  }

  function getLocale(): Locale {
    return currentLocale.value
  }

  function initLocale() {
    const saved = localStorage.getItem('banape-locale') as Locale | null
    if (saved && messages[saved]) {
      currentLocale.value = saved
    }
  }

  // 翻译辅助函数
  function translate(key: string, params?: Record<string, string | number>): string {
    const keys = key.split('.')
    let value: any = t.value
    for (const k of keys) {
      value = value?.[k]
    }
    if (typeof value !== 'string') return key
    
    if (params) {
      return Object.entries(params).reduce(
        (str, [k, v]) => str.replace(new RegExp(`\\{${k}\\}`, 'g'), String(v)),
        value
      )
    }
    return value
  }

  // 组件名称/描述翻译
  function compName(id: string): string {
    return t.value.components?.[id]?.name || id
  }

  function compDesc(id: string): string {
    return t.value.components?.[id]?.desc || ''
  }

  // 分类名翻译
  function catName(cat: string): string {
    return t.value.categories?.[cat] || cat
  }

  // 步骤翻译
  function stepName(idx: number): string {
    return t.value.steps[idx]?.name || `Step ${idx + 1}`
  }

  function stepDesc(idx: number): string {
    return t.value.steps[idx]?.desc || ''
  }

  return {
    locale: computed(() => currentLocale.value),
    t,
    setLocale,
    getLocale,
    initLocale,
    translate,
    $t: translate,
    compName,
    compDesc,
    catName,
    stepName,
    stepDesc,
    localeNames,
    messages,
  }
}
