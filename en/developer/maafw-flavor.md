# Flavor Development

MFW is the general engine for running MaaFramework projects. If a project already has an `interface.json` and you only need to run and configure its tasks in AUTO-MAS, just use the [MFW project guide](/en/docs/script-guide/maafw). You only need to build a "flavor" (called `flavor` in code) when a project really needs its own managed type, wording, or a few pre-run rules.

This page is the how-to guide: follow the steps to build a flavor. For why flavors are designed this way, the full set of hooks and contracts, and the item-by-item notes on M9A and MSS, see the design page [Flavor](./maafw/flavor.md); for the overall MFW design, see [MFW Design Overview](./maafw/index.md).

M9A and MSS are the existing examples: they reuse MFW's project import, runtime environment, updates, task execution, notifications, and periodic tasks, and each declares only its project identification rule and project-specific behavior. Before you start, read the main repository's [`app/task/MaaFW/AGENTS.md`](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/app/task/MaaFW/AGENTS.md) and the [`MaaFWFlavor` contract](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/app/task/MaaFW/tools/embedded/flavor.py) (its module docstring holds the contracts of all optional hooks).

## First Decide Whether You Need a Flavor

| Need | Where it goes |
| --- | --- |
| The project's existing tasks, controllers, resources, and options | The project's `interface.json`; MFW reads it directly |
| A capability every MaaFramework project needs | The MFW general engine |
| A single project's own identity, hints, pre-run task orchestration, pre-run login, a few unique forms | Flavor config and hooks |
| A native editor session, a dedicated UI organized around game semantics | An adapter, not a flavor |
| How in-game tasks run and how task results are judged | The project itself; AUTO-MAS does not rewrite project logic |

If what you are integrating is not a MaaFramework project but a script tool with its own way of running, see [Standalone Adapters](/en/plugin/adapter/standalone).

A flavor is a managed type inside the main program, not a plugin shipped in the project's release package. After an ordinary MFW managed script imports a project claimed by a flavor, it switches in place to that type; it is re-identified when you switch to another project. The managed script ID stays the same, but the project-related task queue needs to be rechecked.

No specific flavor's name may appear in `app/task/MaaFW/`; all of a flavor's logic enters only through the `FLAVOR` hook.

## Step 1: Backend Config Classes and Registration

The following uses a new flavor `X` as the example. M9A and MSS both appear in these places; the consequence of missing each one is noted under it.

1. In `app/models/config/script.py`, right next to `MSSConfig`, create `XUserConfig(MaaFWUserConfig)` and `XConfig(MaaFWConfig)`. The class attributes `DEFAULT_SCRIPT_NAME`, `USER_CONFIG_CLASS`, and `FLAVOR` are required. Keep `FLAVOR` a string so the config model does not load the task module directly at import time. When the account fields have no differences, `XUserConfig` can be just a one-line docstring.

   ```python
   class XUserConfig(MaaFWUserConfig):
       """X 账号配置。"""


   class XConfig(MaaFWConfig):
       """X 托管配置：MaaFW 的特调类型。"""

       DEFAULT_SCRIPT_NAME: ClassVar = "新 X 托管"
       USER_CONFIG_CLASS: ClassVar = XUserConfig
       FLAVOR: ClassVar = "app.task.X.flavor:FLAVOR"

       class InfoFields(MaaFWConfig.InfoFields):
           Name: str = Field(
               default_factory=lambda: "新 X 托管", description="MaaFW 托管名称"
           )

       # 独立配置类型有意覆盖分组及集合，保留专项默认值与准确字段类型。
       Info: InfoFields = Field(  # pyright: ignore[reportIncompatibleVariableOverride]
           default_factory=InfoFields
       )
       UserData: ConfigCollection[XUserConfig] = collection(  # pyright: ignore[reportIncompatibleVariableOverride]
           XUserConfig
       )
   ```

   `script.py` already imports `ClassVar`, `Field`, `ConfigCollection`, and `collection`. The default of `Info.Name` must be written as the same name as `DEFAULT_SCRIPT_NAME` and the frontend descriptor's `defaultScriptName`.

   To change the default of a generic field, follow `M9AConfig`: override that group in the subclass and inherit the other fields:

   - Built-in per-task time limits: write both the class attribute `DEFAULT_TASK_TIME_LIMIT_OVERRIDES` (a JSON string, task name → minutes) and the `default_factory` of `Run.TaskTimeLimitOverrides`, using the same constant in both places. When a generic MFW managed script is swapped to this flavor by a project import, the class attribute's value is filled in only if the script's `Run.TaskTimeLimitOverrides` is still empty. Put only tasks longer than the global default (`Run.TaskTimeLimit`, 45 minutes) here.

     ```python
     _X_TASK_TIME_LIMIT_OVERRIDES = '{"某个耗时任务": 60}'


     class XConfig(MaaFWConfig):
         # 下面这些合进上面的 XConfig，其余成员不变
         DEFAULT_TASK_TIME_LIMIT_OVERRIDES: ClassVar = _X_TASK_TIME_LIMIT_OVERRIDES

         class RunFields(MaaFWConfig.RunFields):
             TaskTimeLimitOverrides: JsonDictString = Field(
                 default_factory=lambda: _X_TASK_TIME_LIMIT_OVERRIDES,
                 description="按任务名覆盖的单任务时限（分钟），值 0 表示该任务不限。",
             )

         Run: RunFields = Field(  # pyright: ignore[reportIncompatibleVariableOverride]
             default_factory=RunFields
         )
     ```

   - Game startup wait time: `Game.WaitTime` defaults to 120 seconds generically, and M9A changed it to 15. The way: extend `MaaFWConfig.GameFields`, override only `WaitTime`, then override the group with `Game: GameFields = Field(default_factory=GameFields)`.
   - Config items unique to the flavor (M9A's `Run.LoginMode`, MSS's `Info.PlanMode` on the account) are likewise added in the inherited groups.

   At the end of the same file two more places need updating: add `"XUserConfig"` and `"XConfig"` to `__all__`; and add one entry each to the type unions `ScriptConfigType` and `UserConfigType`.

