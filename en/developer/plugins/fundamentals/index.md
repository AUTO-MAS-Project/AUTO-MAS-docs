# Plugin Fundamentals

This chapter explains how plugins work and how to develop them.

## Terminology

### Host

The AUTO-MAS backend that runs and manages plugins. It maintains the local plugin registry and market catalog, discovers and loads plugins, coordinates their lifecycle, and exposes host capabilities through the SDK.

### Plugin Registry Entry | PluginRecord

The host maintains one registry entry per plugin. The entry records its identity, source, version, and other metadata, and exposes lifecycle operations through configuration triggers. It also holds the loaded plugin class and the instance created when the plugin is enabled.

### Plugin Market Entry | PluginMeta

A market entry describes an installable release with its plugin ID, distribution name, version, display name, and summary. Its `info.install` trigger requests installation or an update. The plugin manager uses the release specified by the entry.

### Plugin Base Class | BasePlugin

The common base class provided by the SDK. `BasePlugin` defines how plugins declare their configuration and service dependencies and provides lifecycle hooks that they can override. Game adapters and script adapters inherit this interface through `GameAdapterPlugin` and `ScriptAdapterPlugin` respectively.

### Plugin Class

The concrete implementation a developer supplies to the host. It inherits from `BasePlugin` directly or indirectly. Class attributes declare configuration, dependencies, and capabilities, while methods implement plugin behavior. At load time, the host reads the `PLUGIN` export from the top-level package's `__init__.py` and stores the class in the registry entry for use during enablement.

### Plugin Instance

The object the host creates from the loaded plugin class when enabling a plugin. The host passes in the registry entry and configuration and binds a service resolver and a background task group. It uses the instance to call lifecycle hooks and registered service methods. The host releases the instance on disable and creates a fresh one on the next enable.

### Plugin Configuration | PluginConfig

The object that stores a plugin's settings. The developer defines a subclass of `PluginConfig` and declares it as `config_class` on the plugin class. During enablement, the host builds and activates the configuration and binds it to `config/plugins/<plugin ID>.yaml`. The plugin accesses the object through `self.config`, which is `None` when the class declares no plugin configuration.

## Plugin Class Fields and Methods

Plugins declare configuration, services, and HTTP routes through class attributes. Lifecycle hooks implement the work the plugin performs at each stage.

### Class Declaration

```python
from auto_mas_core import BasePlugin


class <plugin class>(BasePlugin):
    config_class = <plugin configuration class>
    requires = ("<required service name>",)
    wants = ("<optional service name>",)
    provides_services = {"<provided service name>": "<service method name>"}
    api_router = <FastAPI router>


PLUGIN = <plugin class>
```

- `<plugin configuration class>` is a subclass of `PluginConfig`. The host uses it to build the plugin configuration during enablement. The default is `None`.
- `<required service name>` identifies a service the plugin needs. The host enables its provider as a prerequisite.
- `<optional service name>` identifies a service the plugin can use when available. The plugin must handle its absence.
- `<provided service name>: <service method name>` maps a service name to an instance method on the plugin class.
- `<FastAPI router>` is the `APIRouter` the host mounts for the plugin.

### Lifecycle Hooks

The lifecycle hooks on `BasePlugin` are asynchronous methods with empty implementations. Plugin classes can override any hooks they need. The host calls them on the plugin instance. See the full call order in [Plugin Lifecycle](./runtime.md).

```python
class <plugin class>(BasePlugin):
    async def pre_enable(self) -> None:
        ...

    async def on_enable(self) -> None:
        ...

    async def pre_disable(self) -> None:
        ...

    async def on_disable(self) -> None:
        ...

    async def pre_reload(self) -> None:
        ...

    async def on_reload(self) -> None:
        ...

    async def pre_unload(self) -> None:
        ...

    async def on_unload(self) -> None:
        ...

    async def on_fail_cleanup(self, during, lifecycle_context, exc) -> None:
        ...

    async def on_teardown(self) -> None:
        ...
```

- **Enable.** Check prerequisites and initialize resources.
  - `pre_enable()` runs after the host creates the instance. The plugin can check external programs or the runtime environment here.
  - `on_enable()` runs after the host enables required service providers and activates the plugin configuration. The plugin can create clients or subscribe to signals here.
- **Disable.** Check whether work can stop and release the resources acquired during enablement.
  - `pre_disable()` runs before the host disables the plugin. The plugin can check whether operations such as configuration sessions have finished.
  - `on_disable()` runs before the host unregisters services and routes. The plugin can close clients, processes, or subscriptions here.
- **Reload.** Check whether the old instance can be reloaded and save any state needed across the reload.
  - `pre_reload()` runs on the old instance. The plugin can check whether its current work permits a reload.
  - `on_reload()` runs before the host disables the old instance. The plugin can save state the new instance needs to restore.
- **Uninstall.** The host calls the uninstall hooks when it has an existing plugin instance.
  - `pre_unload()` runs before uninstall. The plugin can check whether the files it manages can be cleaned up.
  - `on_unload()` runs after the plugin is disabled and before the host removes the distribution. The plugin can clean up resources that only need removal on uninstall.
- **Failure and shutdown.** Release plugin resources for the corresponding operation.
  - `on_fail_cleanup(during, lifecycle_context, exc)` runs when the host enters failure cleanup for the plugin. The plugin can use `during`, `lifecycle_context`, and `exc` to clean up resources that were only partially initialized.
  - `on_teardown()` runs for enabled plugins when the host exits. The plugin can close external processes, connections, or subscriptions.
