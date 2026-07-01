---
title: OK-WW Configuration Guide
description: Wuthering Waves - OK-WW specialized adaptation configuration guide
date: 2026-05-31
---

# OK-WW Configuration Guide

## What is OK-WW?

OK-WW, also known as `ok-ww`, is a third-party automation tool for Wuthering Waves. It supports automated daily tasks, Echo farming, roguelike tasks, Forgery Challenges, Nightmare Nest, Simulation Training, Tacet Fields, and more.

Through specialized adaptation, AUTO-MAS can automatically start ok-ww tasks, monitor logs to determine success or failure, and manage the game lifecycle automatically, providing a complete unattended automation workflow.

**For more information, see:**

<Box :items="[
{ name: 'OK-WW GitHub', link: 'https://github.com/ok-oldking/ok-wuthering-waves', image: { light: '/icons/github.svg', dark: '/icons/github-dark.svg', }, },]"/>

## How It Works

AUTO-MAS takes over the full ok-ww runtime flow:

1. **Configuration management**: MAS writes stored configuration files into the ok-ww working directory so the script uses the correct user configuration.
2. **Game management**: optional game lifecycle management, including starting the game before tasks and closing it after tasks.
3. **Task startup**: starts ok-ww with command-line arguments `-t N -e`, specifying the task index and exiting automatically after completion.
4. **Log monitoring**: continuously reads ok-ww logs and judges task status using the principle that errors take priority over success.
5. **Exception handling**: after failure, automatically cleans up processes, writes back configuration, sends notifications, and retries according to the retry limit.

Key differences from general scripts:

| Feature | General Script | OK-WW Specialized Adaptation |
|---|---|---|
| Task arguments | Filled in manually by the user, such as `-t N -e` | Generated automatically from user `TaskIndex` and `ExitOnFinish` |
| Game management | Generic Client/URL/Emulator | Only Client/URL, with built-in `Client-Win64-Shipping.exe` duplicate launch detection |
| Log judgment | Fully depends on user-configured ErrorLog/SuccessLog | Built-in fatal keywords, such as `connected:False` and game update restart, with user configuration as supplement |
| Configuration mode | One configuration file set | Supports `simple` mode shared by users and `detailed` mode per user |
| Process tracking | Optional | Enabled by default, automatically tracking `pythonw.exe` |

## Prerequisites

