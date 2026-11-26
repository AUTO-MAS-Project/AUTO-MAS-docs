# 托管管理

找到你玩的那个游戏，点进去照着配就行。

玩的游戏不在下面这个列表里？看 [通用调度](/docs/script-guide/general)——只要那个脚本能"启动后自动开跑"并且会写日志，AUTO-MAS 就能管它。

## 桌面分身支持情况 {#desktop-session}

桌面分身是 Windows x64 桌面版的实验功能。下表按开发版本 `be8ef6d5c` 核对。模拟器游戏及其托管请在主会话运行。

队列里的每个托管都须符合下表的条件，且队列的 **完成后操作** 须设为 **无操作**。打开方式和报错处理见[桌面分身使用说明](/docs/advanced-features/desktop-session)。

| 托管类型 | 桌面分身 | 条件与说明 |
| --- | --- | --- |
| [MAA](/docs/script-guide/maa#桌面分身) | 不支持 | 请在主会话运行。 |
| [MaaEnd](/docs/script-guide/maaend#桌面分身) | 有条件支持 | 使用 PC（Win32）控制器。模拟器任务在主会话运行。 |
| SRC | 不支持 | 请在主会话运行。 |
| [M9A](/docs/script-guide/m9a#桌面分身) | 按 MFW 控制器判断 | 常用的 ADB 配置请在主会话运行。 |
| [MFW 项目](/docs/script-guide/maafw#桌面分身) | 有条件支持 | 使用 Win32 控制器。ADB 等其他控制器在主会话运行。 |
| [通用脚本](/docs/script-guide/general#桌面分身) | 有条件支持 | 只用于 PC 任务。由 MAS 启动游戏时，请使用可执行文件或 URL。使用模拟器的任务请留在主会话，即使由脚本自行启动模拟器。 |
| [ok-ww](/docs/script-guide/okww#桌面分身) | 支持 | 适用于鸣潮 PC 端。 |
| ok-nte | 支持 | 适用于异环 PC 端。 |
| [HSR](/docs/script-guide/hsr#桌面分身) | 支持 | PC 客户端可使用三月七小助手或 SRA。 |
| [BetterGI](/docs/script-guide/bettergi#桌面分身) | 有条件支持 | 关闭「接管更新」。若 BetterGI 配置为以管理员权限运行，须以管理员身份启动 AUTO-MAS。 |
| 绝区零一条龙（ZZZ-OD） | 有条件支持 | 须以管理员身份启动 AUTO-MAS。分身中会跳过脚本在任务完成后的关机操作。 |
| BAAH | 不支持 | 请在主会话运行。 |
| 奇想盒（Whimbox） | 有条件支持 | 以管理员身份启动 AUTO-MAS，宿主须提供分身插件接口，奇想盒适配器须为 0.1.2 或更高版本。 |
| MSS | 不支持 | 请在主会话运行。 |

已在 Windows 上用两个 SDK 同为 5.9.0b1 的宿主，通过插件管理器将同一个适配器从 0.1.1 升级到 0.1.2。提供分身接口的宿主支持选择执行会话；同号旧宿主使用主会话，并为后续升级保留已保存的选择。

Windows 11 实测组合为管理员运行的 AUTO-MAS 5.9.0-beta.1（提交 `46d8a148f`）、适配器 0.1.2、奇想盒 App 2.0.6 和后端 3.1.5。分身单托管、整队列以及重启后按已保存会话单独启动，均将奇想盒后端放入子会话。停止任务及断开分身后，主桌面上的同名进程继续运行。本次验证未安装游戏。

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
- 作为 MFW 脚本接入：选中目录自动识别，自动补启动 / 关闭游戏与官服切号

---

### [MFW 项目](/docs/script-guide/maafw)

任何带 `interface.json` 的 MaaFramework 项目

- 不启动项目自己的界面，直接按 interface 跑任务队列
- 在 AUTO-MAS 自己的副本上运行，原目录一个字节不动、导入后可删；同一项目可创建多个托管同时运行
- 项目更新走 GitHub 或 Mirror 酱，可配成运行前 / 运行后自动更新

---

### [OK-WW](/docs/script-guide/okww)

鸣潮（Wuthering Waves） - OK-WW

- 目前支持日常任务与多账号日常
- AUTO-MAS 可以帮你自动开关游戏
- 复杂设置仍在 OK-WW 自己界面里做，AUTO-MAS 只接管高频项
- 只支持官方启动器，不支持 WeGame

---

### [HSR](/docs/script-guide/hsr)

崩坏：星穹铁道 - 三月七小助手（M7A）+ StarRailAssistant（SRA）

- 两款脚本可以混着用，哪个任务交给谁由你定
- 覆盖清体力、领奖励、差分宇宙、货币战争
- 失败自动重跑，不会弄坏你原本的脚本配置

---

### [BetterGI](/docs/script-guide/bettergi)

原神 - BetterGI

- 原生 GUI 配置，账号由 BetterGI 原生管理，复杂设置在 BetterGI 界面完成
- 为每个账号独立管理「一条龙」内置配置组（8 个可开关）与自定义配置组
- 支持填写领取奖励队伍、通用战斗队伍与自动战斗策略，自动读取 `User/AutoFight` 下的战斗脚本
- 切换账号、通知、额外脚本均可独立配置

---

### [通用调度](/docs/script-guide/general)

适用于所有支持启动时运行任务且能够打印日志的脚本

- 支持大多数主流脚本（三月七、SRC、zzzOD、M9A 等）
- 直接获取现成的配置模板，开箱即用
- 支持灵活自定义托管管理方案

---

### [三月七小助手](/docs/script-guide/march7th)

崩坏：星穹铁道 - March7thAssistant

- 🚀 开发中，敬请期待
- 🔐 可配合自动登录脚本使用

---

## 阅读建议

- **新手推荐**：从 [MAA 账号指南](/docs/script-guide/maa) 开始（如果玩明日方舟）
- **鸣潮玩家**：查看 [OK-WW 配置方法](/docs/script-guide/okww) 快速上手
- **1999 玩家**：查看 [M9A 配置方法](/docs/script-guide/m9a) 快速上手
- **其他 MaaFramework 项目**：查看 [MFW 项目配置方法](/docs/script-guide/maafw)
- **星穹铁道玩家**：查看 [HSR 配置方法](/docs/script-guide/hsr) 快速上手
- **原神玩家**：查看 [BetterGI 配置方法](/docs/script-guide/bettergi) 快速上手
- **其他游戏**：查看 [通用调度](/docs/script-guide/general) 并使用现成模板
- **高级用户**：深入了解[通用调度](/docs/script-guide/general)的配置管理逻辑，自定义您的调度方案
