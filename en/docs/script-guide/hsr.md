---
title: HSR Honkai Star Rail User Guide
description: "Schedule Honkai: Star Rail external scripts in AUTO-MAS with M7A and SRA"
date: 2026-06-17
---

# HSR Honkai Star Rail User Guide

::: tip Scope
The HSR specialization is used to schedule Honkai: Star Rail external scripts in AUTO-MAS. It was introduced with [AUTO-MAS PR #249](https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/249) and corresponds to the built-in AUTO-MAS script type **HSR**.
:::

## What Is the HSR Specialization?

The HSR specialization is a built-in AUTO-MAS script adapter that connects two mainstream third-party PC tools for Honkai: Star Rail:

- **March7th Assistant (M7A)**: the M7A route, good at stamina farming scripts and weekly tasks.
- **StarRailAssistant (SRA)**: the SRA route, good at daily rewards, Divergent Universe, and related tasks.

With the HSR specialization, AUTO-MAS can mix both engines under one script and automatically manage game startup, script invocation, failure retries, and weekly/monthly progress.

**Supported gameplay coverage** depends on the script version you use:

- Stamina stages: Calyx (Golden), Calyx (Crimson), Cavern of Corrosion, Planar Ornament Extraction
- Echo of War, reset weekly
- Daily tasks and rewards, such as redemption codes, mail, assignments, Nameless Honor, and Daily Training
- Weekly tasks: Divergent Universe (PVE mode), Currency Wars (PVP mode)
- Monthly tasks: the three endgame modes, Memory of Chaos / Pure Fiction / Apocalyptic Shadow

> **About the three endgame modes**: AUTO-MAS has reserved configuration entries and snapshot import capability for these modes, but during PR #249 the user-page switch is temporarily disabled and marked as "Future feature". Availability and stability depend on your current AUTO-MAS version. Do **not** treat this as a stable feature yet.

**For more information, see:**

<Box :items="[
{ name: 'March7th Assistant Website', link: 'https://m7a.top/', image: 'https://m7a.top/assets/screenshot/March7th.png', },
{ name: 'March7th Assistant GitHub', link: 'https://github.com/moesnow/March7thAssistant', image: { light: '/icons/github.svg', dark: '/icons/github-dark.svg', }, },
{ name: 'SRA Website', link: 'https://starrailassistant.top/', image: 'https://starrailassistant.top/favicon.ico', },
{ name: 'SRA GitHub', link: 'https://github.com/Shasnow/StarRailAssistant', image: { light: '/icons/github.svg', dark: '/icons/github-dark.svg', }, },
]"/>

## Prerequisites

Before creating your first HSR script in AUTO-MAS, complete these steps:

1. **Install AUTO-MAS**: use a version that includes the HSR specialization, meaning a version after PR #249 was merged.
2. **Install March7th Assistant (M7A)**: after extraction, **open it manually at least once** and wait for initialization to complete. The first launch creates files such as `config.yaml`. Confirm that the directory contains `March7th Assistant.exe`.
3. **Install StarRailAssistant (SRA)**: after extraction, **open it manually at least once** so SRA can generate `settings.json`, `configs`, and related directories. Confirm that the directory contains `SRA-cli.exe`.
4. **Install the Honkai: Star Rail PC client**: the CN official client is supported. Confirm that the directory contains `StarRail.exe`.
5. **Add antivirus and Defender trust entries**: add AUTO-MAS, M7A, SRA, and the Honkai: Star Rail game directory to Windows Defender or third-party antivirus trust lists to avoid external scripts being blocked.
6. **Avoid Chinese paths**: place all directories under plain English paths, such as `D:\AUTO-MAS`, `D:\M7A`, `D:\SRA`, and `D:\StarRail`. Chinese paths and paths with spaces have historically caused image recognition and path parsing issues.

::: warning Reminder
When saving paths, AUTO-MAS automatically checks whether the selected directory contains the expected `exe`:

- March7th path: must contain `March7th Assistant.exe`
- SRA path: must contain `SRA-cli.exe`
- Game path: must contain `StarRail.exe`

If you select the wrong directory, the frontend blocks it with a popup and asks you to reselect.
:::