1. Download [OK-WW](https://github.com/ok-oldking/ok-wuthering-waves) and extract it to any directory.
2. Confirm `ok-ww.exe` can start normally and that basic settings have been completed once in ok-ww itself.
3. Use AUTO-MAS v5.3.0-beta.2 or later.

::: warning Reminder
Do not extract OK-WW into a path containing Chinese characters to avoid unnecessary errors.
:::

## Configure the Script

### Step 1: Add an OK-WW Script

1. Go to **Script Management** and click **New Script**.
2. Select the **ok-ww Script** type in the popup.
3. Click OK. MAS creates an empty ok-ww script instance.

### Step 2: Set Script Path

1. Click the newly created script card and enter the edit page.
2. In **ok-ww path**, click **Select directory** and select the ok-ww **root directory**, the directory containing `ok-ww.exe`, such as `D:\ok-ww`.
3. MAS automatically matches the following paths:

| Auto-matched item | Path rule |
|---|---|
| Script program | `{root}/ok-ww.exe` |
| Configuration directory | `{root}/data/apps/ok-ww/working/configs` |
| Log file | `{root}/data/apps/ok-ww/working/logs/ok-script.log` |
| Tracked process | `{root}/data/apps/ok-ww/python/pythonw.exe` |

### Step 3: Configure Game Management (Optional)

If you want MAS to manage the game client automatically, configure the **Game Configuration** area:

| Configuration | Description | Default |
|---|---|---|
| **Enable game configuration** | Master switch. When disabled, MAS does not start or close the game | Disabled |
| **Start game before task** | MAS starts the game before the task and waits for the configured time | Disabled |
| **Close game after task** | MAS closes the game after successful task completion | Enabled |
| **Game root directory** | Select the `Wuthering Waves Game` root directory; MAS auto-matches the client exe | - |
| **Game launch arguments** | Extra command-line arguments for game startup | Empty |
| **Startup wait time** | Wait time after starting the game, in seconds. Adjust according to machine performance | 60 |

> **Game path auto-match**: after selecting the `Wuthering Waves Game` root directory, such as `D:\Wuthering Waves Game`, MAS automatically joins it as `D:\Wuthering Waves Game\Client\Binaries\Win64\Client-Win64-Shipping.exe`.

> **Duplicate launch prevention**: before starting the game, MAS checks whether `Client-Win64-Shipping.exe` is already running. If it is running, startup is skipped to avoid launching duplicates.

**Relationship between the three switches:**

```text
Enable game configuration (master switch)
  ├─ Start game before task -> MAS starts the game before the task
  └─ Close game after task -> MAS closes the game after success
```

- Master switch off: MAS does not participate in game management.
- Only **Start game before task** enabled: MAS starts the game but does not close it.
- Only **Close game after task** enabled: MAS closes the already running game when the task succeeds.
- Both enabled: MAS fully manages the game lifecycle.

### Step 4: Adjust Runtime Configuration

| Configuration | Description | Default |
|---|---|---|
| **Daily proxy count limit** | Maximum automation runs per user per day. `0` means unlimited. | 0 |
| **Retry count limit** | Maximum retries after task failure | 1 |
| **Proxy timeout limit** | If logs have no new content for this many minutes, the task times out | 60 |

## Configure OK-WW

ok-ww configuration files, including task lists, graphics settings, and key mappings, must be edited through ok-ww itself. MAS provides two ways to manage these configurations.

### Method 1: Simple Mode (Recommended for First Use)

All users share one ok-ww configuration:

1. On the Script Management page, find the ok-ww script card.
2. Click **Configure ok-ww** on the card.
3. MAS opens ok-ww. Complete all configuration inside ok-ww.
4. Return to MAS and click **Save Configuration** to end the session.

> The configuration session times out after 30 minutes and disconnects automatically. Complete configuration and save within this time.

### Method 2: Detailed Mode (Independent Configuration per User)

Each user has an independent ok-ww configuration:

1. Enter the user edit page and set **User configuration mode** to **Detailed**.
2. Click **ok-ww Configuration**.
3. Complete this user's independent configuration in ok-ww.
4. Return to MAS and click **Save Configuration**.

### Configuration Write-Back Timing

After each automation run, MAS decides whether to write back configuration according to `UpdateConfigMode`:

| Mode | Behavior |
|---|---|
| `Always` (default) | Write back after every automation run |
| `Success` | Write back only when the task succeeds |
| `Failure` | Write back only when the task fails |
| `Never` | Never write back |

> The ok-ww script uses `Always` by default, ensuring configuration changes made in ok-ww are synchronized back to MAS.

### Configuration Directory Structure

```text
data/{script ID}/
├── Default/
│   └── ConfigFile/          <- Simple mode: shared by all users
├── {user UID}/
│   └── ConfigFile/          <- Detailed mode: independent per user
└── Temp/                    <- Temporary backup of original script configuration
```

## Configure Users

1. In the script card under **Script Management**, click **Add user** to add a user.
2. Fill in user information according to the hints on the settings card.

### User Configuration Fields

#### Basic Information

| Configuration | Description |
|---|---|
| **Username** | Display name used to distinguish accounts |
| **Enabled status** | Whether the user participates in automation. Disabled users are skipped. |
| **Account** | For official server, enter an 11-digit phone number. Leave empty to skip account switching. |
| **Password** | Login password, stored encrypted. Required for PC account switching. |
| **User configuration mode** | `Simple` shares the ok-ww configuration from the script page; `Detailed` gives each user an independent configuration |
| **Game resource** | Currently only `Official server` is supported |
| **Remaining days** | Remaining valid automation days. `-1` means unlimited. After each successful automation, it decreases by 1. When it reaches 0, the user is skipped. |
| **Notes** | Free-form notes |

::: tip Account Switching Notes

- Account ID is an 11-digit phone number, and the password is used for automatic login on PC.
- If account is left empty, AUTO-MAS uses the currently logged-in account directly and does not switch accounts.
- Passwords are stored locally in encrypted form and are not uploaded to any server.
:::

#### Task Configuration

| Configuration | Description |
|---|---|
| **Startup task (`-t N`)** | Select the ok-ww task to run, corresponding to the `-t` argument |
| **Current launch arguments** | Generated automatically, always `-t N -e` |

> **About the `-e` argument**: MAS always passes `-e` to ok-ww, meaning "exit after completion", so the script process exits normally after the task completes. Users cannot disable this argument.

### Supported Task List

| Index | Argument | Task name | Description |
|---|---|---|---|
| 1 | `-t 1` | DailyTask | Daily task automation |
| 2 | `-t 2` | MultiAccountDailyTask | Run daily tasks in multi-account mode |
| 3 | `-t 3` | FarmEchoTask | Automatically farm Echoes |
| 4 | `-t 4` | AutoRogueTask | Semi-automatic roguelike gameplay |
| 5 | `-t 5` | ForgeryTask | Forgery Challenge |
| 6 | `-t 6` | NightmareNestTask | Nightmare Nest |
| 7 | `-t 7` | SimulationTask | Simulation Training |
| 8 | `-t 8` | TacetTask | Tacet Field |

### Notification Configuration

ok-ww users support the following notification methods when task exceptions occur:

- **Email notification**: fill in recipient email address
- **ServerChan**: fill in SENDKEY
- **Custom Webhook**: supports multiple Webhook URLs

## Runtime Flow

### AutoProxy Lifecycle

Each automation run processes each user as follows:

1. **Pre-check**
   - Check whether the script path exists.
   - Check whether daily proxy count has reached the limit, then skip the user if it has.
   - Check whether remaining days have reached zero, then skip the user if so.
   - Check whether MAS-side configuration files exist, then mark an exception if not.
   - Check whether the game path is valid when game management is required.
2. **MAS starts the game** if enabled:
   - `Client` type: check `Client-Win64-Shipping.exe` -> start exe -> wait `WaitTime` seconds.
   - `URL` type: start through protocol URL.
3. **Configuration delivery**: synchronize MAS-side `ConfigFile` to the ok-ww configuration directory.
4. **Start `ok-ww.exe -t N -e`**.
5. **Log monitoring loop**.
6. **Final handling**: kill process -> write back configuration -> record historical logs -> move to next user or retry.

### Log Judgment Rules

MAS follows the principle that **errors take priority over success**. Checks are performed in this order:

| Priority | Condition | Result |
|---|---|---|
| 1, highest | Log contains `connected:False` | Error: OK-WW did not connect to the game client |
| 2 | Log contains `游戏更新成功, 游戏即将重启` | Error: game update triggered restart |
| 3 | Log matches user-configured `ErrorLog` keywords | Error |
| 4 | Log contains `任务执行完成` or `task completed` | Success |
| 5 | ok-ww process exits unexpectedly | Error |
| 6 | Log timeout, no new content for a long time | Error |

::: warning Note
Built-in keywords, `connected:False` and `游戏更新成功, 游戏即将重启`, cannot be overridden or disabled. User-configured `ErrorLog` is only a supplementary condition.

If the user's `ErrorLog` becomes empty after cleanup because all configured keywords are too broad, the system falls back to the default: `connected:False|游戏更新成功, 游戏即将重启|错误`.
:::

### Game Closing Policy

| Scenario | Close ok-ww | Close game |
|---|---|---|
| Automation succeeds and **Close game after task** is enabled | Yes | Yes |
| Automation succeeds and **Close game after task** is disabled | Yes | No |
| Automation fails, game startup fails, or exception occurs | Yes | Yes, for clean retry |

## FAQ

### The app says "Please set ok-ww script path" after starting automation

**Cause**: the script root directory is not set correctly, or `ok-ww.exe` cannot be found in the directory.

**Solution**: enter the script edit page, click **Select directory**, and select the **root directory containing `ok-ww.exe`**, not `ok-ww.exe` itself. MAS automatically matches all subpaths after selection.

### The app says "shared OK-WW configuration file not found" or "user OK-WW configuration file not found"

**Cause**: the ok-ww configuration session has not been completed in MAS.

**Solution**:

- Simple mode: return to the Script Management page, click **Configure ok-ww** on the ok-ww script card, complete configuration, and save.
- Detailed mode: enter the user edit page, click **ok-ww Configuration**, complete configuration, and save.

### The game starts but ok-ww cannot connect to it

**Cause**: the game needs time to load after startup, and the wait time is insufficient.

**Solution**: increase **Startup wait time** on the script edit page. For SSDs, 60-80 seconds is recommended. For HDDs, 120 seconds or more is recommended. Adjust according to your machine.

### Logs show "OK-WW did not connect to game client" (`connected:False`)

Possible causes:

- The game has not fully loaded. Increase **Startup wait time**.
- The ok-ww version is incompatible with the current game version. Update ok-ww.
- The game is blocked by security software. Check antivirus logs.
- The PC client is not the official server client. Confirm the correct game resource is selected.

### Tasks are interrupted automatically after a game update

**Cause**: ok-ww detected a game version update and restarted the game. MAS treats this as a fatal interruption and marks the current user as abnormal.

**Behavior**: this is expected. On game update days, usually maintenance days, manually log in and complete game updates outside automation time to avoid triggering this during automation.

### The scheduler frequently logs "client already running, skip startup"

**Cause**: before starting the game, MAS checks whether `Client-Win64-Shipping.exe` is already running to avoid duplicate launches.

**Behavior**: this is normal. If you manually started the game or a previous automation run started it, MAS uses the existing process. If you want every automation run to start the game fresh, close the game manually before automation starts.

### Task failure is misjudged

**Cause**: normal logs contain keywords matching `ErrorLog`.

**Solution**: check `ErrorLog` configuration. Default is `connected:False|游戏更新成功, 游戏即将重启|错误`. If your normal ok-ww logs contain the word `错误`, remove that keyword from `ErrorLog`. Do not remove the two built-in keywords; they are checked by code, but keeping them in configuration also works.

### Multi-user automation only executed the first user

**Cause**: this was fixed in v5.3.0-beta.3, which fixed AutoProxy executing only the last user in multi-user mode.

**Solution**: update AUTO-MAS to the latest version.

### MAS-side configuration is lost or overwritten after saving

**Cause**: when saving the configuration session, the source configuration did not exist, causing the MAS-side copy to be cleared. This was fixed in v5.3.0-beta.3.

**Solution**: update to the latest version. Before saving configuration, make sure configuration has been saved in ok-ww and that configuration files have been generated.

::: tip General Advice

- Set startup wait time conservatively, usually 60-120 seconds. Waiting slightly longer is better than setting it too short.
- On game update days, manually complete updates before starting automation.
- When troubleshooting, check scheduler logs first. They include game configuration summaries and detailed process logs.
- Join the official AUTO-MAS community for help: QQ group `957750551` or Telegram `@AUTO_MAS_top`.
:::
