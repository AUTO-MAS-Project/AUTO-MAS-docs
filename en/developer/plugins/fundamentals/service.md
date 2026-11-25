# Service System

::: warning
This page follows the `dev` branch implementation as of 2026-11-18. The plugin system is still evolving; refer to your installed AUTO-MAS source for the exact interfaces.
:::

Services let plugins call each other's methods by name, without importing each other's code.

::: tip Differences from the old version
The old `ctx.provide` / `ctx.set` / `ctx.get` / `ctx.inject`, `ctx.service.miss()`, and the class attributes `provides`, `needs`, and `wants` have all been removed. Now:

- A provider uses the class attribute `provides_services = {service name: method name}`. A service is just a method on the plugin; there is no more "declare, then assign".
- Dependencies use `requires` (hard dependency, formerly `needs`) and `wants` (optional dependency).
- A consumer uses `self.services.get(...)` and `self.services.require(...)`.
:::

## Concepts

| Concept | Description |
| --- | --- |
| Service name | A string; a namespace is recommended, such as `arknights.stage_info` |
| Service | A method on the provider plugin. The consumer gets a callable for that method |
| `requires` | Hard dependency. If the service is missing, this plugin cannot be enabled; when the provider is disabled, this plugin is disabled along with it |
| `wants` | Optional dependency. If the service is missing, this plugin is enabled as usual and handles graceful degradation on its own |

A service name can have only one provider.

## Providing a Service

```python
from auto_mas_core import BasePlugin


class PLUGIN(BasePlugin):
    provides_services = {
        "clock.now": "now",
        "clock.minutes_to_seconds": "minutes_to_seconds",
    }

    async def now(self) -> str:
        from datetime import datetime, timezone

        return datetime.now(timezone.utc).isoformat()

    def minutes_to_seconds(self, minutes: int) -> int:
        return minutes * 60
```

- The key is the service name, and the value is the **method name on this class** (a string).
- A method can be sync or async, with any parameters and return value.
- Services are registered **after** `on_enable()` returns, and unregistered on disable (after `on_disable()`). Inside `on_enable`, this plugin's services cannot yet be obtained by others.
- If the attribute the method name points to does not exist or is not callable, that service is **not registered**, and no error is raised. Get the name right.
- If the service name is already registered by another plugin, registration raises `PluginError` and this plugin fails to enable.

## Using a Service

### Hard dependency: requires

```python
from auto_mas_core import BasePlugin
from auto_mas_core.utils import get_logger

logger = get_logger("clock_user")


class PLUGIN(BasePlugin):
    requires = ("clock.now",)

    async def on_enable(self) -> None:
        now = self.services.require("clock.now")
        logger.info(f"Current time: {await now()}")
```

- Write `requires` as a tuple or list. **Do not write it as a string:** writing `requires = "clock.now"` iterates over individual characters.
- When this plugin is enabled, the host first enables the plugins that provide these services, so `on_enable` can always get them.
- `require(name)` raises `PluginError` when the service does not exist, and `payload["service"]` is the service name.

### Optional dependency: wants

```python
from auto_mas_core import BasePlugin, PluginError
from auto_mas_core.utils import get_logger

logger = get_logger("report")


class PLUGIN(BasePlugin):
    wants = ("clock.now",)

    async def build_report(self) -> str:
        now = self.services.get("clock.now")
        if now is None:
            return "Report (no timestamp)"
        try:
            return f"Report {await now()}"
        except PluginError:
            # Provider is being disabled or reloaded; the call was rejected
            return "Report (no timestamp)"
```

- `get(name, default=None)` returns the default when it cannot get the service, and does not raise.
- Get the service at the time you use it; **do not fetch it in `on_enable` and store it**. An optional service may be enabled after you, and may disappear midway. After the provider reloads or is disabled, calling the old callable raises `PluginError`.
- When calling a service, also handle `PluginError`; it is the base class of the error types below.

### Return values of calls

The callable returned by `get` / `require` keeps the sync or async form of the original method: an async method must be awaited, and a sync method returns the result directly.

## What Happens on Enable, Disable, and Reload

Only `requires` takes part in the cascade; `wants` does not.

| Operation | Effect on dependents (requires) |
| --- | --- |
| Enable a dependent | The providers of the services it needs are enabled first; if a provider fails to enable, the dependent fails to enable too |
| Disable a provider | All enabled dependents are disabled first, then the provider |
| Reload a provider | Dependents are detached first; after the provider finishes reloading, the dependents are re-enabled along the dependency tree as it was when they were detached |
| Uninstall a provider | Dependents are uninstalled first |
| Program exit | Dependents run `on_teardown` first, providers after |

Details:

- If no plugin that provides a given service can be found (not installed, or not declared on the class): enabling raises `PluginPrecheckError`, with the service name in the message.
- Dependency cycle: when a loop is detected on the cycle, `PluginPrecheckError` is raised (the message contains the cycle made of registry entry uids). The outer enable wraps it in `PluginOperationError`, and the plugins on the cycle that had already begun enabling go to `fault_runtime`.
- After reloading a provider, if a dependent fails to re-enable: that dependent goes to `fault_runtime`, stays disabled, and `last_error` records the reason; the other dependents continue to be restored.
- Disabling a provider may also disable adapters (script types, game types) along with it. The host provides `disable_impact`, which summarizes the configuration domains that would be deleted on disable, for the frontend to confirm.

### Optional dependencies lack these guarantees

For services in `wants`:

- They do not affect startup order. The provider may be enabled only after you, and `get` then returns the default.
- When the provider is enabled, disabled, or reloaded, you are **not** notified and not reloaded.
- The host only records which optional services were missing at the time; there is no public query interface.

So the pattern for optional dependencies is fixed: call `get` each time you use it, check for `None`, and degrade gracefully.

## When a Call Is Rejected

A service call goes through the provider's call admission:

- The provider is being disabled, uninstalled, or reloaded: new calls are rejected with a subclass of `PluginError` indicating that the plugin is being released and no longer accepts calls. Calls already in progress are waited for until they end.
- The program is exiting: `get` returns the default, and `require` raises.

So when calling another plugin's service for a long time from a background task, handle call failures and `get` again on the next loop.

## A Real Example

The MAA adapter plugin provides three stage-data services:

```python
class Plugin(ScriptAdapterPlugin):
    provides_services = {
        "arknights.stage_info": "stage_info",
        "arknights.stage_data": "stage_data",
        "arknights.stage_overview": "stage_overview",
    }

    async def stage_info(self, kind, *, server="Official"):
        ...
```

A consumer just uses `self.services.get("arknights.stage_info")`, without needing to know the MAA adapter's package name.

## Do Not Register Services Yourself

`auto_mas_core.plugin.services` exposes the host's service registry itself, with `register` / `unregister`. Plugins should not use it to register services: the registered object has no call admission protection, and is not guaranteed to be unregistered on disable, which can leave stale references. Always use `provides_services`.

## Migration Reference

| Old style | Now |
| --- | --- |
| `provides = "clock"` + `ctx.set("clock", value)` | `provides_services = {"clock.now": "now"}`; a service is a method |
| `needs = ["store"]` | `requires = ("store.get",)` |
| `wants = "notify"` | `wants = ("notify.send",)`, must be a tuple or list |
| `ctx.get("clock", {})` | `self.services.get("clock.now")` |
| `ctx.inject(needs=..., ready=...)` | No equivalent; declare `requires` statically and use it in `on_enable` |
| `ctx.service.miss()` | No equivalent |
| Reading an undeclared service only warns | There is no declaration check; if `get` cannot get it, you get the default |
