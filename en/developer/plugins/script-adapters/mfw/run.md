# MFW Runs

`MaaFW/tools/embedded/runner_task.py` turns an account's task snapshot into a run plan and manages device preparation, retries, and wrap-up. Workers under `MaaFW/tools/core/runner/` execute the project according to the plan and report results for each task.

## Task Snapshot

An account's `Task.TaskSnapshot` is a JSON string storing the task queue and option values. Its parsed structure is:

```json
{
  "taskOrder": ["<task instance ID>"],
  "taskChecked": { "<task instance ID>": true },
  "taskOptions": { "<task instance ID>": { "<option name>": "<option value>" } },
  "globalOptions": { "<global option name>": "<global option value>" },
  "taskLabels": { "<task instance ID>": "<custom display name>" }
}
```

- **Queue and selection**
  - `taskOrder`: Execution order of task instance IDs.
  - `taskChecked`: Whether each instance is checked.
  - `taskLabels`: Optional custom display name per instance.
- **Option values**
  - `taskOptions`: Option values stored per task instance ID.
  - `globalOptions`: Global options for the account; see [Global Options](#global-options).

| Design | Why |
| --- | --- |
| Queue entries are **instance IDs**: the first copy uses the task name, and duplicates use `<task name>__MAS_DUP__<random suffix>`; parsing checks first whether the full string is a valid task name ([#660][660]) | Allows queueing the same task multiple times with zero migration for legacy configs, and avoids misparsing task names that already contain the separator |
| MFAA's private `repeatable + repeat_count` extension and same-named entries in task tables are both expanded into duplicate instances (up to 99), with no internal runner loop ([#973][973]) | Failures continue to the next copy under MAS rules |
| "Once daily / weekly / monthly" is tracked by task name and scheduled at most once within the same round (`runner_task._filter_period_once_tasks`) | Otherwise a succeeding first copy marks the task name done and causes remaining copies in the same round to be skipped |
| Display names (`taskLabels`) apply only to that instance without altering the task name; execution and project updates still match by instance ID, and run logs still use the interface label ([#1301][1301]) | Display names affect the UI only |
| When the snapshot is empty, `Task.SelectedPreset` applies the project preset; applying a template or importing another account's queue clears it ([#1260][1260]) | Prevents an empty snapshot from unintentionally applying a preset |
| Saved option values that no longer match the current interface (for example, a renamed case) fall back to defaults and emit one line each under "Run plan notices" at the top of the run log (`run_plan.describe_stale_option_values`) | No silent fallback |
| Tasks removed from the interface remain in the queue as ghosts: skipped at run time and recorded in the run overview; runs with invalid tasks still count as completed, while reports flag "has invalid tasks" and count them in the summary's incomplete total ([#1147][1147]) | Previously, renamed tasks vanished silently and were lost permanently upon editing |

## Global Options

| Design | Why |
| --- | --- |
| `global_option` (including sub-options under cases) is stored once per account in `globalOptions` and displayed once above the queue on the account page ([#1294][1294]) | Matches PI global semantics |
| When the same option is also referenced by a task, resource, or controller, it is stored per task and the task value wins; task values are ignored only for "global-only" options | Matches the PI reference behavior |
| Determining "global-only" uses the option table **unfiltered by the current resource or controller** | Otherwise a global option referenced by an unselected resource would be misclassified and overwrite the account's global value |
| Flavors building plans via `task_ids` must also pass `global_options` to `build_plan` | Omitting it does not raise an error; global options silently revert to defaults |
| Legacy config migration: missing or invalid entries in `globalOptions` pull from the first checked instance in the queue with a valid value, using the same rule on frontend and backend | Preserves values across upgrades |
| **Downgrade-compatible dual write**: on save, global-only values are mirrored onto every task instance (`task_config.mirror_global_options_to_tasks`); newer versions ignore these copies | Older versions read per-task values and drop `globalOptions` on save, so upgrading back recovers them from the mirrored copies |
| Importing a queue from another account under the same managed script does not copy global options | Queue import replaces only the queue |

Options in resource groups and controller groups are stored **per task**, matching MXU.

## Presets, Custom Templates, and Import

| Design | Why |
| --- | --- |
| "Apply preset" replaces the queue directly without an extra confirmation | Existing preset semantics |
| Custom templates are stored at the managed-script level in `Task.Templates`, shared by all accounts under that managed script, inherited automatically by flavor subclasses, and never written into the project directory ([#1260][1260]) | Templates are MAS config, not project files |
| Saving a template strips ghosts, managed tasks, and **password values**; same names do not overwrite; invalid templates remain visible with a count, and when all entries are invalid "Apply" is disabled while deletion remains available | Secrets must not cross accounts, and templates should not vanish silently |
| Importing from another account under the same managed script overwrites the current queue, keeps current leading tasks at the front, skips invalid and managed tasks, and omits password values | Same as above |
| The runner reads only the snapshot and never reads `Task.Templates` | Templates are only a way to populate snapshots |

## Run Plan

`build_maafw_run_plan()` resolves selected tasks and options into data executed by the worker.

- **Resolving task options**: `run_plan.build_maafw_run_plan` produces the pipeline override for each task; `run_plan.resolve_run_selection` is the single source of truth for picking the controller and resource (falling back by controller when the managed-script resource is empty), keeping runtime, save hooks, and flavor sanitization consistent.
- **Merging overrides**: `pipeline_override.deep_merge_pipeline_override` merges sub-keys recursively, with later arrays and scalars overwriting earlier ones. Deep merging preserves fields written separately by project authors, matching CFA; **do not change it to whole-object replacement to match MFAA / MXU**.
- **Reading option defaults**: `select`'s private `default` is intentionally ignored—even authors' own MXU / MFAA shells ignore it, so honoring it would be more aggressive than upstream.
- **Resolving string inputs**: Empty string inputs default to the project interface's `default`. Flavors can declare fields that must send an empty string via `no_default_fallback_inputs`; projects supply defaults, while flavors only add project-specific semantics.
- **Resolving numbers and booleans**: Missing or malformed numeric/boolean inputs skip that option's override with a warning instead of failing the whole round, allowing minor release-package defects to degrade gracefully.
- **Validating checkbox counts**: Violating checkbox `min_count / max_count` raises `MaaFWCheckboxCountError` instead of silently truncating.
- **Loading the project interface**: Missing `import` files, missing `scan_dir`, and missing `interface_version` (treated as 2) only warn, with warnings written to the top of the run log and the import report; because interfaces are often authored for other clients, failing hard would break the entire project.
- **Agent connection timeout**: `agent.timeout` (seconds) only caps waiting for the agent to connect (omitted or `-1` means 10 minutes), with no limit after connection; hard-coded `agent.identifier` values receive an instance suffix at run time while pure numeric ports are used as-is, matching PI semantics and preventing multi-instance collisions.

## Launching the Worker and Collecting Results

The host writes the plan to a job file, and the worker reads it under the same version's contract.

1. **Launching the worker**: Started via `python -m` with the application root directory as its working directory and `PYTHONSAFEPATH=1`. Under supervised layouts, the application root also contains an older bundled source tree that would otherwise be imported.
2. **Protocol compatibility**: Adding fields to the job file requires matching host and worker versions; older workers silently ignore unknown fields for graceful degradation during mixed runs.
3. **Reporting task results**: The worker reports after every completed task, and the host accumulates them; periodic-task completion flags are recorded in the wrap-up `finally` block so timeouts, cancellations, and crashes never lose progress.
4. **Initializing the controller**: After connecting, the worker takes one initial screenshot (`runner._prime_first_screencap`) once per connection, logging any failure without aborting ([#1029][1029]). Before the first capture, `cached_image` raises and `resolution` is `(0, 0)`, causing some projects to read them at the start of task one and stop themselves; MFAAvalonia does the same.
5. **Saving account status**: The managed script holds the config lock, and account runtime state is submitted via a ticketed context, then saved and unlocked by the host. This lets running entries update safely and releases the lock cleanly on wrap-up.
6. **Forwarding thread logs**: Hooks and callbacks running in worker threads always forward log writes back to the event loop via `loop.call_soon_threadsafe`, avoiding `no running event loop` errors being swallowed by fault-tolerance logic into a generic "failed, continuing" message.

## Time Limits

Time limits apply to an account's entire run or to individual tasks. Spinning in place is detected separately.

- **Whole-account run limit**: `Run.RunTimeLimit` (default 120 minutes in MFW) is a **hard timeout** on an account's entire run, despite sharing a name with other adapters' "log stall timeout". In MFW, log output does not reflect whether execution is stuck: a stuck task can keep logging, while a normal long action can stay silent for a long time.
- **Passing the deadline**: The deadline is sent in the job file; when time runs out, the worker stops the current task itself, captures a timeout screenshot, and reports normally with all completed tasks. The host only waits a short grace period before force-killing ([#880][880]), avoiding kills that lose both screenshots and progress.
- **Per-task limit**: `Run.TaskTimeLimit` (default 45 minutes; `0` means unlimited) and per-task overrides in `Run.TaskTimeLimitOverrides`; flavors can bundle defaults via the class attribute `DEFAULT_TASK_TIME_LIMIT_OVERRIDES` ([#1180][1180]) so a single stuck task cannot consume the entire round.
- **Handling task timeout**: A per-task timeout stops only that task and takes a screenshot. If it is the first task in the plan or a flavor's critical task, the round ends; otherwise it records a failure and continues, **waiting for the stop to take effect first** (up to 60 seconds) ([#1296][1296]), because `post_stop` lets the current node finish and newly posted tasks during that window would be rejected by the framework.
- **Timeout vs. forced stop ordering**: Checking "this task timed out" must precede checking "script-side forced stop", because MAS's own `post_stop` also puts the tasker into a stopped state; reversing the order misclassifies timeouts as script-side stops and skips the rest of the round.
- **Unit conversion**: Converting minutes to seconds happens in a single place (`runner_task.task_time_limits_from_config`) on the host and is passed in the job file, because the worker is a separate process without config access.

## Loop Guard (Spinning-in-Place Detection)

| Design | Why |
| --- | --- |
| `Run.LoopGuard`, **experimental and off by default**; toggle sits on the first row of the "Per-task time limits" dialog ([#1223][1223]) | Criteria were tuned statistically, so users opt in first |
| Criteria: node names repeat with a period of at most 8 steps, every cycle performs a physical action (`Custom` and `DoNothing` do not count, and a cycle with no physical action resets the counter), at least 200 cycles lasting at least 10 minutes, at most 4 distinct recognition fingerprints across the last 30 cycles, and median cycle duration ≤ 20 seconds; emits an early warning at 100 cycles / 5 minutes (`runner/loop_guard.py`) | Calibrated by replaying production run logs: zero false positives on normal runs, detects stage-selection dead loops in ~10 minutes, and does not trip on intentional polling |
| Triggers the same wrap-up path as a per-task timeout and captures a `loop` screenshot | — |
| Opt-out uses `"attach": {"auto_mas_loop_guard": false}`, **not inside `attach.auto_mas`** | `attach.auto_mas` ignores the whole signal node on unknown keys and forces nodes declaring it to be enabled by MAS |
| Disabled for the entire run if MaaFramework notifications lack recognition/action details (older versions) | Missing details would look like an unchanged fingerprint and always trigger |
| Does not cover "waiting motionless" hangs | Handled by per-task time limits |

## Retries

| Design | Why |
| --- | --- |
| On any failed round (not only timeouts) with retries remaining: close the game / emulator **started by MAS** and restart it on the next round; never touch instances not started by MAS; clear cached connection info when closing an emulator ([#880][880], [#1028][1028]) | Reattaching to the same stuck window wastes time, and old PIDs are invalid after an emulator restart |
| `Run.RunTimesLimit` defaults to 3 for new managed scripts | Occasional loading timeouts in scripts should not fail the whole run |
| Non-periodic tasks **all rerun by default**; flavors can explicitly list tasks that are pointless to rerun in `skip_on_retry_entries`, while unlisted tasks rerun as usual ([#1296][1296]) | Startup tasks must rerun after a game crash, and only project authors know which tasks can safely be skipped |
| Each attempt keeps its own log; each new attempt begins with "Previous attempt (#N): reason", appending "restarted game/emulator" only if MAS actually closed a client ([#1011][1011], [#1295][1295]) | Prevents new logs from overwriting the failure cause and leaving mysterious retries |
| Architecture mismatches, unusable runtime environments, maintenance/update signals, and required client updates are never retried | Every attempt would fail identically, wasting an emulator restart |
| Pre-task and post-task scripts run once per account ([#638][638]) | Running them three times across three retries would be wrong |

## Critical Tasks, Wrap-up Tasks, and Unselectable Tasks

Declared by flavors; contracts are documented in the module docstring of `MaaFW/tools/embedded/flavor.py` (see [Flavor](./flavor.md)).

| Declaration | Effect |
| --- | --- |
| `abort_round_entries` (critical tasks) | Failure or per-task timeout ends the round immediately and retries as usual |
| `nonfatal_entries` (wrap-up tasks) | Failure, timeout, loop guard, or script-side forced stop only captures a screenshot and logs one line without failing the round or triggering retries; takes precedence over "first-task timeout ends the round". User stop, whole-round time limit, signals, and game launch failure take precedence over it |
| `unselectable_entries` (unselectable tasks) | Preview returns `unselectableReason`; blocked from selection on the account page, while existing entries remain visible and deletable; dropped with a log line before flavor decoration at run time, so the two declarations above do not apply to them |

## Maintenance and Update-Required Signals

| Design | Why |
| --- | --- |
| Projects set `attach.auto_mas` on a node to either a string (`server_maintenance` / `client_update_required`) or a dict `{signal: [OCR texts]}` matched against recognition text, always checking maintenance first ([#1091][1091]) | Maintenance screens are often exitless loop nodes that previously spun until hard timeout and retried; no project names appear in engine code |
| MAS force-enables signal nodes after all overrides | Project-side toggles do not work under MAS |
| Server maintenance: marks the account "skipped", does not consume proxy count or retry, and immediately skips subsequent accounts using the same resource under the same managed script. Client update required: marks failed, does not retry, and does not skip other accounts | Client version is tied to the device |
| No task-page popup; logs one line and sends one formal notification on wrap-up; skips the proxy summary when every account was skipped due to maintenance | Avoids noise |

## Hotkey Mapping

| Design | Why |
| --- | --- |
| PI v2.8 `hotkey` options are stored at the **managed-script level** in `Game.Hotkeys` (only values differing from defaults), edited via a dialog, **Win32 only** ([#1174][1174]) | Keybindings belong to the game client, not individual accounts |
| Precedence at run time: managed-script level > task snapshot > interface default; malformed values fall back only for that single field with a warning | One invalid hotkey should not fail the whole round |
| The dialog permanently displays how many modifier keys the project supports, highlighting mismatches in red and blocking save | — |

## PC Game Launch and Shutdown

`Game.LaunchMode` determines how MAS obtains the game window under Win32 control. Launch, retry, and final wrap-up are orchestrated by `runner_task.py`.

- **Launch mode (`Game.LaunchMode`)**
  - `DirectExe`: Default. MAS launches the game with `Game.LaunchPath` and launch arguments, and closes the launched game on wrap-up.
  - `URL`: MAS opens `Game.URL`. Also configure `Game.LaunchPath` so MAS can identify the game window and process.
  - `AttachOnly`: Attaches to a game window started by the project, the user, or another tool, and handles the process according to `Game.CloseMode` on final wrap-up.
- **Shutdown mode after attaching (`Game.CloseMode`)**
  - `Script`: Left to the project to close.
  - `OnFailure`: Default. Closes matching processes newly spawned during this run if execution did not complete normally.
  - `Always`: Closes matching processes newly spawned during this run on final wrap-up.

1. **Preparing the game window**: `DirectExe` and `URL` reuse an already-running game window when found. Unity resolution overrides apply in both modes via `game_resolution.py` modifying Unity player settings. Wait timeouts are based on `Game.WaitTime`, with `URL` mode allowing longer startup waits.
2. **Running project pre-tasks**: `pretask` from the project interface runs on the first attempt; if it succeeded, retries decide whether to rerun it based on whether the client is still alive. Account-level pre- and post-scripts run once each.
3. **Cleaning up devices**: `DirectExe` and `URL` close only processes started by this run. `AttachOnly` shutdown rules execute only on final wrap-up and leave untouched any processes that existed beforehand or whose start time cannot be verified. Under ADB control, emulator startup can launch the game using the selected package name.

## Screenshots, Logs, and Redaction

| Design | Why |
| --- | --- |
| All failure screenshots are saved to history, but only **final failures** are sent with notifications (up to 4 images); intermediate failures that eventually succeed are not pushed ([#841][841]) | Prevents notification spam |
| `.maafw.log` in history is a byte-exact copy of the native log for this run; MAS's own output goes to `.worker.log`; project agent logs go to `.project.log` ([#881][881], [#1117][1117]) | Can be forwarded verbatim to script authors |
| `maa_option.json` sets `logging` to `true` | Upstream default; MaaFramework has only on/off file logging with no log levels |
| PI `password: true` inputs are stored in snapshots as `mas-dpapi:`-prefixed ciphertext; MAS log outputs redact passwords of length ≥ 4 and **do not redact** shorter ones | Global replacement of 1–3 character strings would make logs unreadable; `debug/maafw.log` written by the framework itself contains plaintext outside MAS's control |
| Lines prefixed with `[MaaFW 详情]` go only to `.worker.log` and backend logs; the UI shows only conclusions | New log outputs must also filter this prefix |
| Subprocess output included in logs is always truncated from the **tail** | Keeping the head drops the actual exception line |
| Resource hash mismatches go only to detail logs ([#1151][1151]) | Users cannot act on them |

[638]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/638
[660]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/660
[737]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/737
[841]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/841
[843]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/843
[855]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/855
[880]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/880
[881]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/881
[889]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/889
[899]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/899
[973]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/973
[1011]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1011
[1028]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1028
[1029]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1029
[1091]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1091
[1117]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1117
[1146]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1146
[1147]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1147
[1151]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1151
[1174]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1174
[1180]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1180
[1223]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1223
[1231]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1231
[1260]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1260
[1294]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1294
[1295]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1295
[1296]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1296
[1301]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1301
