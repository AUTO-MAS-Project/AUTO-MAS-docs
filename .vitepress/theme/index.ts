// https://vitepress.dev/guide/custom-theme
import { h } from "vue";
import { useData, type Theme } from "vitepress";
import DefaultTheme from "vitepress/theme";
import "./style.css";
// theme/index.ts
import "@theojs/lumen/theme";
import { HomeFooter } from "@theojs/lumen";

import { footerData } from "../data/footerData";
import { Announcement } from "@theojs/lumen";
import { ShareButton } from "@theojs/lumen";
import { googleAnalytics } from "@theojs/lumen";
import { DocBox, DocBoxCube, DocLinks, DocPill } from "@theojs/lumen";
import { DocAsideLogo } from "@theojs/lumen";
import { asideData } from "../data/AsideData";
import { getLocaleKey, themeLocaleData } from "../data/themeLocaleData";
import Matomo from "../components/Matomo.vue";
import { getMatomoConfig } from "../utils/matomo";
export default {
  extends: DefaultTheme,
  Layout: () => {
    const { lang } = useData();
    const locale = getLocaleKey(lang.value);
    const localeText = themeLocaleData[locale];

    return h(DefaultTheme.Layout, null, {
      // https://vitepress.dev/guide/extending-default-theme#layout-slots
      "layout-bottom": () => [
        h(HomeFooter, { Footer_Data: footerData[locale] }),
        h(Matomo, { config: getMatomoConfig() })
      ],
      "home-hero-info-before": () => h(Announcement),
      "aside-outline-before": () => h(ShareButton, localeText.shareButton),
      "aside-ads-before": () => h(DocAsideLogo, { Aside_Data: asideData[locale] }),
    });
  },
  enhanceApp({ app, router, siteData }) {
    // 注册 Google Analytics
    googleAnalytics({ id: "G-SE0CWGQ7VL" });
    // 注册自定义组件
    app.component("Box", DocBox);
    app.component("Pill", DocPill);
    app.component("Links", DocLinks);
    app.component("BoxCube", DocBoxCube);
  },
} satisfies Theme;
