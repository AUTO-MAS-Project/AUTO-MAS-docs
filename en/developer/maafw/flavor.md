# Flavor

This page is the **design note** for flavors: why they are designed this way, the full set of hooks and contracts, and what each of the two existing flavors looks like item by item. To build a new flavor, follow the steps in the how-to guide [Flavor Development](../maafw-flavor.md).

## Key Points

- A flavor = an **isomorphic subclass** of `MaaFWConfig` + a class attribute `FLAVOR = "module path:attribute name"` pointing at a hook object. It is not an adapter: running, updating, the embedded copy, notifications, failure screenshots, periodic tasks, and proxy counts are all existing MFW engine semantics, and a flavor rewrites none of them.
- The backend protocol has only three required members (`type_key`, `matches_project`, `decorate_selection`); everything else is an **optional hook or optional declaration probed with `getattr`** and is not part of the protocol.
- No flavor's name appears in `app/task/MaaFW/` (except as examples in comments); flavor logic enters only through `FLAVOR`.
- The frontend registry is the single source of truth: a descriptor object writes only what differs from MFW; customization goes from shallow to deep in three levels: differing fields, section replacement / slots, whole page. **A descriptor object must not contain boolean behavior switches.**
- The managed type is determined by the project: after import, the type is swapped in place according to `matches_project`, the uid does not change, and it works in both directions automatically.

## When to Build a Flavor

| Need | Where it goes |
| --- | --- |
| A capability every MaaFramework project needs | MFW engine |
| A single project's own identity, wording, pre-run task orchestration, pre-run login, a few unique forms | Flavor |
| A native editor session, a dedicated UI organized around game semantics | Adapter (not a flavor) |
| How in-game tasks run and how results are judged | The project itself; AUTO-MAS does not rewrite project logic |

## Backend: the Protocol and All Optional Hooks

The protocol and all contracts are written in the module docstring of `MaaFW/tools/embedded/flavor.py`; the table below is a summary. M9A's implementation is in `app/task/M9A/flavor.py`, and MSS's in `app/task/MSS/flavor.py`.

