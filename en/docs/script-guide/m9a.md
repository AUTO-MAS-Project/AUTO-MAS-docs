# M9A Configuration Guide

::: tip Tip
M9A adaptation is still under development. Bugs may exist during this period. Keep your original general script configuration while using the dedicated script.
:::

## What is M9A?

M9A is a third-party tool for Reverse: 1999. It can handle repetitive tasks such as daily automation, automatic Artificial Somnambulism, and event farming.

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
2. In the opened script configuration, click **Select folder** for **M9A path**, then open the directory where M9A is located.
3. In **Emulator Management**, select the emulator and emulator instance.

> If no emulator appears here, complete **Emulator Management** configuration first.

### Runtime Configuration

The M9A script provides the following runtime control parameters:

| Configuration | Description | Default |
|---------------|-------------|---------|
| Proxy count limit | Maximum automation runs per user per day. `0` means unlimited. | 0 |
| Run count limit | Maximum retry count when a task fails unexpectedly | 3 |
| Runtime limit | Maximum runtime for a single task in minutes. Timeout forces termination. | 10 |
| Auto update after queue ends | After batch tasks complete, automatically update M9A resources if a new version is detected | Disabled |

> Tip: After enabling "Auto update after queue ends", AUTO-MAS starts a virtual user after all real user tasks are completed to update resources. Desktop and Webhook notifications are sent after successful update.
>
> Important: Before using auto update, manually open M9A and separately enable a resource update channel in M9A settings, either MirrorChyan or GitHub. Otherwise auto update cannot work correctly.

## Preparation Before First Run

::: warning Important
Before using M9A in AUTO-MAS for the first time, manually start M9A once and complete the following initialization steps.
:::

1. Manually start the M9A main program, `M9A.exe`.
2. Wait for M9A initialization to complete. After logs show "AgentServer started", wait until "all tasks completed" appears.
3. In M9A settings:
   - Configure **resource download source**, choosing MirrorChyan or GitHub.
   - Fill in **CDK** or **Token** for resource updates.
4. Decide in M9A whether to enable **auto update** and choose the **update channel**.
5. Close M9A after confirmation.

After completing these steps, return to AUTO-MAS and click **Save Configuration**.

## Configure Users

1. In the script table under **Script Management**, click **Add user** to add a user.
2. Fill in user information according to the hints on the settings card.
3. You can add multiple users. AUTO-MAS runs each user's task queue sequentially according to the user list.

### Task Queue Configuration

M9A supported tasks include, depending on the actual software version:

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

On the user configuration page, select tasks from the task list, add them to the task queue, and adjust execution order.

### Preset Template

When the task queue is empty, the system shows the **Daily - Idle** preset template. One click adds common tasks to the queue.

The **Daily - Idle** template includes the following tasks, suitable for daily farming when no event is active or the event shop has been cleared:

| Task | Description |
|------|-------------|
| Collect Wilderness | Collect Wilderness resources |
| Daily Psychube, Insight Analysis | Automatically complete insight analysis |
| Regular Battle | Daily stage battles |
| Auto Artificial Somnambulism | Automatically complete Artificial Somnambulism challenges |
| Auto Anecdote | Automatically complete Anecdote |
| Bank Shopping | Automatically shop in the bank |
| Claim Rewards | Automatically claim various rewards |
| Use Redemption Code | Automatically use redemption codes |

> Tip: Preset templates are only helper entries for quickly adding tasks. You can still manually build task queues through **Add task**, or modify tasks after adding the preset. If some tasks do not have matching script definitions, they are skipped automatically during one-click add.

### Account Switching

M9A supports automatic account switching for multi-account management:

1. Fill in the target account in the **Account information** field on the user configuration page. This works only for the official server.
2. When server resource is **Official server** and account information is provided, AUTO-MAS automatically inserts a **Switch account** task at the beginning of the task queue.
3. The account switching task runs after **Start game** and before user-defined tasks.

> Tip: If account switching is not needed, leave account information empty. Other servers do not support account switching for now.

### Known Limitations

- Official server: supported, and it is the only server supporting account switching.
- Bilibili server: supported, but account switching is not supported due to M9A limitations.
- Other servers: supported, but account switching is not supported due to M9A limitations.
- MuMu emulator: supported.
- LDPlayer: supported.
- General emulator: untested.
- MXU GUI: not supported. M9A adaptation supports only MFAAvalonia.

