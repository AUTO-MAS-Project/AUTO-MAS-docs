# Managed Scripts

Find the game you play below, open its guide, and follow along.

Your game is not in the list? See [General Scheduling](/en/docs/script-guide/general). As long as the script can start running a task on startup and writes a log, AUTO-MAS can manage it.

## Desktop Session compatibility {#desktop-session}

Desktop Session is an experimental feature for Windows x64 desktop builds. The table reflects the development build at commit `be8ef6d5c`. Run emulator games and their managed scripts in the main session.

A queue can run in Desktop Session if every managed script meets the conditions below and its completion action is set to **Do nothing**. See the [Desktop Session guide](/en/docs/advanced-features/desktop-session) for startup and troubleshooting.

| Managed script | Desktop Session | Conditions and notes |
| --- | --- | --- |
| [MAA](/en/docs/script-guide/maa#desktop-session) | Unsupported | Run in the main session. |
| [MaaEnd](/en/docs/script-guide/maaend#desktop-session) | Conditional | Use a PC (Win32) controller. Run emulator tasks in the main session. |
| SRC | Unsupported | Run in the main session. |
| [M9A](/en/docs/script-guide/m9a#desktop-session) | Depends on the MFW controller | Run the usual ADB configuration in the main session. |
| [MFW projects](/en/docs/script-guide/maafw#desktop-session) | Conditional | Use a Win32 controller. Run ADB and other controllers in the main session. |
| [General scripts](/en/docs/script-guide/general#desktop-session) | Conditional | Use it for PC tasks only. If MAS launches the game, use an executable or URL. Keep emulator tasks in the main session even if the script starts the emulator itself. |
| [ok-ww](/en/docs/script-guide/okww#desktop-session) | Supported | For the Wuthering Waves PC client. |
| ok-nte | Supported | For the Neverness to Everness PC client. |
| [HSR](/en/docs/script-guide/hsr#desktop-session) | Supported | Use March7thAssistant or SRA with the PC client. |
| [BetterGI](/en/docs/script-guide/bettergi#desktop-session) | Conditional | Turn off update takeover. If BetterGI is configured to run with administrator privileges, start AUTO-MAS as administrator. |
| Zenless Zone Zero One Dragon (ZZZ-OD) | Conditional | Start AUTO-MAS as administrator. The script's post-task shutdown is skipped in the desktop session. |
| BAAH | Unsupported | Run in the main session. |
| Whimbox | Conditional | Run AUTO-MAS as administrator. Requires the Desktop Session plugin API and Whimbox Adapter 0.1.2 or later. |
| MSS | Unsupported | Run in the main session. |

Windows verification used the same 0.1.2 wheel and the plugin manager to upgrade from 0.1.1 on two SDK 5.9.0b1 hosts. The host with the Desktop Session API supports session selection. The older host runs Whimbox in the main session and preserves the saved selection for a later host upgrade.

On Windows 11, the elevated AUTO-MAS 5.9.0-beta.1 host (commit `46d8a148f`) ran adapter 0.1.2 with Whimbox App 2.0.6 and backend 3.1.5. Dispatching a single script or a queue from Desktop Session started the Whimbox backend in the child session. Starting a single script after restarting AUTO-MAS also used its saved session. A same-name process on the main desktop survived both task cancellation and disconnection. The game was not installed for this test.

## Guide Index

### [MAA](/en/docs/script-guide/maa)

Arknights - MaaAssistantArknights

- Supports all Annihilation and daily automation tasks
- Supports multi-account management for official and Bilibili servers
- Supports stage configuration through weekly plans

### [MaaEnd](/en/docs/script-guide/maaend)

Arknights: Endfield - MaaEnd Assistant

- Supports quick Protocol Space adjustments and automatic check-ins
- Supports PC and emulator control, with PC recommended
- Supports stage configuration through weekly plans, currently in beta

---

### [M9A](/en/docs/script-guide/m9a)

Reverse: 1999 - M9A

- Supports daily automation, event farming, automatic Artificial Somnambulism, and more
- Supports MuMu and LDPlayer emulators
- Added as an MFW script: the directory is recognised automatically, start / close game and official-server account switching are inserted for you

---

### [MFW Projects](/en/docs/script-guide/maafw)

Any MaaFramework project that ships an `interface.json`

- Runs the task queue straight from the interface without starting the project's own UI
- Runs on AUTO-MAS's own copy: the original directory is never touched and can be deleted after import; one project can back several managed scripts running at the same time
- Project updates from GitHub or MirrorChyan, automatically before or after a run

---

### [OK-WW](/en/docs/script-guide/okww)

Wuthering Waves - OK-WW

- Currently supports daily tasks and multi-account dailies
- AUTO-MAS can start and close the game for you
- Complex settings stay in OK-WW's own UI; AUTO-MAS only takes over the high-frequency ones
- Supports the official launcher only, not WeGame

---

### [HSR](/en/docs/script-guide/hsr)

Honkai: Star Rail - March7thAssistant (M7A) + StarRailAssistant (SRA)

- You can mix both scripts and decide which one handles which task
- Covers Trailblaze Power, reward collection, Divergent Universe, and Currency Wars
- Retries failed tasks automatically and will not break your existing script config

---

### [BetterGI](/en/docs/script-guide/bettergi)

Genshin Impact - BetterGI

- Native GUI configuration; accounts are managed natively by BetterGI, complex settings are done in BetterGI's interface
- Manages the One Dragon built-in config groups (8 toggles) and custom config groups independently per account
- Supports reward party, combat party, and auto-battle strategy fields, auto-reading combat scripts under `User/AutoFight`
- Account switching, notifications, and extra scripts can each be configured independently

---

### [General Scheduling](/en/docs/script-guide/general)

For scripts that can run tasks on startup and print logs

- Supports most mainstream scripts, including March7thAssistant, SRC, zzzOD, and M9A
- Ready-made config templates you can use straight away
- Supports flexible custom managed-script configurations

---

### [March7thAssistant](/en/docs/script-guide/march7th)

Honkai: Star Rail - March7thAssistant

- Under development
- Can be used together with automatic login scripts

---

## Reading Recommendations

- **New users**: start with the [MAA guide](/en/docs/script-guide/maa) if you play Arknights
- **Wuthering Waves players**: read the [OK-WW guide](/en/docs/script-guide/okww)
- **Reverse: 1999 players**: read the [M9A guide](/en/docs/script-guide/m9a)
- **Other MaaFramework projects**: read the [MFW project guide](/en/docs/script-guide/maafw)
- **Honkai: Star Rail players**: read the [HSR guide](/en/docs/script-guide/hsr)
- **Genshin Impact players**: read the [BetterGI guide](/en/docs/script-guide/bettergi)
- **Other games**: read [General Scheduling](/en/docs/script-guide/general) and use an existing template
- **Advanced users**: learn the configuration management model in [General Scheduling](/en/docs/script-guide/general) and customize your scheduling plan
