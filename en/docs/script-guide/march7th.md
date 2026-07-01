---
title: March7thAssistant User Guide
description: Schedule March7thAssistant in AUTO-MAS
date: 2025-11-08
---

# March7thAssistant User Guide

## Schedule March7thAssistant in AUTO-MAS

### What is March7thAssistant?

March7thAssistant is a third-party tool for Honkai: Star Rail. It can handle repetitive tasks such as daily automation and Divergent Universe.

**For more information, see:**

<Box :items="[
{ name: 'March7thAssistant Website', link: 'https://m7a.top/#/', image: 'https://m7a.top/assets/screenshot/March7th.png', },
{ name: 'March7thAssistant GitHub', link: 'https://github.com/moesnow/March7thAssistant', image: { light: '/icons/github.svg', dark: '/icons/github-dark.svg', }, },]"/>

### Install March7thAssistant

1. Download the archive from <Pill name="March7thAssistant Website" image="https://m7a.top/assets/screenshot/March7th.png" link="https://m7a.top/#/"/>, <Pill name="March7thAssistant Repository" :image="{ light: '/icons/github.svg', dark: '/icons/github-dark.svg', }" link="https://github.com/moesnow/March7thAssistant/releases/"/>, or <Pill name="MirrorChyan" image="https://mirrorchyan.com/favicon.ico" link="https://mirrorchyan.com/zh/projects?scouce=AUTO-MAS-Web&rid=March7thAssistant&channel=stable"/>.
2. Extract the March7thAssistant archive to any folder.

::: warning Reminder
Do not extract March7thAssistant or other general scripts you need into Chinese-named folders such as **脚本**.

This helps avoid unnecessary errors.
:::

### Configure the Script Instance

1. Open `March7th Launcher.exe`, read and close the default March7thAssistant announcement.
![AUTO-MAS configuration 1](/docs/img/script-guide/March7thAssistan/March7thAssistan-1.png)

2. Close **March7thAssistant**, open **AUTO-MAS**, go to **Script Management**, click **New Script**, and select **General Script** to add a script instance management page.
![AUTO-MAS configuration 2](/docs/img/script-guide/March7thAssistan/AUTO-MAA-1.png)

3. In the popup, select **Create from template**, then click **OK**.
![AUTO-MAS configuration 3](/docs/img/script-guide/March7thAssistan/AUTO-MAA-2.png)

4. In the new window, find and select **March7thAssistant general template reference**, then click **Use this template**.
![AUTO-MAS configuration 4](/docs/img/script-guide/March7thAssistan/AUTO-MAA-3.png)

The script configuration opens shortly:
![AUTO-MAS configuration 4-1](/docs/img/script-guide/March7thAssistan/AUTO-MAA-4-1.png)
![AUTO-MAS configuration 4-2](/docs/img/script-guide/March7thAssistan/AUTO-MAA-4-2.png)

5. In the opened script configuration, click **Select folder** for **Script root directory**, then open the March7thAssistant software directory.
![AUTO-MAS configuration 5](/docs/img/script-guide/March7thAssistan/AUTO-MAA-5.png)

::: warning Reminder
The script configuration field is automatically corrected after selecting the script root directory. Do not change it casually unless you understand what it does.
:::

6. After selecting the March7thAssistant directory, the **script configuration** path is corrected automatically and does not need manual selection. Click the save button in the lower-right corner.
![AUTO-MAS configuration 6](/docs/img/script-guide/March7thAssistan/AUTO-MAA-6.png)

7. Click **Add user**, give the user a name in the username field, then click **Create user** in the upper-right corner.
![AUTO-MAS configuration 7](/docs/img/script-guide/March7thAssistan/AUTO-MAA-7.png)

8. Find the user you just created, click **Edit**, and enter the **General Script** configuration page.
![AUTO-MAS configuration 8](/docs/img/script-guide/March7thAssistan/AUTO-MAA-8.png)

9. Find **General Configuration** in the upper-right corner and click it. When the general script configuration screen appears and March7thAssistant is launched, the general script can be configured normally.
![AUTO-MAS configuration 9](/docs/img/script-guide/March7thAssistan/AUTO-MAA-9.png)

10. After configuring **March7thAssistant**, click **Save Configuration**, then click **Save Changes** again. March7thAssistant is now fully configured in MAS.
![AUTO-MAS configuration 10](/docs/img/script-guide/March7thAssistan/AUTO-MAA-10.png)

11. To start **March7thAssistant** automatically on a schedule, read [Scheduling Queues](/en/docs/task-scheduler).

Need account switching right now? Try [StarRailAutoLogin: an automatic Honkai: Star Rail login script used in AUTO-MAS](https://github.com/Alirea10/StarRailAutoLogin).

More content may be added later.
