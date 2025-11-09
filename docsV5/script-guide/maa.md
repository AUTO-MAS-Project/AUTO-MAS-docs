---
title: MAA用户指南
description: 在AUTO里使用MAA指南
date: 2025-02-10
---

# MAA用户指南

## 在AUTO中调度MAA

### 什么是 MAA？

MAA 是一个明日方舟第三方软件，能够轻松完成明日方舟日常代理、肉鸽存钱等重复性无趣工作。

**详情信息请查阅**：

<Box :items="[
{ name: 'MAA 官网', link: 'https://maa.plus/', image: 'https://maa.plus/favicon.ico', },
{ name: 'MAA GitHub', link: 'https://github.com/MaaAssistantArknights/MaaAssistantArknights', image: { light: '/resource/github.svg', dark: '/resource/github-dark.svg', }, },]"/>

## 

### 安装 MAA

1. 前往 <Pill name="MAA 官网" image="https://maa.plus/favicon.ico" link="https://maa.plus"/>、<Pill name="MAA 仓库" :image="{ light: '/resource/github.svg', dark: '/resource/github-dark.svg', }" link="https://github.com/MaaAssistantArknights/MaaAssistantArknights/releases/latest"/> 或 <Pill name="Mirror 酱" image="https://mirrorchyan.com/favicon.ico" link="https://mirrorchyan.com/zh/projects?rid=MAA&scouce=AUTO_MAA-Web"/> 下载软件压缩包。
2. 将 MAA 压缩包解压至任意文件夹。


### 设置脚本实例

1. 打开 `MAA.exe`，在 **设置 > 切换配置** 中，确保当前配置为 **Default**。
![AUTO_MAS配置1](../img/AUTO_MAA-config-2.png)
2. 关闭 **MAA**，检查 `MAA.exe`，确保其无需以管理员身份运行。即确保 **MAA.exe > 右键 > 属性 > 兼容性 > 设置 > 以管理员身份运行此程序** 处于 **未勾选** 状态。
![AUTO_MAS配置2](../img/AUTO_MAA-config-3.png)
3. 进入 **脚本管理**，单击 **新建脚本** 并选择 **MAA脚本** 以添加脚本实例管理页面。
![AUTO_MAS配置3](../img/script-guide/maa/AUTO-MAA-1.png)
4. 在 **打开的脚本配置** 中的 **MAA路径** 单击 **选择文件夹**，打开 MAA 软件所在目录。
![AUTO_MAS配置4](../img/script-guide/maa/AUTO-MAA-2.png)
5. 在**模拟器管理**中选择模拟器和模拟器实例

  如果此处没有模拟器，请先完成[模拟器管理](/docsV5/advanced-features/emulator-manager.md)

  完成后，点击**右下角蓝色保存按钮**保存。

6. 点击此处在MAA中配置
![AUTO_MAS配置5](../img/script-guide/maa/AUTO-MAA-5.png)


7. 参照 [多开指南 > MAA 设置](/docsV5/advanced-features/multi-instance.md) 完成 **MAA** 的 **启动设置**，并手动取消勾选 **开机自启动MAA**。

  同时根据你的偏好，你可以设置各种比如：

  公招是否自动点五星，是否启用GPU加速，明日方舟的服务器，启动设置中的“等待模拟器启动时间”（此选项会决定AUTO等待模拟器的启动时间）等

8. 完成配置后，关闭 **MAA**。并在AUTO中点击保存配置。
![AUTO_MAS配置6](../img/script-guide/maa/AUTO-MAA-6.png)

### 设置用户配置

1. 在 **脚本管理** 的脚本表格内，单击 **添加用户** 以添加一个用户。
![AUTO_MAS配置7](../img/script-guide/maa/AUTO-MAA-7.png)
2. 按照设置卡相关提示填写用户信息。
![AUTO_MAS配置8](../img/script-guide/maa/AUTO-MAA-8.png)

::: info 注意
开启 **自定义基建** 后需要 **选择配置文件**；**详细** 配置模式下开启 **剿灭任务**、**日常任务** 后需要 **设置具体配置**。

用户名：该用户在脚本的名字，无实际作用，仅用作展示用户和日志分类

账号id：游戏的**绑定手机号（官服）**或**昵称（B服）**

密码：无实际作用，仅用作防止遗忘。该密码会在本地DPAPI加密保存，基于TPM，移动AUTO文件夹至其他电脑或刷机，均会导致密码无法解密和失效，具有强安全性，可根据自己实际情况使用。

其余配置请查看软件内tips。

:::
::: tip 提示

由于 MAA 在 **B服账号切换** 功能中使用的 OCR（文字识别）技术准确率有限，我们建议如下填写方式以提高识别和切换成功率：

### 通用建议

- MAA 的账号切换功能只需**成功识别一个唯一片段**即可完成切换。
- 请填写**仅该账号独有的部分片段**，避免与其他账号重复。
- 建议在 MAA 中测试填写片段能否正常切换账号，确认无误后再填写至 AUTO_MAS。
- 若同区服仅有一个账号，也可将 `账号ID` 留空。

#### 官服

- 官服账号ID为手机号，通常只需填写**后四位数字**即可，无需填写完整手机号。~~（你也不想无意中泄露自己的手机号吧~）~~ 
- 示例：
  - 账号1：`133XXXX1234`
  - 账号2：`133XXXX5678`
  - 若要切换账号1，可填写 `2`、`4`、`12`、`34`、`123`、`234`、`1234` 等仅账号1拥有的片段。

#### B服

- B服账号为 B站昵称，可能包含**中文、英文、数字、特殊符号、日文**等复杂字符，OCR 模型识别准确率偏低。因此，**不建议填写完整昵称**，建议填写 **不易识别错误的、唯一片段**，避免：
  - 生僻字（如：`黍`，可能被识别为其他文字）
  - 下划线 `_`（常被识别错误或漏识）
- 示例：
  - 账号1：`DLmaster_361` → 可填写 `master`、`361`
  - 账号2：`黍的XX_1234` → 可填写 `1234`、`的`、`XX`，**不建议填写“黍”或下划线**
  

经过这些修改，你应该能够稳定的进行账号切换

:::

至此，你的MAA已经配置完成。

你可以：[创造调度队列，设置定时任务](/docsV5/advanced-features/scheduler)
