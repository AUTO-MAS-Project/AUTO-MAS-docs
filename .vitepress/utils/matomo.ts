interface MatomoConfig {
  url: string
  siteId: string
  enabled: boolean
}

export function getMatomoConfig(): MatomoConfig {
  // 根据提供的 Matomo 配置信息
  return {
    url: 'https://statistics.auto-mas.top/', // 您的 Matomo 实例 URL
    siteId: '2', // 您的网站 ID
    enabled: true // 已启用跟踪
  }
}
