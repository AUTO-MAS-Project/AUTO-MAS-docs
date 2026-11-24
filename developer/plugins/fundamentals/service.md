# 服务系统

插件通过服务名调用其他插件公开的方法。宿主在启用后登记服务，在释放实例时撤销登记，并通过提供方的调用门管理在途调用。运行期使用 `self.services` 查找服务。

## 提供服务

在 `BasePlugin` 子类声明 `provides_services`。服务名应体现所属业务，例如 `arknights.stage_info`。

```python
from auto_mas_core import BasePlugin


class <插件类>(BasePlugin):
    provides_services = {"<服务名>": "<方法名>"}

    async def <方法名>(self, <请求参数>):
        return <处理结果>
```

- **服务声明**

  - `<服务名>`：宿主注册表中的唯一名称。一个名称只能由一个插件提供。
  - `<方法名>`：本类中对应的可调用方法名。方法可以是同步或异步。
  - `<请求参数>`、`<处理结果>`：由提供方和消费方约定的调用契约。提供方应保持对已有消费者的兼容。

1. **准备资源**：在提供方的 `on_enable()` 建立方法需要的资源。
2. **登记服务**：钩子成功返回后，由宿主读取 `provides_services` 并登记。名称冲突时启用失败。没有对应可调用方法的声明不会登记。
3. **撤销服务**：停用、重载和卸载过程中，宿主先控制新调用的准入，再随提供方实例撤销服务。

## 使用服务

### 必需服务 | requires

必需服务参与宿主的依赖排序。将名称写入 `requires`，在 `on_enable()` 或运行期使用 `require()` 查找。

```python
class <消费方插件类>(BasePlugin):
    requires = ("<服务名>",)

    async def on_enable(self) -> None:
        <服务方法> = self.services.require("<服务名>")
        <处理结果> = await <服务方法>(<请求参数>)
```

- **依赖与调用**

  - `requires`：由服务名组成的元组或列表。只依赖一个服务时，保留单元素元组的逗号。
  - `require(<服务名>)`：取得已登记的方法，找不到时抛出 `PluginError`。宿主启用消费方时先启用提供方。
  - `<服务方法>`：保持原方法的同步或异步形态。异步方法需要 `await`。

### 可选服务 | wants

可选服务不改变生命周期顺序。调用时查找，服务缺失或提供方已在释放时走同一业务降级路径。

```python
from auto_mas_core import BasePlugin, PluginError


class <消费方插件类>(BasePlugin):
    wants = ("<服务名>",)

    async def <业务方法>(self):
        <服务方法> = self.services.get("<服务名>")
        if <服务方法> is None:
            return <备用结果>
        try:
            return await <服务方法>(<请求参数>)
        except PluginError:
            return <备用结果>
```

- **依赖与调用**

  - `wants`：由可选服务名组成的元组或列表。服务缺失时仍可启用插件。
  - `get(<服务名>, default=<备用值>)`：服务可用时返回方法，否则返回默认值。提供方启用、停用或重载后，下次使用时重新查找。

## 服务依赖与生命周期

依赖排序和操作编排由 `app/core/plugin/manager.py` 负责。

1. **启用消费方**：宿主先启用必需服务的提供方。提供方缺失、启用失败或依赖成环时，消费方无法启用。
2. **停用或卸载提供方**：先处理硬依赖的下游，再释放提供方。受影响的适配配置见[配置文件与数据](./config-files.md)。
3. **重载提供方**：暂时撤下受影响的下游。提供方重新启用后，按原依赖关系逐个恢复。下游恢复失败时，错误写入各自的注册项。
4. **退出宿主**：下游先执行 `on_teardown()`，提供方随后收尾。

`wants` 不参与级联。停用、重载或退出时，如果调用门已禁止新调用，已有方法引用也可能拒绝执行。插件通过类上的 `provides_services` 登记服务，以便宿主跟踪实例和调用生命周期。