2. Register `"X": XConfig` in `CLASS_BOOK` in `app/models/config/__init__.py`. The key is the type name at creation and must also equal the hook object's `type_key`. The engine picks the first flavor that claims a project in the registration order of `CLASS_BOOK`, so placing a new type before or after another is meaningful.
3. In `TYPE_BOOK` in `app/utils/common/constants.py`, add `"XConfig": "X"` (M9A also registered `M9AUserConfig`). The managed script dropdown gets type names from it (`get_script_combox` in `app/core/config/manager.py`); a missing entry raises `KeyError` when fetching the dropdown.
4. In `app/task/MaaFW/registration.py`: import `XConfig` from `app.models.config` and add it to `run_types` of `ScriptRegistration`. The host matches the run index by exact type, and subclasses do not inherit the parent's registration automatically; if missed, the host raises `ValueError` ("adapter capability missing: XConfig") when building the adapter registration. The `isinstance(…, MaaFWConfig)` branch in `app/core/config/manager.py` already covers subclasses; do not add a flavor branch.
5. Fill in the API request and response types. The places to change are the ones below; `app/models/schema.py` need not be touched:
   - `app/api/scripts/management.py`:
     - config payload and input aliases: `XConfig = ConfigPayload[_config_models.XConfig]`, `XConfigInput = ConfigInput[_config_models.XConfig]`;
     - add the class name `"XConfig"` to the enum of `ScriptIndexItem.type`;
     - add `"X"` to the enum of `ScriptCreateIn.type`, and also complete the type list in `description`;
     - union types: add `XConfig` to `ScriptCreateOut.data` and `ScriptGetOut.data`, and `XConfigInput` to `ScriptUpdateIn.data`.
   - `app/api/scripts/users.py` (account side):
     - the aliases `XUserConfig` and `XUserConfigInput`;
     - add `"XUserConfig"` to the enum of `UserIndexItem.type`;
     - union types: add `XUserConfig` to `UserGetOut.data` and `UserCreateOut.data`, and `XUserConfigInput` to `UserUpdateIn.data`.
6. Optional: add the new class to the return type annotations of `add_script` and `add_user` in `app/core/config/manager.py` (annotations only); if the flavor's game has game calendar reminders, add `"X": "<game key in the calendar>"` to `SCRIPT_CALENDAR_GAMES` in `app/core/game_calendar.py`.
7. After the backend changes, start the dev backend and regenerate the frontend API (see "Wrap-up" below); do not hand-edit `frontend/src/api/`.

## Step 2: The Backend Hook Object

Export a `FLAVOR` object from `app/task/X/flavor.py`. It must at least satisfy the `MaaFWFlavor` protocol: a `type_key`, a `matches_project(interface_model)`, and a `decorate_selection(...)`. Minimal example:

```python
from __future__ import annotations

from typing import Any

from app.task.MaaFW.tools.core.interface.models import MaaFWInterface
from app.task.MaaFW.tools.embedded.flavor_support import MaaFWSelectionContext


class XFlavor:
    type_key = "X"

    def matches_project(self, interface_model: MaaFWInterface) -> bool:
        return str(interface_model.mirrorchyan_rid or "").strip().casefold() == "x"

    def decorate_selection(
        self,
        interface_model: MaaFWInterface,
        task_ids: list[str],
        task_options: dict[str, Any],
        *,
        context: MaaFWSelectionContext,
    ) -> tuple[list[str], dict[str, Any]]:
        return list(task_ids), dict(task_options)


FLAVOR = XFlavor()
```

