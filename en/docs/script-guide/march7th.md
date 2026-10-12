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
{ name: 'March7thAssistant Website', link: 'https://m7a.top/#/', image: '/icons/m7a.png', },
{ name: 'March7thAssistant GitHub', link: 'https://github.com/moesnow/March7thAssistant', image: { light: '/icons/github.svg', dark: '/icons/github-dark.svg', }, },]"/>

### Install March7thAssistant

1. Download the archive from <Pill name="March7thAssistant Website" image="/icons/m7a.png" link="https://m7a.top/#/"/>, <Pill name="March7thAssistant Repository" :image="{ light: '/icons/github.svg', dark: '/icons/github-dark.svg', }" link="https://github.com/moesnow/March7thAssistant/releases/"/>, or <Pill name="MirrorChyan" image="/icons/mirrorchyan.ico" link="https://mirrorchyan.com/zh/projects?scouce=AUTO-MAS-Web&rid=March7thAssistant&channel=stable"/>.
2. Extract the March7thAssistant archive to any folder.

::: warning Don't Unpack Into a Non-English Path
Keep March7thAssistant, and any other general script, out of folders with non-ASCII characters in the name, such as `D:\脚本\`. Those paths cause failures that are hard to diagnose. Use a plain English path like `D:\M7A`.
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

::: warning Don't Edit the Paths Below by Hand
Once you pick the script root directory, the paths under **script configuration** are filled in automatically by the template. Leave them alone unless you know what each one does. Wrong values break automation in confusing ways.
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

## Configure Accounts

After adding a user and opening the **General Script** configuration screen, fill in these fields:

| Field | What to put |
| --- | --- |
| **Account Name** | A display name, anything you recognise |
| **Enabled** | Disabled users are skipped |
| **Account ID** | The account identifier in March7thAssistant. Leave it empty and AUTO-MAS will not switch accounts |
| **Password** | Usually not needed; leave login to March7thAssistant's own configuration |
| **Remaining Days** | How many days it may still run. `-1` means unlimited; one is deducted per successful run and the user is skipped at 0 |
| **Notes** | Anything you like |

::: info Note
In **user** configuration mode, don't forget to **set the specific configuration**.
:::

## Where the User Configuration Comes From

March7thAssistant is a general script, so AUTO-MAS switches accounts and tasks by reading and writing March7thAssistant's own configuration file. When you click **General Configuration** in the upper-right corner, AUTO-MAS launches March7thAssistant and opens its configuration screen. Whatever you set there is what AUTO-MAS runs with.

::: warning Save Twice After Configuring
After changing things in March7thAssistant, click **Save Configuration** first, then go back to AUTO-MAS and click **Save Changes**. Clicking only one leaves the changes out of the user configuration, and the next run still uses the old settings.
:::

## Put It in a Scheduling Queue

With the script and users configured, drag the users into a [Scheduling Queue](/en/docs/task-scheduler) and they run on a timer. Several accounts queue up in order, and AUTO-MAS moves to the next one when a run finishes.

## Common Questions

### March7thAssistant won't launch / General Configuration does nothing

Check that **Script Root Directory** points at the folder you extracted March7thAssistant into — not one of its subfolders, and not the AUTO-MAS directory. Non-English characters in the path also stop it from launching.

### Errors about the path being non-English

As in the install section above, put March7thAssistant under a plain English path like `D:\M7A`, not `D:\脚本\`.

### The account isn't switching

Make sure the login details for that account are saved inside March7thAssistant, and that **Account ID** in the AUTO-MAS user configuration matches the account there. If it is left empty, AUTO-MAS will not switch.

### The log says the path can't be found

Most likely the script root directory is wrong, or March7thAssistant was moved. Re-select the root directory and let the paths under **Script Configuration** refresh themselves.

More content may be added later.
