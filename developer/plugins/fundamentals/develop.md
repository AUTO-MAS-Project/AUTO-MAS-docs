# 开发与调试

本页从一个最小源码插件开始。它在启停时记录日志，可用于核对宿主是否发现、加载并运行了当前代码。先完成[开发环境配置](../../getting-start/environment.md)。

## 建立源码项目

源码启动时，宿主从主仓根目录的 `plugins/` 扫描开发插件。以插件 ID 为 `hello_demo` 的项目为例：

```text
plugins/
└─ hello_demo/
   ├─ pyproject.toml
   ├─ README.md
   └─ src/
      └─ hello_demo/
         ├─ __init__.py
         └─ plugin.py
```

- `plugins/<插件 ID>/`：项目根目录。插件源码由自己的仓库维护。
- `src/<导入包名>/`：唯一的普通顶层 Python 包，必须包含 `__init__.py`。插件模块随包保存。

## 声明包元数据

在项目根目录创建 `pyproject.toml`。以下字段声明插件的发行信息和依赖，其余构建设置见后面的完整示例。

```toml
[project]
name = "<发行包名>"
version = "<发行版本>"
readme = "<README 路径>"
requires-python = "<Python 版本范围>"
keywords = ["auto-mas-plugin", "auto-mas-id:<插件 ID>"]
dependencies = ["auto-mas-core<SDK 版本范围>"]
```

- **发行身份**
  - `name`：发行包名，用于安装和版本解析。
  - `version`：PEP 440 版本。
  - `keywords`：保留 `auto-mas-plugin`，并提供一个 `auto-mas-id:<插件 ID>`。插件 ID 使用稳定的合法文件名，`auto_mas_core` 由宿主保留。
- **运行与发布信息**
  - `readme`：指向存在的文件。发布前补齐[市场声明](./publish.md#readme-里的市场声明)。
  - `requires-python`：匹配目标宿主的 Python 版本。
  - `dependencies`：声明 `auto-mas-core` 和其他实际依赖，SDK 范围以实际验证过的宿主版本为准。

下面是 `hello_demo` 的完整示例，运行环境为 Python 3.12。

```toml
[build-system]
requires = ["setuptools>=77", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "auto-mas-hello-demo"
version = "0.1.0"
description = "AUTO-MAS 最小插件示例"
readme = "README.md"
requires-python = ">=3.12,<3.13"
keywords = ["auto-mas-plugin", "auto-mas-id:hello_demo"]
dependencies = ["auto-mas-core>=5,<6"]

[tool.setuptools.packages.find]
where = ["src"]
```

## 导出插件入口

1. **实现插件类**：在 `src/hello_demo/plugin.py` 继承 `BasePlugin`，将业务资源的创建和释放放到钩子中。

   ```python
   from auto_mas_core import BasePlugin
   from auto_mas_core.utils import get_logger

   logger = get_logger("HelloDemo")


   class Plugin(BasePlugin):
       async def on_enable(self) -> None:
           logger.info("hello_demo 已启用")

       async def on_disable(self) -> None:
           logger.info("hello_demo 已停用")
   ```

2. **导出类与协议版本**：在 `src/hello_demo/__init__.py` 将类导出为 `PLUGIN`。

   ```python
   from auto_mas_core import PLUGIN_API_VERSION

   from .plugin import Plugin as PLUGIN

   __all__ = ["PLUGIN", "PLUGIN_API_VERSION"]
   ```

3. **使用 SDK 加载宿主能力**：插件从 `auto_mas_core` 导入契约。钩子的调用时机见[插件基础](./index.md#生命周期钩子)。

## 启用并验证源码插件

1. **启动宿主**：启动或重启后端，从 `plugins/` 登记源码项目。
2. **启用插件**：在插件中心找到插件，确认来源为本地源码，再打开启用开关。
3. **核对日志**：在 `debug/app.log` 查看 `HelloDemo` 的启用日志，停用时检查对应的收尾日志。

宿主运行时新发现的插件可能仍处于未加载状态。新建项目后重启后端，然后再启用。

- **列表没有项目**

  - 核对元数据关键词、`src/` 顶层包和 README 路径。
  - 查看后端控制台的 DEBUG 日志。应用日志文件可能只保留 INFO 及以上。

- **状态为加载失败**

  - 查看插件中心的错误和后端日志。核对包级 `PLUGIN` 导出、SDK 协议版本和模块导入路径。
  - 修正首次加载的问题后重启后端。

## 重载代码和更新依赖

1. **修改代码**：保存 Python 文件或项目元数据后，宿主将可重载的源码标记为 `needs_reload`。
2. **应用改动**：在插件中心选择重载。启用中的插件先按生命周期释放旧实例，再导入源码并启用新实例。重载失败后修正代码，再重试重载。
3. **更新依赖**：修改 `pyproject.toml` 的 `dependencies`，然后重启后端。宿主按锁定依赖与全部插件共同求解共享环境。遇到求解失败时，核对插件声明和目标宿主版本，见[安装与更新机制](./runtime.md#市场、安装和更新)。

## 开发已有插件

将插件自己的仓库克隆到 `plugins/<插件 ID>/`，沿用原项目的插件 ID、发行名和导入名。在插件仓库提交和发布代码。需要发布为独立插件时，为新项目分别指定新的身份。

同 ID 的源码优先于已安装发行包。验证 wheel 前先停后端，把源码移出扫描范围，再按[发布插件](./publish.md)验证安装。
