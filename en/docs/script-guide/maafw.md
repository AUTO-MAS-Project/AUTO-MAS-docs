# MFW Project Configuration Guide

::: tip Tip
Any project built on [MaaFramework](https://github.com/MaaXYZ/MaaFramework) — that is, any directory containing an `interface.json` — can be added to AUTO-MAS as an **MFW script**, regardless of whether its own UI is MFAAvalonia, MXU, MFW-PyQt6 or MaaPiCli. [M9A](/en/docs/script-guide/m9a) is added the same way: pick the M9A directory and AUTO-MAS recognises it and treats it as M9A.
:::

## What is an MFW script

Projects in the MaaFramework ecosystem all share one shape: an `interface.json` declaring controllers, resources and tasks, a `resource/` directory with the images and pipelines used for recognition, sometimes an Agent (a Python script or an executable) for complex logic, and a UI wrapped around it all.

AUTO-MAS's MFW script **does not start that UI**. It reads `interface.json` directly, loads the MaaFramework runtime and Agent shipped with the project, and runs the task queue you arranged. So it does not support one particular game — it supports any project shaped like this.

## Install the project

1. Download the **Windows release package** (usually named `xxx-win-x86_64-vX.Y.Z.zip`) from the project's own GitHub Release or from <Pill name="MirrorChyan" image="https://mirrorchyan.com/favicon.ico" link="https://mirrorchyan.com/en/projects?scouce=AUTO-MAS-Web"/>.
2. Extract it to any folder, e.g. `D:\MaaXXX`. You should see `interface.json` directly inside the extracted directory.

::: warning Note
You do not need to open the project's own UI first. AUTO-MAS prepares the MaaFramework runtime and the Agent's Python environment itself.
:::

## Configure the script

1. Go to **Scripts**, click **New script** and choose **MFW script**.
2. Under **Project directory**, click **Pick a local directory** and select the directory that contains `interface.json`.
3. AUTO-MAS **imports** the project as its own copy (see **Embedded copy** below), reads `interface.json` (an overview table with the project name and the number of tasks / controllers / resources appears underneath) and starts preparing the runtime environment in the **Runtime environment** panel on the right — the first time it has to download MaaFramework, which can take a few minutes; the panel shows progress and the result.
4. Fill in **Control mode and game resource**, **Run configuration** and **Project update** below, then **Save**.

### Embedded copy

An MFW script **always runs on AUTO-MAS's own copy**; the directory you picked is only the "source". The moment you pick it, AUTO-MAS copies the **parts needed to run** into `data/maafw_projects/<script ID>/`: `interface.json` and the resources it references, the Agent, the MaaFramework runtime and Python interpreter shipped with the project, icons and readme files. UI programs, .NET managed libraries, caches and logs are not copied. From then on running, checking for updates and downloading update packages all happen in the copy, and **the source directory is never touched**.

Which also means: **once the import is done, you can delete the source directory.** The copy keeps running and updating; only "Re-import" needs the source to still exist. Think of it as "moving the project into AUTO-MAS" — keep the original directory only if you still want to use the project's own UI.

The **Embedded copy** panel under the project directory shows the copy's state:

| Status line | Meaning |
| --- | --- |
| Copy intact / Copy missing | Whether the copy still exists. A missing copy needs no action: it is rebuilt from the source before the next run (it is only an error if the source is gone too) |
| The copy is xx% of the source (A → B) | Size of the source directory vs. the copy |
| Shell: MXU / MFAAvalonia … | The UI type detected; update packages are picked to match it |
| MaaFramework x.y.z (bundled by the project, copied as is) | The runtime version shipped with the project; the copy uses this one |
| Agent Python x.y (bundled by the project, copied as is) | Shown only when the project ships its own Python interpreter |
| Imported from source vX | The source's version at import time; the copy may be newer once it has updated itself, which is normal |
| Source directory no longer exists | You deleted the original. The copy runs as usual; you just can't re-import |

- **Re-import**: copy the project again from the current source directory. Use it after you updated the original directory by hand, or when you suspect the copy is off.
- **Change directory**: click **Pick a local directory** again and choose another directory — this switches the source and re-imports; if the import fails, the old copy and old source are left untouched. Picking a different project's directory works too: the script becomes that project (an M9A directory turns the script into an M9A script and vice versa); users and run settings are kept, the task queue has to be rebuilt for the new project.
- **Delete script**: the copy is removed with it; the source directory is left alone.

::: tip Three things you get for free
- **One project, several scripts**: each script has its own copy, so they can run at the same time without interfering. Previously one directory could only run one script at a time.
- **Disk space**: runtime files that are identical across copies (MaaFramework, Python, native libraries) are stored once on disk (`data/maafw_blobs/`) and shared; they are reclaimed automatically when scripts are deleted. One M9A copy is about 300 MB; a second one really only costs 70–110 MB more.
- **MFW scripts that existed before you upgraded AUTO-MAS** need nothing from you: they are imported from their original directory once, the first time they run or you open the script page.
:::

### Control mode and game resource

| Setting | Description |
| --- | --- |
| Control mode | Pick from the controllers declared in `interface.json`: Android (ADB) or a PC window (Win32) |
| Game resource | Pick from the resources declared in `interface.json`, usually the server (official / Bilibili / global…). Leave it empty to auto-select the first one matching the control mode |
| Emulator / emulator instance | Required when the control mode is ADB; set the emulator up under **Emulators** first |
| Game package name | Used when the control mode is ADB. Once a resource is chosen AUTO-MAS infers it from the project and fills it in; leave it empty if it can't |
| PC game launch | Shown when the control mode is Win32: **Let MAS launch the game** (point it at the game's exe; MAS closes it when done) or **Start and stop the game some other way** (the script or you handle it; MAS only attaches to the running window) |
| Wait after launch (seconds) | Only when MAS launches the game: wait this long after the window appears before sending the first task, to give the loading screen time |
| Try to change the Unity game's resolution | Unity games only: before launching, MAS temporarily switches the game to windowed mode at the chosen size and restores the original values after the game closes; nothing is changed if the game is already running |

### Run configuration

| Setting | Description | Default |
| --- | --- | --- |
| Runs per day for this user | How many times a single user is run per day; 0 means no limit | 0 |
| Retry limit | How many times to retry after a failed task | 3 |
| Single-run time limit (minutes) | The maximum length of one run for one user; it is force-stopped after that | 120 |
| Skip once done today / this week / this month | The selected tasks are skipped automatically after they have succeeded once that day / week / month | none |

### Project update

| Setting | Description | Default |
| --- | --- | --- |
| Update source | **GitHub**: no setup, downloads straight from the project's GitHub Release; **MirrorChyan**: needs a CDK, faster downloads with sha256 verification | GitHub |
| Update channel | Stable / beta, depending on what the project actually publishes | Stable |
| MirrorChyan CDK | Defaults to the one entered in MAS's own update settings; can be overridden per script | |
| Auto update | **Before run**: check and update before every run; **After run**: update after the run finishes; **Off**: manual only. A failed update never blocks the run | Before run |

You can also **Check for updates** or **Update now** at any time; the **Update progress** panel below shows the download, apply and verify steps live. A differential package is used when a trustworthy baseline exists, otherwise the full package. Updates only write to the copy, never the source directory, and can't be applied while the script is running — wait for it to finish.

## Configure users

1. Click **Add a user** in the script's row under **Scripts**.
2. A name and a note are enough. The account/password fields are local notes for you only; AUTO-MAS does not log in with them.
3. You can add several users; AUTO-MAS runs each user's task queue in list order.

### Task queue

- The task list is exactly the tasks declared in the project's `interface.json`. Select one to add it to the queue, then drag to reorder.
- When the project ships **presets** (e.g. "Daily"), one click adds a whole set of tasks.
- Each task's **Task options** are the options from the project's own UI (server, stage, count…), with the same defaults.
- The queue is handed to MaaFramework as is; how the game is launched and closed is decided by the control mode and launch settings on the script page above. Projects AUTO-MAS recognises (currently M9A) get their start / close game tasks added automatically — see that project's own page.

## What happens on each run

1. If the copy is missing (or the source directory was changed), import it from the source first.
2. Check and update the copy according to **Auto update**.
3. Verify the runtime environment: whether the MaaFramework runtime and Agent environment are still present and whether they changed with the project update.
4. Start the emulator or game, connect the controller, load resources, start the Agent.
5. Run the queue task by task; failures are retried up to the **Retry limit**.
6. Afterwards close the emulator / game that MAS started, write the history entry and send notifications.

## Will my project directory get damaged?

No. Runs, updates, logs and the per-run configuration all land in the copy; the source directory is only **read** once when importing / re-importing. Inside the copy, `interface.json` and `config/` are archived before every run and can be previewed and restored under **Config restore** on the user page.

## FAQ

**The runtime environment panel stays red and says preparation failed**

The first run downloads MaaFramework and the Agent's dependencies from the internet. Check your network or proxy and click **Retry** (the same button; it turns into Retry on failure). Picking the wrong directory (no `interface.json` inside) also stops here.

**The run ended without running anything, the run page says "check failed"**

Most often the chosen **Game resource** does not support the current **Control mode** (e.g. a keyboard-and-mouse-only resource with an Android controller) — pick another resource; or the control mode is ADB but no emulator instance is selected.

**The first run says the Agent process exited / connection timed out**

A few projects' Agents install their own dependencies from the internet on first start; if that takes more than a few dozen seconds they are judged unreachable. Run the project once in its own UI to let it finish installing, click **Re-import**, then run it from AUTO-MAS again.

**"Imported from source v3.10" but the overview table says v3.16**

The former is the source's version at import time, the latter the copy's current version — the copy updated itself. To bring the source up too, update the original directory and click **Re-import** (not doing so doesn't affect runs).

**I deleted the original directory**

That's fine. The copy runs and updates as usual; you just can't **Re-import** any more, and the status line says the source directory no longer exists. If the copy is lost too, download the project again and **Pick a local directory** to point at it.

**Where is the copy?**

`data/maafw_projects/<script ID>/` under the AUTO-MAS installation directory. It is managed by AUTO-MAS — don't edit it by hand; if it gets broken, delete the whole directory and it is rebuilt from the source before the next run.

**After upgrading, the first MFW run took ten-odd seconds longer**

That was the one-time import from the original directory. The original directory is not used afterwards.
