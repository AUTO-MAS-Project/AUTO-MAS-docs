---
title: 开始使用
description: AUTO_MAS 的使用指南
date: 2025-02-10
---

# 开始使用

## 前置信息

### 什么是 AUTO_MAS？

AUTO_MAS 是**基于日志**的多账号管理与自动化软件，通过监听日志和注入配置，结合其他脚本（如MAA） 的能力完成多账号代理。

> 我们不实现具体功能，我们只是 脚本软件 的调度器。

## 使用方法

### 安装 AUTO_MAS

1. 前往 [下载页](/download/auto-maa) 获取最新版本安装包。
2. 解压 AUTO_MAS 压缩包并双击运行 `AUTO_MAS-Setup.exe`。
3. 按照安装指引将 AUTO_MAS安装至任意文件夹。

### 为软件添加信任

运行软件前，请将 `MAS 安装目录`、`AUTO_MAS 安装目录`、`%TEMP%\AUTO_MAS` 添加入 Windows Defender 排除项以及防病毒软件的信任区或开发者目录，避免被误杀。以下展示 **添加 Windows Defender 排除项** 方法：

快速链接：<Pill name="Windows 安全中心" link="ms-settings:windowsdefender"/>

1. 若已安装其它杀毒软件，请先打开 **定期扫描**。
![Windows Defender配置1](img/WD-1.png)

2. **“病毒和威胁防护”设置 > 管理设置**
![Windows Defender配置2](img/WD-2.png)

3. **排除项 > 添加或删除排除项**
![Windows Defender配置3](img/WD-3.png)

4. **添加排除项 > 选中对应目录**
![Windows Defender配置4](img/WD-4.png)

5. 若已安装其它杀毒软件，再次关闭 **定期扫描**。

*这总不需要示意图了吧*

::: warning 注意
即使已经安装其它杀毒软件（如：**火绒**、**360极速版**），**Windows Defender** 防病毒功能仍可能会不定时开启，这可能导致您的 `MAA.exe` 或 `AUTO_MAS.exe` 突然消失。因此，您必须确保以上目录被 **Windows Defender** 排除。
:::

### 初始化 AUTO_MAS 

1. 若你下载的不是全量包，你需要根据指引自行进行下载部分AUTO_MAS的依赖
2. 选择下载渠道
![AUTO_初始化](img\user-guide\AUTO-init.png)

::: tip 有何区别
- **快速安装**
  
  - 直接从AUTO_MAS官方提供的服务器下载，也许速度会更快？
  
- **手动安装**。
  
  - 可从pypi、github、git官方渠道和常见镜像站进行下载和安装
  
  - 如果你对环境有基础的认识、快速安装出现问题或较慢、自己有优质的代理可以跑满带宽下载等，推荐你选择手动安装
  

:::

#### 快速安装
只需等待.....

#### 手动安装

手动安装只需根据你的网络环境自行选择下载原即可

1. 设置主题。![设置主题](img\user-guide\AUTO-init-1.png)
1. 配置Python环境![配置环境](img\user-guide\AUTO-init-2.png)
1. 下载Git便携版![AUTO-init-3](img\user-guide\AUTO-init-3.png)
1. 获取后端源码![AUTO-init-4](img\user-guide\AUTO-init-4.png)
1. 从pip安装AUTO所需Python依赖![AUTO-init-5](img\user-guide\AUTO-init-5.png)
1. 等待后端自动启动并打开界面![AUTO-init-final](img\user-guide\AUTO-init-final.png)

### 模拟器管理配置

如果你需要使用MAA、SRC等需要模拟器的脚本进行代理，AUTO提供了一个模拟器管理功能。

**你必须配置了这个功能才能使用**:

- 专项适配：MAA

![AUTO-EmulatorManager](img\user-guide\AUTO-EmulatorManager.png)

#### 自动搜索多开器

点一下就行

如果你能自动搜索出多开器，那就简单了（

但是凡事不会一帆风顺不是吗（

#### 手动添加多开器

1. 选择模拟器类型

![AUTO-EmulatorManager-new-1](img\user-guide\AUTO-EmulatorManager-new-1.png)

::: tip 最佳实践

AUTO-MAS 强烈推荐使用mumu12v4或雷电模拟器，因其拥有良好的性能和截图效果并且多开器适配完善，如果你从零安装模拟器，务必考虑。

由于mumu12v5当前（2025年10月31日）可能存在问题，如：关闭后进程残留、打开后卡死98%、莫名其妙的启动无响应，因此当前不建议使用。

AUTO-MAS群内提供v4的安装包，尽管v5不是不能用，但可能需要经常查看，无法做到彻底的无人值守。

:::

2. 设定模拟器路径

   此路径是模拟器**多开器**的位置

   ![AUTO-EmulatorManager-new-2](img\user-guide\AUTO-EmulatorManager-new-2.png)

::: tip 常见模拟器路径

mumu12v4：mumu安装目录\shell\MuMuManager.exe

mumu12v5：mumu安装目录\nx_main\MuMuManager.exe

雷电模拟器：雷电安装目录\LDPlayer9\dnplayer.exe



:::

