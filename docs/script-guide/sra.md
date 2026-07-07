---
title: SRA用户指南
description: 在AUTO中调度SRA
date: 2025-11-08
---

# StarRailAssistant用户指南

## 在AUTO中调度SRA

### 什么是 SRA？

StarRailAssistant是一个崩坏星穹铁道的第三方软件，能够轻松完成崩坏星穹铁道日常代理、差分宇宙等重复性无趣工作。

**详情信息请查阅**：

<Box :items="[
{ name: 'SRA 官网', link: 'https://starrailassistant.top/#/', image: 'https://starrailassistant.top/img/SRAico.png', },
{ name: 'SRA GitHub', link: 'https://github.com/Shasnow/StarRailAssistant', image: { light: '/icons/github.svg', dark: '/icons/github-dark.svg', }, },]"/>

## 安装 SRA

1. 前往 <Pill name="SRA 官网" image="https://starrailassistant.top/img/SRAico.png" link="https://starrailassistant.top/#/"/>、<Pill name="SRA 仓库" :image="{ light: '/icons/github.svg', dark: '/icons/github-dark.svg', }" link="https://github.com/Shasnow/StarRailAssistant/releases/"/> 或 <Pill name="Mirror 酱" image="https://mirrorchyan.com/favicon.ico" link="https://mirrorchyan.com/zh/projects?scouce=AUTO-MAS-Web&rid=StarRailAssistant&channel=stable"/> 下载软件压缩包。
2. 将 SRA 压缩包解压至任意文件夹。

::: warning 温馨提醒
请不要将SRA以及其他需要使用的通用脚本解压在中文文件夹，比如**脚本**等等。

以便出现不必要的异常。
:::


## 设置脚本实例

由于 SRA 和 AUTO-MAS 都提供了多用户功能，因此在 AUTO-MAS 中调度 SRA 有两种方式：

### 方式一：基于AUTO-MAS的多用户功能

1. 打开 **AUTO-MAS**，进入 **脚本管理**，单击 **新建脚本** 并选择 **通用脚本** 以添加脚本实例管理页面。
   ![SRA配置1](/docs/img/script-guide/March7thAssistan/AUTO-MAA-1.png)
2. 在弹出的窗口里选择选择**从模板创建**，然后单击 **确定**
   ![SRA配置2](/docs/img/script-guide/March7thAssistan/AUTO-MAA-2.png)
3. 接着在新的窗口界面找到并选择 适用于SRAv2.14及以上版本的 **StarRailAssistant** 模板，并点击**使用此模板**。
4. 稍后会打开脚本的配置，如下图：
   ![SRA配置3](/docs/img/script-guide/sra/mas1.png)
5. 在 **打开的脚本配置** 中的 **脚本根目录** 单击 **选择文件夹**，打开 SRA 软件所在目录。
   ![SRA配置4](/docs/img/script-guide/sra/mas2.png)
   ::: warning 温馨提示
   脚本配置一栏会在选择脚本根目录以后自动修正，请不要在不理解这个功能有什么作用的时候贸然修改，以便给自己在使用AUTO-MAS的过程中带来不愉快。
   :::
6. 选择完 SRA 的目录以后会自动修正**脚本配置**一栏的路径，无需手动选择。
   ![SRA配置5](/docs/img/script-guide/sra/mas3.png)
7. SRA 约定选择 `C:\Users\用户名\AppData\Roaming\SRA` 作为默认配置目录，因此无需修改 **配置文件路径** 一栏。
   ![SRA配置6](/docs/img/script-guide/sra/mas4.png)
8. 脚本配置将自动保存，接下来退出脚本配置页面。
9. 点击**添加用户**，需要自己给添加的用户进行命名（在用户名一栏输入你想要的用户名（这仅仅只是个命名而已）），然后点击右上方的**通用配置**按钮
   ![SRA配置7](/docs/img/script-guide/sra/mas5.png)
10. 这将启动 SRA 窗口，用户可以在此界面中进行 SRA 的相关配置。
   ::: warning 温馨提示
   使用基于AUTO-MAS的多用户功能时，请勿修改配置文件名称，保持配置文件名称为默认的 `Default`
   :::
   ![SRA配置8](/docs/img/script-guide/sra/sra1.png)
11. 配置完成后，点击SRA窗口主页启动按钮右侧的箭头，展开启动选项，选择**仅保存配置**
   ![SRA配置9](/docs/img/script-guide/sra/sra2.png)
12. 点击**仅保存配置**后，关闭 SRA 窗口并点击 AUTO-MAS 中的保存配置按钮，就完成了一个用户的配置。
   ![SRA配置10](/docs/img/script-guide/sra/sra3.png)
13. 如果要添加更多用户，请重复步骤 8-12，您可能注意到当第10步启动 SRA 窗口时，SRA 会自动加载上一次的配置文件，这是正常现象，您只需为新用户修改配置即可。
   ::: warning 温馨提示
   使用基于AUTO-MAS的多用户功能时，请勿修改配置文件名称，保持配置文件名称为默认的 `Default`
   :::

### 方式二：基于SRA的多用户功能

步骤1-7与方式一相同。

8. 修改启动参数一栏，改为 `-e task run`，这将使 SRA 在启动时运行所有配置，而不是单独运行某个配置文件。
   ![修改启动参数](/docs/img/script-guide/sra/mas6.png)
9. 配置将自动保存，接下来退出脚本配置页面。
10. 点击**添加用户**，需要自己给添加的用户进行命名（在用户名一栏输入你想要的用户名（这仅仅只是个命名而已）)，然后点击右上方的**通用配置**按钮
   ![SRA配置7](/docs/img/script-guide/sra/mas5.png)
11. 这将启动 SRA 窗口，你可以在此界面中进行 SRA 的相关配置。
   你可以直接使用Default配置文件，如果你有多个用户，请**在SRA中**创建新的配置，并切换到对应的配置进行修改
   ![SRA配置8](/docs/img/script-guide/sra/sra4.png)
   ![SRA配置8](/docs/img/script-guide/sra/sra5.png)
   ![SRA配置8](/docs/img/script-guide/sra/sra6.png)
12. 每个配置完成后，都需要点击SRA窗口控制面板启动按钮右侧的箭头，展开启动选项，选择**仅保存配置**
   ![SRA配置9](/docs/img/script-guide/sra/sra2.png)
13. 保存好每个配置后，关闭 SRA 窗口并点击 AUTO-MAS 中的保存配置按钮，就完成了所有用户的配置。

### 差异

下面的图片展示了两种方式的逻辑差异：
![对比](/docs/img/script-guide/sra/compare.png)