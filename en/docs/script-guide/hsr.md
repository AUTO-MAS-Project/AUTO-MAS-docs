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

That is the point of the HSR script type: **you do not have to choose one**. Run both under a single script and decide task by task which one handles it. AUTO-MAS starts the game, calls the scripts in order, retries failures, and remembers whether this week's weekly tasks and this month's monthly tasks are already done.

**What it can run** (exact coverage depends on the script versions you install):

- Stamina stages: Calyx (Golden), Calyx (Crimson), Cavern of Corrosion, Planar Ornament Extraction
- Echo of War, reset weekly
- Daily tasks and rewards: redemption codes, mail, assignments, Nameless Honor, Daily Training, and more
- Weekly: Divergent Universe, Currency Wars
- Monthly: the three endgame modes (Memory of Chaos / Pure Fiction / Apocalyptic Shadow)

::: warning The three endgame modes are not usable yet
The configuration entries are in place, but the current version disables the switch in the UI and the feature has not been tested enough. Trust your actual UI, and **do not count on this feature for now**.
:::

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
2. **Open each script manually once and let it finish initializing.** Do not skip this. A script only writes its config files on first launch, and AUTO-MAS reads those files to know which stages you can pick. Skip it and the stage dropdowns will be empty.
3. **Install the Honkai: Star Rail PC client**, the CN official client.
4. **Add everything to your antivirus allowlist**: AUTO-MAS, M7A, SRA, and the game directory. Otherwise the scripts get blocked and tasks fail for no visible reason.
5. **Keep paths free of non-English characters and spaces**, for example `D:\M7A` and `D:\SRA`. Such paths have a long history of breaking image recognition and path parsing.

::: warning Pick the folder, not the exe
When you save, AUTO-MAS checks that the folder you picked contains the matching exe. Pick the wrong thing and a popup stops you right there:

| This field | Needs this file in the folder |
| --- | --- |
| March7th path | `March7th Assistant.exe` |
| SRA path | `SRA-cli.exe` |
| Game path | `StarRail.exe` |
:::

## Create an HSR Script

### 1. Create the script

Go to **Script Management** → **New Script** → select **HSR Script** → confirm. AUTO-MAS opens the script configuration page.

### 2. Fill in paths and basic information

| Configuration | What to enter |
|---|---|
| **Script name** | Any name you will recognize, such as "Main HSR account" |
| **March7th path** | The **folder** M7A is in |
| **SRA path** | The **folder** SRA is in |
| **Game path** | The **folder** Honkai: Star Rail is in |
| **Maximum game startup wait time** | How many seconds to wait after starting the game before acting. Default 60. Raise it on slow machines |
| **Game startup arguments** | Leave empty |

::: tip One script is enough
Fill in either M7A or SRA and leave the other empty. The empty one will not appear in the task assignment below.

After you change a path, task assignment is reshuffled to match the paths you currently have. Go back and check it.
:::

### 3. Set retries and timeouts

These decide how long a task can run before it counts as stuck, and how many times a failure is retried. The defaults suit most people. Raise them on slow machines.

| Configuration | Description | Default |
|---|---|---|
| **Maximum failed task retries** | How many extra attempts a failed task gets | 3 |
| **Daily task timeout limit (minutes)** | Cap for daily, stamina, and reward tasks | 20 |
| **Weekly task timeout limit (minutes)** | Cap for Divergent Universe and Currency Wars | 60 |
| **Monthly task timeout limit (minutes)** | Cap for the three endgame modes | 60 |
| **Enable low-performance compatibility mode** | Only affects M7A running Divergent Universe. Turn it on if M7A runs it unreliably | Disabled |

### 4. Decide which script runs which task

This is the heart of the HSR script type: four groups of tasks, each assigned to one script.

| Module | What it covers | Default |
|---|---|---|
| **Stamina** | Trailblaze Power stage farming, Echo of War | SRA |
| **Daily tasks and rewards** | Redemption codes, mail, assignments, Nameless Honor, Daily Training, and more | SRA |
| **Divergent Universe** | Divergent Universe | SRA |
| **Currency Wars** | Currency Wars | SRA |

