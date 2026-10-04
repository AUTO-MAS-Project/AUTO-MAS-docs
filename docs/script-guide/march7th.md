---
title: 三月七用户指南
description: 在AUTO中调度三月七
date: 2025-11-08
---

# 三月七小助手用户指南

## 在AUTO中调度三月七

### 什么是 三月七小助手？

三月七小助手是一个崩坏星穹铁道的第三方软件，能够轻松完成崩坏星穹铁道日常代理、差分宇宙等重复性无趣工作。

**详情信息请查阅**：

<Box :items="[
{ name: '三月七小助手 官网', link: 'https://m7a.top/#/', image: '/icons/m7a.png', },
{ name: '三月七小助手 GitHub', link: 'https://github.com/moesnow/March7thAssistant', image: { light: '/icons/github.svg', dark: '/icons/github-dark.svg', }, },]"/>

## 

### 安装 三月七

1. 前往 <Pill name="三月七小助手 官网" image="/icons/m7a.png" link="https://m7a.top/#/"/>、<Pill name="三月七小助手 仓库" :image="{ light: '/icons/github.svg', dark: '/icons/github-dark.svg', }" link="https://github.com/moesnow/March7thAssistant/releases/"/> 或 <Pill name="Mirror 酱" image="/icons/mirrorchyan.ico" link="https://mirrorchyan.com/zh/projects?scouce=AUTO-MAS-Web&rid=March7thAssistant&channel=stable"/> 下载软件压缩包。
2. 将 三月七小助手 压缩包解压至任意文件夹。