## Create an HSR Script

### 1. Create a Script

1. Go to **Script Management**.
2. Click **New Script**.
3. In the script type list, select **HSR Script** (type ID: HSR).
4. Click OK. AUTO-MAS creates an HSR script instance and opens the script configuration page.

### 2. Configure Basic Script Information

Fill in the **HSR Script Configuration** page:

| Configuration | Description | Notes |
|---|---|---|
| **Script name** | Give this script instance an easy-to-recognize name | For example, "Main HSR account" or "Official daily" |
| **March7th path** | M7A installation directory containing `March7th Assistant.exe` | Validates the exe; can be cleared with one click |
| **SRA path** | SRA installation directory containing `SRA-cli.exe` | Validates the exe; can be cleared with one click |
| **Game path** | Honkai: Star Rail installation directory containing `StarRail.exe` | Validates the exe |
| **Maximum game startup wait time** | Seconds AUTO-MAS waits after starting the game before the client is considered operable | Default is 60 seconds; increase it for slower machines |
| **Game startup arguments** | Extra command-line arguments passed when starting `StarRail.exe` | Usually left empty |

::: tip Notes
- At least one of the M7A and SRA paths must be configured. Leaving the other side empty is allowed; that side will not appear in **Module Script Assignment**.
- After any path is changed, Module Script Assignment (TaskMapping) is automatically reshuffled according to the currently configured paths.
:::

### 3. Configure Execution Limits

| Configuration | Description | Default |
|---|---|---|
| **Maximum failed task retries** | Upper limit for automatic retries after a task fails | 3 |
| **Daily task timeout limit (minutes)** | Maximum duration for one daily, stamina, or reward task | 20 |
| **Weekly task timeout limit (minutes)** | Maximum duration for one weekly task such as Divergent Universe or Currency Wars | 60 |
| **Monthly task timeout limit (minutes)** | Maximum duration for one monthly task such as the three endgame modes | 60 |
| **Enable low-performance compatibility mode** | Only affects March7th Divergent Universe and maps to `weekly_divergent_stable_mode` | Disabled |

### 4. Module Script Assignment (TaskMapping)

The HSR specialization can assign four modules to M7A or SRA separately:

| Module | Meaning | Default engine |
|---|---|---|
| **Stamina** | Trailblaze Power farming, Echo of War, and related tasks | SRA |
| **Daily tasks and rewards** | Redemption codes, mail, assignments, Nameless Honor, Daily Training, and more | SRA |
| **Divergent Universe** | Divergent Universe PVE mode | SRA |
| **Currency Wars** | Currency Wars PVP mode | SRA |

> TaskMapping options change dynamically based on configured M7A / SRA paths. You can choose between both only when both paths are configured. If only one path is configured, only that engine is available.

When a different engine is selected, the **Weekly Task Execution Strategy** area displays the concrete parameters for that engine. **The user page no longer requires you to fill in these parameters manually**:

- **Divergent Universe**
  - SRA: Divergent Universe adventure notes / farm first stage mode / 20 runs / enable points reward
  - March7th: enable points reward / cyclical extrapolation / low-performance compatibility follows the script-page switch; team, blessings, and extrapolation strategy are decided by the M7A client
- **Currency Wars**
  - SRA: standard game / lowest difficulty / first strategy saved in SRA / 2 runs
  - March7th: enable points reward / standard game / lowest rank / Aglaea strategy / accept restart for specific entries

::: warning SRA Currency Wars Note
After SRA finishes Currency Wars, it **does not automatically claim points rewards**. Claim them manually in game. Other engines handle rewards according to their own client rules.
:::

## Create an HSR User

Add a user under the HSR script:

1. In the **Script Management** table, click **Add User**, or open the created HSR script and click **Add User** there.
2. Fill in **Basic Information**:

| Field | Description |
|---|---|
| **Username** | Display name for the user. It is also written to M7A / SRA as the "Trailblazer name" for Currency Wars |
| **Enabled** | Whether the user participates in automation. Disabled users are skipped |
| **Account** | Login account, such as a phone number. Only used when automatic login/account switching is required |
| **Password** | Login password. Only used when automatic login/account switching is required |
| **Server** | Currently only the official CN server (CN-Official) is supported |
| **Remaining days** | Remaining valid automation days. `-1` means unlimited, `0` means expires today, and a positive value is the remaining day count |
| **Notes** | Free-form notes |

