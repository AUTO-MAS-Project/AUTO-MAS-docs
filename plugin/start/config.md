# 配置文件

::: warning
插件系统还在快速变化。本页以主程序 `dev` 分支 2026-11-18 的实现为准。
:::

插件系统的配置和主程序的其他配置分开存放，都在后端工作目录下。从源码运行时工作目录是 AUTO-MAS 仓库根目录；其他启动方式以后端日志开头「工作目录:」这一行为准。

## 存在哪里

| 内容 | 文件 | 说明 |
| --- | --- | --- |
| 插件注册表 | `config/plugin_registry.yaml` | 每个已发现的插件一项：包名、插件 ID、版本、来源、状态、是否启用、最近错误、代码状态等。由宿主在发现、启用、停用、重载时回写。 |
| 市场目录 | `config/plugin_catalog.yaml` | 插件市场里能看到的条目。刷新市场时整体更新。 |
| 数据站目录缓存 | `data/plugin-market-cache.json` | 上次从数据站读到的目录和 ETag，用于条件请求。 |
| 插件自有配置 | `config/plugins/<插件ID>.yaml` | 每个插件一个文件。只有声明了 `config_class` 的插件才有。 |

另外，从市场安装的插件包和它们的依赖在 `plugins/pypi/site-packages`，不属于配置，不要手动改。

::: tip
注册表和市场目录由宿主维护，字段随版本变化。要启用、停用插件，用插件中心，不要改 YAML。注册表里的 `enabled` 是用户的启用选择，后端启动时按它自动启用插件。
:::

## 插件自有配置

插件需要自己的设置时，继承 `PluginConfig` 写一个配置类，挂到插件类的 `config_class` 上：

```python
from auto_mas_core import BasePlugin, ConfigGroup, PluginConfig
from auto_mas_core.utils import get_logger
from pydantic import Field


class DemoConfig(PluginConfig):
    class Basic(ConfigGroup):
        greeting: str = Field(default="你好", description="问候语")
        repeat: int = Field(default=1, description="重复次数")

    basic: Basic = Field(default_factory=Basic, description="基础设置")


class Plugin(BasePlugin):
    config_class = DemoConfig

    async def on_enable(self) -> None:
        assert isinstance(self.config, DemoConfig)
        get_logger("HelloDemo").info(self.config.basic.greeting)
```

宿主在启用插件时：

1. 以 `config/plugins/<插件ID>.yaml` 为存储文件构造配置实例，文件已存在就读取其中的内容。
2. 构造插件实例时把配置传入，插件里用 `self.config` 访问。
3. 插件启用期间，插件中心这一行的齿轮按钮打开「插件设置」页，按配置声明渲染表单，保存后写回这个文件。插件没启用时，这个页面读不到配置。

你不需要自己处理文件读写，只负责声明。字段的声明方式（`ConfigGroup`、`ConfigEntry`、`ui(...)` 等）可以参考 MAA 适配器的 [`MaaPluginConfig`](https://github.com/AUTO-MAS-Project/AUTO-MAS-MAA-Adapter/blob/master/src/maa_adapter/config/plugin.py) 和 SDK 的 [README](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/plugins/auto_mas_core/README.md)。

::: warning
配置类里的集合字段不能设置 `name=`，否则启用时报错。
:::

## 停用和卸载对配置做什么

| 操作 | 注册表 | 自有配置文件 | 其他 |
| --- | --- | --- | --- |
| 停用 | 保留，`enabled` 改为 false | 先写回磁盘再释放，**文件保留**，再次启用时读回 | 专项适配类插件见下 |
| 重载 | 保留 | 保留 | 不清除任何已有数据 |
| 卸载 | **移除** | **删除** `config/plugins/<插件ID>.yaml` | 卸掉从市场安装的发行包 |

- **专项适配插件停用时会清空数据。** 脚本类适配（如 MAA、奇想盒）在停用时撤销它登记的托管类型，该类型下已有的托管、账号和关联计划一并被清空。游戏类适配会清空对应的游戏及设备配置。界面会在停用前弹窗列出影响范围。操作前先备份。只提供普通功能、不登记托管或游戏类型的插件，停用不影响别的配置。
- **卸载只对从市场安装的插件开放。** 本地源码插件和核心插件 `auto_mas_core` 的「卸载」菜单项是灰的。卸载前界面会弹窗确认，卸载不可撤销。如果卸载过程中删除发行包失败，登记项保留并标成「加载失败」，可以重试。
- **删除本地源码目录不会删配置。** 目录被删后，注册表里的这一项标记为 `package_removed`（包缺失），`config/plugins/<插件ID>.yaml` 保持原样。