- `matches_project` identifies the identity only from the project's `interface.json`. Prefer a stable project identifier, such as `mirrorchyan_rid` or the repository address; the name is suitable as a compatibility criterion. The criterion must not mistake other projects for yours. An exception only warns and does not block the import.
- `decorate_selection` receives the **already-selected task instance IDs** and options before the run plan is generated, and returns a new `(task_ids, task_options)`. It can insert, remove, or reorder project-specific tasks, or adjust options according to the account config; do not redo the generic flows such as updates, retries, timeouts, or game start/stop here.
  - It is called once per account when the run plan is built, and not again on retries. The list passed in has already dropped unselectable tasks and instances not on a run day today; an instance the hook adds back itself runs as usual.
  - The last parameter of the signature is the read-only `context` (`MaaFWSelectionContext`, in `flavor_support.py`), and there are no other keyword parameters besides it:

    | Field | Content |
    | --- | --- |
    | `context.script_config` | The script (managed) config |
    | `context.user_config` | The account config for this run; read account fields such as `Info.Account` from here |
    | `context.resource_name` | The resource actually in effect at run time. The engine has already fallen back by controller when the script resource is empty; do not work it out again yourself |
    | `context.excluded_task_ids` | Instance IDs the engine decided not to run this round (`frozenset`): those the user set to red light, and those not on a run day today |
    | `context.log` | Writes one line to the account run log; the hook runs in a worker thread, and this callback can be called directly |

  - When a hook reads the account config separately to add tasks automatically, **do not add back instances in `excluded_task_ids`**, and do not parse the snapshot yourself to judge red lights or weekdays. When the core builds the plan it also filters the decorated result by red light again, so a red-light instance added back still does not run.
  - A task instance ID does not necessarily equal the task name (when the same task is queued repeatedly it is `<task name>__MAS_DUP__<suffix>`). Resolve it with `resolve_task_instance_name` (`app.task.MaaFW.tools.core.interface.models`); find tasks by the interface task's `entry`, not by task name. `task_name_for_entry(interface_model, entry)` in `flavor_support.py` returns the name of the first task with the same `entry`, or `None` if not found.
  - Return new lists and dicts; do not modify the passed-in objects. When the project lacks an expected task or option, write one readable log line with `context.log` and skip.
  - It runs in a worker thread, with no event loop.
  - Raising an exception means building the run plan for this account fails this time: the account is recorded as "exception", an error notice is sent, it does not run and is not retried, and other accounts proceed as usual; the original exception text is shown to the user after "unable to build the MaaFW run plan" (「无法构建 MaaFW 运行计划」). Do this only when it truly cannot run, and make the message directly presentable to the user.

How to add a project-specific task:

```python
from __future__ import annotations

from typing import Any

from app.task.MaaFW.tools.core.interface.models import MaaFWInterface
from app.task.MaaFW.tools.embedded.flavor_support import (
    MaaFWSelectionContext,
    task_name_for_entry,
)

DAILY_ENTRY = "X_Daily"


class XFlavor:
    type_key = "X"

    def matches_project(self, interface_model: MaaFWInterface) -> bool:
        return str(interface_model.mirrorchyan_rid or "").strip().casefold() == "x"

    def decorate_selection(
        self,
        interface_model: MaaFWInterface,
        task_ids: list[str],
        task_options: dict[str, Any],
        *,
        context: MaaFWSelectionContext,
    ) -> tuple[list[str], dict[str, Any]]:
        ids = list(task_ids)
        options = dict(task_options)
        daily = task_name_for_entry(interface_model, DAILY_ENTRY)
        if daily is None:
            context.log(f"[X] interface 里没有 entry 为 {DAILY_ENTRY} 的任务，未自动加入")
        elif daily not in ids and daily not in context.excluded_task_ids:
            ids.append(daily)
        return ids, options


FLAVOR = XFlavor()
```

### Optional Hooks and Declarations

Implement them only when needed. They are all probed with `getattr`, and **must not be added to the `MaaFWFlavor` protocol**: the protocol is `runtime_checkable`, and once they are added, a flavor that does not implement them would fall back to generic MFW entirely.

