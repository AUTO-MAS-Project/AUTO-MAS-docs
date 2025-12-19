import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
    base: "/",
    title: "AUTO-MAS",
    head: [
        // 添加图标
        ['link', { rel: 'icon', href: '/icons/AUTO-MAS.ico' }]
    ],
    description: "AUTO-MAS",
    themeConfig: {
        logo: { src: '/icons/AUTO-MAS.ico', width: 24, height: 24, alt: 'AUTO-MAS LOGO' },
        // https://vitepress.dev/reference/default-theme-config
        nav: [
            { text: "首页", link: "/" },
            {
                text: "文档",
                items: [
                    { text: "V5 文档", link: "/docsV5/user-guide" },
                    { text: "v4（旧）文档", link: "/docs/user-guide" },
                ],
            },
            { text: "开发", link: "/developer/getting-start.md" },
            { text: "下载", link: "/download/auto-mas" },
        ],

        sidebar: {
            //文档本体
            '/docsV5/': [
                {
                    text: "AUTO-MAS 文档 ",
                    items: [
                        { text: "开始使用", link: "/docsV5/user-guide" },
                        {
                            text: "脚本配置",
                            link: "docsV5/script-guide/__index",
                            items: [
                                { text: "MAA", link: "docsV5/script-guide/maa" },
                                { text: "通用脚本", link: "docsV5/script-guide/general" },
                                { text: "三月七", link: "docsV5/script-guide/march7th" },
                            ]
                        },
                        { text: "任务调度", link: "/docsV5/task-scheduler" },
                        {
                            text: "进阶功能",
                            link: "/docsV5/advanced-features/__index",
                            items: [
                                { text: "推送通知", link: "/docsV5/advanced-features/notification" },
                                { text: "MCP 服务", link: "/docsV5/advanced-features/mcp" },
                            ]
                        },
                        { text: "常见问题", link: "/docsV5/FAQ" },
                    ],
                },
            ],

            //旧文档
            '/docs/': [
                {
                    text: "AUTO_MAA 文档",
                    items: [
                        { text: "开始使用", link: "/docs/user-guide" },
                        { text: "进阶功能", link: "/docs/advanced-features" },
                        { text: "多开指南", link: "/docs/multi-instance" },
                        { text: "通用调度", link: "/docs/general-manager" },
                        { text: "常见问题", link: "/docs/frequently-asked-questions" },
                        { text: "更新日志", link: "/docs/changelog" },
                    ],
                },
            ],


            'developer/': [
                {
                    text: "开发者指南",
                    items: [
                        { text: "开发起步", link: "/developer/getting-start.md" },
                        { text: "开发规范", link: "/developer/development-specifications.md" },
                        { text: "构筑与发布", link: "/developer/build-and-publish.md" },
                    ],
                },
                {
                    text: "开发文档", items: [
                        { text: "API 开发", link: "/developer/API.md" },
                        { text: "配置管理", link: "/developer/config.md" },
                        { text: "专项适配", link: "/developer/script_task.md" },
                    ]
                },
            ],
            //下载站
            '/download/': [
                {
                    text: "软件下载",
                    items: [
                        { text: "AUTO-MAS", link: "/download/auto-mas" },
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
