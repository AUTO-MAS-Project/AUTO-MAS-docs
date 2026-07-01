---
title: General Scheduling
description: General scheduling in AUTO-MAS
date: 2025-07-16
---

# General Scheduling

::: warning Before You Start
General scheduling has a learning curve. To configure a general script on your own, you need to understand the behavior of the script itself.

If you do not know much about the script you want to schedule, you can import a configuration shared by another user. However, general scheduling cannot guarantee the same stability as a dedicated script integration. Do not blame configuration contributors too harshly.

If you decide to use this feature, read this document carefully before asking questions.
:::

::: tip Tip

AUTO-MAS already includes many ready-to-use templates. You can configure them through **New General Script > Create from Template** and only need to fill in the script software path.

Currently, mature templates are available for common scripts such as March7thAssistant, SRC, zzzOD, and M9A.

If you run into problems, join the user group and discuss them with developers and template contributors.

:::

## Scheduling Model

Before using this feature, understand the basic mechanism of **general scheduling**. This makes later troubleshooting much easier.

### Configuration Management

AUTO-MAS manages configuration by directly saving script configuration files or folders. Before a task starts, the corresponding configuration files are imported into the target script location as-is. After the task ends, the original configuration files inside the script are restored.

### Script Monitoring

AUTO-MAS determines script status through **log text**, **log timestamps**, and **whether the script process has ended**. The logic is:

- Success: If **task success logs** are configured and any success log text appears in the **log text**, the task is considered successful. If **task success logs** are empty, and no **task error log** appears in the **log text** when the **script process ends**, the task is considered successful.
- Failure: If the last **log timestamp** exceeds the **auto-proxy timeout limit**, the task is considered failed due to timeout. If a **task error log** appears before a **task success log**, the task is considered failed. If **task success logs** are configured, but no success log appears in the **log text** when the **script process ends**, the task is considered failed.

Using MAA as an example:
![AUTO workflow](/docs/img/AUTO工作逻辑.png)

## Script Settings

To let AUTO-MAS schedule a general script correctly, users need to configure script properties accurately. These settings directly affect automation stability.

### Script Root Directory

- **Type**: folder

- **Description**: This setting helps users relocate the script program. When the script program location changes, you only need to reset the root directory, and other paths will update automatically.

### Script Path

- **Type**: executable file

- **Description**: The script's main program. Both script configuration and task execution require this file.

- **Common issues**:

  - **The program says the selected path is not under the script root directory**: this means exactly what it says. Check the script root directory.
  - **The script cannot start during configuration or auto-proxy**: check whether the path and launch arguments are correct. Specific errors can be found in `debug/AUTO-MAS.log`.

### Script Launch Arguments

- **Type**: text segments separated by `|`, `%`, and spaces

- **Description**: This setting adds extra commands when starting a script task. Some scripts do not provide a `run immediately after startup` option in the UI, but support the same behavior through command-line arguments. For those scripts, configure this item so the script runs its task after startup.

- **How to configure**: Read the script software's official website or documentation, find sections such as **CLI run**, **command-line startup**, or similar, then fill in the required arguments. Make sure the script can run its task automatically with the entered launch arguments. If the arguments for running a task are different from those for configuring the script, enter both and separate them with `|`. If the executable used for running a task is different from the executable used for configuration, enter the executable path relative to the `script path` before the corresponding arguments, separated by `%`.

- **Format**: `{auto-proxy executable path relative to script path}%{auto-proxy task arguments}|{configuration executable path relative to script path}%{configuration task arguments}`

### Track Script Child Processes

- **Type**: toggle

- **Description**: Determines whether child processes are considered when judging **whether the script process has ended**. Some scripts must be opened through a **launcher**. After the main program starts, the launcher exits automatically, so the launcher process alone cannot represent whether the whole script is still running. Enable this option for such scripts.

- **Common issues**:

  - **After manually closing the script, the app does not detect that the script has closed**: try disabling this option.
  - **The script is still running, but the app incorrectly reports that it has exited**: try enabling this option.

### Script Configuration File Path

- **Type**: any file or folder

- **Description**: The file or folder where the script stores configuration.

- **How to configure**: Open the script directory. Usually there is a file or folder named `config`; this is very likely the **script configuration file**.

### Script Log File Path

- **Type**: any file

- **Description**: The file where the script stores logs.

- **How to configure**: Open the script directory and check whether a folder named `debug`, `log`, or similar exists:
  - If it exists, enter that folder and look for files whose names do not include date information, such as `log.txt` or `gui.log`:
    - If such a file exists, select it.
    - If not, select any file that stores log information, usually with a `.log` or `.txt` extension, then configure **script log file name format**.
  - If no such folder exists, check whether the script root contains `.txt` or `.log` files. If so, open them and confirm whether they contain script logs, then select the correct file.

### Script Log File Name Format

- **Type**: text indicating a date/time format

- **Description**: Indicates the naming format of real-time log files. Some scripts do not write logs to one fixed file, but write them to different files by date. For these scripts, configure the log file name format so AUTO-MAS can locate the actual log file.

- **How to configure**: Copy any script log file name into this field, then replace date and time elements with the corresponding symbols according to the [common date/time format symbol reference](/en/docs/advanced-features/#common-date-time-format-symbol-reference). Example: `2019-05-01` -> `%Y-%m-%d`.

### Script Log Timestamp Start/End Position

- **Type**: number

- **Description**: Locates the start and end positions of the timestamp in each log line so the app can parse it.

- **How to configure**: Find any log line with a timestamp. Count from `1` to the first character of the timestamp; that number is the start value. Continue counting to the last character of the timestamp; that number is the end value. For example, in `[2025-06-29 20:00:35.909][INF] <1><> Start task`, the start value is `2` and the end value is `24`.

### Script Log Time Format

- **Type**: text indicating a date/time format

- **Description**: Indicates the format of log timestamps so the app can parse them.

- **How to configure**: Copy any script log timestamp into this field, then replace date and time elements with the corresponding symbols according to the [common date/time format symbol reference](/en/docs/advanced-features/#common-date-time-format-symbol-reference). Example: `2019-05-01 16:00:00.000` -> `%Y-%m-%d %H:%M:%S.%f`.

### Script Success/Failure Logs

- **Type**: text segments separated by `|`

- **Description**: Reference text for judging script runtime status. Multiple entries are supported and separated by `|`.

- **How to configure**: Customize this based on your experience and the script log content.

## Configuration Management

Because general scheduling has a learning curve, general scripts support quick configuration import and export. You can export configuration to a `JSON file` and share it with other users, import a `JSON file` shared by another user, or upload your configuration to the `AUTO-MAS Configuration Sharing Center`. After review, all users can import it with one click.

::: warning Note
- To prevent privacy leaks, the script root directory will be replaced with `C:/ScriptRoot`. Users must reselect it after importing.
- `Game/emulator paths` are not replaced automatically. Check whether these paths may expose personal information.
:::

## Subordinate Users

**Subordinate users** work the same way as **subordinate users** in MAA scripts. Each sub-configuration runs similarly to the detailed mode of MAA user configuration. Each sub-configuration must be configured separately, using the same method as MAA configuration.
