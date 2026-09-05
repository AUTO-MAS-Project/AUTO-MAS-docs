---
title: HSR Honkai Star Rail Configuration Guide
description: "Schedule Honkai: Star Rail external scripts in AUTO-MAS with M7A and SRA"
date: 2026-06-17
---

# HSR Honkai Star Rail Configuration Guide

## What the HSR Script Type Is For

Honkai: Star Rail has two widely used third-party scripts, and each is better at different things:

- **March7th Assistant (M7A)**: better at stamina stages and weekly tasks.
- **StarRailAssistant (SRA)**: better at collecting daily rewards and at Divergent Universe.

That is the point of the HSR script type: **you do not have to choose one**. Run both under a single script and decide task by task which one handles it. AUTO-MAS starts the game, calls the scripts in order, retries failures, and remembers whether this week's weekly tasks are already done.

**What it can run** (exact coverage depends on the script versions you install):

- Stamina stages: Calyx (Golden), Calyx (Crimson), Cavern of Corrosion, Planar Ornament Extraction
- Echo of War, reset weekly
- Daily tasks and rewards: redemption codes, mail, assignments, Nameless Honor, Daily Training, and more
- Weekly: Divergent Universe, Currency Wars

**For more information, see:**

<Box :items="[
{ name: 'March7th Assistant Website', link: 'https://m7a.top/', image: 'https://m7a.top/assets/screenshot/March7th.png', },
{ name: 'March7th Assistant GitHub', link: 'https://github.com/moesnow/March7thAssistant', image: { light: '/icons/github.svg', dark: '/icons/github-dark.svg', }, },
{ name: 'SRA Website', link: 'https://starrailassistant.top/', image: 'https://starrailassistant.top/favicon.ico', },
{ name: 'SRA GitHub', link: 'https://github.com/Shasnow/StarRailAssistant', image: { light: '/icons/github.svg', dark: '/icons/github-dark.svg', }, },
]"/>

## Before You Start

Get these out of the way first. It removes about half the problems people run into later.

1. **Install the scripts you want to use.** At least one of M7A and SRA. Install both if you want to mix them.
2. **Open each script manually once and save its settings.** Do not skip this. A script only writes its config files on first launch, and AUTO-MAS reads those files to know which stages and which options you can pick. Skip it and the stage dropdowns will be empty.
3. **Install the Honkai: Star Rail PC client**, the CN official client.
4. **Add everything to your antivirus allowlist**: AUTO-MAS, M7A, SRA, and the game directory. Otherwise the scripts get blocked and tasks fail for no visible reason.
5. **Keep paths free of non-English characters and spaces**, for example `D:\M7A` and `D:\SRA`. Such paths have a long history of breaking image recognition and path parsing.

::: warning Pick the folder, not the exe
When you save, AUTO-MAS checks that the folder you picked contains the matching exe. Pick the wrong thing and a popup stops you right there:

| For this field | The folder must contain |
| --- | --- |
| March7th path | `March7th Assistant.exe` |
| SRA path | `SRA-cli.exe` |
| Game path | `StarRail.exe` |
:::

## Create an HSR Script

### 1. Create the script

Go to **Script Management** → **New Script** → pick **HSR Script** → confirm. The app jumps to the script configuration page.

### 2. Script and game configuration

| Field | What to enter |
|---|---|
| **Script name** | Any name you will recognize, such as "Main Star Rail account" |
| **March7th path** | The **folder** M7A lives in |
| **SRA path** | The **folder** SRA lives in |
| **SRA configuration profile** | SRA can keep several configuration profiles; this picks which one to use. Defaults to "auto", see below |
| **AUTO-MAS manages the game** | "Yes" lets AUTO-MAS launch the game and wait until it is ready. Pick "No" if the game is already running, or if you want the script to handle launching |
| **Game path** | The **folder** Honkai: Star Rail lives in. Required when "AUTO-MAS manages the game" is "Yes" |
| **Maximum game launch wait** | Seconds to wait after launching before the client is considered ready. Default 60, raise it on slower machines |
| **Run in 1920×1080 windowed mode** | Temporarily rewrites the registry before a task so the game runs at 1920×1080 windowed, and restores it afterwards. The scripts' image recognition is tuned for this resolution, so turn it on if your aspect ratio differs |
| **Run redemption codes only when they change** | On by default. Skips the redemption code step when the codes have not changed, so it does not run pointlessly every day |

