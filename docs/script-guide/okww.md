---
title: OK-WW 配置方法
description: 鸣潮 - OK-WW 专项适配配置指南
date: 2026-05-31
---

# OK-WW 配置方法

## 什么是 OK-WW？

OK-WW（ok-ww）是一个「鸣潮（Wuthering Waves）」第三方自动化工具。

<Box :items="[
{ name: 'OK-WW GitHub', link: 'https://github.com/ok-oldking/ok-wuthering-waves', image: { light: '/icons/github.svg', dark: '/icons/github-dark.svg', }, },]"/>

## 专项适配说明

AUTO-MAS 接管 ok-ww 的配置下发、任务启动（`-t N -e`）、日志监看与游戏生命周期管理，提供比通用脚本更稳定的无人值守体验。

| 特性 | 通用脚本 | OK-WW 专项适配 |
|---|---|---|
| 任务参数 | 用户自行填写 | 自动拼合 |
| 日志判定 | 完全依赖用户配置 | 内置关键词 + 用户配置补充 |
| 配置模式 | 单套 | 简洁（共用）/ 详细（独立） |

## 快速开始

1. 下载 [OK-WW](https://github.com/ok-oldking/ok-wuthering-waves)，解压至纯英文路径。AUTO-MAS ≥ v5.3.0-beta.2。
2. **脚本管理** → **新建脚本** → 选择 **ok-ww 脚本**。
   ![新建okww脚本](/docs/img/script-guide/okww/step1-new-script.png)
3. 设置 **ok-ww 路径**（包含 `ok-ww.exe` 的根目录），MAS 自动匹配子路径。
   ![设置路径](/docs/img/script-guide/okww/step2-set-path.png)
4. 点击「**配置 ok-ww**」，在 ok-ww 本体中完成配置，回到 MAS 保存。
   ![配置okww](/docs/img/script-guide/okww/step3-config-okww.png)
5. **添加用户**，填写账号、密码，选择启动任务。
   ![添加用户](/docs/img/script-guide/okww/step4-add-user.png)
6. 将脚本加入调度队列即可自动运行。

> **简洁模式 vs 详细模式**：简洁模式下所有用户共用一套 ok-ww 配置；详细模式下每个用户独立配置，在用户编辑页切换模式后点击「**ok-ww 配置**」即可。

### 支持的任务

| 序号 | 任务 | 说明 |
|---|---|---|
| 1 | DailyTask | 日常任务 |
| 2 | MultiAccountDailyTask | 多账号日常 |
| 3 | FarmEchoTask | 刷取声骸 |
| 4 | AutoRogueTask | 肉鸽 |
| 5 | ForgeryTask | 凝素领域 |
| 6 | NightmareNestTask | 梦魇巢穴 |
| 7 | SimulationTask | 模拟领域 |
| 8 | TacetTask | 无音区 |

## 常见问题

### 启动任务后莫名其妙中断，状态显示"错误"（ok-ww 3.4.10）

**原因**：ok-ww 3.4.10 更新日志中的「修复洛瑟菈**错误**/失败战斗」命中了 MAS 的异常关键词「错误」，导致 MAS 误判任务失败。

**解决**：此问题已在 MAS v5.3.1 正式版中修复。如升级后仍有此问题，请尝试重建脚本配置。
