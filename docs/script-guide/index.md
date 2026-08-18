# 脚本管理

AUTO-MAS 支持多种游戏脚本的管理与调度。本节将介绍如何在 AUTO-MAS 中使用各类脚本。

## 指南目录

### [MAA](/docs/script-guide/maa)

明日方舟 - MaaAssistantArknights

- 支持所有剿灭与日常代理
- 官服与 B服 支持管理多账号
- 支持使用计划表定制关卡配置

### [MaaEnd](/docs/script-guide/maaend)

明日方舟：终末地 - MaaEnd 终末地小助手

- 支持协议空间快速调整与自动签到
- 支持 PC 与模拟器控制（推荐 PC）
- 支持使用计划表定制关卡配置（内测中）

---

### [M9A](/docs/script-guide/m9a)

1999（亿韭韭韭） - M9A

- 支持日常代理、活动刷取、自动深眠等
- 支持 MuMu 与 雷电模拟器
- 仅支持 MFAAvalonia 界面

---

### [OK-WW](/docs/script-guide/okww)

鸣潮（Wuthering Waves） - OK-WW

- ok-script 家族中的鸣潮子项目，与异环的 OkNte 分开配置
- 当前专项入口接管日常与多账号日常（`-t 1`、`-t 7`）
- 支持 MAS 全自动游戏生命周期管理（启动/关闭）
- 配置来源分为脚本、用户、直控；可选用快速配置覆盖高频任务字段
- 仅支持鸣潮官方启动器和官方资源，不使用 WeGame

---

### [HSR](/docs/script-guide/hsr)

崩坏：星穹铁道 - HSR 专项（M7A / SRA 双引擎）

- 同时支持三月七小助手（M7A）与 StarRailAssistant（SRA）双引擎
- 覆盖日常清体力、奖励领取、差分宇宙、货币战争等任务
- 体力 / 奖励 / 差分 / 货币 四个模块可独立选择 M7A 或 SRA 执行
- 失败任务自动补跑，外部脚本配置零污染

---

### [通用调度](/docs/script-guide/general)

适用于所有支持启动时运行任务且能够打印日志的脚本

- 支持大多数主流脚本（三月七、SRC、zzzOD、M9A 等）
- 直接获取现成的配置模板，开箱即用
- 支持灵活自定义脚本管理方案

---

### [三月七小助手](/docs/script-guide/march7th)

崩坏：星穹铁道 - March7thAssistant

- 🚀 开发中，敬请期待
- 🔐 可配合自动登录脚本使用

---

## 阅读建议

- **新手推荐**：从 [MAA 用户指南](/docs/script-guide/maa) 开始（如果玩明日方舟）
- **鸣潮玩家**：查看 [OK-WW 配置方法](/docs/script-guide/okww) 快速上手
- **1999 玩家**：查看 [M9A 配置方法](/docs/script-guide/m9a) 快速上手
- **星穹铁道玩家**：查看 [HSR 配置方法](/docs/script-guide/hsr) 快速上手
- **其他游戏**：查看 [通用调度](/docs/script-guide/general) 并使用现成模板
- **高级用户**：深入了解[通用调度](/docs/script-guide/general)的配置管理逻辑，自定义您的调度方案
