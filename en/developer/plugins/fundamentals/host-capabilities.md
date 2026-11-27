# Use Host Capabilities

Plugins can import the host's public interfaces from `auto_mas_core` and access the capabilities the host binds to their instance during enablement.

## Import from the SDK

For the target host version, public SDK exports are defined in `plugins/auto_mas_core/src/auto_mas_core/`. Common entry points include the following.

- **Host configuration.** Import `Config` from `auto_mas_core`. For example, `Config.PlanConfig` gives you the plan configuration collection. The SDK also exports `config_manager` from `auto_mas_core`. See [Configuration Framework](../../core/config.md).
- **Notifications.** Import `Notify` from `auto_mas_core.notify` to send notifications through the host.
- **Downloads.** Import `Downloads` from `auto_mas_core.services` and the `DownloadRequest` type from `auto_mas_core`.
- **Logging.** Import `get_logger` from `auto_mas_core.utils` to obtain a logger for your plugin.
- **Frontend messages.** Import `Publisher` from `auto_mas_core` to publish messages to the frontend.
- **Worker threads.** Import `run_in_worker` from `auto_mas_core.services` to run blocking synchronous calls in a worker thread.

## Access Capabilities from the Plugin Instance

### Read Plugin Configuration

When the plugin class declares `config_class`, the instance can access its configuration through `self.config`. The host activates the configuration before calling `on_enable()`, so the hook can read saved values. The value of `self.config` is `None` when the plugin declares no configuration class.

### Submit Background Tasks

The host binds `self.background_tasks` before calling `pre_enable()` and cleans up the task group as part of the instance lifecycle. Call `spawn()` to submit a background task to the group.

```python
<task handle or None> = self.background_tasks.spawn(<coroutine factory>)
```

- **Task submission**
  - `<coroutine factory>` is a callable that returns a coroutine, such as `lambda: self.refresh_cache()`.
  - `<task handle or None>` is a handle when the task is accepted, or `None` when it is declined.

### Call Services from Other Plugins

Declare dependencies by service name on the plugin class. Look up service methods through `self.services` in `on_enable()` or later. The host finds the registered method by name, and the provider plugin implements it. See [Service Declaration](./service.md#service-declaration) for the provider's side.

```python
from auto_mas_core import BasePlugin


class <plugin class>(BasePlugin):
    requires = ("<required service name>",)
    wants = ("<optional service name>",)

    async def on_enable(self) -> None:
        required = self.services.require("<required service name>")
        await required(<service arguments>)

        optional = self.services.get("<optional service name>")
        if optional is not None:
            await optional(<service arguments>)
```

- **Declare dependencies**
  - `<required service name>` belongs to `requires`. The host enables its provider before calling `on_enable()`.
  - `<optional service name>` belongs to `wants`. The plugin can use the service if its provider has registered it and must handle its absence.
- **Look up services**
  - `self.services.require()` retrieves a required service and raises an exception if it is unavailable.
  - `self.services.get()` retrieves an optional service or returns the default, which is `None` unless specified.
