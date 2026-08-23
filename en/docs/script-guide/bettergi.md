---
title: BetterGI Configuration Guide
description: Genshin Impact - BetterGI specialization configuration guide
date: 2026-08-23
---

# BetterGI Configuration Guide

## Two things to know first

**BetterGI** is a third-party tool for Genshin Impact. It can run One Dragon (daily commissions, resin clearing, Ley Line Outcrops, domains, world boss kills, etc.) and many advanced features. It works in a **native GUI direct-control** mode: accounts are managed natively by BetterGI, and complex settings are configured inside BetterGI's own interface.

The AUTO-MAS specialization manages, for each user independently, the **One Dragon** task configuration, **custom config groups**, account switching records, and notifications. Before a run, AUTO-MAS loads that user's independent configuration into BetterGI; when the run ends, it writes it back according to the task strategy. AUTO-MAS does not maintain a full copy of the BetterGI configuration.

<Box :items="[
{ name: 'BetterGI GitHub', link: 'https://github.com/babalae/better-genshin-impact', image: { light: '/icons/github.svg', dark: '/icons/github-dark.svg', }, },
]"/>

## Quick start

1. Download and extract [BetterGI](https://github.com/babalae/better-genshin-impact) from its official repository. It is recommended to use a path without non-ASCII characters, and complete a basic configuration in BetterGI first.
2. Open **Script Management** → **New Script**, choose the **BetterGI script**, and set the BetterGI `RootPath`.
3. Add a user, choose the **Configuration Management Mode** on the user edit page, and configure the One Dragon tasks as needed.
4. Click **Configure BetterGI** in the top-right corner, open the BetterGI native interface and save once; complex settings are also done here.
5. Save, then add the script to the scheduling queue.

## Configuration management mode

Each user can choose a source for their One Dragon configuration:

| Configuration mode | Best for | What it means |
| --- | --- | --- |
| **User-specific config** | Most users | Saves an independent One Dragon configuration per user, loaded before a run and written back after. Different users under the same script can differ. |
| **Script direct control** | Letting BetterGI manage the config | Uses the script's current configuration directly, neither loading nor writing back the user's independent configuration; shared by direct-control users. |

> When **Script direct control** is chosen, the **Task Configuration** and **Custom Config Groups** sections below are greyed out and not editable; BetterGI's native configuration decides.

## Task configuration (One Dragon)

BetterGI stores each One Dragon configuration as an independent JSON file at `{RootPath}/User/OneDragon/{Name}.json` (the `Name` field equals the file name).

### One Dragon config name

The name of a configuration saved in BetterGI's One Dragon page. **Leaving it empty uses "Default Configuration"**.

### Built-in config groups (8 toggles)

The 8 buttons below turn the One Dragon built-in config groups **on / off** and are multi-selectable:

| # | Config group |
| --- | --- |
| 1 | Mail claim |
| 2 | Resin synthesis |
| 3 | Auto Ley Line Outcrop |
| 4 | Auto Domain |
| 5 | Auto world boss hunting |
| 6 | Auto Arcalanos crisis war |
| 7 | Claim daily rewards |
| 8 | Claim Serenitea Pot rewards |

**Important semantics**:

- These buttons are **toggles, not removals**: turning a group off only disables it; **its definition is preserved** and can be turned back on anytime (reversible).
- If a group is "on" but missing from the configuration, it is automatically created and enabled.
- AUTO-MAS only manages these 8 built-in groups; all other settings fields (parties, domains, Ley Line Outcrops, world boss hunting, etc.) are left untouched.

### Daily reward party

The party name used when collecting rewards. **Leaving it empty does not override** BetterGI's existing settings.

### Combat party

The party name used in general combat. Leaving it empty does not override BetterGI's existing settings.

### Combat strategy

Dropdown for the auto-battle strategy. Options are **"Auto-select based on party"** plus the `.txt` combat script file names under `{RootPath}/User/AutoFight/` (scanned in real time; you can drop in `.txt` scripts to add options). **Leaving it empty defaults to "Auto-select based on party"**.

## Custom config groups

Custom config groups are groups you create yourself, beyond the 8 built-in groups above.

### Master toggle "Enable"

Controls whether AUTO-MAS manages the on / off state of custom groups:

- **Off (default)**: custom groups are **left as-is** (their enabled state and order preserved); whether they run is decided by BetterGI's internal configuration.
- **On**: AUTO-MAS manages custom group states through the table below. Groups listed in the table follow their table state; groups present in the BetterGI configuration but not listed default to **enabled**; groups listed as enabled but missing from the configuration are automatically recreated.

> When toggled on for the first time with an empty table, existing custom groups are **auto-loaded from the current BetterGI One Dragon configuration** into the table.

### Add config group

Click **Add Config Group**, enter a name and confirm. New groups are **enabled by default** (names must not duplicate existing groups).

### Table management

The table lists each custom config group, with "Config group name" and "Enabled" columns (click the toggle to switch). You can select several rows with the checkboxes, then click **Delete Selected** to remove them in batch.

## Configure BetterGI (native settings session)

Click the **Configure BetterGI** button in the top-right corner to open a settings session in BetterGI's native interface. A mask appears prompting you to finish configuration in BetterGI, then click **Save Settings** to end the session (the session times out automatically after 30 minutes of no interaction). On save, AUTO-MAS **reads back the current BetterGI One Dragon configuration** and snapshots it as that user's independent copy.

## Per-user One Dragon copy

Each user holds a copy of the One Dragon configuration at `data/{ScriptID}/{UserID}/OneDragon/{Name}.json`. The seed priority when writing is:

1. This user's copy (used first if present)
2. BetterGI's current One Dragon configuration
3. Built-in template

## FAQ

### If I turn off a built-in group, can I get it back later?

Yes. The buttons are toggles: turning off only disables a group without deleting its definition. Turn it back on and its enabled state is restored.

### Why can't I edit task configuration in "Script direct control" mode?

In direct-control mode, the One Dragon configuration is fully decided by BetterGI's native configuration; AUTO-MAS neither loads nor writes back the user's independent configuration, so the related fields are greyed out. Switch back to "User-specific config" to have AUTO-MAS manage it.

### Will my groups created in BetterGI be lost if I turn off the custom groups toggle?

No. When the master toggle is off, custom groups keep their enabled state and order untouched. Only after you turn it on does AUTO-MAS manage their states through the table.

### Why do I see strategy options I did not create?

"Auto-select based on party" is BetterGI's built-in default strategy. The rest come from `.txt` combat scripts under `{RootPath}/User/AutoFight/`, scanned in real time by the system.