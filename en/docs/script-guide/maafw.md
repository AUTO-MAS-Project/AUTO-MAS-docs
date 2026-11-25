<!-- markdownlint-disable MD033 -->
# MFW Project Guide

::: info Version and screenshots
Text was checked against **the dev version following v5.8.0-beta.3**. Older screenshots may show the Chinese UI or an earlier layout. After import, the project directory is locked; the old embedded-copy status table and change-directory workflow are no longer part of the current page.
:::

::: tip Tip
A [MaaFramework](https://github.com/MaaXYZ/MaaFramework) project with an `interface.json` can be added to AUTO-MAS as an **MFW script**. The project's own UI might be MFAAvalonia, MXU, MFW-PyQt6, or MaaPiCli; AUTO-MAS does not launch that UI layer at run time. Projects such as [M9A](/en/docs/script-guide/m9a) are automatically recognized as their project-specific flavor after import.
:::

## What is an MFW Script?

Projects in the MaaFramework ecosystem share the same structure: an `interface.json` file declaring controllers, resources, and tasks, a `resource/` directory holding recognition assets and pipelines, sometimes an Agent (a Python script or executable) for complex logic, and a UI wrapper on top.

<details>

<summary style="color:#3b82f6">Quickly recognize MFW projects by UI (beginner-friendly)</summary>

If your script looks like one of the UIs below when opened, it is very likely built on MaaFramework:

[MFAAvalonia](https://github.com/MaaXYZ/MFAAvalonia):

![MFAAvalonia](/docs/img/script-guide/maafw/maafw_UI/MFAAvalonia.png)

[MFW-PyQt6](https://github.com/overflow65537/MFW-PyQt6):

![MFW-PyQt6](/docs/img/script-guide/maafw/maafw_UI/CFA.png)

[MXU](https://github.com/MistEO/MXU):

![MXU](/docs/img/script-guide/maafw/maafw_UI/MXU.png)

If your script uses one of these three interfaces, it is most likely a MaaFramework project.

</details>

AUTO-MAS's MFW script reads `interface.json` directly, uses the project's resources and runtime environment, and executes the task queue you arrange. It provides a generic integration layer; whether a specific project runs smoothly still depends on whether its release package, controllers, and Agent are compatible with your computer.

Projects such as MaaEnd, Senti (识宝), M9A, and MPA are all listed in the [MaaFramework application directory](https://github.com/MaaXYZ/MaaFramework#%E5%BA%94%E7%94%A8%E7%A8%8B%E5%BA%8F) and [MFW community projects](https://maafw.com/community/projects#%E5%BA%94%E7%94%A8%E7%A8%8B%E5%BA%8F).

<details>
<summary style="color:#3b82f6">MaaFramework ecosystem project list</summary>

Updated 2026-10-20 21:39:36

![MaaFramework ecosystem project list](/docs/img/script-guide/maafw/社区项目长图.png)

</details>

Below we walk through the setup using [M9A](https://github.com/MAA1999/M9A) as an example. Because pages evolve with development, **documentation may lag slightly behind—refer to the version you are running**.

## Before you start

### Download and extract

1. Download a release package matching your OS architecture from the project's release page (**99% of users need Windows x64**). If the project supports [MirrorChyan](https://mirrorchyan.com/), you can also download from its mirror.
2. Extract it to any folder.
3. Make sure the folder you plan to select in AUTO-MAS **directly contains** `interface.json` (not wrapped inside another subfolder).
   Taking M9A as an example, if you see `interface.json` inside the folder, you have the right directory:

![interface.json inside an M9A project directory](/docs/img/script-guide/maafw/M9A目录中的interface示例.png)

::: warning Note
You usually do **not** need to open the project's own UI first, though it causes no harm if you already have. Some Agents perform additional setup on first run; follow the project's release notes if they do.
:::

## Add the managed script

### Import in AUTO-MAS

Continuing with M9A as an example:

1. Open **Managed scripts**, click **New managed script**, and choose **MaaFW** (selecting **M9A** directly also works; here we pick generic MFW to demonstrate automatic flavor recognition).

   ![Select MFW script in New managed script](/docs/img/script-guide/maafw/在新建脚本窗口选择%20MFW%20脚本.png)

2. Choose a project source: select **Create another project** if this is your first time using this project, or reuse an already-imported project from the list below. Reusing shares the managed project files to save disk space while keeping accounts and run settings separate for the new managed script.

   ![Choose a new project or reuse an imported project](/docs/img/script-guide/maafw/选择新项目或复用已导入项目.png)

3. When creating a new project, click **Pick a local directory** under **Project directory** and select the folder that directly contains `interface.json`.

   ![Pick the project directory containing interface.json](/docs/img/script-guide/maafw/选择包含%20interface%20的项目目录.png)

   Select the directory level where `interface.json` sits:

   ![M9A project directory example](/docs/img/script-guide/maafw/M9A项目目录示例.png)

4. Wait for the import to finish, then check the project overview and **Runtime environment** panel. Initial preparation can take a few minutes as AUTO-MAS scans the project and copies the files needed to run it.

   **Scanning project directory:**

   ![Scanning project directory](/docs/img/script-guide/maafw/正在扫描项目目录.png)

   **Copying project files:**

   ![Project files copied](/docs/img/script-guide/maafw/项目文件复制完成.png)

Because we imported M9A, the managed script is automatically recognized as **M9A**; projects without a dedicated flavor remain generic **MFW**. The page then shows AUTO-MAS's internal managed directory, from which tasks are run.

**AUTO-MAS keeps its copy separate from your source directory. While "Follow source directory" is off, once import succeeds and a test run works, you may keep or remove the source folder as you prefer; if source following is enabled or the project is in source form, you must keep the source directory. Never let two programs control the same game or emulator instance at the same time.** Note that rebuilding a missing copy still requires the source directory.

![M9A type recognized and runtime environment ready](/docs/img/script-guide/maafw/M9A导入后识别类型与运行环境.png)

### Control mode, updates, and run configuration

Follow the guided steps to choose the control mode, game resource, and emulator or PC game launch method. Do not pick a control mode the project does not support. Examples for ADB and Win32:

**ADB and emulator:**

![ADB control and emulator example](/docs/img/script-guide/maafw/ADB控制与模拟器配置示例.png)

**Win32 and PC game:**

![Win32 control and PC game launch example](/docs/img/script-guide/maafw/Win32控制与PC游戏启动配置示例.png)

Note: **Try changing Unity game resolution** temporarily updates a Unity game's registry settings before AUTO-MAS launches it, and restores them afterward. Two common resolutions are offered; lower resolutions reduce load on your PC.

![Unity game resolution options](/docs/img/script-guide/maafw/Unity游戏分辨率选项.png)

In the project update step, choose whether to enable automatic updates, the update source, and the channel. GitHub downloads can be slow depending on your network; if you enable automatic updates before runs, allow enough time. Large projects such as MaaFGO also take longer to extract and verify—please wait patiently and report any issues.

![Project update settings and check button](/docs/img/script-guide/maafw/项目更新设置与检查更新入口.png)

Finally, review **Run configuration**. The **single-run time limit** is a hard timeout: because MFW projects run directly inside AUTO-MAS, AUTO-MAS detects runtime errors directly, and the hard timeout prevents infinite task loops.

If the project's original UI already contains saved configurations, you can check the ones you want to import—each selected configuration becomes an account. Alternatively, skip import and create your first account from scratch.

**Import existing configurations:**

![Import existing project configurations as accounts](/docs/img/script-guide/maafw/导入已有配置为用户.png)

**Start with an empty account:**

![Skip importing configurations and create an empty account](/docs/img/script-guide/maafw/不导入已有配置并创建用户.png)

### Configure your first account

If configuration import created accounts, open one of them to edit; otherwise, create your first account. Either way, add a simple test task first to verify the runtime environment works before adding the rest of your tasks. You can add more accounts later from the managed-script list.

You can name accounts freely. Note that history records for accounts with the same name are grouped together, so using names like `Alice (M9A)` and `Alice (MaaEnd)` helps tell different managed scripts apart.

Project-specific flavors may add extra handling beyond generic MFW. For example, on M9A's official server resource, filling in the **Account** field automatically inserts a "Switch account" task at runtime (other resources may not support this). **The Password field is stored only as a local note and is never passed to the script.**

![M9A account info and task queue](/docs/img/script-guide/maafw/M9A用户信息账号与任务队列.png)

Add tasks one by one or apply a preset shipped with the project to add a batch in one click, then adjust each task's options. Changes save automatically.

![Task queue and task options](/docs/img/script-guide/maafw/任务队列与任务选项.png)

Some projects include illustrations in their task descriptions:

![Task description image](/docs/img/script-guide/maafw/任务中的项目说明图片.png)

Images in task descriptions **can be clicked to zoom in**.

::: tip Easter egg
We hid a few easter eggs too—for example, the managed-script name and tag when importing Senti (识宝):

![Senti name in setup wizard](/docs/img/script-guide/maafw/识宝项目引导中的名称.png)
![Senti name and tag in managed-script list](/docs/img/script-guide/maafw/识宝脚本列表中的名称和tag.png)
:::

### Where the project runs

MFW scripts run from a copy managed by AUTO-MAS; the folder you first pick is the **import source**. Importing places the required runtime files into AUTO-MAS's data directory—the project's own UI executable is usually excluded. Runs and project updates target this managed copy without touching your source directory.

After import succeeds, **Project directory** displays AUTO-MAS's internal copy path and the folder picker is locked. **Create a new managed script to use a different project**—do not try to swap folders on an existing managed script as in older guides.

The basic information section also provides **Follow source directory (developer mode)**:

- Normally leave it off and update via **Project update**. A healthy copy runs and updates without the source; rebuilding a missing copy still requires the source directory.
- When enabled, MAS checks and syncs changes from the source directory before each run (or when you click **Sync now**). Project updates are disabled in this mode, and you must keep the source directory.
- Source-form projects always follow their source, with this switch locked on. Keep the local source and its dependencies.
- If synchronization or preparation fails, read the **Runtime environment** log, fix the cause, and click **Retry** or **Sync now**. These actions do not switch to a different project.

::: tip Multiple managed scripts and older configs

- Several managed scripts can use the same project with separate run views and account settings. Ordinary managed scripts for the **same project and same update channel** share the managed project version; managed scripts with source following enabled use their own source instead.
- Legacy MFW managed scripts attempt to import from their original directory the first time they run on the new version. Check the runtime environment and do a test run after upgrading.
- Deleting a managed script removes its run view without touching the source directory. Back up any runtime data you still need before deleting.
:::

::: warning
Do not edit or delete `data/mfw/` by hand. If the copy appears damaged, read the runtime logs and use the preparation or source-sync actions on the page.
:::

## Settings

### Control mode and game resources

| Setting | Description |
| --- | --- |
| Control mode | Choose a controller declared in `interface.json`: Android (ADB) or PC window (Win32). |
| Game resource | Choose a resource declared in `interface.json`, typically a server (Official / Bilibili / Global, etc.). Leave blank to auto-select the first resource compatible with the current control mode. |
| Emulator / instance | Required for ADB; configure the emulator under **Emulators** first. |
| Game package | Used for ADB. AUTO-MAS infers and fills this in after you select a resource; leave blank if it cannot be inferred. |
| PC game launch mode | Shown for Win32: **Let MAS start the game** (select the game's main exe; MAS closes it when done) or **Use another method to start/stop the game** (you or the script manages the game; MAS attaches to the running window). |
| Wait after launch (seconds) | Applies only when MAS starts the game: how long to wait after the game window appears before sending the first task, giving loading screens time to finish. |
| Try changing Unity game resolution | Works for Unity-engine games only: temporarily switches the game to a windowed resolution before MAS launches it and restores the original setting after it closes. Skipped if the game is already running. |

### Run configuration

| Setting | Description | Default |
| --- | --- | --- |
| Runs per day per account | Maximum runs per account per day (`0` means unlimited) | `0` |
| Retry limit | Maximum retries after a task fails | `3` |
| Single-run time limit (minutes) | Hard cap for one account's run; stops the run on timeout | `120` |
| Skip once done today / this week / this month | Selected tasks are skipped for the rest of the day / week / month once they succeed once | None |

### Project update

| Setting | Description | Default |
| --- | --- | --- |
| Update source | **GitHub**: downloads directly from GitHub Releases with no extra setup; **MirrorChyan**: faster downloads with SHA-256 verification (requires a CDK) | GitHub |
| Update channel | Stable / Beta, depending on what the project publishes | Stable |
| MirrorChyan CDK | Requires this project's own CDK; it does not fall back to the CDK used for AUTO-MAS itself | |
| Automatic update | **Before run**: checks and updates the project before a run; **Off**: manual updates only. The old after-run setting is treated as before-run | Before run |

When source following is off, you can also click **Check for updates** or **Update now** on the page; the **Update progress** panel below shows live download, overwrite, and verification logs. Incremental packages are used when a reliable baseline exists; otherwise a full package is downloaded. Updates apply only to the managed copy, never the source folder. A managed script cannot be updated while it is running—wait until the run finishes.

## Accounts and task queues

To add more accounts later, click **Add an account** in the managed-script row under **Managed scripts**. AUTO-MAS runs each account's task queue in list order. In generic MFW, the Account field is usually only a local note; project-specific flavors may use it functionally (such as official-server account switching in M9A). The Password field is only a local note.

- **Task list**: populated from `interface.json`; select tasks to add to the queue and drag to reorder.
- **Presets**: if the project defines presets (such as "Daily"), click once to insert a full set of tasks.
- **Task options**: mirrors the options in the project's native UI with matching defaults.
- **Flavor automation**: generic MFW runs your exact queue; flavors may automatically insert or adjust tasks (for example, M9A manages starting the game, switching accounts, and closing the game). Follow the notices on each flavor's guide page.

## What Happens on Each Run

1. If the managed copy is missing, AUTO-MAS tries to rebuild it from the source; managed scripts following their source check and sync source changes.
2. If source following is off, AUTO-MAS checks and updates the copy according to **Automatic update**.
3. Verifies the runtime environment (MaaFramework runtime and Agent environment) is intact and up to date.
4. Starts the emulator or game, connects the controller, loads resources, and launches the Agent.
5. Executes queued tasks one by one, retrying failures up to the **Retry limit**.
6. Closes the emulator or game started by MAS, writes history records, and sends notifications.

## Will My Project Directory Be Modified?

No. Execution, updates, logs, and runtime configs all stay inside AUTO-MAS's managed copy. When source following is off, the source directory is only read during import or rebuild; when source following is on, MAS checks and syncs source changes before each run. Before every run, `interface.json` and `config/` in the copy are archived so you can preview or restore them anytime under **Config restore** on the account page.

## Troubleshooting

**Q: The Runtime environment panel stays red and says preparation failed.**

A: Some projects need to download MaaFramework or Agent dependencies on first run. Check your network or proxy, then click **Retry** (the same button turns into Retry on failure). Selecting the wrong folder (one without `interface.json`) also stops here.

**Q: The run ends immediately with "Pre-run check failed".**

A: Most often the chosen **Game resource** does not support the current **Control mode** (for example, a PC-only resource paired with an Android controller), or you selected ADB without picking an emulator instance.

**Q: On first run, the Agent process exits or connection times out.**

A: Check the run log and **Runtime environment** panel to confirm the project's Agent dependencies are ready. Different projects have different setup steps; if the original project requires initialization in its native UI, complete it there, then sync from source or re-prepare the runtime environment as prompted.

**Q: It says imported from source v3.10, but the overview shows v3.16.**

A: The first is the source directory's version at import time; the second is the copy's current version after self-updating. Updating your source folder does not change an ordinary managed script's version—use **Project update** for routine updates, or enable **Follow source directory** when developing locally.

**Q: I deleted the original project folder.**

A: While source following is off and the managed copy is healthy, it can still run and update. If source following is enabled or the project is in source form, restore the source folder first. If the copy is also lost, restore the source—or if the page allows picking a folder again, select a freshly downloaded complete package.

**Q: Where is the managed copy stored?**

A: Under `data/mfw/<first 12 hex chars of managed-script ID>/` inside the AUTO-MAS installation directory. It holds runtime data, so do not edit or delete it by hand. If the copy is damaged, inspect the page status and logs, then re-prepare the runtime environment (or click **Sync now** when source following is enabled).

**Q: A managed script is still running and I want to view the native MaaFramework log.**

A: For a finished run, open the record in **History** and download the MaaFramework native log from **Attachments** (current versions store archives per record rather than in legacy date/username folders). To check logs while a run is still in progress, locate the copy directory:

1. Under **Managed scripts** (formerly **Script Management**), open any account in that managed script for editing and click **Open config folder** in the top-right corner.

   ![Open config folder button on account edit page](/docs/img/script-guide/maafw/编辑账号页的打开配置文件夹按钮.png)

2. This opens `data\<managed-script ID>\<account ID>` (it is fine if the folder is empty). Note the managed-script ID right after `data` in the address bar, such as `b7c3bb03-52a5-47c1-a8fc-502c0ffa64cd`.

   ![Managed-script ID in the config folder path](/docs/img/script-guide/maafw/配置文件夹地址中的脚本ID.png)

3. Go back to `data`, open `mfw`, and find the folder named after the **first 12 characters of the managed-script ID with hyphens removed**: for the example above, `b7c3bb03` + `52a5` → `b7c3bb0352a5`. Searching with hyphens will not match.

   ![Copy folder named with first 12 characters of script ID](/docs/img/script-guide/maafw/mfw目录下按脚本ID前12位命名的副本.png)

4. Inside, the layout matches the original project, with logs under `debug`:
   - `debug\maafw.log`: the active MaaFramework log.
   - `debug\maafw.bak.<timestamp>.log`: rotated older log chunks created when `maafw.log` grows large during long runs.
   - Project-specific Agent logs also live under `debug`, such as `debug\agent\maafw.log` for M9A.

   ![Managed copy directory and debug folder](/docs/img/script-guide/maafw/脚本副本目录与debug文件夹.png)

::: warning Before sharing logs
If you entered passwords in task options, `debug\maafw.log` may contain them in plaintext. Archived MaaFramework logs in History redact credentials known for that run—if you can wait for the run to finish, download the log from History attachments instead, and still check for other personal info before sharing.
:::

**Q: The first run after upgrading took an extra dozen seconds.**

A: AUTO-MAS was likely importing a copy from the original directory. With source following off, routine runs afterward no longer read the original directory (though rebuilding a missing copy still needs it). With source following on, each run checks for source changes.

## For developers

To give a project its own script identity, hints, or pre-run orchestration rules, see [Flavor Development](/en/developer/plugins/script-adapters/mfw/develop).
