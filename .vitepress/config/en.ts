import type { DefaultTheme } from "vitepress";

export const enThemeConfig = {
    nav: [
        { text: "Home", link: "/en/" },
        { text: "Docs", link: "/en/docs/user-guide" },
        { text: "Development", link: "/en/developer/", activeMatch: "^/en/developer/" },
        { text: "Download", link: "/en/download/auto-mas" },
        { text: "Disclosure", link: "/en/disclosure/" },
    ],
    sidebar: {
        "/en/docs/": [
            {
                text: "User Guide",
                items: [
                    { text: "Getting Started", link: "/en/docs/user-guide" },
                    {
                        text: "Managed Script Configuration",
                        link: "/en/docs/script-guide/",
                        items: [
                            { text: "MAA", link: "/en/docs/script-guide/maa" },
                            { text: "MAAEND", link: "/en/docs/script-guide/maaend" },
                            { text: "M9A", link: "/en/docs/script-guide/m9a" },
                            { text: "MFW Projects", link: "/en/docs/script-guide/maafw" },
                            { text: "OK-WW", link: "/en/docs/script-guide/okww" },
                            { text: "HSR", link: "/en/docs/script-guide/hsr" },
                            { text: "BetterGI", link: "/en/docs/script-guide/bettergi" },
                            { text: "General Scripts", link: "/en/docs/script-guide/general" },
                            { text: "SRA", link: "/en/docs/script-guide/sra" },
                            { text: "March 7th", link: "/en/docs/script-guide/march7th" },
                        ],
                    },
                    { text: "Task Scheduler", link: "/en/docs/task-scheduler" },
                    {
                        text: "Advanced Features",
                        link: "/en/docs/advanced-features/",
                        items: [
                            { text: "Game Check-in", link: "/en/docs/advanced-features/game-sign" },
                            { text: "Emulator Management", link: "/en/docs/advanced-features/emulator" },
                            { text: "Desktop Session", link: "/en/docs/advanced-features/desktop-session" },
                            { text: "Notifications", link: "/en/docs/advanced-features/notification" },
                            { text: "Failure Replays", link: "/en/docs/advanced-features/failure-replay" },
                            { text: "MCP Service", link: "/en/docs/advanced-features/mcp" },
                            { text: "China Mobile 5G SMS", link: "/en/docs/advanced-features/notification#china-mobile-5g-sms" },
                        ],
                    },
                    { text: "FAQ", link: "/en/docs/FAQ" },
                    { text: "How to Ask Questions", link: "/en/docs/howtoask" },
                ],
            },
        ],
        "/en/download/": [
            {
                text: "Downloads",
                items: [
                    { text: "AUTO-MAS", link: "/en/download/auto-mas" },
                    { text: "MaaZFA", link: "/en/download/maa-zfa" },
                ],
            },
        ],
        "/en/disclosure/": [
            {
                text: "Disclosure",
                link: "/en/disclosure/",
                items: [
                    { text: "Visual Assets License Agreement", link: "/en/disclosure/assets-license" },
                    { text: "Cloud Service Agreement", link: "/en/disclosure/cloud-service-agreement" },
                    { text: "Income and Expenses", link: "/en/disclosure/income-and-expenditures" },
                    { text: "Pillar of Shame", link: "/en/disclosure/pillar-of-shame" },
                ],
            },
        ],
    },
    lastUpdated: {
        text: "Last updated",
        formatOptions: { dateStyle: "full", timeStyle: "full", hourCycle: "h24" },
    },
    editLink: {
        pattern: "https://github.com/AUTO-MAS-Project/AUTO-MAS-docs/edit/master/:path",
        text: "Suggest changes to this page",
    },
    notFound: {
        title: "Page not found",
        quote: "The page may have moved, or the link may be outdated.",
        linkLabel: "Return to the home page",
        linkText: "Return home",
        code: "404",
    },
    docFooter: { prev: "Previous page", next: "Next page" },
    returnToTopLabel: "Return to top",
    sidebarMenuLabel: "Docs",
    outline: {
        level: [2, 4],
        label: "On this page",
    },
} satisfies DefaultTheme.Config;
