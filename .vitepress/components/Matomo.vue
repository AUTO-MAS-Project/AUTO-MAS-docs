<template>
  <!-- Matomo 组件不需要可见内容 -->
</template>

<script setup lang="ts">
import { onMounted } from 'vue'

interface MatomoConfig {
  url: string
  siteId: string | number
  enabled?: boolean
}

const props = defineProps<{
  config: MatomoConfig
}>()

onMounted(() => {
  // 检查是否启用 Matomo
  if (!props.config.enabled) {
    console.log('Matomo tracking is disabled')
    return
  }

  // 检查必要的配置
  if (!props.config.url || !props.config.siteId) {
    console.warn('Matomo configuration is incomplete')
    return
  }

  // 初始化 Matomo
  const _paq = (window as any)._paq = (window as any)._paq || []
  
  // 基础跟踪设置
  _paq.push(['trackPageView'])
  _paq.push(['enableLinkTracking'])
  
  // 可选：启用心跳计时器（用于准确的访问时长统计）
  _paq.push(['enableHeartBeatTimer'])
  
  // 可选：跟踪无 JavaScript 用户
  _paq.push(['trackVisibleContentImpressions'])
  
  // 设置跟踪器
  const matomoUrl = props.config.url.endsWith('/') ? props.config.url : props.config.url + '/'
  _paq.push(['setTrackerUrl', `${matomoUrl}matomo.php`])
  _paq.push(['setSiteId', props.config.siteId])
  
  // 动态加载 Matomo 脚本
  const script = document.createElement('script')
  script.async = true
  script.src = `${matomoUrl}matomo.js`
  script.onload = () => {
    console.log('Matomo tracking initialized')
  }
  script.onerror = () => {
    console.error('Failed to load Matomo script')
  }
  document.head.appendChild(script)
})
</script>
