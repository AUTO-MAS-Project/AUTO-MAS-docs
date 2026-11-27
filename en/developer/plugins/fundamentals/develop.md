# Development and Debugging

Start with a minimal source plugin that logs when it is enabled and disabled. Use the log messages to verify that the host discovers, loads, and runs your code. First complete [Environment Setup](../../getting-start/environment.md).

## Create a Source Project

The host scans the `plugins/` directories under its working directory and source root. If you start the backend from the main repository root, you can place your plugin project there. This example uses `hello_demo` as the plugin ID.

```text
plugins/
└─ hello_demo/
   ├─ pyproject.toml
   ├─ README.md
   └─ src/
      └─ hello_demo/
         ├─ __init__.py
         └─ plugin.py
```

- `plugins/<plugin ID>/` is the project root. Maintain the plugin source in its own repository.
- `src/<import package name>/` is the single regular top-level Python package. It must include `__init__.py` and contain the plugin modules.

## Declare Package Metadata

Create `pyproject.toml` in the project root. These fields describe the distribution and its dependencies. The complete example below also includes the build settings.

```toml
[project]
name = "<distribution name>"
version = "<release version>"
readme = "<README path>"
requires-python = "<Python version range>"
keywords = ["auto-mas-plugin", "auto-mas-id:<plugin ID>"]
dependencies = ["auto-mas-core<SDK version range>"]
```

- `name` is the distribution name used for installation and dependency resolution.
- `version` is a PEP 440 version.
- `keywords` must include `auto-mas-plugin` and one `auto-mas-id:<plugin ID>` entry. Use a stable ID that is valid as a filename. The host reserves `auto_mas_core`.
- `readme` points to an existing file. Add the market display metadata to the [README](./publish.md#readme) before publishing.
- `requires-python` matches the Python version of the target host.
- `dependencies` lists `auto-mas-core` and the other dependencies the plugin uses. Base the SDK range on the host versions you have verified.

Here is the complete `hello_demo` example for Python 3.12.

```toml
[build-system]
requires = ["setuptools>=77", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "auto-mas-hello-demo"
version = "0.1.0"
description = "A minimal AUTO-MAS plugin"
readme = "README.md"
requires-python = ">=3.12,<3.13"
keywords = ["auto-mas-plugin", "auto-mas-id:hello_demo"]
dependencies = ["auto-mas-core>=5,<6"]

[tool.setuptools.packages.find]
where = ["src"]
```

## Export the Plugin Entry Point

1. **Implement the plugin class.** In `src/hello_demo/plugin.py`, subclass `BasePlugin` and use hooks to acquire and release plugin resources.

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

2. **Export the class and protocol version.** In `src/hello_demo/__init__.py`, export the class as `PLUGIN`.

   ```python
   from auto_mas_core import PLUGIN_API_VERSION

   from .plugin import Plugin as PLUGIN

   __all__ = ["PLUGIN", "PLUGIN_API_VERSION"]
   ```

When loading the package, the host checks that `PLUGIN` is a subclass of `BasePlugin`. If the package exports `PLUGIN_API_VERSION`, the host also checks its protocol major version. See when the host calls each hook in [Plugin Lifecycle](./runtime.md).

## Enable and Verify the Source Plugin

1. **Start the host.** Start or restart the backend so it registers source projects under `plugins/`.
2. **Enable the plugin.** Find it in the Plugin Center, confirm its source is local, and turn on its enable switch.
3. **Check the logs.** Look for the `HelloDemo` enable message in `debug/app.log`. Disable the plugin and check for its cleanup message.

A plugin discovered while the host is running may still be unloaded. Restart the backend after creating a new source project, then enable it.

## Reload Code and Update Dependencies

1. **Edit the code.** When you save Python files or project metadata, the host marks source plugins ready for reload as `needs_reload`.
2. **Apply the changes.** Choose Reload in the Plugin Center. For an enabled plugin, the host releases the old instance, imports the source again, and enables a new instance. If reload fails, fix the code and retry.
3. **Update dependencies.** Edit `dependencies` in `pyproject.toml` and restart the backend. The host resolves the application's locked dependencies together with the dependencies declared by plugins. If resolution fails, check those declarations against the target host version.

## Develop an Existing Plugin

Clone the plugin's repository into `plugins/<plugin ID>/` and keep its existing plugin ID, distribution name, and import name. Commit and publish changes from that repository. Give a new, independent plugin its own identity.

If both the source and the installed distribution are present for the same plugin, the host uses the source. The host reports a conflict if multiple source projects or different distributions declare the same plugin ID. To test a wheel, stop the backend and move the same-ID source out of its scan paths. Restart the backend, install and enable the listed distribution in the Plugin Center, and verify its installation source and main features. See [Plugin Market Discovery](./publish.md#plugin-market-discovery) for how the catalog is refreshed.
