import type { FooterData } from "@theojs/lumen";

export const Footer_Data: FooterData = {
  beian: { icp: "浙ICP备2025193608号-1", police: "浙公网安备33068102001344号", showIcon: true },
  author: { name: "AUTO-MAS Team", link: "https://github.com/AUTO-MAS-Project" },
  group: [
    {
      title: "软件仓库",
      icon: "bx:link", // `iconify`图标
      color: "rgba(255, 87, 51, 1)",
      links: [
        {
          name: "AUTO-MAS 仓库",
          link: "https://github.com/AUTO-MAS-Project/AUTO-MAS",
          icon: "octicon:mark-github-16",
        },
      ],
    },
    {
      title: "文档站仓库",
      icon: "bx:link",
      color: "rgba(255, 87, 51, 1)",
      links: [
        {
          name: "AUTO-MAS 文档站仓库",
          icon: "octicon:mark-github-16",
          link: "https://github.com/AUTO-MAS-Project/AUTO-MAS-docs",
        },
      ],
    },
    {
      title: "状态监控",
      icon: "mdi:monitor-dashboard",
      color: "rgba(52, 199, 89, 1)",
      links: [
        {
          name: "AUTO-MAS 状态监控站",
          icon: "mdi:chart-line",
          link: "https://status.auto-mas.top/status/auto-mas",
        },
      ],
    },
  ],
};
