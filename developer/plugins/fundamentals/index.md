# 插件基础

AUTO-MAS 宿主通过 Python 包顶层导出的 `PLUGIN` 加载插件。插件通过 `auto_mas_core` 接入配置、服务和后台任务。宿主负责初始化这些能力，以及在停用、重载和退出时收尾。

## 最小插件

在插件包的 `__init__.py` 中导出 `BasePlugin` 的子类。下面的例子在启用和停用时分别记录一条日志。完整的项目目录和启用步骤见[开发与调试](./develop.md)。

```python
from auto_mas_core import BasePlugin, PLUGIN_API_VERSION
from auto_mas_core.utils import get_logger

logger = get_logger("hello_demo")


class PLUGIN(BasePlugin):
    async def on_enable(self) -> None:
        logger.info("hello_demo 已启用")

    async def on_disable(self) -> None:
        logger.info("hello_demo 已停用")
```

- **包级导出**

  - `PLUGIN`：宿主启用插件时实例化的类。
  - `PLUGIN_API_VERSION`：宿主协议版本。插件依赖的 SDK 发行版本在 `pyproject.toml` 中单独声明。

## 插件实例

`BasePlugin` 定义在主仓 `app/plugin/base/plugin.py`。宿主在启用时创建实例，在执行钩子前绑定服务解析器和后台任务组。需要自定义构造函数时，保留 `config` 和 `record` 参数并调用父类。

- **配置和身份**

  - `self.config`：声明了 `config_class` 时，宿主提供该类的配置实例。启用后直接读分组字段，赋值后通过 `await self.config.commit()` 提交。见[配置声明](./schema.md)。
  - `self.record`：注册表项，包含插件 ID、包名、版本、来源、状态和最近错误。实例只弱引用它。
  - `self.plugin_name`、`self.lifecycle_context`：当前注册名和生命周期上下文。注册项回收后，与其关联的属性会拒绝访问。

- **运行设施**

  - `self.services`：按服务名取得其他插件提供的可调用对象。见[服务系统](./service.md)。
  - `self.background_tasks`：宿主为当前启用实例分配的任务组。`pre_enable()` 开始前完成绑定。

### self.background_tasks

将属于插件实例的后台工作交给 `self.background_tasks`。停用或重载时，宿主关闭该组的新任务入口，取消并等待已有任务。

```python
<任务句柄或 None> = self.background_tasks.spawn(
    factory=<返回协程的工厂函数>,
    retain_result=<是否保留结果>,
)
```

- **提交任务**

  - `factory=`：例如 `lambda: self.refresh()`。任务组受理后再调用工厂创建协程。
  - `retain_result=`：默认保留结果。无需读取结果时传入 `False`。
  - `<任务句柄或 None>`：组已关闭或达到容量上限时返回 `None`。需要限制执行时间时，在任务内部设置超时。

- **等待任务**

  - `handle.cancel()`、`handle.done()`：请求取消、检查是否已经结束。
  - `await handle.wait_done(timeout)`：等待结束，超时时返回 `False`。
  - `await handle.wait_take(timeout)`：等待并取走 `TaskOutcome`，超时抛出 `TimeoutError`。未保留结果时不能取结果。
  - `outcome.unwrap()`：成功时返回结果，失败时重新抛出原异常。

## 生命周期钩子

生命周期由 `app/core/plugin/manager.py` 编排，插件覆盖所需的异步钩子。各操作如何处理依赖、配置和在途调用，见[宿主与运行机制](./runtime.md)。

- **启用**

  - `pre_enable()`：检查启用前置条件。此时服务和后台组已经绑定，配置尚未激活，必需服务的提供方还可能未启用。
  - `on_enable()`：启动业务资源。此时必需服务已可用，自有配置已激活。

- **停用、重载和卸载**

  - `pre_disable()`、`on_disable()`：前者检查停用条件，后者关闭启用期间取得的进程、订阅和其他资源。
  - `pre_reload()`、`on_reload()`：对旧实例执行重载预检和处理。新实例依旧通过 `on_enable()` 建立业务资源。
  - `pre_unload()`、`on_unload()`：处理发行包卸载前的检查和专属清理。

- **失败与退出**

  - `on_fail_cleanup(during, lifecycle_context, exc)`：按失败阶段清理已取得的业务资源。
  - `on_teardown()`：后端进程退出时回收资源，保留下次启动所需的启用状态。

## 使用 SDK 能力

插件只通过 `auto_mas_core` 的公开导出调用宿主。导出名称和类型声明以目标宿主附带的 `plugins/auto_mas_core/src/auto_mas_core/` 为准。

- **日志与同步工作**

  - `auto_mas_core.utils.get_logger(<模块名>)` 取得日志器。输出运行信息时保持密钥、令牌和账号凭据脱敏。
  - `auto_mas_core.services.run_in_worker` 可以在异步流程中执行阻塞的同步工作。

- **后台能力**

  - `auto_mas_core.services.Downloads` 提供宿主统一的下载生命周期，示例见[宿主与运行机制](./runtime.md#背景任务、下载与预下载)。
  - `auto_mas_core.notify` 导出宿主的 `Notify`、消息和报告模型。按[通知系统](../../core/notification.md)选择全局、账号或任务通知入口。
  - `auto_mas_core.Publisher` 负责向宿主前端推送消息。见[事件系统](./event.md#websocket-推送)。

- **HTTP 接口**

  - `api_router` 提供类型化的 FastAPI 路由并参与 OpenAPI 生成。
  - `@plugin_route` 提供插件通用网关上的接口。两种声明的示例见[前端与后端的对接](./frontend.md#插件的-http-接口)。