::: tip One script is enough
Fill in only M7A or only SRA and leave the other empty. The empty one will not appear as an engine choice on the user page.
:::

::: tip About SRA configuration profiles
SRA stores its settings as one or more profiles under `%APPDATA%\SRA\configs`. Which one you pick decides three things: what the AUTO-MAS-managed task options show, which profile script direct control runs, and which one gets copied when you pin a snapshot.

Leaving it on "auto" prefers `Default` and otherwise takes the first profile in filename order — which is exactly what AUTO-MAS always did before. If the profile you picked is later deleted or renamed, AUTO-MAS falls back to auto and says so on both the script page and the user page, rather than switching silently.
:::

::: warning Change the resolution while the game is closed
"Run in 1920×1080 windowed mode" works by writing the registry, which does not take effect while the game is already running. AUTO-MAS logs a note asking you to close the game and run again; the current round still proceeds, just without the resolution change.
:::

### 3. Set retries and timeouts

These control timeout and retry behavior. The defaults suit most setups; adjust them for your machine.

| Field | What it does | Default |
|---|---|---|
| **Maximum attempts for a failed task** | How many extra tries a failed task gets | 3 |
| **Daily task timeout (minutes)** | Cap for daily / stamina / reward tasks | 20 |
| **Weekly task timeout (minutes)** | Cap for Divergent Universe and Currency Wars | 60 |
| **Enable low-performance compatibility mode** | Only affects M7A running Divergent Universe. Turn it on if M7A runs it unreliably | Off |

::: tip Task assignment is not on this page
Which script runs which task is decided **per user**, on the user page, not here. Two users under the same script can use different engines.
:::

## Create an HSR User

In the **Script Management** table, click **Add User**, then fill in the basics:

| Field | What to enter |
|---|---|
| **User name** | Display name. It is also sent to the script as the "Trailblazer name" for Currency Wars |
| **Enabled** | Disabled users are skipped |
| **Account** | Phone number or similar login. Only needed when SRA has to switch accounts for you |
| **Password** | Login password, same condition |
| **Server** | CN official only for now |
| **Days remaining** | How many more days to run this user. `-1` means no limit; otherwise it drops by one per run and the user is skipped at 0 |
| **Note** | Anything you want |

::: warning About the account and password
They are stored locally and encrypted automatically. Nothing is uploaded.

**These two fields only appear in AUTO-MAS-managed mode when SRA is actually used.** In every other case they are not needed and stay hidden.

Also, do not share your `data/` directory or script configuration JSON with anyone. They contain your encrypted credentials.
:::

## Run Mode: AUTO-MAS Managed or Script Direct Control

Every user picks a **run mode**, which decides who is in charge:

| Mode | Who decides what runs | Who it suits |
|---|---|---|
| **AUTO-MAS managed** (default) | AUTO-MAS. Task switches, engines, stages and every option are set on this page | People who want to manage every account from one place |
| **Script direct control** | M7A / SRA themselves. AUTO-MAS only launches them on schedule | People already comfortable in the scripts who just want a scheduler |

### Script direct control

Direct control needs **no configuration in AUTO-MAS at all**. Whatever you set up in M7A / SRA is exactly what runs, and changes take effect immediately — there is nothing to sync back here.

Two steps: pick the run mode, then switch on the scripts you want to run.

::: warning Direct control ignores the rest of this page
Task switches, account and password, stamina stages, and the options under AUTO-MAS-managed tasks have **no effect** in direct control. They belong to managed mode.
:::

#### When you actually need "pin as a snapshot"

Exactly one situation: **several game accounts under the same HSR script, and you want each to run a different plan**.

