# MFW Project Configuration

::: tip
Any project built on [MaaFramework](https://github.com/MaaXYZ/MaaFramework) that ships an `interface.json` can be added to AUTO-MAS as an **MFW script**, whichever UI it comes with (MFAAvalonia, MXU, MFW-PyQt6 or MaaPiCli). Projects that already have a dedicated adapter ([M9A](/en/docs/script-guide/m9a), [MaaEnd](/en/docs/script-guide/maaend)) should use that adapter instead.
:::

## What is an MFW script?

MaaFramework projects all share the same shape: an `interface.json` that declares controllers, resources and tasks, a `resource/` directory with the images and pipelines used for recognition, sometimes an Agent (a Python script or an executable) for complex logic, and a UI program wrapped around all of it.

The MFW script in AUTO-MAS **never starts that UI**. It reads `interface.json` directly, loads the MaaFramework runtime and the Agent shipped with the project, and runs the task queue you configured. That is why it supports not one game but any project that looks like this.

## Installing a project

1. Download the **Windows release package** (usually named `xxx-win-x86_64-vX.Y.Z.zip`) from the project's GitHub Releases or from <Pill name="Mirror酱" image="https://mirrorchyan.com/favicon.ico" link="https://mirrorchyan.com/en/projects?scouce=AUTO-MAS-Web"/>.
2. Extract it to any folder, for example `D:\MaaXXX`. You should see `interface.json` right inside the extracted directory.

::: warning
You do not need to open the project's own UI first. AUTO-MAS prepares the MaaFramework runtime and the Agent's Python environment by itself.
:::

## Configuring the script

1. Open **Scripts**, click **New Script** and choose **MFW**.
2. Under **Local project directory**, click **Pick a local directory** and select the directory that contains `interface.json`.
3. AUTO-MAS reads `interface.json` right away (a summary table with the project name and the number of tasks / controllers / resources appears below) and starts preparing the runtime in the **Runtime environment** panel on the right. The first time it has to download MaaFramework, which can take a few minutes; the panel shows the progress and the outcome.
4. Fill in **Control mode and game resource**, **Run configuration** and **Project update** below, then **Save**.

### Run from an embedded copy

This is the switch right under the project directory, off by default. It decides **which directory AUTO-MAS runs the project from**:

| | Off (default) | On |
| --- | --- | --- |
| Runs and updates happen in | the directory you picked | AUTO-MAS's own copy |
| Your original directory | project updates are written into it, runs write config and logs into it | **not a single byte is touched** |
| Disk usage | nothing extra | one more copy, usually only 20%–60% of the original |

When enabled, AUTO-MAS copies **the parts needed to run** into its own `data/maafw_projects/<script id>/`: `interface.json` and the resources it references, the Agent, the MaaFramework runtime and the Python interpreter shipped with the project, icons and readme files. The UI program, .NET managed libraries, caches and logs are not copied. From then on, runs, update checks and update downloads all happen in the copy; the original directory is kept only as the "source".

Once enabled, **Local project directory** becomes **Source directory** and a status line appears below it:

| Status | Meaning |
| --- | --- |
| Copy intact / Copy missing | Whether the copy is still there. A missing copy needs no action: it is rebuilt from the source directory before the next run |
| Copy saves xx% (A → B) | Size of the original directory versus the copy |
| Shell: MXU / MFAAvalonia … | The UI family that was detected; update packages are picked to match it |
| MaaFramework x.y.z (bundled by the project, copied as is) | The runtime version shipped with the project; the copy uses exactly that one |
| Agent Python x.y (bundled by the project, copied as is) | Shown only when the project ships its own Python interpreter |
| Imported from source vX | The version of the source directory at import time. The copy will be newer once it has updated itself, which is normal |
| Source directory no longer exists | You deleted the original directory. The copy keeps running, it just cannot be re-imported |

- **Re-import**: copies the project again from the current source directory. Use it after updating the original directory by hand, or whenever the copy looks wrong.
- **Changing the directory**: while embedded, **Pick a local directory** picks a new source and re-imports. If the import fails, the old copy and the old source are left untouched.
- **Leave embedded mode**: turn the switch off. AUTO-MAS deletes its copy and runs from the original directory again. The original was never modified.

::: tip When it is worth turning on
- You do not want AUTO-MAS to touch your working copy of the project (for example because you still use its own UI).
- You want to use the same project directory for several scripts: without embedding, only one script can run on a directory at a time; with embedding each script has its own copy and they do not interfere.
- You have several MaaFramework projects installed: runtime files of the same version are stored only once on disk (`data/maafw_blobs/`) and shared between copies; they are reclaimed automatically after you leave embedded mode or delete the script.
:::

### Control mode and game resource

| Setting | Description |
| --- | --- |
| Control mode | One of the controllers declared in `interface.json`: Android (ADB) or a PC window (Win32) |
| Game resource | One of the resources declared in `interface.json`, usually the server (official / Bilibili / global…). Left empty, the first resource matching the control mode is used |
| Emulator / emulator instance | Required for ADB control; set the emulator up in **Emulator Management** first |
| Game package name | Used with ADB control. After a resource is selected AUTO-MAS infers the package from the project and fills it in; leave it empty if nothing could be inferred |
| How the PC game is launched | Shown for Win32 control: **Let MAS launch the game** (pick the game's own exe; MAS closes it afterwards) or **Start and stop the game another way** (the script or you start it; MAS only takes over the running window) |
| Settle after launch (s) | Only when MAS launches the game: wait this long after the window appears before the first task is sent, to give the loading screen time |
| Try to set the resolution of Unity games | Unity games only: before MAS launches the game it is temporarily switched to a window of the selected size; the original value is restored after the game closes. Nothing is changed if the game is already running |

### Run configuration

| Setting | Description | Default |
| --- | --- | --- |
| Runs per day for this user | How many times a single user may be run per day; 0 means no limit | 0 |
| Retry limit | How many times a failed task is retried | 3 |
| Single-run time limit (minutes) | Maximum duration of one run of one user; it is stopped when exceeded | 120 |
| Skip once done today / this week / this month | Selected tasks are skipped for the rest of the day / week / month once they succeeded | none |

### Project update

| Setting | Description | Default |
| --- | --- | --- |
| Update source | **GitHub**: no setup needed, downloads straight from the project's GitHub Releases; **MirrorChyan**: needs a CDK, faster downloads with sha256 verification | GitHub |
| Update channel | Stable / beta, depending on what the project actually publishes | Stable |
| MirrorChyan CDK | Prefilled from the CDK in AUTO-MAS's update settings; can be overridden for this script | |
| Auto update | **Before run**: check and update before every run; **After run**: update once the run has finished; **Off**: manual updates only. A failed update never blocks the run | Before run |

You can also **Check for updates** or **Update now** at any time; the **Update progress** panel below streams the download, apply and verification progress with the log. An incremental package is used when a trusted baseline exists, otherwise the full package is downloaded.

For embedded scripts the update is written to the copy only; otherwise it is written to your project directory.

## Configuring users

1. In the script table of **Scripts**, click **Add a user**.
2. A user name and a note are enough; the account and password fields are a local note for yourself, AUTO-MAS never uses them to log in.
3. You can add several users; AUTO-MAS runs each user's task queue in list order.

### Task queue

- The task list is exactly the set of tasks declared in the project's `interface.json`. Pick one to add it to the queue, then drag to reorder.
- When the project ships **preset templates** (for example "Daily"), one click adds a whole group of tasks.
- Each task's **task options** are the same options the project shows in its own UI (server, stage, count…), with the project's defaults.
- The queue is handed to MaaFramework as is; how the game is launched and closed is decided by the control mode and launch mode on the script page.

## What happens on every run

1. The project is checked and updated according to **Auto update** (the copy, when embedded).
2. The runtime is verified: the MaaFramework runtime and the Agent environment must still be there and match the project after an update.
3. The emulator or the game is launched, the controller connected, resources loaded and the Agent started.
4. The queue is executed task by task; failures are retried up to the **Retry limit**.
5. Emulators / games launched by MAS are closed, history is written and notifications are sent.

## Will my project directory get damaged?

- **Embedded on**: no. Not a single byte of the original directory is touched; even logs go to the copy.
- **Embedded off**: project updates are written into your directory, and each run writes the configuration for that run into the project's `config/`. `interface.json` and `config/` are archived before that; **Config restore** on the user page lets you preview and restore them at any time.

## FAQ

**The runtime panel stays red and says preparation failed**

The first preparation downloads MaaFramework and the Agent dependencies from the internet. Check your network or proxy and click **Retry** (the same button turns into Retry after a failure). A wrong project directory (no `interface.json` inside) also stops here.

**The run ends immediately with "check failed"**

Most often the selected **game resource** does not support the current **control mode** (for example a keyboard-and-mouse resource combined with the Android controller); pick another resource. Or the control mode is ADB but no emulator instance is selected.

**The first run reports that the Agent exited / could not connect**

A few projects' Agents install their own dependencies from the internet on first start; if that takes more than a few dozen seconds it counts as unreachable. Run the project once in its own UI so it can finish installing (then click **Re-import** if embedded) and come back to AUTO-MAS.

**"Imported from source v3.10" but the summary shows v3.16**

The first is the version of the source directory at import time, the second is the current version of the copy, which updated itself. To bring the source up to date as well, update the original directory and click **Re-import** (running does not depend on it).

**I deleted the original directory**

The copy keeps running and updating; only **Re-import** is no longer possible and the status line says the source directory no longer exists. Do not **Leave embedded mode** now: leaving deletes the copy and goes back to the original directory, which is gone.

**Where is the copy?**

Under the AUTO-MAS installation directory: `data/maafw_projects/<script id>/`. It is managed by AUTO-MAS; do not edit it by hand. If it gets broken, delete the whole directory and it is rebuilt from the source before the next run.