::: warning Account and Password Security
- Accounts and passwords are stored locally and encrypted automatically by AUTO-MAS when saved.
- **If the SRA path is not configured, or TaskMapping does not assign a module to SRA, the account and password are not used for account switching**. They are only reserved fields.
- **Do not publicly share your `data/` directory or script configuration JSON files**, because they contain encrypted credentials.
:::

### Task Switches

Configure which modules this user should run:

| Switch | Description | Default |
|---|---|---|
| **Stamina** | Whether to run stamina stages and Echo of War | Disabled |
| **Daily tasks and rewards** | Whether to run redemption codes, mail, assignments, Nameless Honor, Daily Training, and more | Disabled |
| **Three endgame modes** (monthly, all three run together) | The current UI marks this as **disabled**. Whether it is available depends on your actual version | Disabled |
| **Divergent / Currency** | Three-way choice: Off / Divergent Universe / Currency Wars | Disabled |

The UI shows the execution strategy for the engine selected in TaskMapping, consistent with the script page.

## Configure Stamina Stages

In the **Stamina Configuration** area, you can see four independent dropdowns:

| Channel | Corresponding stage type |
|---|---|
| **Calyx (Golden)** | Character EXP / Light Cone EXP / Credits |
| **Calyx (Crimson)** | Trace materials. Golden and Crimson selections do not overwrite each other and can be saved at the same time |
| **Cavern of Corrosion** | Relic stages |
| **Planar Ornament Extraction** | Planar Ornament stages |

Each channel is independently selectable. Leave stages empty if you do not want to farm them.

The following fields are also available:

- **Stage to farm**: choose the channel to farm this time, such as Golden, Crimson, Relic, or Ornament. This writes to `Stage.Channel`.
- **Current active stage**: the UI displays the stage name or stage ID corresponding to `Stage.ScriptStage`.
- **Echo of War**: choose one Echo of War stage read from the external script. Leave empty if you do not want to run it.
- **Echo of War start day**: Monday through Sunday. Once the start day is reached and this week is not complete, AUTO-MAS asks M7A / SRA to try completing it. After logs confirm completion, it will not run again that week.

### Where Do Stage Options Come From?

Stamina stage options **only come from the stage configuration exposed by the external script**, read dynamically according to the engine selected for the **Stamina** module in TaskMapping:

- M7A: read from `instance_names.json`
- SRA: read from `trailblaze_power.toml`

If a dropdown is empty, common causes are:

1. The external script, M7A or SRA, has not been initialized. Open it manually once first.
2. The external script path is wrong, so AUTO-MAS cannot find configuration files.
3. You changed the Stamina execution engine in TaskMapping, for example from SRA to M7A, and need to **reselect stages**.

> When the Stamina execution engine is changed, the Stamina Configuration area displays a yellow notice: "The stamina execution script has changed. Please reselect stages."

## Weekly and Monthly Notes

HSR weekly and monthly progress is recorded automatically by AUTO-MAS. **The user page does not require manual choices such as "Divergent Universe 1 / Divergent Universe 2"**:

- **Divergent Universe and Currency Wars**: weekly tasks. Completion is recorded by ISO week, such as `2025-W23`. If the weekly task is already complete this week, the next run skips it.
- **Three endgame modes**: monthly tasks. They run once per month and consist of three snapshots, Memory of Chaos / Pure Fiction / Apocalyptic Shadow. Completion is recorded by calendar month, such as `2025-06`.
- **Echo of War**: reset by ISO week. Users can specify an **Echo of War start day**. It is only attempted after that day is reached, and it will not repeat after completion this week.

### Progress and Reset

The **Progress and Reset** area at the bottom of the user page provides three manual controls:

- **Echo of War**: shows "completed this week / not completed" and the latest completion date. Provides **Mark Complete** and **Reset** buttons.
- **Weekly**: same behavior, judged by ISO week.
- **Three endgame modes**: judged by calendar month. Provides **Mark Complete This Month** and **Reset** buttons.

