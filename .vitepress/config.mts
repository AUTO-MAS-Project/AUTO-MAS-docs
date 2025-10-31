import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  base: "/",
  title: "AUTO_MAA",
  head: [
    // 添加图标
    ['link', { rel: 'icon', href: '/resource/AUTO_MAA.png' }]
  ],
  description: "AUTO_MAA",
  themeConfig: {
    logo: { src: '/resource/AUTO_MAA.png', width: 24, height: 24, alt: 'AUTO_MAA LOGO' },
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "首页", link: "/" },
      { text: "文档", link: "/docs/user-guide" },
      { text: "下载", link: "/download/auto-maa" },
    ],

    sidebar: {
      '/docs/': [
        {
          text: "AUTO_MAA 文档",
          items: [
            { text: "开始使用", link: "/docs/user-guide" },
            { text: "MAA用户指南", link: "/docs/user-guide-maa" },
            { text: "进阶功能", link: "/docs/advanced-features" },
            { text: "多开指南", link: "/docs/multi-instance" },
            { text: "通用调度", link: "/docs/general-manager" },
            { text: "常见问题", link: "/docs/frequently-asked-questions" },
            { text: "更新日志", link: "/docs/changelog" },
          ],
        },
      ],
      '/download/': [
        {
          text: "软件下载",
          items: [
            { text: "AUTO_MAA", link: "/download/auto-maa" },
          ],
        },
      ],
    },
    search: {
      provider: 'local'
    },
    socialLinks: [
      { icon: "github", link: "https://github.com/DLmaster361/AUTO_MAA" },
    ],
    lastUpdated: {
      text: '最后更新于',
      formatOptions: { dateStyle: 'full', timeStyle: 'full', hourCycle: 'h24' }
    },
    // 修改链接
    editLink: {
      pattern: 'https://github.com/AUTO-MAS-Project/AUTO-MAS-docs/edit/master/:path',
      text: '为此页提供修改建议'
    },
    notFound: {
      title: '找不到页面',
      quote: '页面不见了，也许它去找寻新的冒险了！',
      linkLabel: '返回首页重新探索',
      linkText: '返回首页',
      code: '404'
    },
    docFooter: { prev: '上一篇', next: '下一篇' },
    // 移动端 - 返回顶部
    returnToTopLabel: '返回顶部',

    // 移动端 - menu
    sidebarMenuLabel: '文档',
    // markdown 外部链接图标
    externalLinkIcon: true,
    // 右侧目录（TOC）配置
    outline: {
      level: [2, 4], // 允许 h2 到 h4 级别的标题出现在右侧目录
      label: "目录", // 自定义目录标题
    },
  },
});
