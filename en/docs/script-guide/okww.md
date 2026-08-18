---
title: OK-WW Configuration Guide
description: Wuthering Waves - OK-WW specialized adaptation guide
date: 2026-08-18
---

# OK-WW Configuration Guide

## Two things to know first

**OK-WW** is a third-party automation tool for Wuthering Waves. It is part of the **ok-script family**. **OkNte**, used for Neverness to Everness, is a separate project; this page only covers OK-WW.

The AUTO-MAS adaptation handles the repetitive parts: starting a task, monitoring logs, and optionally starting or closing the game. Complex OK-WW settings stay in the native OK-WW GUI, so MAS does not need to maintain a full copy of every setting.

<Box :items="[
{ name: 'OK-WW GitHub', link: 'https://github.com/ok-oldking/ok-wuthering-waves', image: { light: '/icons/github.svg', dark: '/icons/github-dark.svg', }, },]"/>

## Quick Start

1. Download [OK-WW from its official repository](https://github.com/ok-oldking/ok-wuthering-waves) and extract it. A path without Chinese characters is recommended. Open OK-WW once and complete its basic setup.
2. Open **Script Management** → **New Script** and select **ok-ww Script**.
3. Set **ok-ww path** to the directory containing `ok-ww.exe`, not the executable itself.
4. On the user page, click **Configure OK-WW** to open the native OK-WW GUI and save its basic configuration once. Complex settings are also edited there.
5. If MAS should manage the game lifecycle, enable **Game Configuration** and select the official Wuthering Waves `launcher.exe`.
6. Add a user, choose a configuration source, and decide whether to enable task configuration takeover.
7. Save the script and add it to the schedule.

## The Three Configuration Sources

The user editor offers **Script**, **User**, and **Direct control**:

| Source | Choose it when | Meaning |
| --- | --- | --- |
| **Script** | Most users have the same settings | Uses shared MAS high-frequency settings at script level. |
| **User** | Accounts need different settings | Stores independent MAS high-frequency settings for this user. |
| **Direct control** | OK-WW should own the complex settings | Reads the existing OK-WW configuration first; complex settings are edited in the native OK-WW GUI. |

As a shortcut: choose **Script** for shared settings, **User** for per-account high-frequency settings, and **Direct control** when you want to keep the native OK-WW configuration and let MAS mainly schedule it.

> **Direct control does not mean that MAS never reads or writes files.** It means that MAS prefers the script's existing configuration. When task configuration takeover is enabled, MAS still applies a temporary override to the high-frequency fields for that run.

## Task Configuration Takeover

The **Take over task configuration** switch controls the high-frequency task panel shown below it:

- **Enabled**: regardless of the selected source, the panel overrides the high-frequency fields for the next run.
- **Disabled**: the task uses the complete settings from the selected source. Direct control keeps the full native OK-WW configuration.

Task configuration takeover only covers fields exposed by MAS. It does not replace the full OK-WW configuration. To run entirely from complex settings in the OK-WW GUI, choose **Direct control** and disable this option.

## When Configuration Is Restored

Quick configuration is a **temporary runtime override**. It does not permanently rewrite the original OK-WW configuration:

1. Before each user's task, MAS backs up the original files under OK-WW `working/configs`.
2. During the task, MAS may write the selected user or quick configuration.
3. After success, failure, an exception, or a stop, MAS restores the original files from the start of the task.

This prevents one user's quick settings from affecting the next user and prevents quick settings from being saved permanently in OK-WW. Only changes deliberately saved in the **Configure OK-WW** session through the native GUI become the script's original configuration.

## Official Launcher Requirement

- The OK-WW adaptation uses only official Wuthering Waves resources and the official launcher.
- The game path must be the official `launcher.exe`; **WeGame resources and the WeGame launcher are not supported**.
- If MAS should not manage game startup and shutdown, disable **Game Configuration**. This does not change the official-resource requirement for OK-WW.

## Startup Tasks Currently Supported

The MAS OK-WW entry currently takes over only these tasks:

| Index | Arguments | Description |
| --- | --- | --- |
| 1 | `-t 1 -e` | `DailyTask` |
| 7 | `-t 7 -e` | `MultiAccountDailyTask` |

MAS always appends `-e`, which makes OK-WW exit after the task finishes. For other OK-WW tasks, use the native OK-WW entry instead of selecting an unsupported task index in MAS.

## FAQ

### Where did Simple/Detailed mode go?

The old **Simple/Detailed** names were replaced by **Script/User/Direct control**. Existing configurations are migrated for compatibility; use the new names for new settings.

### Did task configuration takeover overwrite my original OK-WW settings?

No. It only overrides settings during the current task. MAS restores the original configuration after success, failure, an exception, or a stop.

### Which source should I choose?

Choose **Script** for shared settings, **User** for different high-frequency settings per account, and **Direct control** when the complex settings are maintained in the OK-WW GUI. Disable quick configuration if direct control must use the complete native settings.

### Why is my game path rejected?

Select the official Wuthering Waves `launcher.exe`. A WeGame launcher, a WeGame directory, or the game client executable cannot be used as the game path for this adaptation.

### Are OK-WW and OkNte the same script?

No. They are separate ok-script family projects for Wuthering Waves and Neverness to Everness, with independent configuration, tasks, and adaptation logic.
