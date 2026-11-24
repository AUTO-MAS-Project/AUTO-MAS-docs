# Event System

::: warning
This guide describes the host contract after the Game Management migration. Check the SDK that ships with your target AUTO-MAS version.
:::

## The Old Event Bus Is Gone

None of the following from the old docs can be found in the current implementation:

- `ctx.event.on` / `off` / `emit` / `emit_async`
- The `@on_event` decorator and the `mas.plugins` module
- Events in the payload such as `task.start`, `task.progress`, `task.log`, `task.exit`, `script.start`, and `script.exit`, and the envelope fields `event_version` and `source`

Plugins no longer have a bus for "subscribe and broadcast by event name", so there is also no `scope`, `priority`, or `error_policy`. An `@on_event("script.exit")` written from the old docs receives nothing.

::: tip
When you need "do something when X happens", pick in this order: lifecycle hooks, config signals, service calls, config triggers. Each has different boundaries, covered one by one below.
:::

## What to Use Instead

| What you want | What to use |
| --- | --- |
| Run code when the plugin starts, stops, or reloads | [Lifecycle hooks](#lifecycle-hooks) |
| Run code when a config field changes | [Config signals](#config-signals) |
| Let other plugins call your capabilities, or call other plugins | [Service System](./service.md) |
| Run code when the user clicks a button in the UI | [Triggers](./schema.md#triggers) |
| Push a message to the frontend | [WebSocket push](#websocket-push) |
| Send the user a notification | [Notifications](./index.md) |
| Do something when a task or account run ends | [No public entry point](#task-and-account-run-events) |

## Lifecycle Hooks

`BasePlugin` provides a set of `async` hooks: `pre_enable`, `on_enable`, `pre_disable`, `on_disable`, `pre_reload`, `on_reload`, `pre_unload`, `on_unload`, `on_teardown`, and `on_fail_cleanup`. For their order and caveats, see [Core Capabilities](./index.md#lifecycle-hooks).

```python
from auto_mas_core import BasePlugin
from auto_mas_core.utils import get_logger

logger = get_logger("demo")


class PLUGIN(BasePlugin):
    async def on_enable(self) -> None:
        logger.info("启用")

    async def on_disable(self) -> None:
        logger.info("停用")
```

## Config Signals

Config nodes come with change signals, built on blinker. The SDK exports `FieldChangeEvent`, and the host itself subscribes to task and config changes with the same mechanism.

How to subscribe:

```python
from auto_mas_core import BasePlugin, FieldChangeEvent, PluginConfig


class MyConfig(PluginConfig):
    ...


async def on_interval_changed(sender, event: FieldChangeEvent) -> None:
    print(event.group, event.field, event.old_value, "->", event.value)


class PLUGIN(BasePlugin):
    config_class = MyConfig

    async def on_enable(self) -> None:
        MyConfig.connect(
            on_interval_changed, kind="set", group="general", field="interval"
        )

    async def on_disable(self) -> None:
        MyConfig.disconnect(
            on_interval_changed, kind="set", group="general", field="interval"
        )
```

- The callback arguments are `(sender, event)`, and it can be a sync or async function. Async callbacks are awaited one after another.
- Use a module-level function as the callback. Instance methods cannot be attached, and the default weak-reference subscription raises `TypeError`.
- `kind="set"` means a runtime change, and `"init_set"` is the initial assignment when the config is activated. `phase` defaults to `"runtime"`, which only receives runtime events.
- `group` and `field` filter; omit them to receive changes to every field of the class.
- Use `MyConfig.connect` to subscribe to the whole class, and `self.config.connect` to subscribe to this one instance only.
- Subscribing twice with the same `(phase, kind, group, field, sender)` raises `ValueError`; `disconnect` first.
- If a callback changes the config, it must `commit()` itself; the staged changes of that round must be committed before the callback returns, otherwise `RuntimeError` is raised.

::: warning
A signal fires only when the field value really changes. Writing the same value does not fire it.
:::

## Service Calls

To have another plugin called at the right moment, do not build your own subscription mechanism; use services: the callee declares `provides_services`, and the caller uses `self.services.get(...)`. See [Service System](./service.md).

A service is an "active call", not a "passive notification". A provider cannot broadcast to consumers. For broadcast scenarios, have the consumer poll, or have the consumer pass a callback function as an argument to the provider's service method for the provider to store. If the provider stores callbacks, it must clear them in `on_disable`; otherwise the callback is still there after the consumer is uninstalled.

## Triggers

The config manager runs `@trigger_field` handlers in a background task. A `config.ui.changed` WebSocket event tells the frontend to re-read affected configuration. Query the task result by UUID. See [Configuration Schema](./schema.md#triggers).

For host UI, `config.ui.changed` is sent after a successful root transaction. It identifies the changed entries and collections by UUID. It does not carry field values, hints, or task outcomes. Re-read the snapshot to obtain those values. Use the shared frontend synchronization path instead of broadcasting a custom event for a config field. See [Configuration UI Synchronization](./frontend.md#configuration-ui-synchronization).

## WebSocket Push

`auto_mas_core.Publisher` is the host's outlet for sending messages to the frontend, and `auto_mas_core.protocol` holds the message type constants. Both are loaded lazily.

```python
from auto_mas_core import Publisher

sent = await Publisher.send(
    id="my_plugin",
    type="my_plugin.changed",
    data={"count": 3},
)
```

- `send(id, type, data)` sends one `{id, type, data}` envelope message to the frontend's main connection and returns whether it was sent. When the main connection is not established, the message is dropped and `False` is returned; it is neither cached nor resent.
- `data` can be a pydantic model or a dict.
- This is a **one-way** channel: it only goes to the frontend and never reaches other plugins.
- The types in `protocol` are all agreed between the host and the frontend (tasks, updates, emulators, plugin capability changes, and so on). The frontend only handles types it recognizes. The current SDK has no entry point for a plugin to load its own frontend code, so for messages of a custom type there is no page in the frontend to consume them.

::: warning
Do not borrow an existing host type constant for your `type`; the frontend would parse the payload in that type's format.
:::

## Task and Account Run Events

When a task starts or ends, and at script execution boundaries, the host pushes messages such as `task.created`, `task.completed`, `task.script.started`, and `task.script.finished` to the frontend. They are defined in `protocol`, **go only to the frontend, and have no subscription entry point for ordinary plugins**.

The host also has internal signals such as `TaskManager.task_ended`, but the SDK does not export them and they are not part of the plugin contract. Do not depend on them.

If what you need is "insert my own logic while a script runs", that is an adapter: register a script type with `ScriptTypeDecl`, and implement a `UserExpander` and the matching `ModeWorker`, which the host schedules within the task. See [Standalone Adapters](/en/developer/plugins/script-adapters/standalone).

## The Plugin System's Own Signals

`app/plugin/signals.py` has a set of blinker signals used internally by the plugin system: `service_changed`, `sources_changed`, `capabilities_changed`, `install_progress`, `upgrade_progress`, and `precheck_failed`.

- They are not in the SDK export table and are internal host implementation.
- Currently only `service_changed` (when a service is registered or unregistered), `sources_changed` (when the source directory changes), and `capabilities_changed` (when a plugin is enabled or disabled) are sent from anywhere; the other three are declared but have no sender.
- Do not subscribe to them in a plugin. To know whether a service is available, just call `self.services.get(...)` and check the return value.

## Migration Reference

| Old style | Now |
| --- | --- |
| `ctx.event.on("script.exit", ...)` | No equivalent; see [Task and account run events](#task-and-account-run-events) |
| `ctx.event.emit_async("x", ...)` | If the other side is your own plugin: provide a service; if it is the frontend: `Publisher.send` |
| `@on_event(...)` | Config signals or lifecycle hooks |
| `scope="instance"` / `"global"` | There is no scope concept any more |
| `async def on_start(self)` | `async def on_enable(self)` |
| `async def on_stop(self, reason)` | `async def on_disable(self)`, with no `reason` parameter |
