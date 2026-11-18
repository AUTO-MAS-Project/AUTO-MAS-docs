# 核心功能

::: warning
本文以 `dev` 分支 2026-11-18 的实现为准。插件系统仍在变化，接口以你安装的 AUTO-MAS 源码为准。
:::

插件开发需要的核心能力分三处：

- **插件实例**：`BasePlugin` 子类的属性和生命周期钩子，如 `self.config`、`self.services`、`self.background_tasks`。
- **SDK**：`auto_mas_core` 顶层和 `utils`、`services`、`notify` 子模块导出的类与函数。
- **宿主**：这些导出背后的真实实现。SDK 不提供独立运行的宿主，插件只能在 AUTO-MAS 里运行。

::: tip
旧文档里的 `ctx`（`ctx.logger`、`ctx.event`、`ctx.cache`、`ctx.runtime`）已经不存在。插件不再接收 `PluginContext`，对应能力的去向见本文各节和[事件系统](./event)。
:::

## 最小插件

插件是普通 Python 包，包的 `__init__.py` 导出名为 `PLUGIN` 的 `BasePlugin` 子类。

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

加载器检查 `PLUGIN` 必须是 `BasePlugin` 子类，并读取类上的 `requires`、`wants`。包里如果声明了 `PLUGIN_API_VERSION`，主版本必须和宿主一致。

## 实例上的能力

宿主在启用时构造实例，构造参数是 `config` 和 `record`。你不需要自己写 `__init__`；确实要写时保持签名并调用父类。

```python
class PLUGIN(BasePlugin):
    def __init__(self, config=None, record=None) -> None:
        super().__init__(config, record)
```

| 成员 | 类型 | 说明 |
| --- | --- | --- |
| `self.config` | `config_class` 的实例，没有声明时为 `None` | 插件自己的配置，见[配置声明](./schema) |
| `self.services` | `ServiceResolver` | 读取其他插件提供的服务，见[服务系统](./service) |
| `self.background_tasks` | `BackgroundTaskGroup` | 本实例的后台任务组 |
| `self.record` | `PluginRecord` | 注册表项，含版本、状态等元数据 |
| `self.plugin_name` | `str` | 注册名 |
| `self.lifecycle_context` | `LifecycleContext` | 当前生命周期语境 |

类属性：

| 属性 | 作用 |
| --- | --- |
| `config_class` | 配置类，见[配置声明](./schema) |
| `provides_services` | 提供的服务，`{服务名: 方法名}` |
| `requires` / `wants` | 依赖的服务名，元组 |
| `api_router` | 一个 FastAPI `APIRouter`，启用时挂载 |

### self.config

`self.config` 的类型是 `PluginConfig | None`。声明了 `config_class` 才有值，用之前先收窄类型。

```python
from auto_mas_core import BasePlugin, PluginConfig


class MyConfig(PluginConfig):
    ...


class PLUGIN(BasePlugin):
    config_class = MyConfig

    async def on_enable(self) -> None:
        assert isinstance(self.config, MyConfig)
```

读配置直接取属性。写配置先赋值，再 `await commit()`：

```python
# General 是 MyConfig 里声明的分组，字段怎么写见「配置声明」
self.config.general.interval = 10
await self.config.commit()
```

- 赋值只是暂存，`commit()` 才落地并触发信号。提交失败会抛 `ConfigAggregateError`。
- 配置被锁定时不能写入。
- 配置文件是 `config/plugins/<插件名>.yaml`。停用时保存并释放，再次启用时从文件重建。卸载插件时删除。

### self.background_tasks

插件的后台工作放进这个任务组，不要自己 `asyncio.create_task`。停用、卸载、重载时宿主会取消并等待组内任务结束。

```python
async def refresh(self) -> None:
    ...


async def on_enable(self) -> None:
    handle = self.background_tasks.spawn(lambda: self.refresh())
    if handle is None:
        # 任务组已关闭，或并发数/保留数已满
        return
```

