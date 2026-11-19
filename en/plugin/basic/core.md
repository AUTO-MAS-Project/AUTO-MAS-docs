# Core Capabilities

::: warning
This page follows the `dev` branch implementation as of 2026-11-18. The plugin system is still changing; the interfaces are whatever your installed AUTO-MAS source says.
:::

The core capabilities you need for plugin development live in three places:

- **Plugin instance:** the attributes and lifecycle hooks of a `BasePlugin` subclass, such as `self.config`, `self.services`, and `self.background_tasks`.
- **SDK:** the classes and functions exported by the top level of `auto_mas_core` and its `utils`, `services`, and `notify` submodules.
- **Host:** the real implementation behind those exports. The SDK does not provide a standalone host; plugins can only run inside AUTO-MAS.

::: tip
The `ctx` of the old docs (`ctx.logger`, `ctx.event`, `ctx.cache`, `ctx.runtime`) no longer exists. Plugins no longer receive a `PluginContext`. Where each capability went is covered in the sections below and in [Event System](./event).
:::

## Minimal Plugin

A plugin is an ordinary Python package whose `__init__.py` exports a `BasePlugin` subclass named `PLUGIN`.

```python
from auto_mas_core import BasePlugin
from auto_mas_core.utils import get_logger

logger = get_logger("demo")


class PLUGIN(BasePlugin):
    async def on_enable(self) -> None:
        logger.info(f"{self.plugin_name} 已启用")

    async def on_disable(self) -> None:
        logger.info(f"{self.plugin_name} 已停用")
```

The loader checks that `PLUGIN` is a `BasePlugin` subclass and reads `requires` and `wants` from the class. If the package declares `PLUGIN_API_VERSION`, its major version must match the host's.

## Capabilities on the Instance

The host constructs the instance on enable, with `config` and `record` as constructor arguments. You do not need to write `__init__` yourself; if you must, keep the signature and call the parent class.

```python
class PLUGIN(BasePlugin):
    def __init__(self, config=None, record=None) -> None:
        super().__init__(config, record)
```

| Member | Type | Description |
| --- | --- | --- |
| `self.config` | An instance of `config_class`, or `None` if none is declared | The plugin's own config; see [Configuration Schema](./schema) |
| `self.services` | `ServiceResolver` | Reads services provided by other plugins; see [Service System](./service) |
| `self.background_tasks` | `BackgroundTaskGroup` | This instance's background task group |
| `self.record` | `PluginRecord` | The registry entry, with metadata such as version and state |
| `self.plugin_name` | `str` | The registered name |
| `self.lifecycle_context` | `LifecycleContext` | The current lifecycle context |

Class attributes:

| Attribute | Purpose |
| --- | --- |
| `config_class` | The config class; see [Configuration Schema](./schema) |
| `provides_services` | The services provided, `{service name: method name}` |
| `requires` / `wants` | Names of the services depended on, as a tuple |
| `api_router` | A FastAPI `APIRouter`, mounted on enable |

### self.config

The type of `self.config` is `PluginConfig | None`. It only has a value if `config_class` is declared, so narrow the type before using it.

```python
from auto_mas_core import BasePlugin, PluginConfig


class MyConfig(PluginConfig):
    ...


class PLUGIN(BasePlugin):
    config_class = MyConfig

    async def on_enable(self) -> None:
        assert isinstance(self.config, MyConfig)
```

To read config, just read attributes. To write, assign and then `await commit()`:

```python
# General 是 MyConfig 里声明的分组，字段怎么写见「配置声明」
self.config.general.interval = 10
await self.config.commit()
```

- Assignment only stages the change; `commit()` persists it and fires the signals. A failed commit raises `ConfigAggregateError`.
- A locked config cannot be written.
- The config file is `config/plugins/<plugin name>.yaml`. It is saved and released on disable, and rebuilt from the file on the next enable. It is deleted when the plugin is uninstalled.

### self.background_tasks

Put a plugin's background work in this task group; do not call `asyncio.create_task` yourself. On disable, uninstall, and reload, the host cancels the tasks in the group and waits for them to finish.