::: warning 别解压到中文路径
三月七小助手（以及其他通用脚本）都不要放在带中文的文件夹里，比如 `D:\脚本\`。中文路径容易引发莫名其妙的报错，用 `D:\M7A` 这样的纯英文路径。
:::


### 设置脚本实例

1. 打开 `March7th Launcher.exe`，阅读并关闭三月七小助手的默认公告。
![AUTO_MAS配置1](/docs/img/script-guide/March7thAssistan/March7thAssistan-1.png)

2. 关闭 **三月七小助手**，打开**AUTO-MAS**，进入 **脚本管理**，单击 **新建脚本** 并选择 **通用脚本** 以添加脚本实例管理页面。
![AUTO_MAS配置2](/docs/img/script-guide/March7thAssistan/AUTO-MAA-1.png)

3. 在弹出的窗口里选择选择**从模板创建**，然后单击 **确定**
![AUTO_MAS配置3](/docs/img/script-guide/March7thAssistan/AUTO-MAA-2.png)

4. 接着在新的窗口界面找到并选择**三月七小助手的通用模板参考**，并点击**使用此模板**。
![AUTO_MAS配置4](/docs/img/script-guide/March7thAssistan/AUTO-MAA-3.png)

稍后会打开脚本的配置，如下图：
![AUTO_MAS配置4](/docs/img/script-guide/March7thAssistan/AUTO-MAA-4-1.png)
![AUTO_MAS配置4](/docs/img/script-guide/March7thAssistan/AUTO-MAA-4-2.png)

5. 在 **打开的脚本配置** 中的 **脚本根目录** 单击 **选择文件夹**，打开 三月七小助手 软件所在目录。
![AUTO_MAS配置5](/docs/img/script-guide/March7thAssistan/AUTO-MAA-5.png)
::: warning 下面那些路径别手动改
选好脚本根目录之后，**脚本配置** 一栏的各个路径会自动填好。模板已经帮你配对了，不清楚每项是什么意思就别动它，改错了代理会各种出问题。
:::

6. 选择完 三月七小助手 的目录以后会自动修正**脚本配置**一栏的路径，无需手动选择。同时我们需要点击右下方的保存按钮
![AUTO_MAS配置6](/docs/img/script-guide/March7thAssistan/AUTO-MAA-6.png)

7. 点击**添加用户**，需要自己给添加的用户进行命名（在用户名一栏输入你想要的用户名（这仅仅只是个命名而已）），同时点击右上方的**创建用户**按钮
![AUTO_MAS配置7](/docs/img/script-guide/March7thAssistan/AUTO-MAA-7.png)

8. 找到刚刚创建的用户，点击**编辑**，进入**通用脚本**配置界面
![AUTO_MAS配置8](/docs/img/script-guide/March7thAssistan/AUTO-MAA-8.png)

9. 找到右上方的**通用配置**并点击，当出现正在配置通用脚本配置的界面并唤醒三月七小助手的时候，说明已经可以正常配置通用脚本了
![AUTO_MAS配置9](/docs/img/script-guide/March7thAssistan/AUTO-MAA-9.png)

10. 当配置完**三月七小助手**以后点击**保存配置**，然后再次点击**保存修改**就完成了对**三月七小助手**的在MAS内的全部配置
![AUTO_MAS配置10](/docs/img/script-guide/March7thAssistan/AUTO-MAA-10.png)

11. 如果需要全自动定时启动**三月七小助手**请查阅 [调度队列](/docs/task-scheduler)

现在立刻马上就要切换账号的功能？你可以试试[StarRailAutoLogin: 一个在AUTO-MAS项目中使用的崩坏：星穹铁道自动登录脚本](https://github.com/Alirea10/StarRailAutoLogin)

## 配置账号

添加用户并进入 **通用脚本** 配置界面之后，按下面几项填：

| 配置项 | 填什么 |
| --- | --- |
| **账号名称** | 显示名称，自己认得出就行 |
| **启用状态** | 关掉的用户会被跳过 |
| **账号 ID** | 三月七里的账号标识，留空就不切号 |
| **密码** | 一般不用填，交给三月七自己的登录配置 |
| **剩余天数** | 还能代理几天。`-1` 是不限制，跑成功一次减一天，减到 0 就跳过这个用户 |
| **备注** | 随便写 |

::: info 注意
**用户** 配置模式下不要忘了 **设置具体配置**。
:::

## 用户配置怎么来的

三月七是通用脚本，AUTO-MAS 通过读写三月七自己的配置文件来切换账号和任务。点右上方的 **通用配置** 时，AUTO-MAS 会唤醒三月七小助手并打开它的配置界面，你在里面怎么设，AUTO-MAS 就按这份配置跑。

::: warning 配置完要保存两次
在三月七里改完，先点 **保存配置**，再回到 AUTO-MAS 点 **保存修改**。只点一个的话改动不会落到用户配置里，下次跑还是老样子。
:::

## 使用 HSR 脚本类型接入

除了通用脚本方式，也可以用 **HSR 脚本** 类型接入三月七小助手——由 AUTO-MAS 直接管理三月七路径、游戏路径与多账号。完整图文部署实录（31 张过程截图，红圈标注点击位置）见 [三月七 · HSR 脚本部署实录](/docs/script-guide/march7th-hsr)。

## 放进调度队列

脚本和用户都配好之后，把用户拖进 [调度队列](/docs/task-scheduler) 就能定时自动跑。多个账号按顺序排队，一个跑完自动切下一个。

需要开机就跑、或者跳过锁屏密码的话，看 [让电脑定时上班](/docs/advanced-features/skip-password)。

## 常见问题

### 唤醒不了三月七 / 通用配置点了没反应

先确认 **脚本根目录** 选的是三月七的解压目录，不是它的子文件夹，也不是 AUTO-MAS 的目录。路径里带中文也会导致唤不起来。

### 中文路径报错

和前面安装那节说的一样，把三月七放在 `D:\M7A` 这样的纯英文路径下，别用 `D:\脚本\`。

### 账号没切换

确认三月七里已经保存了对应账号的登录信息，并且 AUTO-MAS 用户配置里的 **账号 ID** 和三月七里的账号对得上。留空的话 AUTO-MAS 不会切号。

### 日志里报路径找不到

多半是脚本根目录选错，或者三月七被移动过位置。重新选一次根目录，让 **脚本配置** 一栏的路径自动刷新。

