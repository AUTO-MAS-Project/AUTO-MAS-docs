# 配置声明

插件自有配置继承 `PluginConfig`。在插件类上声明 `config_class` 后，宿主负责激活、校验和持久化，并根据界面提示渲染插件设置页。共用机制见[配置框架](../../core/config.md)，文件位置见[配置文件与数据](./config-files.md)。

## 最小示例

```python
from typing import Annotated, Literal

from pydantic import Field

from auto_mas_core import BasePlugin, ConfigGroup, PluginConfig, encrypted


class DemoConfig(PluginConfig):
    class General(ConfigGroup):
        interval: int = Field(default=5, ge=1, le=60, description="检查间隔")
        mode: Literal["always", "failed_only", "never"] = "always"
        token: Annotated[str, encrypted()] = ""

    general: General = Field(default_factory=General)


class PLUGIN(BasePlugin):
    config_class = DemoConfig

    async def on_enable(self) -> None:
        assert isinstance(self.config, DemoConfig)
        interval = self.config.general.interval
```

- **声明和读写**

  - `ConfigGroup`：用于声明一个分组的字段和约束。用 `default_factory` 为每个配置实例创建分组。
  - `config_class`：宿主启用插件时激活的自有配置类型。
  - `self.config.<分组名>.<字段名>`：读取值或暂存修改。修改后调用 `await self.config.commit()`。
  - `ui` 为配置框架保留的属性名。配置分组使用其他名称。

## 界面控件

宿主根据字段生成 `ui.hints`，通用设置页在 `frontend/src/components/DeclarativeControl.vue` 渲染支持的控件。

- **常用字段**

  - `bool` 生成开关。`int` 生成数字输入，字段约束仍由后端校验。
  - `Literal[...]` 或 `Enum` 生成选项，对应的列表类型可用于多选。
  - `str` 通常为文本框。`Annotated[str, encrypted()]` 为加密字段，界面按敏感字段提示处理。
  - `FilePath`、`FolderPath`、`ExecutablePath` 使用路径控件，选择操作由 Electron 提供系统对话框。
  - `date` 使用日期选择器。`Trigger` 对应操作按钮。

- **覆盖控件或增加下拉选项**

  ```python
  <字段名>: Annotated[
      <字段类型>,
      ui(widget=<控件名>, deps=<依赖字段列表>),
      select(endpoint=<宿主选项接口>),
  ] = <默认值>
  ```

  - `widget=`：覆盖类型推导结果。渲染器的控件能力见[声明式设置页](./frontend.md#声明式设置页)。
  - `endpoint=`：使用前端已接入的宿主选项接口。
  - `deps=`：选项依赖的字段路径。例如选设备前，先用 `["general.game"]` 取得游戏标识。支持的接口见[动态选项](./frontend.md#动态选项)。

## 显隐与只读

后端负责求值字段提示。需要动态判断时，用 `@ui_visibility` 绑定同步方法。静态情况可用 `ui(visibility=...)`，同一位置使用一种方式。

```python
@ui_visibility("<分组名>.<字段名>")
def <显隐方法>(self) -> UiVisibility:
    return <显隐值>
```

- **UiVisibility 取值**

  - `SHOW`：显示并允许编辑。
  - `HIDE`：隐藏字段，接口输出省略该字段。
  - `DISABLE`：显示字段，通用保存跳过对该字段的修改。
  - `MASKED`：显示占位符，原样提交时保留原值。该值适用于单个字段。

分组可以统一隐藏或禁用。受业务锁约束时，后端同时限制字段写入。动态提示随配置快照刷新，页面上的编辑草稿由前端自己维护。

## 触发器

触发器用于执行一次独立操作。把操作参数放在普通配置字段中，先保存，再触发。下面的例子使用宿主后台任务执行处理器。

```python
from auto_mas_core import ConfigGroup, PluginConfig, Trigger, trigger, trigger_field
from auto_mas_core.utils import get_logger
from pydantic import Field

logger = get_logger("demo")


class ActionFields(ConfigGroup):
    message: str = "Test message"
    send_test: Trigger = trigger(timeout=120)


class DemoConfig(PluginConfig):
    action: ActionFields = Field(default_factory=ActionFields)

    @trigger_field("action.send_test")
    def on_send_test(self) -> None:
        logger.info(self.action.message)
```

- **字段与处理器**

  - `Trigger`：空闲时读到 `None`，执行期间读到任务 UUID。使用 `trigger(timeout=...)` 可指定时限，默认 900 秒。
  - `@trigger_field("<分组名>.<字段名>")`：绑定无参实例方法。同步方法由工作线程执行，异步方法由事件循环执行。
  - `finish_on_cancel=True`：需要完整结束的处理器可使用此选项。已运行的同步工作也需实际结束后才能结清。

- **命令值**

  - `True`：启动或复用当前任务，返回任务 UUID。
  - `False`：请求取消。字段在实际结束前保持忙状态。
  - `None`：跳过请求。

Python 中使用 `<配置实例>.write_trigger(<分组名>, <字段名>, <命令值>)`。前端的触发与结果查询见[配置触发器接口](./frontend.md#配置触发器接口)。

## 界面文案与保存

配置字段的标签、说明和选项词条放在插件包的 `i18n/` 目录。命名见[词条](./frontend.md#词条)。

插件设置页通过 `/api/plugin_config/update` 提交配置。宿主以插件声明的类型重新校验，并使用配置框架提交。触发器的运行状态由宿主内存维护，普通配置保存与备份只处理相应的持久字段。