```python
async def refresh(self) -> None:
    ...


async def on_enable(self) -> None:
    handle = self.background_tasks.spawn(lambda: self.refresh())
    if handle is None:
        # 任务组已关闭，或并发数/保留数已满
        return
```

`spawn(factory)` takes a function that returns a coroutine and returns a `BackgroundTaskHandle`. When rejected it returns `None`; it does not raise and does not create the coroutine.

| Method | Purpose |
| --- | --- |
| `handle.cancel()` | Cancels the task |
| `handle.done()` | Whether it has finished |
| `await handle.wait_done(timeout)` | Waits for it to finish without taking the result; returns `False` on timeout |
| `await handle.wait_take(timeout)` | Waits for and takes the result as a `TaskOutcome`; raises `TimeoutError` on timeout, and `KeyError` for a task with `retain_result=False` |
| `outcome.unwrap()` | The return value on success; otherwise re-raises the original exception |

- The group's concurrency limit and retention limit both equal the plugin operation limit in settings (`Config.setting.plugin.max_operations`).
- Results are kept for 300 seconds by default. When you do not need the result, pass `spawn(factory, retain_result=False)`.
- There is no execution timeout. Use `asyncio.timeout` yourself if you need one.
- The task group is bound before `pre_enable()`. Accessing `self.background_tasks` earlier raises `RuntimeError`.

### self.record and self.plugin_name

```python
info = self.record.info
logger.info(f"{self.plugin_name} {info.version} 来源={info.source}")
```

Commonly used fields of `info`: `plugin_name`, `package_name`, `version`, `source` (`installed` or `local`), `state`, `last_error`, `is_core`, and `is_local`. `self.record` holds only a weak reference. Accessing it after the registry entry has been reclaimed raises `RuntimeError`, so do not cache it long-term in the background.

## Lifecycle Hooks

All hooks are `async def` and do nothing by default. Override only what you need.

| Hook | When |
| --- | --- |
| `pre_enable` | Precheck before enable. Services and the task group are already bound, the config is not yet active, and upstream dependencies are not yet enabled |
| `on_enable` | Upstream dependencies are enabled and the config is active. Start your business resources here |
| `pre_disable` | Precheck before disable |
| `on_disable` | On disable. The background task group has already been cancelled and waited for |
| `pre_reload` / `on_reload` | Precheck before reload / during reload, acting on the old instance |
| `pre_unload` / `on_unload` | Precheck before uninstall / during uninstall |
| `on_teardown` | Program exit. It does not replace `on_disable`, does not persist anything, and does not change the enable state |
| `on_fail_cleanup` | Cleanup after a lifecycle operation fails; receives the failing stage, the context, and the exception |

Enable order: construct the instance → bind the task group and services → `pre_enable` → enable the upstream plugins in `requires` → activate the config → `on_enable` → register `provides_services` → mount HTTP.

Disable order: `pre_disable` → first disable downstream plugins that depend on this plugin's services → cancel background tasks → `on_disable` → save and release the config → unregister services and HTTP.

Reload: `pre_reload` → detach the downstream plugins that depend on this plugin → `on_reload` → disable the old instance → re-import the plugin code → enable the new instance → restore the downstream plugins as they were.

- When `pre_enable` raises, enable is aborted and the host releases the bound task group and config. When a stage after `pre_enable` (enabling upstreams, activating config, `on_enable`, registering services and HTTP) raises, the plugin goes to `fault_runtime` and `on_fail_cleanup` is called.
- `pre_*` hooks are business prechecks. When a precheck fails, raise `PluginPrecheckError` (exported by the SDK).
- The core plugin `auto_mas_core` cannot be disabled, reloaded, or uninstalled.

::: warning
On disable and uninstall, the host stops admitting new calls and waits for in-flight calls to finish. Release private resources such as external processes, subscriptions, and temporary files yourself in `on_disable`.
:::

## Logging

```python
from auto_mas_core.utils import get_logger

logger = get_logger("我的插件")
logger.debug("调试信息")
logger.info("常规信息")
logger.warning("警告")
logger.error("错误")
```

