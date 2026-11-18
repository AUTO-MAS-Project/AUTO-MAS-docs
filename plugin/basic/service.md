# 服务系统

::: warning
本文以 `dev` 分支 2026-11-18 的实现为准。插件系统仍在变化，接口以你安装的 AUTO-MAS 源码为准。
:::

服务让插件之间按名字调用对方的方法，不用导入对方的代码。

::: tip 和旧版的区别
旧版的 `ctx.provide` / `ctx.set` / `ctx.get` / `ctx.inject`、`ctx.service.miss()`，以及类属性 `provides`、`needs`、`wants` 都已移除。现在：

- 提供方用类属性 `provides_services = {服务名: 方法名}`，服务就是插件上的一个方法，不再「声明后再赋值」。
- 依赖用 `requires`（硬依赖，旧名 `needs`）和 `wants`（可选依赖）。
- 消费方用 `self.services.get(...)` 和 `self.services.require(...)`。
:::

## 概念

| 概念 | 说明 |
| --- | --- |
| 服务名 | 字符串，建议带命名空间，如 `arknights.stage_info` |
| 服务 | 提供方插件上的一个方法。消费方拿到的是这个方法的可调用对象 |
| `requires` | 硬依赖。服务缺失时本插件不能启用；提供方被停用时本插件跟着停用 |
| `wants` | 可选依赖。服务缺失时本插件照常启用，由你自己降级 |

一个服务名只能有一个提供方。

## 提供服务

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

- 键是服务名，值是**本类上的方法名**（字符串）。
- 方法可以是同步或异步，参数和返回值自定。
- 服务在 `on_enable()` 返回**之后**登记，停用时（`on_disable()` 之后）注销。`on_enable` 里本插件的服务还不能被别人取到。
- 方法名指向的属性不存在或不可调用时，该服务**不会登记**，也不报错。名字写对。
- 服务名已被另一个插件登记时，登记抛 `PluginError`，本插件启用失败。

## 使用服务

### 硬依赖：requires

```python
from auto_mas_core import BasePlugin
from auto_mas_core.utils import get_logger

logger = get_logger("clock_user")


class PLUGIN(BasePlugin):
    requires = ("clock.now",)

    async def on_enable(self) -> None:
        now = self.services.require("clock.now")
        logger.info(f"现在是 {await now()}")
```

- `requires` 写成元组或列表。**不要写成字符串**：`requires = "clock.now"` 会被拆成单个字符。
- 启用本插件时，宿主先启用提供这些服务的插件，所以 `on_enable` 里一定取得到。
- `require(name)` 在服务不存在时抛 `PluginError`，`payload["service"]` 是服务名。

### 可选依赖：wants

```python
from auto_mas_core import BasePlugin, PluginError
from auto_mas_core.utils import get_logger

logger = get_logger("report")


class PLUGIN(BasePlugin):
    wants = ("clock.now",)

    async def build_report(self) -> str:
        now = self.services.get("clock.now")
        if now is None:
            return "报告（无时间）"
        try:
            return f"报告 {await now()}"
        except PluginError:
            # 提供方正在停用或重载，调用被拒绝
            return "报告（无时间）"
```

- `get(name, default=None)` 取不到时返回默认值，不抛异常。
- 用到时再取，**不要在 `on_enable` 里取了存起来**。可选服务可能晚于你启用，也可能中途消失。提供方重载或停用后，旧的可调用对象再调用会抛 `PluginError`。
- 调用服务时一并处理 `PluginError`，它是下面几类错误的基类。

### 调用的返回值

`get` / `require` 返回的可调用对象保持原方法的同步或异步形态：异步方法要 `await`，同步方法直接得到结果。

## 启用、停用、重载时发生什么

只有 `requires` 参与级联，`wants` 不参与。

| 操作 | 对依赖方（requires）的影响 |
| --- | --- |
| 启用依赖方 | 先启用它所需服务的提供方；提供方启用失败，依赖方也启用失败 |
| 停用提供方 | 先停用所有启用中的依赖方，再停用提供方 |
| 重载提供方 | 依赖方先被拆下，提供方重载完成后，依赖方按拆下时的依赖树重新启用 |
| 卸载提供方 | 依赖方先被卸载 |
| 程序退出 | 依赖方先执行 `on_teardown`，提供方后执行 |

细节：

- 找不到提供某个服务的插件（没安装、或类上没有声明）：启用抛 `PluginPrecheckError`，信息里有服务名。
- 依赖成环：在环上检测到回环时抛 `PluginPrecheckError`（信息里有由注册项 uid 组成的环路），它被外层的启用包成 `PluginOperationError` 抛出，环上已开始启用的插件进入 `fault_runtime`。
- 重载提供方后，某个依赖方重新启用失败：该依赖方进入 `fault_runtime`，保持停用，`last_error` 记录原因；其他依赖方继续恢复。
- 停用提供方可能连带停用专项适配（脚本类型、游戏类型）。宿主提供 `disable_impact`，汇总停用时会删除的配置领域，供前端确认。

### 可选依赖没有这些保证

`wants` 里的服务：

- 不影响启动顺序。提供方可能在你之后才启用，此时 `get` 返回默认值。
- 提供方启用、停用、重载时，你**不会**被通知，也不会被重载。
- 宿主只记录当时缺哪些可选服务，没有对外的查询接口。

所以可选依赖的写法固定为：每次用的时候 `get`，判空，降级。

## 调用被拒绝的情况

服务调用经过提供方的调用准入：

- 提供方正在停用、卸载、重载：新调用被拒绝，抛 `PluginError` 的子类，信息是「插件正在释放，不再受理调用」。已经开始的调用会被等待到结束。
- 程序正在退出：`get` 返回默认值，`require` 抛错。

因此在后台任务里长期调用别的插件的服务时，要处理调用失败，并在下次循环里重新 `get`。

## 一个真实的例子

MAA 适配器插件提供三个关卡数据服务：

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

消费方按 `self.services.get("arknights.stage_info")` 取用即可，不需要知道 MAA 适配器的包名。

## 不要自己登记服务

`auto_mas_core.plugin.services` 暴露的是宿主的服务注册表本体，带 `register` / `unregister`。插件不要用它登记服务：登记的对象没有调用准入保护，停用时也不保证被注销，可能留下失效的引用。一律用 `provides_services`。

## 迁移对照

| 旧写法 | 现在 |
| --- | --- |
| `provides = "clock"` + `ctx.set("clock", value)` | `provides_services = {"clock.now": "now"}`，服务是方法 |
| `needs = ["store"]` | `requires = ("store.get",)` |
| `wants = "notify"` | `wants = ("notify.send",)`，必须是元组或列表 |
| `ctx.get("clock", {})` | `self.services.get("clock.now")` |
| `ctx.inject(needs=..., ready=...)` | 没有对应能力；静态声明 `requires`，在 `on_enable` 里取用 |
| `ctx.service.miss()` | 没有对应能力 |
| 读取未声明的服务只告警 | 没有声明检查；`get` 取不到就是默认值 |