`spawn(factory)` 接收一个返回协程的函数，返回 `BackgroundTaskHandle`；被拒绝时返回 `None`，不会抛异常，也不会创建协程。

| 方法 | 作用 |
| --- | --- |
| `handle.cancel()` | 取消任务 |
| `handle.done()` | 是否已结束 |
| `await handle.wait_done(timeout)` | 等待结束，不取结果；超时返回 `False` |
| `await handle.wait_take(timeout)` | 等待并取走结果，得到 `TaskOutcome`；超时抛 `TimeoutError`，`retain_result=False` 的任务抛 `KeyError` |
| `outcome.unwrap()` | 成功返回值，否则重新抛出原异常 |

- 任务组的并发数和保留数都等于设置里的插件操作上限（`Config.setting.plugin.max_operations`）。
- 结果默认保留 300 秒。不需要结果时传 `spawn(factory, retain_result=False)`。
- 没有执行超时。需要超时自己用 `asyncio.timeout`。
- 任务组在 `pre_enable()` 之前绑定，之前访问 `self.background_tasks` 会抛 `RuntimeError`。

### self.record 与 self.plugin_name

```python
info = self.record.info
logger.info(f"{self.plugin_name} {info.version} 来源={info.source}")
```

`info` 里常用的字段：`plugin_name`、`package_name`、`version`、`source`（`installed` 或 `local`）、`state`、`last_error`、`is_core`、`is_local`。`self.record` 只持有弱引用，注册项被回收后访问会抛 `RuntimeError`，不要在后台长期缓存。

## 生命周期钩子

所有钩子都是 `async def`，默认什么都不做。需要的才覆盖。

| 钩子 | 时机 |
| --- | --- |
| `pre_enable` | 启用前的预检。此时服务、后台任务已绑定，配置尚未激活，上游依赖尚未启用 |
| `on_enable` | 上游依赖已启用、配置已激活。启动你的业务资源 |
| `pre_disable` | 停用前的预检 |
| `on_disable` | 停用时。后台任务组此时已被取消并等待完毕 |
| `pre_reload` / `on_reload` | 重载前预检 / 重载时，作用于旧实例 |
| `pre_unload` / `on_unload` | 卸载前预检 / 卸载时 |
| `on_teardown` | 程序退出。不替代 `on_disable`，不落盘、不改启用状态 |
| `on_fail_cleanup` | 生命周期操作失败后的清理，收到出错阶段、语境和异常 |

启用顺序：构造实例 → 绑定后台任务组和服务 → `pre_enable` → 启用 `requires` 里的上游 → 激活配置 → `on_enable` → 登记 `provides_services` → 挂载 HTTP。

停用顺序：`pre_disable` → 先停用依赖本插件服务的下游 → 取消后台任务 → `on_disable` → 保存并释放配置 → 注销服务和 HTTP。

重载：`pre_reload` → 拆下依赖本插件的下游 → `on_reload` → 停用旧实例 → 重新导入插件代码 → 启用新实例 → 按原样恢复下游。

- `pre_enable` 抛异常时启用被中止，宿主释放已绑定的任务组和配置。`pre_enable` 之后的阶段（启用上游、激活配置、`on_enable`、登记服务和 HTTP）抛异常时，插件进入 `fault_runtime`，并调用 `on_fail_cleanup`。
- `pre_*` 钩子是业务预检，预检不通过时抛 `PluginPrecheckError`（SDK 已导出）。
- 核心插件 `auto_mas_core` 不可停用、重载和卸载。

::: warning
停用和卸载时，宿主关闭新调用准入并等待在途调用结束。外部进程、订阅、临时文件等私有资源由你在 `on_disable` 里释放。
:::

## 日志

```python
from auto_mas_core.utils import get_logger

logger = get_logger("我的插件")
logger.debug("调试信息")
logger.info("常规信息")
logger.warning("警告")
logger.error("错误")
```