If you filled in only one script path, that is the only option here. Fill in both to choose freely.

Once you choose, the page shows the exact strategy that script uses for the task. **These strategies are fixed. You do not fill them in, and you do not set them again on the user page**:

- **Divergent Universe**
  - SRA: Paradise Chronicle / farm the first stage / 20 runs / claim points rewards
  - M7A: claim points rewards / cyclical extrapolation / low-performance compatibility follows the switch above. Team, blessings, and extrapolation strategy are up to M7A
- **Currency Wars**
  - SRA: standard game / lowest difficulty / the first strategy saved in SRA / 2 runs
  - M7A: claim points rewards / standard game / lowest rank / Aglaea strategy / restarts on certain entries

::: warning Running Currency Wars on SRA means claiming rewards yourself
SRA **does not claim points rewards** after it finishes Currency Wars. You have to collect them in game. Assign Currency Wars to M7A instead and the problem goes away.
:::

## Create an HSR User

Click **Add User** in the **Script Management** table, then fill in the basics:

| Field | What to enter |
|---|---|
| **Username** | Display name. It is also passed to the script as the "Trailblazer name" for Currency Wars |
| **Enabled** | Disabled users are skipped |
| **Account** | Login account such as a phone number. Only needed for automatic account switching |
| **Password** | Login password, same as above |
| **Server** | Only the official CN server for now |
| **Remaining days** | How many days of automation are left. `-1` means unlimited. Each run subtracts one day, and at 0 the user is skipped |
| **Notes** | Anything you like |

::: warning About the account and password
They are stored locally and encrypted automatically. Nothing is uploaded.

**If you did not fill in an SRA path, or no task is assigned to SRA, the account and password are never used at all.** Leave them empty.

Also, do not hand your `data/` directory or script configuration JSON to anyone else. They contain your encrypted credentials.
:::

### Which tasks this user runs

| Switch | Description | Default |
|---|---|---|
| **Stamina** | Stamina stages plus Echo of War | Disabled |
| **Daily tasks and rewards** | Redemption codes, mail, assignments, Nameless Honor, Daily Training, and more | Disabled |
| **Three endgame modes** | Once a month, all three together. Disabled in the current UI | Disabled |
| **Divergent / Currency** | Pick one of three: neither / Divergent Universe / Currency Wars | Disabled |

Turn a switch on and the page shows which script will run it and with which strategy, matching what the script page showed. There is nothing to set twice.

## Configure Stamina Stages

The **Stamina Configuration** area has four dropdowns, one per stage type. Leave any of them empty if you do not want to farm it:

| Dropdown | What it farms |
|---|---|
| **Calyx (Golden)** | Character EXP / Light Cone EXP / Credits |
| **Calyx (Crimson)** | Trace materials |
| **Cavern of Corrosion** | Relics |
| **Planar Ornament Extraction** | Planar Ornaments |

Golden and Crimson do not affect each other. You can keep a selection in both.

Below that:

- **Stage to farm**: from the four types above, pick the one to actually farm this time.
- **Current active stage**: shows the stage you selected, just so you can confirm it.
- **Echo of War**: pick one of the stages read from the script, or leave it empty to skip.
- **Echo of War start day**: set a day of the week. From that day on, AUTO-MAS runs it if it is not done yet this week, and stops once it is done.

### Dropdowns are empty?

Those stage options are not invented by AUTO-MAS. They are **read out of your M7A or SRA**, specifically whichever script you assigned Stamina to. So an empty list almost always comes down to one of three things:

1. **The script was never initialized** → open M7A or SRA manually once and let it finish. This is by far the most common cause.
2. **The script path is wrong** → AUTO-MAS cannot find the config files. Go back and check the path.
3. **You just switched which script runs Stamina** (for example SRA to M7A) → the two have different stage lists, so you need to **pick your stages again**. The page shows a yellow notice when this happens.

