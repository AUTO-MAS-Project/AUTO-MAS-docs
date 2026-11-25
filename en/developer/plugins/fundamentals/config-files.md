# Configuration File

::: warning
The plugin system is changing fast. This page follows the `dev` branch implementation as of 2026-11-18.
:::

The plugin system's configuration is stored separately from the main program's other configuration, all under the backend working directory. When running from source, the working directory is the AUTO-MAS repository root. For other launch methods, use the "工作目录:" (working directory) line at the start of the backend log.

## Where It Is Stored

| Content | File | Description |
| --- | --- | --- |
| Plugin registry | `config/plugin_registry.yaml` | One entry per discovered plugin: package name, plugin ID, version, source, state, whether enabled, last error, code status, and so on. The host writes it back on discovery, enable, disable, and reload. |
| Marketplace catalog | `config/plugin_catalog.yaml` | The entries visible in the plugin marketplace. Replaced as a whole when the marketplace is refreshed. |
| Data site catalog cache | `data/plugin-market-cache.json` | The catalog and ETag last read from the data site, used for conditional requests. |
| Plugin's own config | `config/plugins/<plugin ID>.yaml` | One file per plugin. Only plugins that declare `config_class` have one. |

Plugin packages installed from the marketplace, and their dependencies, live in `plugins/pypi/site-packages`. That is not configuration; do not edit it by hand.

::: tip
The registry and the marketplace catalog are maintained by the host, and their fields change between versions. To enable or disable a plugin, use the Plugin Center rather than editing the YAML directly. `enabled` in the registry stores the user's enable choice, and the backend uses it to enable plugins automatically at startup.
:::

## A Plugin's Own Config

When a plugin needs its own settings, write a config class that extends `PluginConfig` and attach it to the plugin class as `config_class`:

```python
from auto_mas_core import BasePlugin, ConfigGroup, PluginConfig
from auto_mas_core.utils import get_logger
from pydantic import Field


class DemoConfig(PluginConfig):
    class Basic(ConfigGroup):
        greeting: str = Field(default="Hello", description="Greeting message")
        repeat: int = Field(default=1, description="Repeat count")

    basic: Basic = Field(default_factory=Basic, description="Basic settings")


class Plugin(BasePlugin):
    config_class = DemoConfig

    async def on_enable(self) -> None:
        assert isinstance(self.config, DemoConfig)
        get_logger("HelloDemo").info(self.config.basic.greeting)
```

When the host enables the plugin, it:

1. Builds the config instance with `config/plugins/<plugin ID>.yaml` as its storage file, and reads the existing content if the file already exists.
2. Passes the config in when constructing the plugin instance; the plugin accesses it as `self.config`.
3. While the plugin is enabled, the gear button on that row in the Plugin Center opens the "Plugin Settings" page, which renders a form from your config declaration and writes it back to this file on save. While the plugin is not enabled, this page cannot read the config.

You do not handle file I/O yourself—just declare the schema. For how to declare fields (`ConfigGroup`, `ConfigEntry`, `ui(...)`, and so on), see the MAA adapter's [`MaaPluginConfig`](https://github.com/AUTO-MAS-Project/AUTO-MAS-MAA-Adapter/blob/master/src/maa_adapter/config/plugin.py) and the SDK [README](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/plugins/auto_mas_core/README.md).

::: warning
Collection fields in a config class cannot set `name=`; otherwise enabling raises an error.
:::

## What Disable and Uninstall Do to Config

| Operation | Registry | Own config file | Other |
| --- | --- | --- | --- |
| Disable | Kept; `enabled` set to false | Written to disk first, then released. **The file is kept** and read back on the next enable | See below for adapter plugins |
| Reload | Kept | Kept | Existing data is not cleared |
| Uninstall | **Removed** | `config/plugins/<plugin ID>.yaml` is **deleted** | The distribution installed from the marketplace is removed |

- **Disabling an adapter plugin clears data.** A script-type adapter (such as MAA or Whimbox) revokes the managed type it registered when disabled, and all managed scripts, accounts, and linked plans under that type are cleared with it. A game-type adapter clears the matching game and device configuration. The UI shows a dialog listing the affected scope before disabling. Back up before you act. A plugin that only provides ordinary features and registers no managed or game type does not affect other configuration when disabled.
- **Uninstall is only available for plugins installed from the marketplace.** The "Uninstall" menu item is grayed out for local source plugins and for the core plugin `auto_mas_core`. The UI asks for confirmation before uninstalling, and uninstalling cannot be undone. If deleting the distribution fails during uninstall, the registry entry is kept and marked "Load failed", and you can retry.
- **Deleting the local source directory does not delete the config.** After the directory is gone, the registry entry is marked `package_removed` (package missing), and `config/plugins/<plugin ID>.yaml` stays as it was.