## Configuration Notes

The following describes AUTO-MAS configuration behavior for M9A in **auto-proxy** mode.

1. Configuration items shown on the user configuration page take priority.
2. AUTO-MAS automatically builds M9A instance configuration files according to the configured task queue.
3. **Task queue auto-build rules**:
   - Automatically add **Start game** at the beginning of the queue.
   - If the server is official and account information is provided, automatically insert **Switch account** after **Start game**.
   - Automatically filter user-added **Start game**, **Close game**, and **Switch account** tasks to avoid duplication.
   - Automatically add **Close game** at the end of the queue.
   - Automatically skip standalone tasks marked as `standalone`.
   - Final execution order: `Start game -> [Switch account] -> User-defined tasks -> Close game`.
4. **Configuration safety**:
   - AUTO-MAS automatically backs up the entire M9A `config` directory before running.
   - During execution, only `config/instances/default.json` is modified.
   - Your global `config.json` is not modified.
   - Original configuration is restored after the task ends.

## Configuration Isolation

### How It Works

AUTO-MAS implements a complete configuration isolation mechanism to keep your original M9A configuration safe:

1. **Backup before run**: back up the entire `config` directory to a temporary path.
2. **Runtime isolation**: modify only the instance configuration file while keeping global configuration unchanged.
3. **Restore after run**: fully restore the original configuration state.

### Benefits

- Configuration safety: no need to worry about configuration pollution.
- Automatic restoration: every run starts from a clean state.
- Debug-friendly: historical configuration backups are saved automatically under `data/script_id/test*.json`.

## M9A Auto Update

### Feature Description

When **Auto update after queue ends** is enabled, AUTO-MAS automatically detects and updates M9A resources after all real user tasks complete:

1. **Version detection**: during the first user run, AUTO-MAS monitors M9A logs and detects whether a new version is available.
2. **Virtual user update**: if a new version is detected, the system starts a virtual user without connecting an emulator, used only for resource update.
3. **Update monitoring**: the update process is monitored in real time for network interruption, HTTP errors, timeout, and other exceptions.
4. **Notifications**: after update succeeds or fails, desktop and Webhook notifications are sent.

### Update Failure Handling

If update fails, the system:

- Records detailed error logs under `data/script_id/`.
- Sends a notification containing the failure reason, such as network interruption or HTTP request failure.
- Keeps the current version and does not affect the next normal run.

### Notes

- M9A automatically restarts during update. This is normal.
- Update timeout is 10 minutes.
- Enable auto update only in a stable network environment.

## FAQ

### Q: Can I add multiple users under one script?

A: Check your M9A version first. Newer M9A versions support specified account switching, so you can build account switching tasks based on newer M9A. AUTO-MAS supports managing multiple users under one script and runs each user's task queue sequentially.

### Q: Which emulators are supported?

A: **MuMu emulator** and **LDPlayer** have been tested. Other emulators are untested and may have compatibility issues.

### Q: Is the MXU GUI supported?

A: No. M9A adaptation supports only the **MFAAvalonia** GUI.

### Q: Will my M9A configuration be modified?

A: No. AUTO-MAS only modifies `config/instances/default.json` and fully restores your original configuration after the task ends.

### Q: How do I view previously run configurations for debugging?

A: Each run configuration is saved to `data/script_id/test*.json`, keeping up to the latest 5 backups.

### Q: What should I do if task execution fails?

1. Check emulator connection status during runtime.
2. Check log files to analyze the error reason.
3. Compare historical configurations under `data/script_id/`.

### Q: Do I need to manually add "Start game" and "Close game" to the task queue?

A: No. AUTO-MAS automatically adds **Start game** at the beginning and **Close game** at the end. If you add these tasks manually, the system filters them to avoid duplicate execution.

### Q: How does "Auto update after queue ends" work?

A: After it is enabled, the system checks whether M9A has a new version after all real user tasks complete. If a new version exists, a virtual user is started, without connecting an emulator, to download and apply the latest M9A resource package. A notification is sent after update completes.

### Q: Why did my proxy count not increase?

A: Proxy count uses the date in the UTC+4 timezone. If the day's count has already reached the **proxy count limit** in script configuration, later users are skipped. The counter resets automatically on the first automation run each day.
