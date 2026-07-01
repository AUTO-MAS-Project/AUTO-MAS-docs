import type { AsideItem } from "@theojs/lumen";

export const asideData = {
  root: [
    {
      link: "https://qm.qq.com/q/hdV065dcbu",
      image: "/icons/tencentqq.svg",
      name: "QQ 官方群",
      info2: "群号：957750551",
    },
    {
      link: "https://t.me/AUTO_MAS_top",
      image: "/icons/telegram.svg",
      name: "Telegram 群",
      info2: "@AUTO_MAS_top",
    },
  ],
  en: [
    {
      link: "https://qm.qq.com/q/hdV065dcbu",
      image: "/icons/tencentqq.svg",
      name: "Official QQ Group",
      info2: "Group ID: 957750551",
    },
    {
      link: "https://t.me/AUTO_MAS_top",
      image: "/icons/telegram.svg",
      name: "Telegram Group",
      info2: "@AUTO_MAS_top",
    },
  ],
} satisfies Record<"root" | "en", AsideItem[]>;
