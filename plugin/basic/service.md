# 服务系统

服务系统用于在插件之间建立“低耦合能力共享”，让插件通过服务名进行协作，而不是硬编码引用具体插件类。

本文介绍当前 AUTO-MAS 中服务机制的设计、生命周期行为和最小可用写法。

## 服务模型

### 核心概念

- 服务名：字符串键，例如 `clock`、`store`、`auth`。
- 服务声明：声明“我提供这个服务名”，但不一定立即可用。
- 服务赋值：将服务实例写入注册中心，使服务变为可读。
- 硬依赖（needs）：缺失时实例不会启动。
- 软依赖（wants）：缺失时允许启动。

### 声明与赋值分离

服务注册采用两步：

1. 声明：`provide(name)` 只登记服务槽位和提供者。
2. 赋值：`set(name, value)` 才写入实例值并触发依赖方重算。

这能避免“服务尚未 ready 就被误用”的时序问题。

## 插件侧入口

插件可以通过 `ctx` 直接使用服务能力，也可以通过 `ctx.service` 使用同名接口。

### 快速方法

- `ctx.provide(name)`：声明服务。
- `ctx.set(name, value)`：设置服务值。
- `ctx.get(name, default=None)`：读取服务值。
- `ctx.inject(needs=None, wants=None, ready=None)`：动态注入依赖并在满足时触发回调。

### Service 方法

- `ctx.service.provide(name)`
- `ctx.service.set(name, value)`
- `ctx.service.get(name, default=None)`
- `ctx.service.inject(needs=None, wants=None, ready=None)`
- `ctx.service.miss()`：返回当前实例缺失的硬依赖集合。

## 静态依赖声明

插件类支持静态声明字段：

- `provides`：当前实例提供哪些服务。
- `needs`：启动必须满足的服务。
- `wants`：可选服务。

声明值支持两种写法（会统一归一化）：

- 字符串：`provides = "clock"`
- 列表：`provides = ["clock", "ticker"]`

示例：

```python
class Plugin:
    provides = "clock"
    needs = ["store"]
    wants = "notify"

    def __init__(self, ctx):
        self.ctx = ctx
```

注意：

- 当前开发版本不提供旧式静态 `inject` 兼容层。
- 静态声明仅识别 `provides`、`needs`、`wants`。

## 启动顺序与依赖策略

### 加载顺序

加载器会先扫描启用实例并解析 `provides/needs/wants`，然后构建依赖图进行拓扑排序。

::: warning 重复服务名会怎么样？

同一个服务名只允许一个提供者。

- 若另一实例尝试声明已存在的服务名，实例启动会被拒绝并记录错误。
- 该错误不会中断插件系统整体启动流程，但该实例会处于失败态。

:::

排序目标：

- provider 优先于 consumer 启动。
- 若检测到依赖环，会记录 warning，并将剩余实例按原顺序继续尝试加载。

### 缺失硬依赖

当实例存在缺失 `needs` 时：

- 该实例会被跳过启动并标记错误。
- 其他实例继续启动，不会中断插件系统整体启动。

这与“单实例失败不拖垮全局”的策略一致。

## 服务变化与自动重算

注册中心在 `set` / `drop` 时会触发两阶段通知：

- `before`：通知依赖方准备停用。
- `after`：通知依赖方尝试重新加载。

加载器监听这两个阶段并执行重算流程：

1. 收集受影响且处于 active 的实例。
2. 先卸载这些实例。
3. 再按记录的配置与依赖声明尝试重新加载。

## 未声明访问告警

`ctx.get(name)` 在读取服务时会检查该服务是否在当前实例声明过（来自 `provides/needs/wants` 或动态 `inject`）。

- 已声明：正常读取。
- 未声明：记录 warning，但不会立即抛异常。

这是一种“渐进约束”策略，便于旧插件平滑迁移。

## 最小可用示例

### 1) 提供服务的插件

```python
class Plugin:
    provides = {"clock"}

    def __init__(self, ctx):
        self.ctx = ctx

    async def on_start(self):
        self.ctx.set("clock", {"now": "2026-04-25T12:00:00+08:00"})

    async def on_stop(self, reason: str):
        _ = reason
```

### 2) 依赖服务的插件

```python
class Plugin:
    needs = {"clock"}

    def __init__(self, ctx):
        self.ctx = ctx

    async def on_start(self):
        clock = self.ctx.get("clock", {})
        self.ctx.logger.info(f"clock={clock}")

    async def on_stop(self, reason: str):
        _ = reason
```

### 3) 动态注入回调

```python
class Plugin:
    def __init__(self, ctx):
        self.ctx = ctx

    async def on_start(self):
        def ready(ctx):
            data = ctx.get("clock", {})
            ctx.logger.info(f"inject ready: {data}")

        self.ctx.inject(needs={"clock"}, wants={"notify"}, ready=ready)

    async def on_stop(self, reason: str):
        _ = reason
```