`get_logger` returns the host's loguru logger; the module name marks the source. Before writing a log line you can use `sanitize_log_message(text)` to strip sensitive parameters such as tokens from the text.

## SDK Exports

### auto_mas_core top level

| Category | Names |
| --- | --- |
| Base classes | `BasePlugin`, `ExtensionPlugin`, `GameAdapterPlugin`, `ScriptAdapterPlugin`, `CorePlugin` |
| Plugin config | `PluginConfig`, `PluginCommandEntry`, `trigger_field`, `ConfigEntry`, `ConfigGroup`, `ConfigCollection`, `collection`, `ConfigTableColumn` |
| Fields and UI | `Trigger`, `Virtual`, `virtual_field`, `ui`, `ui_visibility`, `UiVisibility`, `select`, `encrypted`, `FilePath`, `FolderPath`, `ExecutablePath`, `JsonDictString`, `JsonListString` |
| Config signals and locks | `FieldChangeEvent`, `LockTicket` |
| Services and HTTP | `ServiceResolver`, `plugin_route` |
| Background tasks | `BackgroundTaskGroup`, `BackgroundTaskHandle` |
| Downloads | `DownloadRequest`, `DownloadSnapshot` |
| Errors and state | `PluginError`, `PluginPrecheckError`, `PluginState`, `PLUGIN_API_VERSION` |
| Adapters | `ScriptTypeDecl`, `GameTypeDecl`, `UserExpander`, `ModeWorker`, `AutoProxyWorker`, `ScriptConfigWorker`, `ScriptViewWorker`, `MODE_WORKERS`, `AdapterRuntime`, `GameRuntime`, `GameControl`, `EmulatorControl`, `EmulatorExpect`, `DeviceHandle`, `DeviceStatus`, `DeviceSubscription`, `DeviceSubscribeError`, `TRIGGER_TICKET` |
| Host data models | `ScriptEntry`, `UserEntry`, `PlanEntry`, `GameEntry`, `GameDeviceEntry`, `EmulatorEntry`, `EmulatorDeviceEntry`, `TagItem`, `TaskMode`, `TaskScriptItem`, `TaskUserItem`, `LogRecord` |

For the adapter-related classes, see [Standalone Adapters](/en/plugin/adapter/standalone).

The following names are loaded lazily: the host singleton is only obtained on first access.

| Name | Description |
| --- | --- |
| `Config` | The host's global configuration |
| `Plugin` | The plugin manager |
| `Publisher` | The WebSocket publisher; see [Event System](./event) |
| `protocol` | WebSocket message type constants |
| `History` | History records |
| `WSTaskNoticeData` | The task notice data model |

```python
from auto_mas_core import Config

version = Config.VERSION
```

Names not in the export table raise `AttributeError`.

### auto_mas_core.utils

| Name | Purpose |
| --- | --- |
| `get_logger` | Gets a logger |
| `get_setting(group, name, default)` | Reads the host setting `Config.setting.<group>.<name>`; returns the default if it cannot be read |
| `ProcessRunner` | Runs a subprocess and collects its output |
| `ProcessManager`, `ProcessInfo`, `ProcessResult` | Process management and results |
| `is_process_running` | Checks whether a process is running |
| `WebSocketClient` | WebSocket client |
| `read_file` / `write_file` | Reads and writes config files |
| `decode_bytes` | Tries several encodings to decode bytes |
| `dpapi_encrypt` / `dpapi_decrypt` | Windows DPAPI encryption and decryption |
| `sanitize_log_message` | Log redaction |
| `busy_wait` | High-precision busy wait; occupies the CPU |
| `to_pep440` | Converts an app version tag to PEP 440 |

Running a subprocess:

```python
from auto_mas_core.utils import ProcessRunner

result = await ProcessRunner.run_process("python", "--version", timeout=10)
if result.returncode == 0:
    print(result.stdout)
else:
    print(result.failure_detail())
```

The `timeout` of `run_process` defaults to 60 seconds.

### auto_mas_core.services

Exports four host singletons.