| Name | Purpose |
| --- | --- |
| `sanitize_task_snapshot(...)` | Tidies tasks the flavor takes over before the account task snapshot is saved, and can also write account `Info` fields. Called only when the project can be read and the write includes `Task.TaskSnapshot`; at run time, old snapshots should still be handled by the same rule. M9A's managed tasks are the reference implementation |
| `async ensure_game_updated(...)` | Implement only when you need to check the game client version. Called only with ADB control and game updates enabled on the managed script, after the emulator starts and before the first task; returns `app.utils.common.game_update.GameUpdateResult` (fields `status`, `message`, `resource_version`; `status` is one of `Skipped` / `UpToDate` / `Updated` / `NeedManualUpdate`). Updating the project's release package is still managed by the generic MFW flow; these two "updates" are not the same thing |
| `async ensure_logged_in(...)` | Pre-run login; see "Pre-run login" below |
| `abort_round_entries = {entry: explanation}` | Critical tasks: if one fails or times out the round ends (retried as usual) |
| `nonfatal_entries = {entry: explanation}` | Wrap-up tasks: if one fails, times out, or is forcibly stopped, it does not count as a round failure and is not retried |
| `skip_on_retry_entries = frozenset({entry, ...})` | On retry, these tasks that already completed in this run are not rerun. List only tasks where a rerun is pure waste and that do not depend on earlier steps of this round; write them one by one, and new tasks are rerun as before by default |
| `unselectable_entries = {entry: reason}` | Unselectable tasks: the account page does not let them be selected, and at run time they are dropped and one line is logged. For tasks MAS cannot run unattended |
| `no_default_fallback_inputs = frozenset({(option name, field name), ...})` | When a string input is not filled in, the engine by default falls back to the `default` in the interface; fields listed here do not fall back and send an empty string if unfilled. Use it for fields whose `default` is a literal placeholder left by the project that would cause an error if sent (the "占位" of M9A's custom redeem code). `/maafw/preview` blanks the `default` of these fields so the account page does not show it as the value for "not filled". Do not list a `default` that works |
| `retype_notice_lines(*, entering, has_users) -> list[str]` | The explanation lines appended under the system notice "identified by project as …" when a script is swapped to / away from this flavor after a project import. Called once with `entering=True` for swapping in and once with `entering=False` for swapping out; `has_users` is whether the script has accounts right now, and do not return lines that only apply to existing accounts when there are none |

For signatures, call timing, and precedence, see the [module docstring of `flavor.py`](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/app/task/MaaFW/tools/embedded/flavor.py); for the design notes, see [Flavor](./maafw/flavor.md).

### Pre-run Login: `ensure_logged_in`

Implement it only when the game must be logged in by MAS before tasks start. It is called with an ADB controller, after MAS starts the emulator (which also brings up the game) and before the first task is dispatched, right after `ensure_game_updated`, and is `await`ed in the event loop. Whether to actually do anything is up to the hook, according to config; if it does nothing, return an empty string.

```python
from collections.abc import Awaitable, Callable
from typing import Any

from app.task.MaaFW.tools.embedded.flavor import MaaFWLoginRequired, StartupTask


class XFlavor:
    # 其余成员同上

    async def ensure_logged_in(
        self,
        *,
        script_config: Any,
        user_config: Any,
        resource_name: str | None,
        package_name: str | None,
        adb_path: str | None,
        adb_address: str,
        progress: Callable[[str], Awaitable[None]] | None,
        host_adb: Any = None,
    ) -> str | StartupTask:
        if not adb_address:
            raise MaaFWLoginRequired("这种登录方式只支持 ADB 控制器")
        # ……截图、识别、点击，完成登录
        return "X 登录完成"
```

- Parameters: `user_config.Info.Account` is the account field; `package_name` is the package name brought up with the emulator this time (an empty string means it was not brought up, and the hook may start it by package name itself); when `host_adb` is non-empty, screenshots and commands must go through this private channel, otherwise connect with `adb_path`; `progress` is an async callback that writes the user run log. A Win32 controller calls it with an empty `adb_address`, and the hook uses this to reject unsupported login modes.
- Return value: a string is written to the run log, and an empty string is not. Completion is recorded only after a **successful** login; on failure the next attempt calls it again, and after a retry closes the game / emulator it is verified again too.
- Failure: raise `MaaFWLoginRequired`, whose message is shown directly to the user. This account's attempt is judged failed and goes through the normal failure retry; raising any other exception is treated as a failure as well, and the engine wraps it in `MaaFWLoginRequired` with the message "pre-run login error: &lt;original exception text&gt;" (「运行前登录出错：<异常原文>」).
- Cancellation: the `CancelledError` raised when the user stops must propagate as usual and must not be swallowed.
- When the project has to start the game after account selection, return `StartupTask(deadline, entry)` instead of a string: `deadline` is an absolute `time.monotonic()` instant (the end of the remaining login time limit), and `entry` is the entry of the first public startup task (M9A returns `StartupTask(deadline, "StartUp")`). Requirements:
  - The entry of the first task in the run plan must equal it, otherwise this attempt is judged failed (so `decorate_selection` has to put this task at the front);
  - The full plan is still executed by one worker, and the first task's per-task time limit is tightened to within the remaining login time limit; if the first task fails the round ends, with no post-start verification;
  - When a `StartupTask` is returned, the engine writes no log text and does not yet record "logged in"; it records logged in only when the first startup task appears among the completed tasks reported by the worker.

## Step 3: The Frontend Descriptor Object

The managed page, account page, create flow, and routes all reuse MFW's shared pages. A new flavor writes just one descriptor object. The order below matches the list in the header of `frontend/src/composables/useMaaFWFlavor.ts`; the item there about the "词表 zh-CN.ts" (string table zh-CN.ts) is out of date, and strings follow the backend's `res/i18n` (see "Wrap-up").

1. After the backend is written and OpenAPI is regenerated, `ScriptType` automatically gains `'X'`: it is derived from the generated API's create type (`frontend/src/types/script.ts`), so there is no separate copy to keep. The icon and display name in `scriptLogos.ts` are derived from `SCRIPT_METADATA` in the next item, and the config class name in `useScriptApi.ts` comes from the flavor registry; none of these need manual changes.
2. Add an `X` entry to `SCRIPT_METADATA` in `frontend/src/utils/scriptMetadata.ts` (that table is `satisfies Record<ScriptType, ScriptMetadata>`, and typecheck fails if you miss it): `logo` (put the image in `frontend/src/assets/` and import it at the top), `label`, `typeTagColor`, `routeSuffix`, `routeTitles` (`script` / `userAdd` / `userEdit`), `createOrder`, and `create` (`titleKey`, `descriptionKey`, `keywords` for search, `group`).
3. Add `'X'` to `MaaFWFlavorType` in `frontend/src/composables/maafwFlavorTypes.ts`.
4. Create the directory `frontend/src/views/EditView/MaaFWFlavor/x/`, and export the descriptor object `X_FLAVOR` in `index.ts` with `defineMaaFWFlavor`. **Write only what differs from MFW:** identity fields, `routes.suffix`, and `create.card` are required; for string keys write only the ones you replace; if there is no unique block, do not write `slots`.
5. Register it in `FLAVOR_REGISTRY` in `frontend/src/composables/useMaaFWFlavor.ts` (typecheck fails if missed) and import it at the top of the file.
6. For the help link `docUrl`, use `MAS_DOC_URLS` in `frontend/src/utils/openExternal.ts`: the generic one is `MAS_DOC_URLS.scripts`; for one with a dedicated doc page, add an entry to `MAS_DOC_URLS.scriptTypes` (M9A does this).
7. When the flavor has unique fields on the script or account config and frontend components need to read and write them, add the fields as optional items to `MaaFWScriptConfig` / `MaaFWUserConfig` in `frontend/src/types/script.ts` (such as `Run.LoginMode?`, `Info.PlanMode?`).
8. Add strings; see "Wrap-up" below.

Minimal example (written like `mss/index.ts`: first take `SCRIPT_METADATA.X`, then use `metadata.label` and so on):

```ts
import { MAS_DOC_URLS } from '@/utils/openExternal'
import { SCRIPT_METADATA } from '@/utils/scriptMetadata'
import { defineMaaFWFlavor } from '../defineFlavor'

const metadata = SCRIPT_METADATA.X

export const X_FLAVOR = defineMaaFWFlavor({
  type: 'X',
  scriptConfigType: 'XConfig',
  userConfigType: 'XUserConfig',
  defaultScriptName: '新 X 托管',
  typeTagLabel: metadata.label,
  typeTagColor: metadata.typeTagColor,
  logo: metadata.logo,
  docUrl: MAS_DOC_URLS.scripts,
  routes: { suffix: metadata.routeSuffix, titles: metadata.routeTitles },
  create: {
    card: {
      ...metadata.create,
      after: null,
    },
  },
})
```

`create.card.after` is the type after which this card is placed in the create dialog; `null` puts it last (M9A uses `'MaaEnd'`).

Routes need no changes: `frontend/src/router/maafwFlavorRoutes.ts` generates four routes for each type from the registry (managed-script edit, project onboarding, add account, edit account), and the page host picks the page by the managed script's actual type.

### What You Can Write in a Descriptor Object

First, which fields are merged as "differences". `scriptPage.text`, `userPage.text`, and `userPage.managed` are shallow-merged by field onto MFW's defaults: omit to inherit MFW, write `null` to explicitly turn off (only fields marked nullable can take `null`). `page`, `sections`, `slots`, and `prepare` (including `create.sections` / `create.slots`) are not inherited, and omitting them means none.

`scriptPage.text` (all values are string keys):

| Field | Purpose | MFW default |
| --- | --- | --- |
| `titleKey` | The script edit page title; nullable, and when `null` it is "&lt;project name&gt; project config". The onboarding form is always "&lt;project name&gt; project onboarding" | `null` |
| `sourceDirectoryKey` | The label of the project directory field | `core.edit.localProjectDirectory` |
| `sourceHintKey` | The question-mark tooltip on the project directory label (after import the field is locked, and the tooltip changes to the uniform "already imported" line) | `core.edit.pickMfwProjectDirectory` |
| `sourcePlaceholderKey` | The placeholder of the project directory input | `core.edit.pickActualMfwProject` |

M9A and MSS change only `sourceHintKey`, to name the project in the directory tooltip.

`userPage.text`:

| Field | Purpose | MFW default |
| --- | --- | --- |
| `accountPlaceholderKey` | The placeholder of the account field. If the account has another use, state its purpose | `core.edit.localNoteOnly` |
| `accountTooltipKey` | The question-mark tooltip on the account label; nullable, and when `null` no question mark is attached | `null` |
| `queueHintKey` | The hint at the top of the task queue area, with `\n` for line breaks, one box per line; nullable, and when `null` nothing is shown | `null` |

`userPage.managed`: write it only when the flavor puts some tasks under the backend's full control (managed tasks); with none, the MFW empty value is inherited. The fields must be the same set of entries as the backend's tidying rules.

| Field | Purpose | MFW default |
| --- | --- | --- |
| `entries` | Tasks users may not add themselves (the interface tasks' `entry`): they appear neither in the account page's "Add task" nor in preset templates; ones already in the queue are shown as usual (visible and deletable) | `[]` |
| `accountTask` | The managed account-switch task `{ entry, resources }`: when the resource is in `resources` and the number of its effective instances in the queue (with a non-empty target account) is ≥ 2, the backend refuses to run the account and asks to split it into several accounts. Nullable | `null` |
| `warningKey` | The warning string for when accounts need splitting, interpolating `count` (the number) and `accounts` (each target account); nullable | `null` |
| `noticeKey` | The hint string for when other managed tasks remain in the queue, interpolating `tasks` (task names), and the run proceeds as usual; nullable | `null` |

For how M9A writes it, see `m9a/index.ts`: `entries: ['StartUp', 'SwitchAccount', 'Close1999']`, `accountTask: { entry: 'SwitchAccount', resources: ['官服'] }`.

`prepare` (`scriptPage.prepare` / `userPage.prepare`): `async () => { ... }`, called in parallel with other reads while the page loads, used to prefetch data for unique blocks so they do not appear empty and then pop in. Do not throw failures outward; handle them yourself. `scriptPage.prepare` is called after the script details are read, and called again when the type becomes this flavor after an import; `userPage.prepare` runs in parallel with reading the interface. The create flow has no `prepare`. Put the data-fetching logic in an on-demand module; the registry is imported by the script list, routes, and other places, so do not bring in API dependencies. How MSS writes it:

```ts
prepare: async () => {
  const { loadMSSPlanComboxItems } = await import('./planModeOptions')
  await loadMSSPlanComboxItems()
},
```

### When You Need a Unique UI

From shallow to deep; use the shallower one when it will do:

1. **Slots:** write `defineMaaFWLazyComponent(() => import('./XField.vue'))` into `scriptPage.slots` / `userPage.slots` / `create.slots`; the value is an array of components, rendered in order. A component receives the context as a `context` prop, and hands changes back according to the page type: the script page emits `change(category, key, value)`, the account page emits `save(key, value)`, and the create flow's slots are read-only. The existing slots:
   - Managed page: `afterBasicInfo`, `beforeControl`, `besidePackageName`, `afterControl`, `afterUpdate`, `afterRun`;
   - Account page: `afterBasicInfo`, `beforeTaskQueue`, `afterTaskQueue` (none go inside the two columns of the task queue);
   - Create flow: `afterSourceStep` (read-only).

   `besidePackageName` is special: it is not a place the page puts directly, but a scoped slot of the same name provided by the **control method section** (`control`). Under ADB it comes after "game package name" and "wait time" and before the screenshot / input strategy table, one cell per component. The page forwards cell by cell according to the number of registered components. So when you only use slots you need not care; **when replacing the `control` section, you must keep this slot in the new section yourself** (forward once per cell by `besidePackageNameCount`, passing `{ index }`), otherwise these components will not show.

   An account page slot component example (written like `mss/MSSPlanModeField.vue`):

   ```vue
   <!-- eslint-disable vue/no-mutating-props -- Slot components edit the parent-owned reactive draft; persistence stays in the parent. -->
   <template>
     <a-form-item label="X 选项">
       <a-select
         v-model:value="context.formData.Info.Notes"
         :options="options"
         :disabled="context.loading"
         @change="emit('save', 'Info.Notes', context.formData.Info.Notes)"
       />
     </a-form-item>
   </template>

   <script setup lang="ts">
   import type { MaaFWUserSlotContext } from '@/composables/maafwFlavorTypes'

   defineProps<{
     context: MaaFWUserSlotContext
   }>()

   const emit = defineEmits<{
     save: [key: string, value: unknown]
   }>()

   const options = [
     { label: '甲', value: 'a' },
     { label: '乙', value: 'b' },
   ]
   </script>
   ```

   The example uses `Info.Notes` as the field being saved; in real use, swap in the field you added in `types/script.ts`. The `eslint-disable vue/no-mutating-props` line at the top of the file cannot be omitted; every existing slot component is written that way: the config draft is owned by the page, the component edits the draft directly, and persistence is done by the page. For example, MSS puts the plan table dropdown and the event-first switch in `userPage.slots.beforeTaskQueue`, and M9A puts the "Login mode" and "Game update" dropdowns in `scriptPage.slots.besidePackageName` (`M9ALoginModeField.vue`, `M9AGameUpdateField.vue`).
2. **Section replacement:** write `defineMaaFWSection(part, key, load)` into `sections`. The replacement component must accept the props of that section in `MaaFWFlavor/sectionContracts.ts` (typecheck fails if it cannot) and emit events per that section's `…Emits` (or pass `$attrs` through to the MFW section it wraps), otherwise changes will not be saved.
3. **Whole-page replacement:** `scriptPage.page` / `userPage.page`, assembled from the sections and orchestration layer exported by `views/EditView/Script/MaaFWScriptEdit/pageKit.ts` and `views/EditView/User/MaaFWUserEdit/pageKit.ts`; do not copy the MFW page.

If you need a slot that does not exist yet: add the name to `MaaFWFlavorSlotContextMap` in `maafwFlavorTypes.ts`, and place `<MaaFWFlavorSlot>` at the corresponding spot in the shared page. Do not write `type === 'X'` branches in shared pages, and do not add boolean switches to the descriptor object; neither has an automated check, and both rely on review.

## When You Need a Plan Table (MSS as the Example) {#plan-table}

When a flavor needs accounts to reference a plan table that gives a different configuration by weekday or rotation day, beyond the steps above you must do the following extra steps. Each one has matching code in MSS.

Backend:

1. Add a reference field to the account config and let it reach the plan table collection. How `MSSUserConfig` writes it (`script.py`): the class attribute `related_config: ClassVar = {}`, and `Info.PlanMode` uses `ref("PlanConfig", default="Fixed", allow_values=("Fixed",), entry_types=(MSSPlanConfig,), display_field="Info.Name")`, with default `"Fixed"`. `related_config` is filled in from a list by `app/core/config/manager.py` at initialization, and account config classes are not in `CLASS_BOOK`, so you must also add `XUserConfig` to the tuple there that lists `MSSUserConfig` and `BAAHUserConfig`, and import it from `app.models.config` at the top of `manager.py`.
2. The plan table config class: create `XPlanConfig` in `app/models/config/plan.py` (MSS extends `WeeklyKeyPlan`), with each date slot storing one key model. The key model `XPlanKey` is written in `app/models/schema.py` (take `MSSPlanKey` as the example, `extra="forbid"`), the legacy data normalization function is written in `app/models/config/shared.py` (`normalize_mss_plan_key`), and it is called in `XPlanConfig.normalize_payload`. In `plan.py`, add the new class to `__all__` and the `PlanConfigType` union.
3. Register in `PLAN_BOOK` in `app/models/config/__init__.py`, with the plan table class name as key: `create_type` (the type name used at creation, such as `"MSSPlan"`), `config_class`, `consumer` (take `PLAN_CONSUMER_VALUES[index]`), `script_class` (the managed class `XConfig`), and `field_group` and `field_name` (the field on the account that references the plan table, such as `"Info"`, `"PlanMode"`). When a plan table is deleted, the manager uses the last three to find the accounts that reference it (first filter managed scripts by `script_class`, then compare this field on the accounts) and sets the reference back to `"Fixed"` (the plan-delete logic in `app/core/config/manager.py`), so the reference field on the account must have `"Fixed"` as a value.
4. In `PLAN_CONSUMER_VALUES` in `app/utils/common/constants.py`, **only append** new consumers at the end: `PLAN_BOOK` references it by index, and inserting in the middle would point existing plan table types at other consumers (the constant's docstring says so; `"mss"` sits after `"baah"`, which is how the index came about).
5. Add the same string to `PlanComboxConsumer` (a `Literal`) in `app/models/schema.py`; the plan dropdown endpoint (`app/api/info.py`) validates against it.
6. In `app/api/plan.py`: add the `XPlanConfig` / `XPlanConfigInput` aliases, `PlanCreateType` (a `Literal`), and the union types `PlanConfigData` and `PlanConfigDataInput`. The type enums of the plan table index and create endpoints are generated from `PLAN_BOOK` automatically and need no hand-writing.
7. Get the day's slot in the flavor hook: `type(user_config).related_config["PlanConfig"][uuid.UUID(mode)]`, then call the plan table's `get_current_key()` (see `_current_plan_key` in `app/task/MSS/flavor.py`). When the referenced plan table has been deleted or has the wrong type, treat it as "Fixed" and write a log line.

Frontend:

1. `frontend/src/utils/planTypeRegistry.ts`: add `X: PlanIndexItem.type.…` to `PLAN_CONFIG_TYPES` (the enum name is generated by OpenAPI), and add a description to `PLAN_TYPE_REGISTRY`: `configType`, `createType`, `displayNameKey`, `defaultName` (it is written into the plan name and used to judge "still the default name", so keep it in Chinese), `selectorTag`, `reloadAfterSave`, `supportsSimpleView`, `tableComponent`.
2. The plan table component `frontend/src/views/plan/tables/XPlanTable.vue`, with props `tableData`, `currentMode`, `viewMode`, `planId`, and `handlePlanChange`; write it like `MSSPlanTable.vue`.
3. The plan table dropdown on the account page: a unique-block slot + `userPage.prepare` prefetch. Options come from `Service.getPlanComboxApiInfoComboxPlanPost({ consumer: PlanComboxIn.consumer.X })` (see `mss/planModeOptions.ts`), and the first item is always "Fixed" (value `Fixed`).
4. The string `core.plan.type.x` (see "Wrap-up" below).

## Wrap-up

1. **Add strings.** Strings are in the backend resources `res/i18n/zh_CN.json`, `en.json`, and `ja.json`, as flat dotted keys. Chinese is the source language and must be added; `en.json` and `ja.json` may lack keys, and M9A and MSS filled in all three, so do likewise:
   - `core.scripts.type.X`: the type name;
   - `core.scripts.create.typeDesc.X`: the type description in the create dialog;
   - the string keys used in the descriptor object and slot components (such as `core.edit.xFlavorSourceHint`);
   - for those with a plan table, also `core.plan.type.x`.
2. **Regenerate OpenAPI.** First start the dev backend of the target worktree (the `openapi` script in `frontend/package.json` fetches the schema from `http://127.0.0.1:36164/openapi.json`, so the backend must listen on this port), then run `corepack yarn openapi` from that worktree's `frontend` directory. After generation this script runs `scripts/postprocess-openapi.mjs`, which normalizes the generated files' CRLF line endings to LF; do not hand-edit `frontend/src/api/`.

## Checklist

Check each item before committing:

- [ ] Import once with the target project and once with an ordinary MaaFramework project; the type switches correctly and there is no false claim.
- [ ] No flavor name appears in `app/task/MaaFW/`; there are no type branches in shared frontend pages; there are no booleans in the descriptor object.
- [ ] `decorate_selection` handles duplicate task instances, an empty queue, missing tasks or options, and a queue saved after user edits; auto-added tasks do not add back instances in `excluded_task_ids`; if there is a snapshot-tidying hook, the result after restoring config was also checked.
- [ ] The tasks the flavor takes over (if any) are the same set of entries in the backend tidying rules and the frontend descriptor object.
- [ ] Newly added optional hooks were not added to the `MaaFWFlavor` protocol.
- [ ] All registration places in Step 1 are filled in: `script.py` (including `__all__` and the type unions), `CLASS_BOOK`, `TYPE_BOOK`, `run_types` in `registration.py`, `management.py`, and `users.py`.
- [ ] The flavor-specific run order or options were verified, and it was confirmed that MFW's generic running, project updates, and account config still work.
- [ ] In the target worktree, with its local dependencies, `tests/task/test_maafw_core.py` and the related backend checks were run, along with the frontend `typecheck`, `lint`, and related tests; the commands actually run and their results were recorded. If the schema changed, OpenAPI was regenerated from that worktree's dev backend.

Read-only references: [M9A flavor](https://github.com/AUTO-MAS-Project/AUTO-MAS/tree/dev/app/task/M9A), [MSS flavor](https://github.com/AUTO-MAS-Project/AUTO-MAS/tree/dev/app/task/MSS), [frontend flavor directory](https://github.com/AUTO-MAS-Project/AUTO-MAS/tree/dev/frontend/src/views/EditView/MaaFWFlavor).
