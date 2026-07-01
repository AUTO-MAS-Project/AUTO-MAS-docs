import type { DefaultTheme } from "vitepress";

export const sharedThemeConfig: DefaultTheme.Config = {
    logo: { src: "/icons/AUTO-MAS.ico", width: 24, height: 24, alt: "AUTO-MAS LOGO" },
    search: {
        provider: "local",
        options: {
            locales: {
                root: {
                    translations: {
                        button: {
                            buttonText: "搜索",
                            buttonAriaLabel: "搜索文档",
                        },
                        modal: {
                            displayDetails: "显示详情",
                            resetButtonTitle: "清除查询",
                            backButtonTitle: "关闭搜索",
                            noResultsText: "没有找到结果",
                            footer: {
                                selectText: "选择",
                                navigateText: "切换",
                                closeText: "关闭",
                            },
                        },
                    },
                },
                en: {
                    translations: {
                        button: {
                            buttonText: "Search",
                            buttonAriaLabel: "Search documentation",
                        },
                        modal: {
                            displayDetails: "Display detailed list",
                            resetButtonTitle: "Reset search",
                            backButtonTitle: "Close search",
                            noResultsText: "No results found",
                            footer: {
                                selectText: "to select",
                                navigateText: "to navigate",
                                closeText: "to close",
                            },
                        },
                    },
                },
            },
        },
    },
    socialLinks: [
        { icon: "github", link: "https://github.com/AUTO-MAS-Project/AUTO-MAS" },
    ],
    externalLinkIcon: true,
};
