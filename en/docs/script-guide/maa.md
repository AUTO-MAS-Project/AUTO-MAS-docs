# MAA Configuration Guide

## What is MAA?

MAA is a third-party Arknights tool that can handle repetitive tasks such as daily automation and Integrated Strategies credit farming.

**For more information, see:**

<Box :items="[
{ name: 'MAA Website', link: 'https://maa.plus/', image: 'https://maa.plus/favicon.ico', },
{ name: 'MAA GitHub', link: 'https://github.com/MaaAssistantArknights/MaaAssistantArknights', image: { light: '/icons/github.svg', dark: '/icons/github-dark.svg', }, },]"/>

## Install MAA

1. Download the archive from <Pill name="MAA Website" image="https://maa.plus/favicon.ico" link="https://maa.plus"/>, <Pill name="MAA Repository" :image="{ light: '/icons/github.svg', dark: '/icons/github-dark.svg', }" link="https://github.com/MaaAssistantArknights/MaaAssistantArknights/releases/latest"/>, or <Pill name="MirrorChyan" image="https://mirrorchyan.com/favicon.ico" link="https://mirrorchyan.com/zh/projects?rid=MAA&scouce=AUTO-MAS-Web"/>.
2. Extract the MAA archive to any folder.

## Configure the Script

1. Go to **Script Management**, click **New Script**, and select **MAA Script** to add a script instance management page.
![AUTO-MAS configuration 1](/docs/img/script-guide/maa/AUTO-MAA-1.png)

2. In the opened script configuration, click **Select folder** for **MAA path**, then open the directory where MAA is located.
![AUTO-MAS configuration 2](/docs/img/script-guide/maa/AUTO-MAA-2.png)

3. In **Emulator Management**, select the emulator and emulator instance.

> If no emulator appears here, complete **Emulator Management** configuration first.

4. Click **Configure MAA** and configure it in MAA.
![AUTO-MAS configuration 5](/docs/img/script-guide/maa/AUTO-MAA-5.png)

5. Manually uncheck **Start MAA on boot**, complete ADB connection settings, and configure other options according to your preferences.

6. After configuration, close **MAA** and click **Save Configuration** in AUTO-MAS.
![AUTO-MAS configuration 6](/docs/img/script-guide/maa/AUTO-MAA-6.png)

## Configure Users

1. In the script table under **Script Management**, click **Add user** to add a user.
![AUTO-MAS configuration 7](/docs/img/script-guide/maa/AUTO-MAA-7.png)

2. Fill in user information according to the hints on the settings card.
![AUTO-MAS configuration 8](/docs/img/script-guide/maa/AUTO-MAA-8.png)

::: info Note
In **detailed** configuration mode, do not forget to **set specific configuration**.
:::

::: tip Account ID Tips

Because MAA's **Bilibili server account switching** uses OCR with limited accuracy, we recommend the following input strategy to improve recognition and switching success.

#### General Advice

- MAA account switching only needs to **recognize one unique fragment** to complete switching.
- Enter only a fragment unique to that account and avoid fragments shared with other accounts.
- Test in MAA whether the fragment can switch accounts correctly before entering it into AUTO-MAS.
- If there is only one account under the same server, `Account ID` can be left empty.

#### Official Server

- The official server account ID is a phone number. Usually, entering only the **last four digits** is enough. You do not need to enter the full phone number.
- Example:
  - Account 1: `133XXXX1234`
  - Account 2: `133XXXX5678`
  - To switch to Account 1, you can enter `2`, `4`, `12`, `34`, `123`, `234`, `1234`, or any other fragment unique to Account 1.

#### Bilibili Server

- Bilibili server accounts use Bilibili nicknames, which may include Chinese, English, numbers, special symbols, Japanese, and other complex characters. OCR accuracy may be low. Therefore, **do not enter the full nickname**. Enter a **unique fragment that is unlikely to be misrecognized**, and avoid:
  - Rare characters, such as `黍`, which may be recognized as something else
  - Underscores `_`, which are often misrecognized or missed
- Example:
  - Account 1: `DLmaster_361` -> enter `master` or `361`
  - Account 2: `黍的XX_1234` -> enter `1234`, `的`, or `XX`; do **not** enter `黍` or `_`

With these changes, account switching should become more stable.
:::

### Configuration Notes

The following describes AUTO-MAS configuration behavior for MAA in **auto-proxy** mode.

1. Configuration items shown on the user configuration page take priority.
2. In **Annihilation** tasks, only **Start wakeup** and **Use sanity** are enabled. In **Use sanity**, only **Annihilation mode** stage automation is performed, and task configuration is generated automatically from user settings. In **Daily** tasks, tasks enabled in **Task configuration** are enabled, and task order is fixed.
3. **Scheduled execution** remains disabled. **Behavior after task completion**, **behavior after MAA startup**, **MAA minimization settings**, and **update settings** are automatically adjusted according to actual configuration and execution.
4. In **simple** configuration mode, other settings use **MAA global settings**. In **detailed** configuration mode, other settings use the **user-specific configuration**. In task configuration, only the first task of each type takes effect. If no task of that type is found, the default value is used.

## Plans

With plans, you can customize stage automation by week.

![plan](/docs/img/advanced-features/plan-1.png)

It is designed to be easy to understand.

After switching the configuration mode to weekly plan mode, you can decide what to farm at different times.

Switching to simplified view provides an editing experience similar to mower.

Then, in the MAA user interface, select the plan in the stage configuration mode.

![plan](/docs/img/advanced-features/plan-2.png)

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

3. Enter `<Token>` into the corresponding option tab in the app.

4. If you need to obtain **Hypergryph account login credentials** for multiple accounts in a row, clear browser cookies to remove login state. Logging out directly from the web page will make the token expire.

::: tip Reminder
Do not enter the quotation marks around `content`, and do not enter the entire returned page content into the option tab.
:::
