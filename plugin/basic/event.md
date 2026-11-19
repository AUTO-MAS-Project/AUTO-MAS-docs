# 事件系统

::: warning
本文以 `dev` 分支 2026-11-18 的实现为准。插件系统仍在变化，接口以你安装的 AUTO-MAS 源码为准。
:::

## 旧事件总线已移除

旧版文档里的这些东西，在当前实现里都找不到：

- `ctx.event.on` / `off` / `emit` / `emit_async`
- `@on_event` 装饰器和 `mas.plugins` 模块
- 载荷里的 `task.start`、`task.progress`、`task.log`、`task.exit`、`script.start`、`script.exit` 等事件，以及 `event_version`、`source` 这些信封字段

插件不再有「按事件名订阅、广播」的总线，所以也没有 `scope`、`priority`、`error_policy`。照旧文档写的 `@on_event("script.exit")` 不会收到任何东西。

::: tip
需要「某件事发生时做点什么」，按下面的顺序选：生命周期钩子、配置信号、服务调用、配置触发器。每种的边界都不同，下文逐个说明。
:::

## 现在用什么代替

| 想做的事 | 用什么 |
| --- | --- |
| 在插件启停、重载时执行代码 | [生命周期钩子](#生命周期钩子) |
| 配置某个字段变化时执行代码 | [配置信号](#配置信号) |
| 让别的插件调用你的能力，或调用别的插件 | [服务系统](./service) |
| 用户在界面点按钮时执行代码 | [触发器](./schema#触发器) |
| 向前端推一条消息 | [WebSocket 推送](#websocket-推送) |
| 给用户发通知 | [通知](./core) |
| 在任务、账号运行结束时做点什么 | [没有公开入口](#任务与账号运行事件) |

## 生命周期钩子

`BasePlugin` 提供一组 `async` 钩子：`pre_enable`、`on_enable`、`pre_disable`、`on_disable`、`pre_reload`、`on_reload`、`pre_unload`、`on_unload`、`on_teardown`、`on_fail_cleanup`。顺序和注意事项见[核心功能](./core#生命周期钩子)。

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

## 配置信号

配置节点自带变更信号，基于 blinker。SDK 导出 `FieldChangeEvent`，宿主自己也用同一机制订阅任务和配置的变化。

订阅写法：

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

- 回调的参数是 `(sender, event)`，可以是同步或异步函数。异步回调会被依次等待。
- 回调要用模块级函数。实例方法不能挂属性，默认的弱引用订阅会抛 `TypeError`。
- `kind="set"` 表示运行期变更，`"init_set"` 是配置激活时的初始化赋值。`phase` 默认 `"runtime"`，只收运行期事件。
- `group`、`field` 用来过滤，省略则收该类全部字段的变化。
- 用 `MyConfig.connect` 订阅整个类；用 `self.config.connect` 只订阅这一个实例。
- 同一组 `(phase, kind, group, field, sender)` 重复订阅会抛 `ValueError`，先 `disconnect`。
- 在回调里改配置要自己 `commit()`；回调返回前必须提交当次暂存，否则抛 `RuntimeError`。

::: warning
信号只在字段值真的改变时触发。写入同一个值不触发。
:::

## 服务调用

要让另一个插件在合适的时候被调用，不要自己造订阅机制，用服务：被调用方 `provides_services`，调用方 `self.services.get(...)`。见[服务系统](./service)。

服务是「主动调用」，不是「被动通知」。提供方无法向消费方广播。需要广播的场景，让消费方轮询，或者让消费方把回调函数当参数传给提供方的服务方法，再由提供方保存。保存回调时，提供方要在 `on_disable` 里清掉，否则消费方卸载后回调还在。

## 触发器

用户在插件设置页点按钮，宿主调用配置类里 `@trigger_field` 标记的方法，作为后台操作运行，结束后通过 WebSocket 发布操作终态（`id="Operations"`、`type="operation.updated"`）。见[配置声明](./schema#触发器)。

## WebSocket 推送

`auto_mas_core.Publisher` 是宿主向前端发消息的出口，`auto_mas_core.protocol` 是消息类型常量。两者按需加载。

```python
from auto_mas_core import Publisher

sent = await Publisher.send(
    id="my_plugin",
    type="my_plugin.changed",
    data={"count": 3},
)
```

- `send(id, type, data)` 向前端的主连接发一条 `{id, type, data}` 信封消息，返回是否发出。主连接未建立时消息被丢弃，返回 `False`，不缓存、不重发。
- `data` 可以是 pydantic 模型或字典。
- 这是**单向**通道：只发往前端，不会送到其他插件。
- `protocol` 里的类型都是宿主与前端约定好的（任务、更新、模拟器、插件能力变化等）。前端只处理它认识的类型。当前 SDK 没有让插件加载自己的前端代码的入口，所以自定义类型的消息，前端不会有页面去消费。

::: warning
`type` 不要借用宿主已有的类型常量冒充，前端会按那个类型的载荷格式解析。
:::

## 任务与账号运行事件

宿主在任务开始、结束、脚本执行边界时会向前端推送 `task.created`、`task.completed`、`task.script.started`、`task.script.finished` 等消息，它们定义在 `protocol` 里，**只发往前端，没有提供给普通插件的订阅入口**。

宿主内部还有 `TaskManager.task_ended` 这类信号，但 SDK 没有导出，也不属于插件契约，不要依赖。

如果你的需求是「在某个脚本运行时插入自己的逻辑」，这属于专项适配：用 `ScriptTypeDecl` 注册脚本类型，并实现 `UserExpander` 和对应的 `ModeWorker`，由宿主在任务里调度它们。见[独立专项适配](../adapter/standalone)。

## 插件系统自己的信号

`app/plugin/signals.py` 里有一组插件系统内部用的 blinker 信号：`service_changed`、`sources_changed`、`capabilities_changed`、`install_progress`、`upgrade_progress`、`precheck_failed`。

- 它们不在 SDK 导出表里，属于宿主内部实现。
- 目前只有 `service_changed`（服务登记或注销时）、`sources_changed`（源码目录变化时）、`capabilities_changed`（插件启用或停用时）有地方发送；其余三个只有声明，没有发送者。
- 不要在插件里订阅它们。需要知道服务是否可用时，直接 `self.services.get(...)` 看返回值。

## 迁移对照

| 旧写法 | 现在 |
| --- | --- |
| `ctx.event.on("script.exit", ...)` | 没有对应入口，见[任务与账号运行事件](#任务与账号运行事件) |
| `ctx.event.emit_async("x", ...)` | 对方是你自己的插件：提供服务；对方是前端：`Publisher.send` |
| `@on_event(...)` | 配置信号或生命周期钩子 |
| `scope="instance"` / `"global"` | 不再有作用域概念 |
| `async def on_start(self)` | `async def on_enable(self)` |
| `async def on_stop(self, reason)` | `async def on_disable(self)`，没有 `reason` 参数 |
