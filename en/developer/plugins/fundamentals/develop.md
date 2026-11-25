# Plugin Development

See [Plugin Host and Runtime](./runtime.md) for discovery, lifecycle, and installation. See [Game Adapters](../game-adapters.md) for device integration.

::: warning
The plugin system is changing fast. This page follows the `dev` branch implementation as of 2026-11-18.
:::

This page builds a minimal plugin that logs one line when it is enabled and one when it is disabled. When you finish, you will know how to lay out the directory, write the entry point, enable the plugin, and get code changes to take effect.

First set up the development environment by following [Environment Setup](/en/developer/getting-start/environment). Relative paths below are relative to the backend working directory. When running from source, that is the AUTO-MAS repository root.

## Directory Layout

Create a directory named after the plugin ID under `plugins/` in the repository root. The plugin ID in this example is `hello_demo`.

```text
AUTO-MAS/
└── plugins/
    └── hello_demo/
        ├── pyproject.toml
        ├── README.md
        └── src/
            └── hello_demo/
                ├── __init__.py
                └── plugin.py
```

- `plugins/<plugin ID>/` is the project root. Name the directory after the plugin ID.
- `src/` must contain exactly one top-level package. Its name is the plugin's import name. If there are zero or multiple packages, the directory is skipped.
- The package must be a regular package with an `__init__.py`. All plugin code must live inside it. Importing modules outside the package makes loading fail.
- The main program's Git ignores every directory under `plugins/` except `auto_mas_core`. Commit your plugin in its own repository.

## pyproject.toml

```toml
[build-system]
requires = ["setuptools>=77", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "auto-mas-hello-demo"
version = "0.1.0"
description = "Minimal AUTO-MAS plugin example"
readme = "README.md"
requires-python = ">=3.12,<3.13"
keywords = ["auto-mas-plugin", "auto-mas-id:hello_demo"]
dependencies = ["auto-mas-core>=5,<6"]

[tool.setuptools.packages.find]
where = ["src"]
```

Get these fields right:

| Field | Requirement |
| --- | --- |
| `keywords` | Must contain `auto-mas-plugin` and exactly one `auto-mas-id:<plugin ID>`. If either is missing, the project is not recognized as a plugin. |
| Plugin ID | The part after `auto-mas-id:`. It is used as the config file name, so it cannot contain whitespace or characters that are illegal in Windows file names, such as slashes and colons. Stick to letters, digits, and underscores. `auto_mas_core` is reserved for the core package. |
| `dependencies` | Must include `auto-mas-core`. Without it, the backend reports "plugin does not declare an SDK dependency" while preparing the plugin environment, and backend initialization fails. Use `>=5,<6` as in the official samples; narrow it to the host versions you actually tested if you want it stricter. |
| `requires-python` | The main program uses Python 3.12; the samples use `>=3.12,<3.13`. |
| `version` | A PEP 440 version. After you publish to the marketplace, clients compare versions with it. |
| `name` | The distribution name. For the marketplace data site to discover the plugin automatically, the name must contain `auto-mas` or `automas`. Otherwise a maintainer has to add it by hand; see [Publishing Plugins](./publish.md). |

The file that `readme` points to must exist. Otherwise the whole project fails to parse and is not discovered.

## README.md

```markdown
<!-- auto-mas:i18n
{
  "display_name_i18n": { "zh_CN": "你好示例", "en": "Hello Demo" },
  "description_i18n": { "zh_CN": "最小插件示例", "en": "Minimal plugin example" },
  "tags_i18n": { "zh_CN": ["示例"], "en": ["Example"] }
}
-->

# Hello Demo

Minimal AUTO-MAS plugin example.
```

The comment at the top sets the name, description, and tags shown in the Plugin Center and the marketplace. You can leave it out during local development. When it is missing, the plugin ID is used as the name, `description` as the description, and no tags are shown. Add it before publishing; the format and the consequences of omitting it are in [Publishing Plugins](./publish.md).

## Entry Point

`src/hello_demo/__init__.py`:

```python
from auto_mas_core import PLUGIN_API_VERSION

from .plugin import Plugin as PLUGIN

__all__ = ["PLUGIN", "PLUGIN_API_VERSION"]
```

`src/hello_demo/plugin.py`:

```python
from auto_mas_core import BasePlugin
from auto_mas_core.utils import get_logger

logger = get_logger("HelloDemo")


class Plugin(BasePlugin):
    async def on_enable(self) -> None:
        logger.info("hello_demo enabled")

    async def on_disable(self) -> None:
        logger.info("hello_demo disabled")
```

- The host only recognizes `PLUGIN` exported from the package. It must be a subclass of `BasePlugin` (the class itself, not an instance). If it is missing or the wrong type, the plugin ends up in "load failed".
- `PLUGIN_API_VERSION` is the plugin protocol version. Re-export it from `auto_mas_core`. If the exported major version differs from the host's, the host refuses to load the plugin.
- Import capabilities only from `auto_mas_core`; do not `import app.*` directly. For what `auto_mas_core` offers, see the SDK [README](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/plugins/auto_mas_core/README.md) and the export list in `__init__.py`.

### Lifecycle Hooks

An instance is created on enable and discarded after disable. Do not put business logic in `__init__`; put it in the hooks.