> These buttons only modify the local `Data` field and **do not actually drive external scripts**. They are used to quickly synchronize state when an external script has already completed the task, or when you want to force a rerun.

### About the Three Endgame Modes

AUTO-MAS has prepared a complete pipeline for the three endgame modes, including:

- User page: switch ForgottenHall and import three snapshots in the UI
- Script page: import three snapshots from M7A `config.yaml` in one click: Memory of Chaos / Pure Fiction / Apocalyptic Shadow

However, **during PR #249 the user-page switch for these modes is disabled** to avoid misuse before sufficient testing. It will be opened gradually in later versions. Use the actual AUTO-MAS UI as the source of truth.

## Runtime and Logs

After configuring scripts and users, add the script to the task scheduler queue for execution. Daily-use notes:

- **AUTO-MAS restarts the game when switching between M7A and SRA**. This avoids state pollution between external scripts and is expected behavior.
- **AUTO-MAS does not destroy M7A / SRA's own configuration**. Before running, it backs up `config.yaml`, `settings.json`, `cache.json`, and `configs`, then restores them automatically after the run.
- **Automatic retry after failure**: when one task inside a module fails, AUTO-MAS retries according to **Maximum failed task retries**. Before retrying, AUTO-MAS restarts the game.

### Log Locations

When troubleshooting, provide:

- `debug/app.log`: AUTO-MAS main process log
- `debug/frontend.log`: frontend log

If the issue is related to a specific external script, also include the M7A / SRA runtime log directory. See each script's official documentation for its location.

## FAQ

### The HSR script type cannot be found

- Confirm that your AUTO-MAS version has merged [PR #249](https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/249).
- Restart AUTO-MAS so the frontend OpenAPI client is regenerated.

### Path validation fails

- The **March7th path** points to the wrong directory. It must contain `March7th Assistant.exe`.
- The **SRA path** points to the wrong directory. It must contain `SRA-cli.exe`.
- The **Game path** points to the wrong directory. It must contain `StarRail.exe`.
- Note: select the **directory** (folder), not the `exe` file itself.

### Stage list is empty

- Confirm that M7A / SRA has been opened manually once and initialized `config.yaml` / `settings.json`.
- Confirm that the script path points to the external script **root directory**, not a subdirectory.
- If you just switched the execution engine for the **Stamina** module, follow the page notice and reselect stages.

### Task completion status is unexpected

- Check whether the **Daily Tasks** and **Weekly/Monthly** switches match your expectations.
- Weekly tasks reset by ISO week, and monthly tasks reset by calendar month. State resets automatically after week/month changes.
- Check `debug/app.log` for M7A / SRA subprocess exit codes and marker judgment logs.
- In **Progress and Reset**, you can manually mark completion or reset state to synchronize it.

### M7A Divergent Universe seems unstable

- Enable **Low-performance compatibility mode** on the script page.
- Team, blessings, and extrapolation strategy are decided by the M7A client. Configure them in M7A itself in advance.

### SRA Currency Wars finishes but there are no points

- This is known behavior. SRA **does not automatically claim points rewards** after Currency Wars completes. Claim them manually in game.

### The game restarts repeatedly during scheduling

- When different modules for the same user are handled by different engines, for example one by M7A and one by SRA, AUTO-MAS restarts the game during engine switches to avoid script state pollution. This is expected.
- To reduce restarts, assign multiple modules to the same engine in TaskMapping.

### A task fails but logs show no M7A / SRA output

- Confirm that Windows Defender or antivirus software did not block the subprocess.
- Confirm that external script paths do not contain Chinese characters, spaces, or symbolic links.
- Add the M7A, SRA, and Honkai: Star Rail installation directories to the antivirus trust list and try again.

## Feedback and Help

- AUTO-MAS issue feedback: [GitHub Issues](https://github.com/AUTO-MAS-Project/AUTO-MAS/issues)
- March7th Assistant: [m7a.top](https://m7a.top/) / [GitHub](https://github.com/moesnow/March7thAssistant)
- StarRailAssistant: [starrailassistant.top](https://starrailassistant.top/) / [GitHub](https://github.com/Shasnow/StarRailAssistant)
