# VitePress 中集成 Matomo 跟踪

本项目已经集成了 Matomo 网站分析跟踪功能。以下是配置和使用说明。

## 快速配置

### 1. 配置 Matomo 设置

编辑 `.vitepress/utils/matomo.ts` 文件，修改以下配置：

```typescript
export function getMatomoConfig(): MatomoConfig {
  return {
    url: 'https://your-matomo-instance.com/', // 替换为您的 Matomo 实例 URL
    siteId: '1', // 替换为您的网站 ID
    enabled: true // 设为 true 启用跟踪
  }
}
```

### 2. 获取 Matomo 配置信息

您需要从您的 Matomo 仪表板获取以下信息：

- **Matomo URL**: 您的 Matomo 实例地址（如：`https://analytics.yoursite.com/`）
- **Site ID**: 在 Matomo 中创建网站后获得的站点 ID

### 3. 启用跟踪

将 `enabled` 设为 `true` 即可启用 Matomo 跟踪。

## 高级配置

### 环境变量方式（可选）

如果您希望使用环境变量来管理配置，可以：

1. 复制 `.env.example` 为 `.env`
2. 修改其中的配置值
3. 在 `matomo.ts` 中使用环境变量

### 自定义跟踪事件

您可以在任何 Vue 组件中添加自定义跟踪：

```typescript
// 在组件中添加自定义事件跟踪
const trackCustomEvent = (category: string, action: string, name?: string) => {
  if (typeof window !== 'undefined' && (window as any)._paq) {
    (window as any)._paq.push(['trackEvent', category, action, name])
  }
}

// 使用示例
trackCustomEvent('Downloads', 'Click', 'AUTO_MAA')
```

### 跟踪下载链接

```typescript
// 跟踪文件下载
const trackDownload = (filename: string) => {
  if (typeof window !== 'undefined' && (window as any)._paq) {
    (window as any)._paq.push(['trackLink', filename, 'download'])
  }
}
```

## 功能特性

当前 Matomo 集成包含以下功能：

- ✅ 页面浏览跟踪
- ✅ 链接点击跟踪
- ✅ 心跳计时器（访问时长统计）
- ✅ 可见内容展示跟踪
- ✅ 错误处理和日志记录
- ✅ 开发环境可选启用/禁用

## 调试

### 检查 Matomo 是否正常工作

1. 在浏览器开发者工具中查看控制台日志
2. 查看网络请求，确认有发送到 Matomo 的请求
3. 在 Matomo 仪表板中查看实时访问数据

### 常见问题

**Q: Matomo 脚本加载失败**
A: 检查 Matomo URL 是否正确，确保可以正常访问

**Q: 数据不显示在 Matomo 中**
A: 确认 Site ID 正确，并且 Matomo 中已正确配置网站

**Q: 本地开发时不想启用跟踪**
A: 将 `enabled` 设为 `false`

## 文件结构

```
.vitepress/
├── components/
│   └── Matomo.vue           # Matomo 跟踪组件
├── utils/
│   └── matomo.ts           # Matomo 配置管理
├── theme/
│   └── index.ts            # 主题配置（已集成 Matomo）
└── env.d.ts               # TypeScript 类型声明
```

## 性能考虑

- Matomo 脚本异步加载，不会阻塞页面渲染
- 仅在启用时才加载 Matomo 脚本
- 包含错误处理，避免脚本错误影响网站功能
