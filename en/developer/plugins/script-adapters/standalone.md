# Standalone Adapters

::: warning
The plugin system is changing fast. This page follows the `dev` branch implementation as of 2026-11-18.
:::

An adapter plugin brings a script tool into MAS: it registers a managed type and provides its config, executor, and dedicated API. MAA and Whimbox have been migrated from the main program into this kind of plugin, and this page uses them as examples.

## First Decide Whether You Need a Standalone Adapter

| What you are integrating | Approach |
| --- | --- |
| A script tool with its own way of running, config format, and update channel (such as MAA or Whimbox) | A standalone adapter; read this page |
| A MaaFramework `interface.json` project where you only want to add a few special behaviors on top of MFW's general capabilities | [Flavor development](/en/developer/plugins/script-adapters/mfw/develop) |

## Two Samples

| | MAA | Whimbox |
| --- | --- | --- |
| Plugin ID / import name | `maa_adapter` | `whimbox_adapter` |
| PyPI package name | `auto-mas-maa-adapter` | `auto-mas-whimbox-adapter` |
| Repository | [AUTO-MAS-MAA-Adapter](https://github.com/AUTO-MAS-Project/AUTO-MAS-MAA-Adapter) | [AUTO-MAS-Whimbox-Adapter](https://github.com/AUTO-MAS-Project/AUTO-MAS-Whimbox-Adapter) |
| Managed type key | `MAA` | `Whimbox` |
| Plan table type | Yes (`MaaPlanConfig`) | No |
| Supported run modes | Auto proxy, config, view | Auto proxy, config |
| Plugin's own config | Yes (stage cache) | No |
| Services provided | Three stage-info services | None |

## Where the Code Goes

- **The backend adapter goes in a separate repository.** The main program repository tracks only the SDK `plugins/auto_mas_core`; every other directory under `plugins/` is ignored by Git. During development, clone the plugin repository into the main program's `plugins/<plugin ID>/`; after release, users install it from the Plugin Center (插件中心).
- **The dedicated pages are still built with the host for now.** The main program cannot yet load a frontend shipped with a plugin. The managed-script edit page, the account edit page, and the plan table live in the main repository under `frontend/src/views/<adapter>/` and ship with the desktop app. Updating the plugin's wheel does not replace pages that are already packaged.

So one adapter change is often two commits in two repositories: the backend goes into the plugin repository, and the pages go into the main repository.

## Skeleton of the Plugin Repository

Using Whimbox as the example. `pyproject.toml`:

```toml
[build-system]
requires = ["setuptools>=77", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "auto-mas-whimbox-adapter"
version = "0.1.1"
readme = "README.md"
requires-python = ">=3.12,<3.13"
keywords = ["auto-mas-plugin", "auto-mas-id:whimbox_adapter"]
dependencies = ["auto-mas-core>=5,<6"]

[tool.setuptools.packages.find]
where = ["src"]
```

The package entry `src/whimbox_adapter/__init__.py` exports `PLUGIN`:

```python
from auto_mas_core import PLUGIN_API_VERSION

from .plugin import Plugin as PLUGIN

__all__ = ["PLUGIN", "PLUGIN_API_VERSION"]
```

`src/whimbox_adapter/plugin.py` holds all the declarations:

```python
from auto_mas_core import Config, ScriptAdapterPlugin, ScriptTypeDecl, TaskMode
from fastapi import APIRouter

from .api import router as task_router
from .backup_api import router as backup_router
from .config import WhimboxConfig, WhimboxUserConfig
from .manager import WhimboxManager, check_prefetch

router = APIRouter()
router.include_router(backup_router)
router.include_router(task_router, prefix="/api/scripts")


class Plugin(ScriptAdapterPlugin):
    script_type_key = "Whimbox"
    api_router = router
    decl = ScriptTypeDecl(
        script_type=WhimboxConfig,
        user_type=WhimboxUserConfig,
        expander_class=WhimboxManager,
        supported_modes=(TaskMode.AUTO_PROXY, TaskMode.SCRIPT_CONFIG),
        prefetch_check=check_prefetch,
        activity_game="infinitynikki",
    )

    async def pre_enable(self):
        refs = {"PlanConfig": Config.PlanConfig}
        WhimboxConfig.related_config = refs
        WhimboxUserConfig.related_config = refs
```

### The `ScriptTypeDecl` Fields

| Field | Meaning |
| --- | --- |
| `script_type` | The managed config class, extending `ScriptEntry`. Must have `info` and `users` |
| `user_type` | The account config class, extending `UserEntry`; must have `info`. `script_type.users` must be narrowed with `collection(user_type)` to contain only this one type |
| `expander_class` | The executor, extending `UserExpander` |
| `plan_type` | The plan table config class; omit it if you do not use plan tables. If set, `user_type.info` must have `plan_id` |
| `supported_modes` | The supported run modes; cannot be empty. When omitted, auto proxy, view, and config are all supported by default |
| `expect_builder` | Resolves the emulator expectation (`EmulatorExpect`) from the managed config. The host currently only checks that it is callable; nothing reads it yet |
| `prefetch_check` | Pre-run prefetch declaration: an async function that takes the managed config and returns a list of `DownloadRequest`; an empty list means nothing to download |
| `activity_game` | The game identifier used for the home page activity display and activity reminders |

A plugin declares only one managed type.

### Config Classes

Config classes take host capabilities only from `auto_mas_core` and do not `import app` directly:

```python
from auto_mas_core import ScriptEntry, UserEntry
from auto_mas_core.host.config import ConfigGroup, collection, ui
```

When a managed script or account references another host config collection (plan tables, emulators), bind `related_config` in `pre_enable`, as in the example above. `pre_enable` runs before the domain config is activated, so references in old configs can be restored as they were.

### Dedicated API

Hand a FastAPI router to `api_router`. Routes are mounted into the host at the paths they declare, so the adapter's existing `/api/scripts/...` paths can be kept unchanged; put new endpoints under `/api/plugin/<plugin ID>/...`. These paths are removed when the plugin is disabled.

## Frontend Pages

Add `frontend/src/views/<adapter>/entry.ts` in the main repository. Using Whimbox's `frontend/src/views/whimbox/entry.ts` as the example:

```ts
import type { ScriptFrontendAdapter } from '@/plugins/scriptAdapters'
import whimboxIcon from './whimbox.png'

export default {
  type: 'Whimbox',
  metadata: {
    logo: whimboxIcon,
    label: 'Whimbox',
    typeTagColor: 'pink',
    routeSuffix: 'whimbox',
    routeTitles: {
      script: '编辑奇想盒脚本',
      userAdd: '添加奇想盒用户',
      userEdit: '编辑奇想盒用户',
    },
    createOrder: 12,
    create: {
      titleKey: 'core.scripts.type.Whimbox',
      descriptionKey: 'core.scripts.create.typeDesc.Whimbox',
      keywords: ['whimbox', '奇想盒', '无限暖暖', 'nikki'],
      group: 'specialized',
    },
  },
  editors: {
    script: () => import('./script/WhimboxScriptEdit.vue'),
    user: () => import('./WhimboxUserEdit.vue'),
  },
  routes: [],
} satisfies ScriptFrontendAdapter
```

Then change two registrations:

1. `frontend/src/plugins/scriptAdapters.ts`: import the entry (`import whimbox from '@/views/whimbox/entry'`) and add it to the `adapters` array.
2. `frontend/src/utils/scriptMetadata.ts`: add a line to `SCRIPT_METADATA`: `Whimbox: SCRIPT_PLUGIN_METADATA.Whimbox`. The create card and the edit page routes both read their metadata from `SCRIPT_METADATA`; without this line they will not appear.

`type` must match the backend's `script_type_key`. The frontend's `ScriptType` comes from the generated `ScriptCreateIn` enum, so OpenAPI must be regenerated while the plugin is enabled for type checking to recognize the key. The entry for creating a managed script is opened according to the types the backend currently has enabled, and the card does not appear when the plugin is not enabled. For an adapter that uses plan tables, add a `plan` section as well; see `frontend/src/views/maa/entry.ts` for how. For more detail, see [Frontend and Backend Integration](../fundamentals/frontend.md).

## Migrating an Old Built-in Adapter into a Plugin

Do it in this order.

### 1. Set the version boundaries

- Decide the **minimum host version**: from which main program version the plugin system is included and this built-in adapter is removed.
- Decide the **SDK lower bound**: the range of `auto-mas-core` in `dependencies`. Before the first release, tighten the lower bound to the SDK release that actually pairs with it, and verify with that SDK distribution. Development source with the same version number may contain interfaces not yet released, so you cannot assume an older SDK is compatible.

### 2. Keep the identity of the old data

- Do not change the class names of the config classes. The `type:` in the config file stores the class name (things like `WhimboxConfig` and `MaaUserConfig`); if the class name is unchanged, old managed scripts are read back as they were.
- Keep UIDs, order, and field names unchanged. When the structure must change, convert it at load time instead of rebuilding: on activation, Whimbox converts the old `UserData` to `users`, and the accounts remain the same ones.

### 3. Tell the host which plugin the old type belongs to

Add the mapping to `_ADAPTER_PLUGINS` in the main program's `app/core/config/plugin_upgrade.py`. The key is the config class name recorded in the index; list the class names of managed scripts and plan tables (including old, renamed class names). The value is the plugin ID and the PyPI package name. Current content:

```python
_ADAPTER_PLUGINS = {
    "WhimboxConfig": ("whimbox_adapter", "auto-mas-whimbox-adapter"),
    "MaaConfig": ("maa_adapter", "auto-mas-maa-adapter"),
    "MaaPluginConfig": ("maa_adapter", "auto-mas-maa-adapter"),
    "MaaPlanConfig": ("maa_adapter", "auto-mas-maa-adapter"),
    "MaaPluginPlanConfig": ("maa_adapter", "auto-mas-maa-adapter"),
}
```

Configs before v1.8 were saved per directory and have no collection index. You also need to add a directory check to `required_adapter_plugins` in the same file. For Whimbox the addition was:

```python
    if any(config_path.glob("WhimboxConfig/*/config.json")):
        required["whimbox_adapter"] = "auto-mas-whimbox-adapter"
```

Every time the host starts and loads config (not only on a version upgrade), it scans the indexes `ScriptConfig.json`, `PlanConfig.json`, `scripts.yaml`, and `plans.yaml`. When it finds these types, it installs, loads, and enables the matching plugin **before** the domain config is activated. If the plugin is already installed, or local source with the same ID exists, it does not reinstall and only adds loading and enabling. If installation or enabling fails, it raises, config loading aborts, and managed scripts are never activated without their plugin.

### 4. Remove the built-in implementation from the host

When Whimbox was moved out (commit `e7970529a`, #2039), the main program changed the following places, which you can use as a checklist:

| Location | Action |
| --- | --- |
| `app/task/<adapter>/` | Delete the whole directory |
| `app/models/config/script.py`, `app/models/config/__init__.py` | Delete the config class and its registration |
| `app/api/scripts/__init__.py`, `management.py`, `users.py` | Delete the route mounting, the type enums, and the union types |
| `app/core/config/manager.py` | Delete the imports and return types |
| `app/task/__init__.py`, `base.py`, `builtin_registration.py` | Delete the built-in registration |
| `app/core/task_manager.py` | Use the host's type table instead of hard-coded types |
| `app/core/desktop_session/lifecycle.py`, `app/core/game_calendar.py`, `app/core/mcp.py` | Delete the branches and entries hard-coded per adapter |
| `app/core/notify/formats/<adapter>.py` | Move into the plugin |
| `plugins/auto_mas_core/src/auto_mas_core/host/` | For host modules the plugin needs that the SDK does not yet export, add a re-export file here |
| `frontend/src/views/<adapter>/` | Gather the pages here and add `entry.ts` (see "Frontend Pages" above) |
| `frontend/src/plugins/scriptAdapters.ts`, `frontend/src/utils/scriptMetadata.ts` | Register the entry; delete the previously hard-coded edit pages from `frontend/src/router/scriptEditorRoutes.ts` |
| `frontend/src/api/` | Regenerate with the plugin enabled |
| `tests/requirements-plugins.txt` | Add the plugin wheel's version and SHA256 |

### 5. Publish in order

**Publish the plugin wheel first, then ship the main program version that removes the built-in implementation.** In the opposite order, users cannot get the plugin after upgrading, and the upgrade aborts.

### 6. Verify the old-managed-script upgrade with the wheel from PyPI

You must do this in an environment **without local adapter source**. Local source overrides a distribution with the same ID, and leaving it in place means you never exercise the real install path.

1. Stop the backend and move the `plugins/<plugin ID>/` source project out of the plugin scan directory.
2. Prepare a sanitized old-version config (the `config` directory from before the upgrade).
3. Start the candidate main program and confirm the plugin was installed automatically from PyPI on first start.
4. Restart once and check that the UIDs, order, and settings of managed scripts and accounts are all still there.

## Testing

- The plugin's own tests live in the plugin repository and run against matching main program source: set `AUTO_MAS_HOST_SOURCE` to the main repository, then run `python -m pytest tests -q`.
- The main repository's integration regressions use published wheels: the version and SHA256 are pinned in `tests/requirements-plugins.txt`, and `scripts/prepare_test_plugins.py` installs them into the ignored `.test-plugins`. When you upgrade the pinned version, change both together.

## What to Tell Users in Advance

- **Disabling an adapter plugin deletes all of that adapter's managed scripts and accounts**, and, if `plan_type` is declared, that kind of plan table as well. Re-enabling does not restore them.
- Uninstalling also deletes the plugin's own settings file `config/plugins/<plugin ID>.yaml`.

State this in the plugin README and remind users to back up first.

## Desktop Session

New adapters opt in to Desktop Session through `ScriptTypeDecl`. The host looks up the enabled adapter and uses its declaration for script details, queue previews, and dispatch. Desktop Session dispatch requires `TaskMode.AUTO_PROXY`.

For example, an adapter that requires an elevated host adds the `Run` field to its existing script config and passes the callback to its existing `ScriptTypeDecl`:

```python
from auto_mas_core import DesktopSessionCheckReason, DesktopSessionRunFields, ScriptEntry
from pydantic import Field

# Add to the script config class. If it already has RunFields, inherit DesktopSessionRunFields.
Run: DesktopSessionRunFields = Field(default_factory=DesktopSessionRunFields)

def desktop_session_check(
    config: ScriptEntry, elevated: bool
) -> DesktopSessionCheckReason | None:
    return None if elevated else "admin_required"

# Add desktop_session_check=desktop_session_check to ScriptTypeDecl(...).
```

The host runs the callback synchronously during preview and admission, then checks again after acquiring the script configuration lock. The callback only reads script settings and host privileges. Return `None` to allow the run, `admin_required` if elevation is required, or `unsupported_config` for an incompatible configuration. Adapters without a callback can run in the main session. Manual single-script starts read `Run.ExecutionSession`, which defaults to `main` when missing. Preserve the field path and existing config identities when migrating an adapter. A queue has one execution session for the entire run.

The host in the main session handles L2, L3, and L4 orchestration and configuration locks. During a Desktop Session task, use `auto_mas_core.utils.ProcessRunner.run_process()` to run and await an upstream command, or keep the `ProcessManager` used by `open_process()` and call `await manager.kill()` during cleanup. The host uses the task context to start these commands in the child session. Direct calls to `subprocess` and in-process window operations run in the host session. If an adapter needs to tell a launcher exit apart from its descendants staying alive, use `retain_desktop_descendants=True` to retain ownership and read the launcher's exit status through `manager.launched_process.returncode`.
