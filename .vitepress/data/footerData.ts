import type { FooterData } from "@theojs/lumen";

export const Footer_Data: FooterData = {
  beian: { icp: "浙ICP备2025193608号-1", police: "浙公网安备33068102001344号", showIcon: true },
  author: { name: "AUTO-MAS Team", link: "https://github.com/AUTO-MAS-Project" },
  group: [
    {
      title: "本体链接",
      icon: "bx:link", // `iconify`图标
      color: "rgba(255, 87, 51, 1)",
      links: [
        {
          name: "AUTO-MAS本体GitHub",
          link: "https://github.com/AUTO-MAS-Project/AUTO-MAS",
          icon: "octicon:mark-github-16",
        },
      ],
    },
    {
      title: "文档站链接",
      icon: "bx:link",
      color: "rgba(255, 87, 51, 1)",
      links: [
        {
          name: "AUTO-MAS文档站GitHub",
          icon: "octicon:mark-github-16",
          link: "https://github.com/AUTO-MAS-Project/AUTO-MAS-docs",
        },
      ],
    },
  ],
};
