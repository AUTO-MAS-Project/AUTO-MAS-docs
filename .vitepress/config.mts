import { defineConfig, type DefaultTheme } from "vitepress";
import { enThemeConfig } from "./config/en";
import { sharedThemeConfig } from "./config/shared";
import { zhThemeConfig } from "./config/zh";

function developerSidebar(locale: "zh" | "en"): DefaultTheme.SidebarItem[] {
    const prefix = locale === "en" ? "/en" : "";
    const text = (zh: string, en: string) => locale === "en" ? en : zh;
    const page = (zh: string, en: string, path: string): DefaultTheme.SidebarItem => ({
        text: text(zh, en),
        link: prefix + path,
    });

    const pluginRuntime = page("宿主与运行机制", "Host and Runtime", "/developer/plugins/fundamentals/runtime");
    const pluginDevelop = page("开发与调试", "Development and Debugging", "/developer/plugins/fundamentals/develop");

    return [
        page("导读", "Developer Guide", "/developer/"),
        {
            text: text("开发起步", "Getting Started"),
            items: [
                page("环境配置", "Environment Setup", "/developer/getting-start/environment"),
                page("开发规范", "Development Standards", "/developer/getting-start/development-specifications"),
                page("测试与验证", "Testing and Verification", "/developer/getting-start/testing"),
                page("构建与发布", "Build and Release", "/developer/getting-start/build-and-publish"),
            ],
        },
        {
            text: text("主程序开发", "Core Application Development"),
            items: [
                page("项目结构", "Project Structure", "/developer/core/project-structure"),
                page("术语解释", "Terminology", "/developer/core/terminology"),
                page("配置框架", "Configuration Framework", "/developer/core/config"),
                page("前端开发", "Frontend Development", "/developer/core/api"),
                page("调度逻辑", "Task Scheduling", "/developer/core/scheduling"),
                page("通知系统", "Notification System", "/developer/core/notification"),
            ],
        },
        {
            text: text("插件开发", "Plugin Development"),
            items: [
                {
                    ...page("插件基础", "Plugin Fundamentals", "/developer/plugins/fundamentals/"),
                    collapsed: true,
                    items: [
                        ...(locale === "zh" ? [pluginDevelop, pluginRuntime] : [pluginRuntime, pluginDevelop]),
                        page("配置声明", "Configuration Schema", "/developer/plugins/fundamentals/schema"),
                        page("配置文件与数据", "Configuration Files and Data", "/developer/plugins/fundamentals/config-files"),
                        page("事件系统", "Event System", "/developer/plugins/fundamentals/event"),
                        page("服务系统", "Service System", "/developer/plugins/fundamentals/service"),
                        page("前后端对接", "Frontend and Backend Integration", "/developer/plugins/fundamentals/frontend"),
                        page("打包与发布", "Packaging and Publishing", "/developer/plugins/fundamentals/publish"),
                    ],
                },
                page("游戏适配插件", "Game Adapter Plugins", "/developer/plugins/game-adapters"),
                {
                    ...page("脚本专项适配", "Script Adapters", "/developer/plugins/script-adapters/"),
                    collapsed: true,
                    items: [
                        page("独立脚本插件", "Standalone Script Plugins", "/developer/plugins/script-adapters/standalone"),
                        page("主程序内置专项", "Built-in Script Adapters", "/developer/plugins/script-adapters/#built-in-adapters"),
                        page("专项配置语义", "Script Configuration Semantics", "/developer/plugins/script-adapters/config-semantics"),
                        {
                            ...page("MFW 风味专项", "MFW Flavor Adapters", locale === "zh" ? "/developer/plugins/script-adapters/mfw/" : "/developer/plugins/script-adapters/mfw/develop"),
                            collapsed: true,
                            items: [
                                locale === "zh"
                                    ? page("风味专项开发", "Flavor Adapter Development", "/developer/plugins/script-adapters/mfw/develop")
                                    : page("框架设计总览", "Framework Design Overview", "/developer/plugins/script-adapters/mfw/"),
                                page("总体架构", "Architecture", "/developer/plugins/script-adapters/mfw/architecture"),
                                page("导入与载荷视图", "Import and Payload", "/developer/plugins/script-adapters/mfw/import-and-payload"),
                                page("运行", "Execution", "/developer/plugins/script-adapters/mfw/run"),
                                page("运行环境", "Runtime Environment", "/developer/plugins/script-adapters/mfw/runtime-env"),
                                page("项目更新", "Project Updates", "/developer/plugins/script-adapters/mfw/update"),
                                page("风味机制", "Flavor Mechanism", "/developer/plugins/script-adapters/mfw/flavor"),
                                page("前端与交互", "Frontend and Interaction", "/developer/plugins/script-adapters/mfw/frontend"),
                                page("外壳配置导入与问题包", "Shell Import and Issue Reports", "/developer/plugins/script-adapters/mfw/shell-import-and-issue-report"),
                                page("设计取舍", "Design Trade-offs", "/developer/plugins/script-adapters/mfw/tradeoffs"),
                            ],
                        },
                    ],
                },
            ],
        },
    ];
}

// https://vitepress.dev/reference/site-config
export default defineConfig({
    base: "/",
    lang: "zh-CN",
    title: "AUTO-MAS",
    head: [
        // 添加图标
        ["link", { rel: "icon", href: "/icons/AUTO-MAS.ico" }],
    ],
    description: "AUTO-MAS",
    srcExclude: ["README.md", "public/**"],
    vite: {
        ssr: {
            noExternal: ["@theojs/lumen", "dayjs"],
        },
    },
    themeConfig: sharedThemeConfig,
    locales: {
        root: {
            label: "简体中文",
            lang: "zh-CN",
            link: "/",
            themeConfig: {
                ...zhThemeConfig,
                sidebar: {
                    ...zhThemeConfig.sidebar,
                    "/developer/": developerSidebar("zh"),
                },
            },
        },
        en: {
            label: "English",
            lang: "en-US",
            link: "/en/",
            title: "AUTO-MAS",
            description: "AUTO-MAS documentation",
            themeConfig: {
                ...enThemeConfig,
                sidebar: {
                    ...enThemeConfig.sidebar,
                    "/en/developer/": developerSidebar("en"),
                },
            },
        },
    },
});
