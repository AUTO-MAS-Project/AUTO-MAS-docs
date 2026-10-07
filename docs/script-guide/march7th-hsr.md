---
title: 三月七小助手 HSR 脚本部署实录
description: 使用 HSR 脚本类型在 AUTO-MAS 中接入三月七小助手的完整部署实录
date: 2026-09-05
---

# 三月七小助手 · HSR 脚本部署实录

除了上面的 **通用脚本** 方式，也可以用 **HSR 脚本** 类型接入三月七小助手——由 AUTO-MAS 直接管理三月七路径、游戏路径与多账号。本篇以 **AUTO-MAS v5.4.0 + March7thAssistant v2026.8.28** 为例，全文截图均出自该版本部署录屏，红圈 / 红框为该步骤鼠标点击位置。通用脚本方式的接入见 [三月七小助手用户指南](/docs/script-guide/march7th)，两种方式二选一。

### 1. 新建 HSR 脚本

脚本管理 → 新建脚本 → 选择 **HSR 脚本** → 创建并配置。
![新建脚本-选择脚本类型](/docs/img/video-tutorial/02-新建脚本-选择脚本类型.jpg)

### 2. 选择三月七路径与游戏路径

在脚本配置页选择三月七所在目录，再把游戏可执行文件指到 `StarRail.exe`：
![选择三月七路径-文件对话框](/docs/img/video-tutorial/04-选择三月七路径-文件对话框.jpg)
![三月七路径已选择](/docs/img/video-tutorial/05-三月七路径已选择.jpg)
![选择游戏路径-文件对话框](/docs/img/video-tutorial/06-选择游戏路径-文件对话框.jpg)

配置完成后保存：
![HSR脚本配置完成](/docs/img/video-tutorial/07-HSR脚本配置完成.jpg)

### 3. 添加用户（脚本直控）并导入配置

添加用户时选择 **脚本直控** 模式，AUTO-MAS 会一键导入三月七的用户快照：
![添加用户-脚本直控](/docs/img/video-tutorial/08-添加用户-脚本直控.jpg)
![用户快照导入成功](/docs/img/video-tutorial/09-用户快照导入成功.jpg)

### 4. 核对三月七原生设置（可选）

打开三月七主界面核对工具箱、体力、程序、货币战争等设置页：
![M7A 安装目录内容](/docs/img/video-tutorial/10-M7A安装目录内容.jpg)
![M7A 主界面-工具箱](/docs/img/video-tutorial/11-M7A主界面-工具箱.jpg)
![M7A 设置-体力页](/docs/img/video-tutorial/12-M7A设置-体力页.jpg)
![M7A 设置-程序页](/docs/img/video-tutorial/13-M7A设置-程序页.jpg)
![M7A 设置-货币战争页](/docs/img/video-tutorial/14-M7A设置-货币战争页.jpg)

### 5. 调度中心执行自动代理

调度中心选择脚本与用户后启动，游戏窗口会嵌入显示，跑完自动结算：
![调度中心-选择脚本](/docs/img/video-tutorial/15-调度中心-选择脚本.jpg)
![调度中心-任务启动成功](/docs/img/video-tutorial/16-调度中心-任务启动成功.jpg)
![调度中心-停止并重新执行](/docs/img/video-tutorial/17-调度中心-停止并重新执行.jpg)
![调度中心-游戏窗口嵌入](/docs/img/video-tutorial/18-调度中心-游戏窗口嵌入.jpg)

运行过程实拍：
![运行演示-活动试用界面](/docs/img/video-tutorial/19-运行演示-活动试用界面.jpg)
![运行演示-手机菜单](/docs/img/video-tutorial/20-运行演示-手机菜单.jpg)
![运行演示-每日实训活跃度](/docs/img/video-tutorial/29-运行演示-每日实训活跃度.jpg)
![运行演示-位面饰品提取](/docs/img/video-tutorial/30-运行演示-位面饰品提取.jpg)
![运行演示-货币战争卡牌对局](/docs/img/video-tutorial/31-运行演示-货币战争卡牌对局.jpg)
![调度中心-任务完成](/docs/img/video-tutorial/21-调度中心-任务完成.jpg)

### 6. 多用户、重命名与重新导入配置

![添加第二个用户](/docs/img/video-tutorial/22-添加第二个用户.jpg)
![脚本列表-两个HSR脚本](/docs/img/video-tutorial/23-脚本列表-两个HSR脚本.jpg)
![脚本重命名](/docs/img/video-tutorial/24-脚本重命名.jpg)
![重命名后-用户编辑页](/docs/img/video-tutorial/25-重命名后-用户编辑页.jpg)
![重新导入M7A原生配置](/docs/img/video-tutorial/26-重新导入M7A原生配置.jpg)

### 7. 历史记录与日志排查

![历史记录页](/docs/img/video-tutorial/27-历史记录页.jpg)
![详细日志查看器](/docs/img/video-tutorial/28-详细日志查看器.jpg)

::: tip 两种方式怎么选
- **通用脚本**：通用性强，靠三月七自己的配置文件驱动（上文方式）；
- **HSR 脚本**：AUTO-MAS 托管更深，游戏路径 / 多账号 / 调度集成更紧密（本节方式）。
:::

*整理自 2026-09-05 部署演示录屏。界面文字以 AUTO-MAS v5.4.0 / March7thAssistant v2026.8.28 实际显示为准。*
