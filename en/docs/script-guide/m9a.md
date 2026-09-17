# M9A Configuration Guide

::: tip Tip
M9A support is still under development, so bugs are possible. Keep your existing general script configuration around while you try the dedicated script.
:::

## What is M9A?

M9A is a third-party tool for Reverse: 1999. It handles repetitive work such as daily automation, Artificial Somnambulism, and event farming.

It is powered by [MaaFramework](https://github.com/MaaXYZ/MaaFramework) image recognition technology.

**For more information, see:**

<Box :items="[
{ name: 'M9A GitHub', link: 'https://github.com/MAA1999/M9A', image: { light: '/icons/github.svg', dark: '/icons/github-dark.svg', }, },
{ name: 'M9A Documentation', link: 'https://1999.fan/', image: 'https://1999.fan/images/m9a-logo_256x256.png', },]"/>

## Install M9A

1. Download the archive from <Pill name="M9A Repository" :image="{ light: '/icons/github.svg', dark: '/icons/github-dark.svg', }" link="https://github.com/MAA1999/M9A/releases/latest"/> or <Pill name="MirrorChyan" image="https://mirrorchyan.com/favicon.ico" link="https://mirrorchyan.com/zh/projects?rid=M9A&scouce=AUTO-MAS-Web"/>.
2. Extract the M9A archive to any folder. A path such as `D:\M9A` is recommended.

::: warning Note

- Make sure [VCRedist x64](https://aka.ms/vs/17/release/vc_redist.x64.exe) is installed.
- Make sure [.NET 10 Desktop Runtime](https://dotnet.microsoft.com/en-us/download/dotnet/10.0) is installed.
:::

## Configure the Script

1. Go to **Script Management**, click **New Script**, and select **M9A Script** to add a script instance management page.
2. In the script configuration that opens, click **Select folder** for **M9A path** and open the directory M9A is in.
3. In **Emulator Management**, select the emulator and the emulator instance.

> If no emulator appears here, complete **Emulator Management** configuration first.

### Runtime Configuration

| Configuration | Description | Default |
|---------------|-------------|---------|
| Proxy count limit | Maximum automation runs per user per day. `0` means unlimited | 0 |
| Run count limit | How many times a failed task is retried | 3 |
| Runtime limit | Maximum minutes for a single task. It is forced to stop on timeout | 10 |
| Auto update after queue ends | After the queue finishes, update M9A resources if a new version is detected | Disabled |

::: warning Auto update needs an update channel enabled inside M9A first
**Auto update after queue ends** relies on M9A's own update feature. Open M9A by hand and **enable a resource update channel** in its settings, either MirrorChyan or GitHub. Without that, this switch does nothing.
:::

## Required Before the First Run

Before you use M9A from AUTO-MAS the first time, you **must open M9A by hand once** so it can initialize itself:

1. Launch `M9A.exe`.
2. Wait for initialization to finish. The log shows "AgentServer started", then wait for "all tasks completed".
3. In M9A settings, configure the **resource download source** (MirrorChyan or GitHub) and the matching **CDK / Token**.
4. While you are there, decide whether to enable M9A's own **auto update**. That is up to you.
5. Close M9A.

Back in AUTO-MAS, click **Save Configuration** and you are ready to go.

## Configure Users

1. In the script table under **Script Management**, click **Add user** to add a user.
2. Fill in the user information following the hints on the settings card.
3. You can add several users. AUTO-MAS runs each user's task queue in the order they appear in the list.

### Task Queue Configuration

M9A supports the following tasks, depending on the version you have:

| Task | Description |
| ---- | ----------- |
| Collect Wilderness | Collect Wilderness resources |
| Daily Psychube, Insight Analysis | Automatically complete insight analysis |
| Regular Battle | Daily stage battles |
| Event Farming | Automatically farm event stages |
| Auto Artificial Somnambulism | Automatically complete Artificial Somnambulism challenges |
| Auto Anecdote | Automatically complete Anecdote |
| Bank Shopping | Automatically shop in the bank |
| Claim Rewards | Automatically claim various rewards |

On the user configuration page, pick the tasks you want from the task list, add them to the task queue, and adjust the execution order.

### Preset Template

Do not feel like adding tasks one at a time? While the task queue is empty, a **Daily - Idle** template appears. One click adds the common tasks: Collect Wilderness, Daily Psychube (Insight Analysis), Regular Battle, Auto Artificial Somnambulism, Auto Anecdote, Bank Shopping, Claim Rewards, and Use Redemption Code. It suits ordinary days with no event running, or when you have already cleared the event shop.

You can still add, remove, and reorder tasks afterwards. Any task in the template that your M9A version does not have is skipped automatically.

### Automatic Account Switching

**Only the official server supports this.** Other servers cannot, because of an M9A limitation.

Fill in the target account under **Account information** on the user configuration page and you are done. AUTO-MAS then inserts a **Switch account** task at the front of the queue, after Start game and before your own tasks. Leave it empty if you do not need to switch accounts.

### What Is Supported

| | Status |
|---|---|
| Official server | Supported, and the only server where accounts switch automatically |
| Bilibili and other servers | Supported, but no automatic account switching (an M9A limitation) |
| MuMu emulator / LDPlayer | Supported |
| Other emulators | Untested, may have problems |
| MXU GUI | Not supported. Only MFAAvalonia is supported |

## Will Your M9A Configuration Get Wrecked?

No. Before running, AUTO-MAS backs up M9A's whole `config` directory. While running, it touches exactly one file, the instance configuration at `config/instances/default.json`. Your global `config.json` is never modified. When the run ends, the original configuration is put back.

If you want to compare configurations afterwards, every run's actual configuration is kept at `data/script_id/test*.json`, with the last 5 retained.

### How the Task Queue Is Built

The tasks you arrange in the UI are not handed to M9A as-is. AUTO-MAS fills in both ends:

```text
Start game -> [Switch account] -> your tasks -> Close game
```

So two things are not your problem:

- **Do not add Start game and Close game yourself.** They are added for you. If you add them manually, they get filtered out, so nothing runs twice.
- **Do not add Switch account either.** It is inserted automatically on the official server when account information is filled in.

## How Auto Update Works

With **Auto update after queue ends** enabled, the sequence goes like this. While the first user runs, AUTO-MAS glances at the M9A log for a new-version notice. If there is one, it waits until every user has finished, then does one separate update run, with no emulator connected, purely to update resources. When that is done, it sends you a notification with the result.

- M9A restarts itself during the update. That is normal.
- The update waits at most 10 minutes.
- Do not enable this on a flaky connection. A failure does not affect your next run, but you waited for nothing. The reason is written to the `data/script_id/` directory and included in the notification.

## FAQ

### Can I add multiple users under one script?

Yes. AUTO-MAS runs each user's task queue in list order. Automatic account switching needs a newer M9A version, the kind that supports switching to a specified account, and works only on the official server.

### A task failed. How do I investigate?

In order: check whether the emulator is connected, then look through the log for the error, and if that is still unclear, open `data/script_id/` and compare the configuration this run actually used against the last successful one.

### Why isn't the proxy count going up?

**The count uses dates in the UTC+4 timezone**, so it can be several hours off from your computer's date. The rollover point is not your local midnight.

Also, once the day's count reaches the **proxy count limit**, later users are skipped outright. The counter resets on the day's first automation run.