`get_logger` 返回宿主的 loguru 日志器，模块名用于标记来源。写日志前可以用 `sanitize_log_message(text)` 去掉文本里的令牌等敏感参数。

## SDK 导出

### auto_mas_core 顶层

| 类别 | 名称 |
| --- | --- |
| 基类 | `BasePlugin`、`ExtensionPlugin`、`GameAdapterPlugin`、`ScriptAdapterPlugin`、`CorePlugin` |
| 插件配置 | `PluginConfig`、`PluginCommandEntry`、`trigger_field`、`ConfigEntry`、`ConfigGroup`、`ConfigCollection`、`collection`、`ConfigTableColumn` |
| 字段与界面 | `Trigger`、`Virtual`、`virtual_field`、`ui`、`ui_visibility`、`UiVisibility`、`select`、`encrypted`、`FilePath`、`FolderPath`、`ExecutablePath`、`JsonDictString`、`JsonListString` |
| 配置信号与锁 | `FieldChangeEvent`、`LockTicket` |
| 服务与 HTTP | `ServiceResolver`、`plugin_route` |
| 后台任务 | `BackgroundTaskGroup`、`BackgroundTaskHandle` |
| 下载 | `DownloadRequest`、`DownloadSnapshot` |
| 错误与状态 | `PluginError`、`PluginPrecheckError`、`PluginState`、`PLUGIN_API_VERSION` |
| 专项适配 | `ScriptTypeDecl`、`GameTypeDecl`、`UserExpander`、`ModeWorker`、`AutoProxyWorker`、`ScriptConfigWorker`、`ScriptViewWorker`、`MODE_WORKERS`、`AdapterRuntime`、`GameRuntime`、`GameControl`、`EmulatorControl`、`EmulatorExpect`、`DeviceHandle`、`DeviceStatus`、`DeviceSubscription`、`DeviceSubscribeError`、`TRIGGER_TICKET` |
| 宿主数据模型 | `ScriptEntry`、`UserEntry`、`PlanEntry`、`GameEntry`、`GameDeviceEntry`、`EmulatorEntry`、`EmulatorDeviceEntry`、`TagItem`、`TaskMode`、`TaskScriptItem`、`TaskUserItem`、`LogRecord` |

专项适配相关的类见[独立专项适配](../adapter/standalone)。

以下名称按需加载，第一次访问时才取得宿主单例：

| 名称 | 说明 |
| --- | --- |
| `Config` | 宿主全局配置 |
| `Plugin` | 插件管理器 |
| `Publisher` | WebSocket 发布器，见[事件系统](./event) |
| `protocol` | WebSocket 消息类型常量 |
| `History` | 历史记录 |
| `WSTaskNoticeData` | 任务通知数据模型 |

```python
from auto_mas_core import Config

version = Config.VERSION
```

未列入导出表的名称会抛 `AttributeError`。

### auto_mas_core.utils

| 名称 | 作用 |
| --- | --- |
| `get_logger` | 取得日志器 |
| `get_setting(group, name, default)` | 读取宿主设置 `Config.setting.<group>.<name>`，读不到返回默认值 |
| `ProcessRunner` | 运行子进程并取得输出 |
| `ProcessManager`、`ProcessInfo`、`ProcessResult` | 进程管理与结果 |
| `is_process_running` | 判断进程是否在运行 |
| `WebSocketClient` | WebSocket 客户端 |
| `read_file` / `write_file` | 读写配置文件 |
| `decode_bytes` | 按多种编码尝试解码字节 |
| `dpapi_encrypt` / `dpapi_decrypt` | Windows DPAPI 加解密 |
| `sanitize_log_message` | 日志脱敏 |
| `busy_wait` | 高精度忙等待，占用 CPU |
| `to_pep440` | 应用版本标签转 PEP 440 |

运行子进程：

