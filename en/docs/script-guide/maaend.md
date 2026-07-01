---
title: MaaEnd Configuration Guide
description: "Arknights: Endfield - MaaEnd assistant configuration guide"
date: 2026-04-28
---

# MaaEnd Configuration Guide

## What is MaaEnd?

MaaEnd is a third-party automation tool for Arknights: Endfield. Based on visual AI technology, it can automatically complete repetitive daily tasks such as Protocol Space.

**For more information, see:**

<Box :items="[
{ name: 'MaaEnd Website', link: 'https://maaend.com/', image: 'https://maaend.com/favicon.ico', },
{ name: 'MaaEnd GitHub', link: 'https://github.com/MaaEnd/MaaEnd', image: { light: '/icons/github.svg', dark: '/icons/github-dark.svg', }, },]"/>

## Install MaaEnd

1. Download the archive from <Pill name="MaaEnd Website" image="https://maaend.com/favicon.ico" link="https://maaend.com/"/> or <Pill name="MaaEnd Repository" :image="{ light: '/icons/github.svg', dark: '/icons/github-dark.svg', }" link="https://github.com/MaaEnd/MaaEnd/releases/latest"/>.
2. Extract the MaaEnd archive to any folder.

::: warning Reminder
Do not extract MaaEnd into a path containing Chinese characters to avoid unnecessary errors.
:::

## Configure the Script

1. Go to **Script Management**, click **New Script**, and select **Create new script -> MaaEnd Script** to add a script instance management page.
![Create MaaEnd script](/docs/img/script-guide/maaend/选择脚本.png)

2. In the opened script configuration, click **Select folder** for **MaaEnd path**, then open the directory where MaaEnd is located.
<!-- ![Set MaaEnd path](/docs/img/script-guide/maaend/maaend-2.png) -->

3. Adjust the following configuration as needed:

   | Configuration | Description |
   | --- | --- |
   | **Controller type** | Select the control method. This overrides the setting configured in MaaEnd. We recommend using the PC client instead of an emulator. |
   | **Game path** | Path to the Endfield game executable |
   | **Game launch arguments** | Extra command-line arguments when launching the game. Leave empty if not needed. |
   | **Wait time after game startup** | How many seconds to wait after launching the game before automation starts. Default is 60 seconds. |
   | **Close game after task completion** | Whether to close the game automatically after the last user task completes |
   | **Proxy timeout limit** | Consider the task timed out if logs do not change for this many minutes. Default is 10 minutes. |
   | **Daily proxy count limit** | Maximum number of automation runs per user per day. `0` means unlimited. |
   | **Maximum retry count per run** | Maximum retries after automation failure. Default is 3. |

4. Click **Save Configuration**.

::: info About Emulators

- **ADB**: controls Android emulators through the ADB protocol. The emulator must be configured in **Emulator Management**.
:::

## Configure Users

1. In the script table under **Script Management**, click **Add user** to add a user.

2. Fill in user information according to the hints on the settings card.
![User information](/docs/img/script-guide/maaend/用户配置.png)

### User Configuration Fields

#### Basic Information

| Configuration | Description |
| --- | --- |
| **Username** | Display name used to distinguish accounts |
| **Enabled status** | Whether the user participates in automation. Disabled users are skipped. |
| **Account ID** | Endfield login phone number, 11 digits. Leave empty to skip account switching. |
| **Password** | Endfield login password, stored encrypted. Required for PC account switching. |
| **Configuration mode** | `Simple` uses global MaaEnd configuration; `Detailed` uses this user's independent MaaEnd configuration |
| **Server** | Currently only `Official server` is supported |
| **Remaining days** | Remaining valid automation days. `-1` means unlimited. After each successful automation, it decreases by 1. When it reaches 0, the user is skipped. |
| **Notes** | Free-form notes |

::: tip Account Switching Notes

- Account ID is an 11-digit phone number, and the password is used for automatic login on PC.
- If Account ID is left empty, AUTO-MAS uses the currently logged-in account directly and does not switch accounts.
- Passwords are stored locally in encrypted form and are not uploaded to any server.
:::

#### Task Configuration

MAS can currently help control the "Protocol Space" farming type in MaaEnd.

After configuring it in MAS, you still need to manually add the "Protocol Space" task when configuring MaaEnd. MAS will automatically overwrite the first "Protocol Space" task.

## Skland Automatic Check-In

::: warning Note
AUTO-MAS processes check-in requests locally and does not upload tokens to third-party servers.

Automatic check-in has risks. AUTO-MAS is not responsible for any results caused by automatic check-in. Using this feature means you agree to bear the related risks yourself.
:::

### Get Hypergryph Account Login Credentials

1. Log in to the [Skland web page](https://www.skland.com/).

2. Visit this [URL](https://web-api.skland.com/account/info/hg).

   It returns information similar to:

   ```json
   {
     "code": 0,
     "data": {
       "content": "<Token>"
     },
     "msg": "The API returns the login credential of your Hypergryph account. This credential can be used by the Hypergryph account system to verify login validity. Leaking login credentials is extremely dangerous. For account security, do not disclose this credential to anyone in any form."
   }
   ```

3. Enter `<Token>` into the **Skland check-in** area of the user configuration and enable the check-in switch.

4. If you need to obtain **Hypergryph account login credentials** for multiple accounts in a row, clear browser cookies to remove login state. Logging out directly from the web page will make the token expire.

::: tip Reminder
Do not enter the quotation marks around `content`, and do not enter the entire returned page content into the option tab.
:::

### Configuration Notes

The following describes AUTO-MAS configuration behavior for MaaEnd in **auto-proxy** mode.

1. Configuration items shown on the user configuration page take priority.
2. Users run sequentially according to the user list. Each user starts the game and MaaEnd process independently.
3. In **simple** configuration mode, all users share MaaEnd global settings in **script configuration**. In **detailed** configuration mode, each user uses an independent MaaEnd configuration file.
4. During automation, AUTO-MAS automatically writes MaaEnd configuration, including Protocol Space tasks and controller type, then restores the original configuration after the task completes.
5. **Skland check-in** runs before automation tasks. Each account checks in only once per day.
6. Each user's automation result, including success, failure, and proxy count, is recorded automatically and displayed as tags in the UI.

## FAQ

1. Endfield takes a relatively long time to start. We recommend not lowering the default wait time of 60 seconds.
2. Foreground mode fully occupies the mouse. Operating the keyboard or mouse during automation may cause automation to fail.
3. Make sure the game path points to `Endfield.exe`, not the Hypergryph launcher.
4. In fullscreen mode, resolution is determined by monitor resolution settings. Changing it inside the game is meaningless. MaaEnd requires a 16:9 resolution ratio.
5. Do not enable frame interpolation or similar features, as they may cause MaaEnd screenshots to fail.
