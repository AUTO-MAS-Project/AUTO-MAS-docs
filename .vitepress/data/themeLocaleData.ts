export type LocaleKey = "root" | "en";

export const defaultLocale: LocaleKey = "root";

export const langToLocale = {
  "zh-CN": "root",
  "en-US": "en",
} satisfies Record<string, LocaleKey>;

export const themeLocaleData = {
  root: {
    shareButton: {
      buttonText: "分享此页面",
      copiedText: "链接已复制!",
    },
  },
  en: {
    shareButton: {
      buttonText: "Share this page",
      copiedText: "Link copied!",
    },
  },
} satisfies Record<LocaleKey, {
  shareButton: {
    buttonText: string;
    copiedText: string;
  };
}>;

export function getLocaleKey(lang: string): LocaleKey {
  return langToLocale[lang as keyof typeof langToLocale] ?? defaultLocale;
}