```python
from auto_mas_core.utils import ProcessRunner

result = await ProcessRunner.run_process("python", "--version", timeout=10)
if result.returncode == 0:
    print(result.stdout)
else:
    print(result.failure_detail())
```

`run_process` 的 `timeout` 默认 60 秒。

### auto_mas_core.services

导出四个宿主单例。

| 名称 | 作用 |
| --- | --- |
| `Downloads` | 统一下载中心 |
| `Operations` | 插件配置触发器的操作调度 |
| `Notify` | 通知分发 |
| `BackgroundTaskManager` | 应用级后台任务管理器。插件用 `self.background_tasks`，一般不直接用它 |

下载：

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

- `request()` 被拒绝（容量忙）时返回 `False`；下载中心已关闭时抛 `RuntimeError`。
- `key` 可以替代 URL 作为资源键，URL 变化时不会误当成新资源。文件身份按 `rid`、`content_hash`、空身份的顺序确定。
- 用完调用 `release()`。最后一个使用者释放时，未完成的下载被取消。
- `query(request)` 立即取快照，`list(owner=...)` 列出某个使用者的下载。

Operations 的用法见[配置声明](./schema#触发器)。

### 通知（auto_mas_core.notify）

```python
from auto_mas_core.notify import Notify, NotifyPayload

result = await Notify.send_global(
    NotifyPayload(title="通知标题", text="通知正文"),
    include_system=True,
)
if result.attempted == 0:
    ...  # 没有可用渠道，并不代表已投递
```

| 方法 | 作用 |
| --- | --- |
| `send_global(payload, include_system=False, ...)` | 发到全局通知渠道 |
| `send_user(payload, user_config, ...)` | 发到某个用户配置的渠道 |
| `send_statistics(payload, user_config, ...)` | 按用户和全局的统计通知策略发送 |
| `push_proxy_result(title=..., report=..., ...)` | 推送代理结果报告 |

同一模块还导出 `DispatchResult`、`NotificationImage`、`TaskResultReport`、`image_reference`。没有插件专用的通知协议，插件复用宿主的渠道和模板。

## HTTP 接口

插件可以挂自己的 HTTP 接口，路径前缀是 `/api/plugin/<插件名>`。

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

- 接口地址是 `/api/plugin/<插件名>/ping`。`methods` 省略时是 `GET`。
- 处理函数要么没有参数（`self` 不算），要么只有一个参数，收到的是请求对象 `Request`。
- 返回 `dict`、`list` 或 `None` 时按 JSON 响应；返回 `Response` 原样发出；其他值转成文本。抛异常时返回 500，详情只在本地日志。
- 启用时挂载，停用时卸下。宿主暂停调用期间（停用、重载、卸载进行中）和程序退出时返回 503；已停用的插件没有挂载，返回 404。
- 需要类型化的路由和 OpenAPI 时，在类上设置 `api_router = APIRouter()`，写法见[前端与后端的对接](../adapter/frontend)。

## 同步与异步

- 生命周期钩子、`Downloads.wait`、`Notify.*`、`ProcessRunner.run_process` 都是协程，必须 `await`。
- 后台任务的 `factory` 要返回协程，如 `lambda: self.refresh()`。
- `plugin_route` 的处理函数可以是同步或异步。同步函数直接在事件循环里执行，耗时操作请用异步或放进后台任务。

## 常见问题

### 访问 self.background_tasks 报「插件未绑定背景任务组」

在构造函数里访问了。任务组在 `pre_enable()` 之前才绑定，请在钩子里使用。

### spawn 返回 None

任务组已关闭（正在停用），或同时运行/保留的任务数已达上限。检查返回值，不要假定一定成功。

### 访问 auto_mas_core 的某个名称报 AttributeError

名称不在 SDK 导出表里。不要直接导入宿主 `app.*` 的内部模块，它们不属于 SDK 契约，随时可能变化。