| Hook | When it is called |
| --- | --- |
| `pre_enable` / `on_enable` | Precheck before enable / enable |
| `pre_disable` / `on_disable` | Precheck before disable / disable; release the resources created in `on_enable` |
| `pre_reload` / `on_reload` | Precheck before reload / called on the old instance during reload. Reloading an enabled plugin then runs `on_disable`, re-imports the code, and runs `on_enable` |
| `pre_unload` / `on_unload` | Precheck before uninstall / uninstall. Only plugins installed from the marketplace can be uninstalled; local source plugins never reach these two hooks |
| `on_teardown` | Cleanup when the backend process exits. It does not change the user's enable choice |
| `on_fail_cleanup` | Cleans up resources after a lifecycle operation fails |

## Enabling the Plugin

1. Start (or restart) the backend and the frontend. On startup the backend scans `plugins/` and registers new plugins.
2. Open the Plugin Center, find "Hello Demo" (whose source shows "Local"), and turn on the switch in the "Enabled" column of that row.
3. Check the backend log `debug/app.log`. You should see a line like this:

```text
... | INFO     | HelloDemo | hello_demo enabled
```

The enabled state is saved and applied automatically the next time the backend starts.

::: tip
A plugin directory added while the backend is running is also discovered and registered, but until the backend restarts its state is "Uninstalled" and the "Enabled" switch is unavailable. Restart the backend once after creating a new plugin.
:::

If the plugin does not appear, the problem is in the discovery stage. A project with the wrong `keywords` is skipped silently with no log. A missing `pyproject.toml` or `src/`, a wrong number of top-level packages under `src/`, or a missing `readme` file only produces DEBUG-level output on the backend console; `debug/app.log` records INFO and above only, so you will not see it there.

If the state is "Load failed", the Error column in the Plugin Center and `debug/app.log` contain the import error. Common causes: `PLUGIN` is not exported, or the code imports a module outside the package. After fixing the code of a plugin in "Load failed", restart the backend.

## Applying Source Changes

The backend watches the `plugins/` directory. After you save a `.py` or `.pyi` file and stop editing for about 0.75 seconds, the "Code status" in the expanded details of that plugin in the Plugin Center becomes `needs_reload`. **The host does not swap code automatically.** Click "Reload" in that row's "More actions" menu yourself.

- When an enabled plugin reloads, the host runs the old instance's `on_reload`, then `on_disable`, re-imports the source, then the new instance's `on_enable`. Local source is imported without the `.pyc` cache, so the latest file is always read.
- Changing only metadata in `pyproject.toml`, such as the version or package name, is also marked `needs_reload`.
- If the reload fails, the plugin goes to "Runtime fault". Fix it and click "Reload" again. Turning on the "Enabled" switch at that point uses the old code from the last successful import.
- Only changes to `.py`, `.pyi`, `.toml`, and `.json` files trigger rediscovery. Editing only the README does not.
- The watcher ignores `.git`, `__pycache__`, `build`, `dist`, `node_modules`, `pypi`, `site-packages`, and `.egg-info`, `.dist-info`, `.data` directories.

## Adding Dependencies

Put dependencies in `dependencies` in `pyproject.toml`, then **restart the backend**. On startup the backend resolves the main program's dependencies together with those of all plugins and installs them into `plugins/pypi/site-packages`.

- All plugins share this one environment. Plugins have no virtual environments of their own.
- The dependency versions currently installed for the main program are fixed, and plugins cannot change them. If they cannot be satisfied together with the main program's, backend initialization fails and the log reports "Plugin auto-upgrade failed: the target main program and the plugins' dependencies cannot be satisfied together". Adjust your version ranges.
- Resolving dependencies needs network access. If no mirror can complete the resolution, backend initialization also fails and the log reports "Compatibility check did not complete; check the network or try again later".
- Do not delete the `auto-mas-core` line.

## Building on Someone Else's Plugin

To modify another author's plugin or contribute to it:

1. Fork it on the plugin's repository page. Without write access to the original repository you must go through a fork.
2. Clone your fork into `plugins/<plugin ID>/`.

   ```bash
   cd plugins
   git clone https://github.com/<your-username>/AUTO-MAS-Whimbox-Adapter.git whimbox_adapter
   ```

   The command above uses the Whimbox adapter as an example; use the ID of the plugin you are modifying as the directory name.
3. Restart the backend, enable it in the Plugin Center, and reload after edits as described in the previous section.
4. Commit, push, and release in the plugin's own repository.

Keep in mind:

- When the same plugin exists both as a marketplace install and as source under `plugins/`, the source wins. To verify the real "installed from the marketplace" behavior, stop the backend, move the source directory out of `plugins/`, then start the backend and install from the marketplace.
- Do not change the value of `auto-mas-id:`. If you want to publish your modified version as a separate plugin, you must change the package name, the plugin ID, and the package name under `src/` (the import name) together. Plugin discovery reports a conflict outright when one plugin ID maps to different package names, or when two plugins share a package name or import name, and the backend then fails to initialize at startup.
- MAA and Whimbox are real, complete samples with their own config, executor, and API; see [Standalone Adapters](/en/developer/plugins/script-adapters/standalone).

Once your plugin works, publish it with [Publishing Plugins](./publish.md). To find out where a plugin's config file is stored, see [Configuration File](./config-files.md).