## AUTO-MAS Handles Weekly and Monthly Tasks for You

AUTO-MAS keeps this bookkeeping itself. **The user page never asks you to pick something like "Divergent Universe 1 / Divergent Universe 2"**:

- **Divergent Universe, Currency Wars, Echo of War**: recorded weekly. Once done this week, later runs skip it, and it unlocks again on Monday.
- **Three endgame modes**: recorded monthly, run once a month.

### Changing progress by hand

The **Progress and Reset** area at the bottom of the user page shows whether Echo of War, the weekly tasks, and the three endgame modes are done this week or this month. Each one has **Mark Complete** and **Reset**.

Two situations call for it: you already did it yourself in game and want AUTO-MAS to skip it, or you want to force a re-run, in which case you hit Reset.

::: tip These buttons only change records
They change AUTO-MAS's own bookkeeping and **do not drive the scripts to run anything**.
:::

## Running and Logs

Once configured, add the script to the [task scheduler queue](/en/docs/task-scheduler) and it runs on its own. Three things worth knowing:

- **The game may restart repeatedly**: when one user's tasks are split across two different scripts, the game restarts at each switch. That is intentional. It stops the two scripts' states from interfering with each other.
- **Your M7A and SRA configuration is safe**: AUTO-MAS backs it up before running and restores it afterwards.
- **Failures are retried automatically**: retried up to your **Maximum failed task retries**, with a game restart before each retry.

### Which logs to attach when reporting a problem

- `debug/app.log` — the AUTO-MAS main process log
- `debug/frontend.log` — the frontend log

If it looks like a problem inside one of the scripts, attach that script's own log too. See each script's documentation for where it lives.

## FAQ

### There is no HSR option when creating a script

Your AUTO-MAS is too old to have the HSR script type. Update it from the [download page](/en/download/auto-mas), then restart the app once.

### The path is rejected no matter what I enter

**You probably selected the exe itself. This field wants the folder.** Check that the exe sits directly in that folder: `March7th Assistant.exe` for M7A, `SRA-cli.exe` for SRA, `StarRail.exe` for the game. Do not pick a subdirectory either.

### The stage list is empty

Check in this order: have you opened M7A or SRA manually once (most common), does the path point at the script's root directory, and did you just change which script runs Stamina (if so, reselect your stages).

### Task status is wrong, things run that shouldn't or don't run that should

- Check the task switches on the user page first.
- Weekly tasks reset weekly and monthly tasks reset monthly. State clears automatically at the boundary.
- **Mark Complete** or **Reset** in the **Progress and Reset** area fixes the state immediately.
- Still wrong? Check `debug/app.log`. It records how the scripts exited.

### M7A runs Divergent Universe unreliably

Turn on **Enable low-performance compatibility mode** on the script page. Team, blessings, and extrapolation strategy are outside AUTO-MAS's control, so set those up in M7A beforehand.

### SRA finished Currency Wars but there are no points

Known behavior. SRA does not claim them, so collect them in game. To skip that step, assign Currency Wars to M7A instead.

### The game keeps restarting

One user's tasks are split across two different scripts, so the game restarts at each switch. It is intentional, and it keeps the two scripts' states from interfering. If it bothers you, assign all the tasks to one script.

### A task failed, but there is no script output in the log

The script never started, and that almost always means antivirus blocked it. Add the M7A, SRA, and game directories to your antivirus allowlist and try again. While you are there, confirm the paths contain no non-English characters, spaces, or symbolic links.

## Feedback and Help

- AUTO-MAS issue feedback: [GitHub Issues](https://github.com/AUTO-MAS-Project/AUTO-MAS/issues)
- March7th Assistant: [m7a.top](https://m7a.top/) / [GitHub](https://github.com/moesnow/March7thAssistant)
- StarRailAssistant: [starrailassistant.top](https://starrailassistant.top/) / [GitHub](https://github.com/Shasnow/StarRailAssistant)
