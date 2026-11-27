# 调用宿主能力

插件可以从 `auto_mas_core` 导入宿主公开的接口，也可以通过插件实例访问宿主在启用时绑定的入口。

## 从 SDK 导入

SDK 的公开导出位于目标宿主版本的 `plugins/auto_mas_core/src/auto_mas_core/`。常用入口如下。

- **宿主配置**：从 `auto_mas_core` 导入 `Config` 访问宿主配置，例如用 `Config.PlanConfig` 访问计划配置集合。配置框架的操作入口 `config_manager` 也从 `auto_mas_core` 导入。见[配置框架](../../core/config.md)。
- **通知**：从 `auto_mas_core.notify` 导入 `Notify`，通过宿主发送通知。
- **下载**：从 `auto_mas_core.services` 导入 `Downloads` 取得下载服务。下载请求类型 `DownloadRequest` 从 `auto_mas_core` 导入。
- **日志**：从 `auto_mas_core.utils` 导入 `get_logger`，为插件取得日志记录器。
- **前端消息**：从 `auto_mas_core` 导入 `Publisher`，向前端发布消息。
- **工作线程**：从 `auto_mas_core.services` 导入 `run_in_worker`，将同步阻塞调用交给工作线程。

## 从插件实例访问

### 读取插件配置

插件类声明 `config_class` 后，插件通过 `self.config` 访问插件配置。宿主在调用 `on_enable()` 前激活该配置，插件可以在 `on_enable()` 中读取保存的值。未声明 `config_class` 时，`self.config` 为 `None`。

### 提交后台任务

宿主在调用 `pre_enable()` 前绑定 `self.background_tasks`，并随插件实例的生命周期回收任务组。插件使用 `spawn()` 向当前任务组提交后台任务。

```python
<任务句柄或 None> = self.background_tasks.spawn(<协程工厂>)
```

- **提交任务**
  - `<协程工厂>`：调用后返回协程的函数，例如 `lambda: self.refresh_cache()`。
  - `<任务句柄或 None>`：任务受理后返回句柄，未受理时返回 `None`。

### 调用其他插件的服务

插件在类上按服务名声明依赖，并在 `on_enable()` 或之后通过 `self.services` 取得服务方法。宿主负责按服务名找到已登记的方法，提供方插件负责执行具体操作。提供方的写法见[服务声明](./service.md#服务声明)。

```python
from auto_mas_core import BasePlugin


class <插件类>(BasePlugin):
    requires = ("<必需服务名>",)
    wants = ("<可选服务名>",)

    async def on_enable(self) -> None:
        required = self.services.require("<必需服务名>")
        await required(<服务参数>)

        optional = self.services.get("<可选服务名>")
        if optional is not None:
            await optional(<服务参数>)
```

- **声明依赖**
  - `<必需服务名>`：`requires` 中的服务名。宿主在调用 `on_enable()` 前启用其提供方。
  - `<可选服务名>`：`wants` 中的服务名。插件可以使用提供方当前已登记的服务，并处理服务缺失的情况。
- **调用服务**
  - `self.services.require()`：取得必需服务，服务不可用时抛出异常。
  - `self.services.get()`：取得可选服务，服务不可用时返回默认值，默认值为 `None`。
