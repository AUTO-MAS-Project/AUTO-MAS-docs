---
title: 开始使用
description: AUTO-MAS 的使用指南
date: 2025-02-10
---

# 开始使用

## 前置信息

### 什么是 AUTO-MAS？

**AUTO-MAS** 是基于日志监看的多脚本多配置管理与自动化软件。修改配置文件和通过监听日志，控制其他脚本（如 MAA） 完成多账号代理。

> AUTO-MAS: Make ALL Scripts Auto

## 使用方法

### 安装 AUTO-MAS

1. 前往 [下载页](/download/auto-mas) 获取最新版本安装包。
2. 解压 AUTO-MAS 压缩包并双击运行 `AUTO-MAS-Setup.exe`。
3. 按照安装指引将 AUTO-MAS安装至任意文件夹。

### 为软件添加信任

运行软件前，请将 `MAS 安装目录`、`AUTO-MAS 安装目录`、`%TEMP%\AUTO-MAS` 添加入 Windows Defender 排除项以及防病毒软件的信任区或开发者目录，避免被误杀。以下展示 **添加 Windows Defender 排除项** 方法：

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
即使已经安装其它杀毒软件（如：**火绒**、**360极速版**），**Windows Defender** 防病毒功能仍可能会不定时开启，这可能导致您的 `MAA.exe` 或 `AUTO-MAS.exe` 突然消失。因此，您必须确保以上目录被 **Windows Defender** 排除。
:::

### 初始化 AUTO-MAS 

1. 若你下载的不是全量包，你需要根据指引自行进行下载部分AUTO-MAS的依赖
2. 选择下载渠道
![AUTO_初始化](img\user-guide\AUTO-init.png)

::: tip 有何区别
- **快速安装**
  
  - 直接从AUTO-MAS官方提供的服务器下载，也许速度会更快？
  
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


至此你已经完成AUTO_MAS的基础配置，接下来你可以：
1. 参考[脚本指南](./script-guide/__index.md)添加脚本
2. 参考[模拟器管理](./advanced-features/emulator-manager.md)添加模拟器