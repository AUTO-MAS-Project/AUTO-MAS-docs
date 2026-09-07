---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "AUTO-MAS"
  text: "游戏脚本多账号管理与自动化软件"
  tagline: "优化脚本多账号体验，提高代理稳定性"
  image:
    src: /icons/AUTO-MAS.ico
    alt: "AUTO-MAS Logo"
  actions:
    - theme: brand
      text: 新手上路
      link: /docs/user-guide
    - theme: alt
      text: 脚本配置
      link: /docs/script-guide/
    - theme: alt
      text: 任务调度
      link: /docs/task-scheduler
    - theme: alt
      text: 进阶功能
      link: /docs/advanced-features/
    - theme: alt
      text: 常见问题
      link: /docs/FAQ

features:
  - title: 多账号统一管理
    details: 多个脚本、多个账号的配置都在一处，不用再开一堆窗口来回切。
  - title: 异常自动重试
    details: 持续监看脚本日志，发现报错或卡死自动重试，无人值守也能跑完。
  - title: 灵活的调度队列
    details: 用调度队列排好执行顺序，支持开机自动运行与定时运行。
  - title: 结果全程留痕
    details: 每次代理的结果和关键日志都留档，哪个号哪一步出错一目了然。
---

## 为什么选择 AUTO-MAS？

如果你有好几个号要代理，你大概经历过这些：一个个手动改脚本配置、跑到一半卡住了没人管、第二天发现某个号漏了但不知道为什么。

**AUTO-MAS** 不替代 MAA、M9A 这些脚本，而是负责调度它们：切换账号配置、按顺序启动、根据日志判断成功或失败、失败自动重试，并记录每次运行结果。

- **稳**：全程监看日志并处理异常，尽量让任务真的跑完，而不是看起来跑完了。
- **省事**：不用手动改配置文件，界面上点几下就行。
- **通吃**：几乎所有自动化脚本都能接，只要它能"启动后自动开跑"并且会写日志。

## 特别声明

免费代码签名由 [SignPath.io](https://signpath.io/) 提供，证书由 [SignPath Foundation](https://signpath.org/) 提供。

开发者 AI API 额度由 <a href="https://www.packyapi.ai/register?aff=zKkA"><img src="https://camo.githubusercontent.com/c6e2cac1447e67d9f6c882b2faf234aed2e4d8a14fdc5bc1071d335cb6b5a32c/68747470733a2f2f7777772e7061636b796170692e61692f6c6f676f2d66756c6c2e737667" alt="PackyCode" style="display:inline-block;width:120px;height:auto;vertical-align:middle;"></a> 赞助。PackyCode 是一家稳定、高效的 API 中转服务商，提供 Claude Code、Codex、Gemini 等多种中转服务，具备自动故障转移、智能路由和无限并发等功能。[免费注册 PackyCode](https://www.packyapi.ai/register?aff=zKkA)。