M7A and SRA each keep a single configuration of their own, so multiple direct-control users share it. In that case, open one user and click **Pin the current configuration as a snapshot**. AUTO-MAS copies the script's configuration as it is right now and stores it under that user, who then runs that copy independently of the others.

::: warning A snapshot is frozen
Once pinned, a snapshot **does not follow later changes in the script**. Change a setting in SRA and the pinned user still runs the old copy. Click **Re-pin to the current configuration** to refresh it, or **Go back to using the script's current configuration** to drop it.

With only one user, skip snapshots — the default live configuration is simpler.
:::

## Configure Stamina Stages

The **Stamina configuration** area has four dropdowns, one per stage type. Leave a dropdown empty for stages you do not want to farm:

| Dropdown | What it farms |
|---|---|
| **Calyx (Golden)** | Character EXP / Light Cone EXP / Credits |
| **Calyx (Crimson)** | Trace materials |
| **Cavern of Corrosion** | Relics |
| **Planar Ornament Extraction** | Planar ornaments |

Golden and Crimson are independent and can both be saved at once.

Below that:

- **Stage to farm**: which of the four types this run actually farms.
- **Active stage**: shows the stage you selected, for confirmation only.
- **Echo of War**: pick one of the stages read from the script, or leave it empty to skip.
- **Echo of War start day**: pick a weekday. From that day on, if this week's run has not happened yet, it runs. Once done, it does not repeat that week.

::: tip Stage options follow the engine
These stage options are not written by AUTO-MAS — they are **read out of your M7A / SRA**, from whichever engine handles stamina. The two lists differ, so **your stage choice is stored per engine**. Switch engines and you have to pick again; switch back and your earlier choice is still there.
:::

## AUTO-MAS-Managed Tasks

This is the main area of managed mode: the four task modules on the left, and the selected module's detailed options on the right.

| Module | What it covers | Switch default |
|---|---|---|
| **Stamina and cultivation targets** | Trailblaze Power stages, Echo of War | On |
| **Daily tasks and rewards** | Redemption codes, mail, assignments, Nameless Honor, Daily Training, and more | On |
| **Divergent Universe** | Divergent Universe | Off |
| **Currency Wars** | Currency Wars | Off |

Divergent Universe and Currency Wars are two independent switches. Turn both on if you want both.

Each module picks its own **engine** (SRA or March7th Assistant). With only one script path filled in, that is the only choice.

### Where the detailed options come from

The options on the right are **not defined by AUTO-MAS**. They are read live out of your SRA / March7th Assistant configuration file, showing whatever the script has saved right now. When you change one here, AUTO-MAS records **only the fields you changed** (its "overrides"); everything else keeps following the script.

That has a few direct consequences:

- New options added by a script update show up here on their own — you do not have to wait for an AUTO-MAS update.
- **Switching the engine swaps the entire set of options**, because the two engines do not share field names. Values you changed under the current engine are not carried over, but they are kept, so switching back brings them along.
- Some things are deliberately left to the script, such as SRA's redemption code list. Those can only be entered in the script's own UI.

::: warning Two kinds of settings are force-disabled by AUTO-MAS
In managed mode, AUTO-MAS forces the script's own **notifications** and **after-task action** off in the temporary configuration (March7th's `after_finish`, SRA's `missionAccomplished`).

The reason is that those actions close the game, or the whole computer. March7th closes the game on the way out whenever `after_finish` is anything but "None", which AUTO-MAS then misreads as the task failing part way through; set to "Shutdown" it really does shut the machine down 60 seconds later. Notifications are sent by AUTO-MAS itself instead.

This only affects the run AUTO-MAS performs in managed mode — your own settings in the script are not damaged. **Script direct control is not affected at all**: whatever you set is what runs.
:::

### Reset to source configuration

**Reset to source configuration**, at the top right of the module list, **deletes every override this user has in AUTO-MAS** (all modules, all fields). Everything then displays and runs according to the script's current configuration. The source configuration file itself is not touched.