| Member | Required | When it is called / effect |
| --- | --- | --- |
| `type_key: str` | Yes | The managed type key, used in logs and notifications |
| `matches_project(interface_model) -> bool` | Yes | Decides the managed type after import. `flavor.decide_project_config_class` takes the first claimant in the registration order of `CLASS_BOOK`, otherwise the generic `MaaFWConfig`; an exception only warns |
| `decorate_selection(interface_model, task_ids, task_options, *, context)` | Yes | Called once after queue normalization (and after dropping unselectable tasks and instances not on a run day today) and before the run plan is built; returns a new `(task_ids, task_options)`. Runs in a worker thread. `context` is a read-only `MaaFWSelectionContext`; see "Selection context" below for its fields |
| `sanitize_task_snapshot(interface_model, snapshot, *, script_config, resource_name, user_info)` | No | Called before the account page save, the shell-config import creating an account, and config restore write `Task.TaskSnapshot`; returns the cleaned snapshot and the fields to write into `Info` as well. Not called when the interface cannot be read |
| `async ensure_game_updated(*, script_config, resource_name, package_name, adb_path, adb_address, if_auto_install, progress)` | No | When `Run.GameUpdateMode` is not off and the controller is ADB, called once per account per run after the emulator starts and before the first task. Returns `app.utils.common.game_update.GameUpdateResult`: when a manual update is needed, this account is judged failed and **not retried**; an exception only warns, and `CancelledError` must propagate |
| `async ensure_logged_in(*, script_config, user_config, resource_name, package_name, adb_path, adb_address, progress, host_adb=None)` | No | With an ADB controller, after MAS starts the emulator and before the first task, right after `ensure_game_updated`. Returns `str` (written to the run log; an empty string means nothing was done) or `StartupTask(deadline, entry)`; completion is recorded only on a successful login, and on failure it is called again on the next attempt and after a retry restarts the client. When it cannot continue, raise `MaaFWLoginRequired`; the message is shown directly to the user and it goes through the normal failure retry; `CancelledError` must propagate. A Win32 controller calls it with an empty `adb_address` |
| `abort_round_entries: dict[str, str]` | No | Critical tasks (entry → explanation on failure): a failure or per-task timeout ends the round, retried as usual |
| `nonfatal_entries: dict[str, str]` | No | Wrap-up tasks: failure, timeout, spinning in place, or a forced stop from the script side only take a screenshot and log one line; they do not count as a round failure and do not trigger a retry. This takes precedence over "the first task timing out ends the round"; if declared as both critical and wrap-up, it is treated as wrap-up. User stop, the whole-round time limit, signals, and the game failing to start are handled before it ([#1296][1296]) |
| `skip_on_retry_entries: frozenset[str]` | No | On retry, these tasks that completed in any attempt of this run are not run again (judged by task name; if the plan has several copies of a same-named task, all are rerun); an attempt where the worker crashed and returned no result does not count ([#1296][1296]) |
| `unselectable_entries: dict[str, str]` | No | Unselectable tasks (entry → reason): the preview returns `unselectableReason`; the account page does not let them be selected, and those already in the queue are still shown and can be deleted; at run time they are dropped before `decorate_selection` and one line is logged. So this takes precedence over the three declarations above ([#1296][1296]) |
| `no_default_fallback_inputs: frozenset[tuple[str, str]]` | No | `(option name, field name)`: when a string input is not filled in, the engine by default falls back to the interface's `default`; fields listed here do not fall back and send an empty string if unfilled. Both paths that build a plan pass it; `/maafw/preview` blanks the `default` of these fields so the account page does not show it as the value for "not filled". List only fields whose `default` is a literal placeholder that would cause an error if actually sent |
| `retype_notice_lines(*, entering, has_users) -> list[str]` | No | When a script is retyped by project after a project import, the flavor swapped in is called with `entering=True` and the one swapped out with `entering=False`, once each; the returned lines are appended under the system notice "identified by project as …". When `has_users` is false, do not return lines that only make sense for existing accounts. An exception only warns |
| Config class attribute `DEFAULT_TASK_TIME_LIMIT_OVERRIDES` | No | The flavor's built-in per-task time limits (a JSON string, task name → minutes); when a generic MFW managed script is swapped to this flavor by project import, it is filled in only if the script's `Run.TaskTimeLimitOverrides` is still empty (`flavor.user_config_type_transform`) |

### Selection Context and Shared Helpers

The `context` of `decorate_selection` and two helpers are in `app/task/MaaFW/tools/embedded/flavor_support.py`. This module depends only on the standard library and the interface model: flavor packages are imported by config migration very early at startup, and the import must not pull in the core runner that builds plans. `flavor.py` re-exports them as they are.

| Name | Content |
| --- | --- |
| `MaaFWSelectionContext.script_config` | The script (managed) config |
| `MaaFWSelectionContext.user_config` | The account config for this run |
| `MaaFWSelectionContext.resource_name` | The resource actually in effect at run time (`resolve_run_selection`, already falling back by controller when the script resource is empty); flavors must not fall back again themselves |
| `MaaFWSelectionContext.excluded_task_ids` | Instance IDs the engine decided not to run this round (`frozenset`): red-light ones and ones not on a run day today |
| `MaaFWSelectionContext.log` | Writes one line to the account run log; the hook can call it directly in the worker thread |
| `task_name_for_entry(interface_model, entry)` | The name of the first task in the interface whose `entry` equals the given value, or `None` if not found |
| `config_value(config, group, name)` | `config.<group>.<name>`, or `None` if the group or field is missing |

When a hook reads the account config to add tasks itself, it must not add back instances in `excluded_task_ids`, and it must not parse the snapshot itself to judge red lights or weekdays. When the core builds the plan it also filters the decorated result by red light again, so a red-light instance added back still does not run; instances not on a run day today are taken away by the engine before it hands over to the hook, and when they are absent from the hook's output the engine appends them after the decorated result for the core to exclude before building the plan and record as skipped.

### Why Optional Members Are Not in the Protocol

`MaaFWFlavor` is a `runtime_checkable` Protocol. If optional hooks were written into the protocol, a flavor that does not implement them would fail the protocol in the `isinstance` check and the whole thing would fall back to generic MFW. So optional members are always probed with `getattr` (`resolve_game_update_hook`, `resolve_snapshot_sanitizer`, `unselectable_entries`, and so on), and a missing one is treated as absent. `skip_on_retry_entries` is a set while the others are dicts because it has nothing per-task to say: the skip reason is always "an earlier attempt already completed".

### Wiring Constraints

| Constraint | Why |
| --- | --- |
| The config class is in `app/models/config/script.py`; `XConfig(MaaFWConfig)` is isomorphic to `MaaFWConfig`: the class attributes `DEFAULT_SCRIPT_NAME`, `USER_CONFIG_CLASS`, and `FLAVOR` are required, and defaults of groups such as `Info.Name`, `Run.TaskTimeLimitOverrides`, and `Game.WaitTime` are overridden as needed (`M9AConfig` is the reference); `FLAVOR` is a string, imported by the engine on demand | `app.models` cannot import `app.task` in reverse; a `ConfigBase` subclass cannot override entries after `super().__init__()`, so class attributes are used |
| The type is registered in `CLASS_BOOK` in `app/models/config/__init__.py` (plan table classes are in `plan.py`, and `PLAN_BOOK` is also in this `__init__.py`); `TYPE_BOOK` is in `app/utils/common/constants.py` | The registration order of `CLASS_BOOK` decides who wins when several flavors claim; the managed script dropdown gets type names from `TYPE_BOOK`, and a missing entry raises `KeyError` |
| The run entry is in `app/task/MaaFW/registration.py`: a flavor must be written into `run_types` of the family declaration `ScriptRegistration` (the old `_MANAGER_BOOK` no longer exists); the `isinstance(…, MaaFWConfig)` branch in `app/core/config/manager.py` (formerly `app/core/config.py`) naturally covers subclasses, so do not add flavor branches | The run index generated by the host matches types exactly, so subclasses must be registered separately |
| API types are in `app/api/scripts/management.py` and `app/api/scripts/users.py`: the config payloads and input aliases, the enums of `ScriptIndexItem.type` / `UserIndexItem.type` and `ScriptCreateIn.type`, and three union types (script side `ScriptCreateOut`, `ScriptGetOut`, `ScriptUpdateIn`; account side `UserGetOut`, `UserCreateOut`, `UserUpdateIn`) | M9A and MSS appear in all of these places; `app/models/schema.py` does not hold these types |
| Wherever account sub-configs are installed by class, use `type(script_config).USER_CONFIG_CLASS` | Hard-coding `MaaFWUserConfig` makes all of a subclass's accounts disappear, reporting only "MFW has no runnable accounts" (the check message in `embedded_manager.py`) |
| `decorate_selection` runs in a worker thread: write logs with `context.log`, which the engine forwards back to the event loop; prompts from the check stage are merged in once this account's log is created ([#1197][1197]) | Writing UI logs directly raises "no running event loop" |
| A flavor raising in `decorate_selection` = the run plan fails to build: that account is recorded as "exception", an error notice is sent, it does not run and is not retried, and the original exception text is shown to the user after "unable to build the MaaFW run plan" (「无法构建 MaaFW 运行计划」); other accounts proceed as usual | M9A uses this to reject accounts that need to be split |
| Return new lists and dicts without modifying the arguments; when no task is found by entry, write one account log line and skip | Task names may change with project versions |

## Frontend: Registry, Descriptor Objects, and Contracts

| Design | Why |
| --- | --- |
| The registry `frontend/src/composables/useMaaFWFlavor.ts`: `FLAVOR_REGISTRY satisfies Record<MaaFWFlavorType, MaaFWFlavor>`, so adding a type member without registering it makes typecheck fail immediately; shared code reads values only through descriptor objects and never writes `type === 'X'` ([#1013][1013]) | Scattered type branches were once missed when a flavor was added |
| App-wide script type registration is written once, in `SCRIPT_METADATA` in `frontend/src/utils/scriptMetadata.ts` (icon, display name, tag color, route suffix and title, create-card data); `ScriptType` is derived from the generated API, and `scriptLogos.ts` is derived from it, so the type union is no longer copied by hand | That table is `satisfies Record<ScriptType, ScriptMetadata>`, so if the backend adds a type and it is missed here, typecheck fails immediately |
| A descriptor object is declared with `defineMaaFWFlavor` from `views/EditView/MaaFWFlavor/defineFlavor.ts` and writes only differences: identity fields, `routes.suffix`, and `create.card` are required; `scriptPage.text`, `userPage.text`, and `userPage.managed` are shallow-merged by field (omit to inherit MFW, write `null` to explicitly turn off); `page`, `sections`, `slots`, and `prepare` are not inherited | A flavor maintains only its own small piece |
| Section replacement: `defineMaaFWSection(part, key, load)`; the replacement component must accept the props of that section in `MaaFWFlavor/sectionContracts.ts` (typecheck fails if it cannot); events are not covered by type checking, so they must be emitted per that section's `…Emits`, or `$attrs` must be passed through to the MFW section it wraps ([#1163][1163]) | The page passes the same props and listeners to the default section and the replacement section |
| Sections: managed page `basicInfo / control / update / run / shellImport`; account page `header / basicInfo / queueHeader / taskQueue`; create flow `source` | — |
| Slots (`defineMaaFWLazyComponent`; the component receives the context as a single `context` prop): managed page `afterBasicInfo / beforeControl / besidePackageName / afterControl / afterUpdate / afterRun`; account page `afterBasicInfo / beforeTaskQueue / afterTaskQueue` (**none go inside the two columns of the task queue**); create flow `afterSourceStep` (read-only). Among them `besidePackageName` is provided by the scoped slot of the same name in the `control` section (under ADB it comes after "game package name" and "wait time", one cell per component; the page tells the section how many cells with `besidePackageNameCount`), and when replacing the `control` section you must keep this slot yourself | The two-column framework of the account page is untouched |
| Whole-page replacement: `scriptPage.page` / `userPage.page`, assembled from the sections and orchestration layer (`useMaaFWScriptPage` / `useMaaFWUserPage`) exported by `views/EditView/Script/MaaFWScriptEdit/pageKit.ts` and `views/EditView/User/MaaFWUserEdit/pageKit.ts`, without copying the MFW page | These two files are deliberately not named `index.ts`: when vite resolves, `.vue` ranks before directories, so importing by directory name would get the same-named page itself, while typecheck passes all the same |
| The `prepare` hook: prefetches whole-page, section, and slot components and data while the page loads; failures are not thrown outward | Avoids unique blocks being empty at first and then popping in; a failure in a unique block does not drag down the page |
| To get a new slot: add the name to `MaaFWFlavorSlotContextMap` in `maafwFlavorTypes.ts`, and place `<MaaFWFlavorSlot>` at the corresponding spot in the shared page | No type branches in shared pages |

### Routes and the Page Host

- Routes are generated from the registry by `frontend/src/router/maafwFlavorRoutes.ts`, four per type: managed-script edit, project onboarding, add account, edit account. Adding a flavor needs no route changes.
- The page host `MaaFWFlavor/MaaFWPageHost.vue` (logic in `useMaaFWPageHost.ts`) picks the page by the **actual type** once it has read the managed script's details: when the address suffix does not match the type, it `replace`s to the right one; when the type changes within the same session after an import, it swaps the page in place without navigating, and the onboarding progress is not lost ([#1163][1163]).
- Cost: the page waits until the managed script's details have been read (a few milliseconds) before rendering; in exchange, the very first frame has the right flavor's icon and title.

### Why Descriptor Objects Must Not Have Boolean Switches

A descriptor object carries data such as identity, wording, and managed tasks, plus unique components; it does not carry booleans of the form "should this flavor do a certain generic behavior". Behavior differences are either data (the managed task list, slot components) or derived from the managed script's actual type. Adding a boolean switch is the same as stuffing `type === 'X'` back into shared code in a different spelling; combine a few switches and the shared page is once again branching on type. There is currently no automated check for this; it relies on review (see the checklist at the end).

## M9A

Backend `app/task/M9A/` (`flavor.py`, `managed.py`, `game_update.py`, `login.py`, `migration.py`, plus `login_assets/`), frontend `MaaFWFlavor/m9a/`.

| Item | Design | Why |
| --- | --- | --- |
| Claiming | Any of `mirrorchyan_rid == M9A`, `github` pointing to `MAA1999/M9A`, or `name == m9a` | A managed script already running the M9A project through generic MFW is also switched to M9A automatically |
| Managed tasks | The entries `StartUp` (start the game), `SwitchAccount` (switch account), and `Close1999` (close the game) are fully controlled by MAS; they may not stay in the account queue and cannot be added manually; judged by entry, not by task name. The rule is written in one place only, `managed.py`, and pre-run decoration, startup cleanup, legacy migration, and the save entry all call it ([#1023][1023]) | An order the user arranges by hand conflicts with the automatic additions |
| Account-switch criterion | "The resource supports account switching" is judged in one place only, `managed.supports_account_switch`: the resource is the official server and `Run.LoginMode` is not `Mas`. Pre-run decoration, startup cleanup, legacy migration, and the save entry all call it; do not each read the resource name | When `LoginMode` is `Mas`, login (including account selection) is done by MAS, and the switch-account task is no longer sent |
| Switch-account arithmetic | Count only those that are "checked and with a non-empty account": 1 → the account goes into the "Account" field above (`Info.Account`) and is removed from the queue; if the original value differs, the queue wins and the original goes to the note; 0 → removed directly; 2 or more and switching supported → kept as is, the note and startup notice say how many accounts to split into, and the account is rejected at run time; 2 or more but switching not supported → removed directly | Each MAS account entry corresponds to one game account; switching only works on the official server with non-MAS login |
| Preserved state | The criterion is the queue content itself (effective switches ≥ 2 and switching supported), not a flag; the frontend must not filter out managed tasks already in the queue before saving | A normal save handing back the same queue is judged as preserved all the same |
| Official-server check | Always take the engine's `run_plan.resolve_run_selection` (falls back by controller when the resource is empty); a flavor does not write its own fallback; the decorate hook takes `context.resource_name`, and the save hook takes the `resource_name` parameter | The three entry points must be consistent |
| Pre-run decoration | First remove all three kinds of managed tasks, then add them in a fixed order: start game first → if switching is supported and there is an account, switch account right after (the account goes into its first input) → … → close game last; if nothing is left after removal, add nothing. Does not touch retries, periodic runs, timeouts, or updates | The order the user arranges does not affect these three |
| Critical / wrap-up | Switch account is a critical task; close game is a wrap-up task | After a failed account switch, later tasks would all run on the wrong account; redoing the whole round just because closing failed is not worth it |
| No rerun on retry | Lists the entries of all non-managed tasks of the current version one by one; tasks newly added to M9A are rerun as before by default | Add them only after confirming that rerunning is pointless |
| Unselectable | 5 mini-games (`8bit`, `CritterCrash`, `MusesBoxStartParty`, `PreStormProtocolStart`, `CompleteInduction`) | They require the user to stop on the corresponding page by hand first, and M9A has no navigation |
| Redeem code does not fall back | `no_default_fallback_inputs = {("自定义兑换码", "兑换码")}` | Its `default` is the literal "占位" (placeholder), and the legacy migration specifically clears it to an empty string when prefilled; if handed to the agent, a code that cannot be submitted would interrupt the whole redemption, and the real redeem codes after it would not be submitted either |
| Retype notice | `retype_notice_lines`: on switching in, explains that start/close game is added automatically and that the official-server account is used for switching, and, when accounts exist, reminds the user to change the "Account" field that used to be only a note; on switching out (spoken only if there are accounts), explains that the leading/trailing tasks are no longer added and switching no longer happens | The engine does not write adapter semantics; the flavor supplies the explanation itself |
| Built-in time limits | Auto Deep Sleep 60 and Auto Dream Awakening 60 minutes (`M9AConfig.DEFAULT_TASK_TIME_LIMIT_OVERRIDES`; only tasks longer than the global default of 45 minutes go here) | The duration of these two tasks varies a lot |
| Login mode | `Run.LoginMode`: `Project` (default, the M9A project logs in by itself) / `Mas`. `ensure_logged_in` only acts in `Mas` and otherwise returns an empty string: it requires an ADB controller, the resource to be "B服" (Bilibili server), the package name to be the B-server package, and a non-empty account; it uses `adb` screenshots plus OCR to handle the privacy policy prompt and to pick the account by nickname in the login records; on success it returns `StartupTask(deadline, "StartUp")`, and the same worker runs the public `StartUp` first. If conditions are not met or login fails, it raises `MaaFWLoginRequired`. Auto-agreeing to the privacy prompt reuses the global `IfAgreeBilibili` (off by default) | Upstream "switch account" declares only the official server, and the B server has no entry for picking an account by nickname; this is a stopgap |
| Game client update | `Run.GameUpdateMode` (off / check only / auto download and install, default off; the field lives on `MaaFWConfig` to stay isomorphic); only checks the official server with package name `com.shenlan.m.reverse1999`; the direct link comes from the official website API, and the version is read from the installer manifest with HTTP Range only; when the local versionCode is higher than the official package (installed from another channel), it is judged failed immediately without downloading; download and install each have a 60-minute cap ([#1010][1010]) | After a forced update, the old client pops a dialog as soon as it starts, and the whole round fails with no hint at all |
| Frontend | The "Login mode" and "Game update" dropdowns hang on the slot `besidePackageName`, one cell each: `M9ALoginModeField.vue` (writes `Run.LoginMode`) and `M9AGameUpdateField.vue`; `userPage.managed.entries` is the same set as `MANAGED_ENTRIES` in `managed.py`, so managed tasks are kept out of "Add task" and the presets; `accountTask` (switch account + official server) is the criterion for splitting: only when the resource in effect on the page is the official server and effective switches ≥ 2 does the "will not run before splitting" warning appear, and other leftovers get only a light hint (`views/EditView/User/maafwManagedTasks.ts`). When `Run.LoginMode` is `Mas`, the account page sets `accountTask` to empty (`useMaaFWUserPage.ts`), the same criterion as the backend's `supports_account_switch` | Change one side of frontend/backend and the other must be synced |
| Legacy migration | Edits the raw JSON **before** `ScriptConfig.connect()`, backs up before rewriting and keeps only three copies, idempotent; `Run.RunTimeLimit` is fixed at 120; the auto-update mode is written explicitly; account-level server is converted to managed-level resource (inconsistent accounts are disabled and a note is written); the CDK is derived in turn from the shell config, the MAS global, and GitHub; for input values, only the single sentinel "占位" (placeholder) is cleared ([#973][973], [#1012][1012]) | The config loader only recognizes declared entries, so old keys are lost at the moment of loading; the old run time limit was a log-stall threshold, with different semantics |
| Startup cleanup | On every startup, all M9A accounts are tidied by `managed.py`; it writes to disk, backs up, and sends one notice only if something changed | Gathers leftovers from all sources |
| Backup and restore | Goes through MFW's backup pool; legacy M9A archives are no longer listed or restorable, and are left in place | Different archive root |

## MSS (Stella Sora / MaaStellaSora)

Backend `app/task/MSS/flavor.py`, frontend `MaaFWFlavor/mss/`.

| Item | Design | Why |
| --- | --- | --- |
| Claiming | `mirrorchyan_rid == SSAH`, or the `github` repository name / `name` equals `MaaStellaSora` or starts with `MaaStellaSora-`; if only an organization home page is given, no repository name can be obtained and it does not claim ([#1141][1141]) | Derivatives such as the official version and personal versions are all claimed; the same prefix without a hyphen is not |
| Claiming decides only the type | The update lineage is still separated by the original `mirrorchyan_rid` / `github` / `name` in the interface | A derivative is not overwritten by official updates, unless it copied the official `mirrorchyan_rid` verbatim |
| Plan table | When an account's `Info.PlanMode` references an MSS plan table, the five options of entry `战斗_入口` (Bounty Trial quick battle) are rewritten per the day's slot; if the queue does not have this task it is added; if the stage name is not among the interface's cases the whole entry is skipped; nothing is touched when "Fixed" is chosen ([#959][959]) | Preserves the original adapter's plan table capability |
| Events | When the queue has `活动快速战斗_入口`, the official website schedule is queried once (`app/tools/common/game_calendar/stella.py`): if there is an event, it is moved ahead of Bounty Trial; if there really is none, it is removed; if no data can be obtained, it runs as is; it is not removed when only event tasks remain in the queue. When `Info.IfActivityFirst` (on by default) is on and it is confirmed to be in the event period, it is added automatically even if the queue did not include it | Emptying the queue makes the engine judge "unable to build the run plan"; adding tasks blindly when data cannot be obtained would run a task that should not run |
| Weekly | `星塔_入口_agent` (the new tower climb) is moved to the end of the queue; once-per-week is handled by the generic `Run.WeeklyOnceTasks` | The flavor does not track weeks |
| No rerun on retry | Declares `skip_on_retry_entries`: confirmed daily and challenge tasks (event quick battle, Bounty Trial, new tower climb, Catastrophe Defense Line and 15 entries in all) that succeeded in this run are not rerun on retry; failed, timed-out, or not-yet-reached ones run as usual. Login is not listed, since after a retry restarts the game it must still log in | Rerunning already collected or already finished content is just waiting; tasks newly added upstream are kept for rerun by default |
| Desktop only | The first step of the control method on the managed page states the reason at the top; the engine does not block ADB | The game cannot be started on the emulator side |
| Config | `MSSConfig` is isomorphic to `MaaFWConfig` (only the `Info.Name` default is overridden); `MSSUserConfig` provides the plan table and event-first fields `Info.PlanMode` (`ref("PlanConfig", …)`, default `Fixed`) and `Info.IfActivityFirst`. The plan table class `MSSPlanConfig` is in `app/models/config/plan.py`, registered in `PLAN_BOOK`, and the consumer is `mss` in `PLAN_CONSUMER_VALUES`. For the extra steps a new flavor that needs a plan table must take, see the how-to guide [When you need a plan table](../maafw-flavor.md#plan-table) | — |
| Frontend | The managed page's `beforeControl` hangs the "desktop only" notice; the account page's `beforeTaskQueue` hangs the plan table dropdown and the event-first switch; `prepare` prefetches the plan table options | Unique blocks load on demand |
| Administrator privileges | The original adapter's "start as administrator" is not needed: the main program `AUTO-MAS.exe` has a `requireAdministrator` manifest, inherited all the way by Runtime, the backend, the worker, and the game | The built-in run no longer launches a shell |

## Checklist (when reviewing flavor-related changes)

- Does any flavor name or type branch appear in `app/task/MaaFW/`?
- Are newly added optional hooks / declarations written into the module docstring of `MaaFW/tools/embedded/flavor.py` and probed with `getattr`?
- Are there booleans in the frontend descriptor objects? Is there any `type === 'X'` in shared pages?
- Are the managed tasks the same set of entries on frontend and backend?
- Where `decorate_selection` was changed, does it return new objects, find tasks by entry, and log when one is not found? Does auto-adding avoid `context.excluded_task_ids`?

[959]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/959
[973]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/973
[1010]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1010
[1012]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1012
[1013]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1013
[1023]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1023
[1141]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1141
[1163]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1163
[1197]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1197
[1296]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1296