| Name | Purpose |
| --- | --- |
| `Downloads` | The unified download center |
| `Operations` | Operation scheduling for plugin config triggers |
| `Notify` | Notification dispatch |
| `BackgroundTaskManager` | The app-level background task manager. Plugins use `self.background_tasks` and generally do not use this directly |

Downloads:

```python
from auto_mas_core import DownloadRequest
from auto_mas_core.services import Downloads

request = DownloadRequest(url="https://example.com/data.zip", key="my_plugin:data")
if Downloads.request(request, owner="my_plugin"):
    snapshot = await Downloads.wait(request)
    if snapshot.state == "succeeded":
        print(snapshot.path)
    await Downloads.release(request.cache_key, owner="my_plugin")
```

- `request()` returns `False` when rejected (capacity busy), and raises `RuntimeError` if the download center is closed.
- `key` can replace the URL as the resource key, so a changed URL is not mistaken for a new resource. File identity is determined in the order `rid`, `content_hash`, empty identity.
- Call `release()` when done. When the last user releases, an unfinished download is cancelled.
- `query(request)` takes a snapshot immediately, and `list(owner=...)` lists one user's downloads.

For Operations, see [Configuration Schema](./schema#triggers).

### Notifications (auto_mas_core.notify)

```python
from auto_mas_core.notify import Notify, NotifyPayload

result = await Notify.send_global(
    NotifyPayload(title="通知标题", text="通知正文"),
    include_system=True,
)
if result.attempted == 0:
    ...  # 没有可用渠道，并不代表已投递
```

| Method | Purpose |
| --- | --- |
| `send_global(payload, include_system=False, ...)` | Sends to the global notification channels |
| `send_user(payload, user_config, ...)` | Sends to the channels configured for a user |
| `send_statistics(payload, user_config, ...)` | Sends according to the user's and the global statistics notification policy |
| `push_proxy_result(title=..., report=..., ...)` | Pushes a proxy result report |

The same module also exports `DispatchResult`, `NotificationImage`, `TaskResultReport`, and `image_reference`. There is no plugin-specific notification protocol; plugins reuse the host's channels and templates.

## HTTP API

A plugin can mount its own HTTP endpoints under the path prefix `/api/plugin/<plugin name>`.

```python
from starlette.requests import Request

from auto_mas_core import BasePlugin, plugin_route


class PLUGIN(BasePlugin):
    @plugin_route("/ping", methods=["GET"])
    async def ping(self) -> dict:
        return {"ok": True}

    @plugin_route("/echo", methods=["POST"])
    async def echo(self, request: Request) -> dict:
        return await request.json()
```

- The endpoint address is `/api/plugin/<plugin name>/ping`. When `methods` is omitted it is `GET`.
- A handler either has no parameters (not counting `self`) or exactly one, which receives the request object `Request`.
- Returning `dict`, `list`, or `None` produces a JSON response; returning a `Response` sends it as is; other values are converted to text. When it raises, a 500 is returned and the details appear only in the local log.
- Mounted on enable and removed on disable. While the host has calls paused (during disable, reload, or uninstall) and at program exit it returns 503; a disabled plugin is not mounted and returns 404.
- When you need typed routes and OpenAPI, set `api_router = APIRouter()` on the class; see [Frontend and Backend Integration](/en/plugin/adapter/frontend) for how.

## Sync and Async

- Lifecycle hooks, `Downloads.wait`, `Notify.*`, and `ProcessRunner.run_process` are coroutines and must be `await`ed.
- A background task's `factory` must return a coroutine, such as `lambda: self.refresh()`.
- A `plugin_route` handler can be sync or async. A sync function runs directly in the event loop, so use async or a background task for slow operations.

## FAQ

### Accessing self.background_tasks reports "plugin has no background task group bound"

You accessed it in the constructor. The task group is only bound before `pre_enable()`, so use it inside the hooks.

### spawn returns None

The task group is closed (the plugin is being disabled), or the number of concurrently running or retained tasks has reached the limit. Check the return value; do not assume it always succeeds.

### Accessing a name on auto_mas_core raises AttributeError

The name is not in the SDK export table. Do not import the host's internal `app.*` modules directly; they are not part of the SDK contract and may change at any time.