The action cannot be undone, so it asks for confirmation first.

### What "N invalid overrides" means

After a script update, or after switching SRA configuration profiles, a field you once changed may no longer exist, or its type may no longer match. Such an override is marked invalid:

- **It is ignored at run time and the source value is used instead.** It never causes an error or interrupts the task.
- The detail panel lists which ones, why they became invalid, and the value you had saved.
- **Clear invalid overrides** removes them from this user's configuration. That too only touches the AUTO-MAS side, never the source file.

## AUTO-MAS Handles Weekly Progress for You

AUTO-MAS keeps its own records for these:

- **Divergent Universe, Currency Wars, Echo of War**: tracked per week. Once done, later runs skip them, and they unlock again on Monday.

### Changing progress by hand

The **Progress and reset** area at the bottom of the user page shows whether Echo of War and the weekly tasks are done this week, with **Mark as done** and **Reset** for each.

Two uses: you already did it yourself in game and want AUTO-MAS to stop trying, or you want to force a re-run.

::: tip These buttons only change records
They only touch AUTO-MAS's own bookkeeping. They **do not make the scripts run anything**.
:::

## Running and Logs

Once configured, add the script to a [task queue](/en/docs/task-scheduler) and it runs automatically. Three things worth knowing:

- **The game may restart repeatedly**: when one user's tasks are split across two scripts, the game restarts on each switch. That is deliberate, to keep the two scripts' states from interfering.
- **Your M7A / SRA configuration is safe**: AUTO-MAS backs it up before running and restores it afterwards.
- **Failures are retried**: according to your "maximum attempts for a failed task", restarting the game before each retry.

### Which logs to attach when reporting a problem

- `debug/app.log` — the AUTO-MAS backend log
- `debug/frontend.log` — the UI log

If the problem looks like it is in one of the scripts, attach that script's own log as well (see its documentation for the location).

## FAQ

### There is no HSR option when creating a script

Your AUTO-MAS is too old to have the HSR script type. Update from the [download page](/en/download/auto-mas) and restart the app afterwards.

### The path is rejected no matter what I enter

**You probably selected the exe itself. This field wants the folder.** Make sure the folder directly contains the matching exe: `March7th Assistant.exe` for M7A, `SRA-cli.exe` for SRA, `StarRail.exe` for the game. Do not select a subfolder either.

### The stage list is empty

Check in this order: have you opened M7A / SRA once and saved settings there (most common cause) → is the path the script's root folder → did you just switch the stamina engine (switching requires picking stages again, and the page says so).

### Task status is wrong, things run that shouldn't or don't run that should

- Check the task switches on the user page first.
- Weekly tasks reset weekly; crossing into a new week clears them automatically.
- **Progress and reset** lets you mark or reset a task to fix the state immediately.
- If it is still wrong, check `debug/app.log` for how the script exited.

### M7A runs Divergent Universe unreliably

Turn on **Enable low-performance compatibility mode** on the script page. Team, blessings and simulation strategy are outside AUTO-MAS's control — set those up in M7A beforehand.

### SRA finished Currency Wars but there are no points

Known behavior: SRA does not collect them automatically, so collect them in game. Hand Currency Wars to M7A instead if you want to skip that step.

### The game keeps restarting

One user's tasks are split across two scripts, so the game restarts on each switch. That is deliberate, to prevent state interference. Assign everything to one script if it bothers you.

### A task failed, but there is no script output in the log

That means the script never started, which is almost always antivirus. Add the M7A, SRA and game directories to your antivirus allowlist and try again. Also confirm the paths contain no non-English characters, spaces or symbolic links.

## Feedback and Help

- AUTO-MAS issues: [GitHub Issues](https://github.com/AUTO-MAS-Project/AUTO-MAS/issues)
- March7th Assistant: [m7a.top](https://m7a.top/) / [GitHub](https://github.com/moesnow/March7thAssistant)
- StarRailAssistant: [starrailassistant.top](https://starrailassistant.top/) / [GitHub](https://github.com/Shasnow/StarRailAssistant)
