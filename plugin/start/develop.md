# 插件开发

::: warning
插件系统还在快速变化。本页以主程序 `dev` 分支 2026-11-18 的实现为准。
:::

本页做一个最小的插件：启用和停用时各打一行日志。做完你会知道目录怎么放、入口怎么写、怎么启用、改了代码怎么生效。

开始前先按[开发起步](./start)跑起开发环境。下文的相对路径都以后端的工作目录为准：从源码运行时就是 AUTO-MAS 仓库根目录。

## 目录结构

在仓库根目录的 `plugins/` 下建一个以插件 ID 命名的目录。本例的插件 ID 是 `hello_demo`。

```text
AUTO-MAS/
└── plugins/
    └── hello_demo/
        ├── pyproject.toml
        ├── README.md
        └── src/
            └── hello_demo/
                ├── __init__.py
                └── plugin.py
```

- `plugins/<插件ID>/` 是工程根。目录名建议和插件 ID 一致。
- `src/` 下只能有一个顶层包，包名就是插件的导入名。有零个或多个，这个目录会被跳过。
- 包必须是带 `__init__.py` 的普通包。插件的代码必须全部放在这个包里，引用包外的模块会加载失败。
- 主程序的 Git 忽略 `plugins/` 下除 `auto_mas_core` 以外的目录。你的插件在自己的仓库里提交。

## pyproject.toml

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

几个字段不能错：

| 字段 | 要求 |
| --- | --- |
| `keywords` | 必须同时有 `auto-mas-plugin` 和且仅有一个 `auto-mas-id:<插件ID>`。缺了任何一个，这个工程不会被识别为插件。 |
| 插件 ID | `auto-mas-id:` 后面的部分。它会被用作配置文件名，不能含空白，也不能含斜杠、冒号等不能出现在 Windows 文件名里的字符。建议只用字母、数字和下划线。`auto_mas_core` 是核心包保留的 ID。 |
| `dependencies` | 必须包含 `auto-mas-core`。缺了它，后端启动时准备插件环境会报「插件未声明 SDK 依赖」，整个后端初始化失败。版本范围照官方样例写 `>=5,<6`；想更严格就收窄到你实际验证过的宿主版本。 |
| `requires-python` | 主程序用 Python 3.12，样例写 `>=3.12,<3.13`。 |
| `version` | 使用 PEP 440 版本号。发布到市场后，客户端按它比较新旧。 |
| `name` | 发行包名。想让插件市场的数据站自动发现，包名里要带 `auto-mas` 或 `automas`，否则只能请维护者补录，见[发布插件](./publish)。 |

`readme` 指向的文件必须存在，否则整个工程解析失败，不会被发现。

## README.md

```markdown
<!-- auto-mas:i18n
{
  "display_name_i18n": { "zh_CN": "你好示例", "en": "Hello Demo" },
  "description_i18n": { "zh_CN": "最小插件示例", "en": "Minimal plugin example" },
  "tags_i18n": { "zh_CN": ["示例"], "en": ["Example"] }
}
-->

# Hello Demo

AUTO-MAS 最小插件示例。
```

开头这段注释决定插件在插件中心和市场里显示的名称、简介和标签。本地开发可以先不写，缺失时用插件 ID 当名称、`description` 当简介、不显示标签。发布前必须补上，写法和缺失的后果见[发布插件](./publish)。

## 入口

`src/hello_demo/__init__.py`：

```python
from auto_mas_core import PLUGIN_API_VERSION

from .plugin import Plugin as PLUGIN

__all__ = ["PLUGIN", "PLUGIN_API_VERSION"]
```

`src/hello_demo/plugin.py`：

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

- 宿主只认包里导出的 `PLUGIN`，它必须是 `BasePlugin` 的子类（类本身，不是实例）。找不到或类型不对，插件进入「加载失败」。
- `PLUGIN_API_VERSION` 是插件协议版本，从 `auto_mas_core` 重新导出即可。导出的协议主版本和宿主不一致时，宿主拒绝加载。
- 只从 `auto_mas_core` 导入能力，不要直接 `import app.*`。`auto_mas_core` 里有什么，看 SDK 的 [README](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/plugins/auto_mas_core/README.md) 和 `__init__.py` 的导出清单。

### 生命周期钩子

实例在启用时才创建，停用后丢弃。不要在 `__init__` 里写业务，放进钩子。

| 钩子 | 什么时候调用 |
| --- | --- |
| `pre_enable` / `on_enable` | 启用前的预检 / 启用 |
| `pre_disable` / `on_disable` | 停用前的预检 / 停用，释放 `on_enable` 里创建的资源 |
| `pre_reload` / `on_reload` | 重载前的预检 / 重载时在旧实例上调用。重载已启用的插件还会依次走 `on_disable`、重新导入代码、`on_enable` |
| `pre_unload` / `on_unload` | 卸载前的预检 / 卸载。只有从市场安装的插件能卸载，本地源码插件不会走到这两个钩子 |
| `on_teardown` | 后端进程退出时的收尾。它不改变用户的启用选择 |
| `on_fail_cleanup` | 生命周期操作失败后清理资源 |

## 启用插件

1. 启动（或重启）后端和前端。启动时后端会扫描 `plugins/` 并登记新插件。
2. 打开「插件中心」，找到「你好示例」，来源显示为「本地」。打开这一行「已启用」列的开关。
3. 看后端日志 `debug/app.log`，应有类似下面的行：

```text
... | INFO     | HelloDemo | hello_demo 已启用
```

启用状态会保存，下次后端启动时自动启用。

::: tip
后端运行期间新放入的插件目录也会被发现并登记，但在重启后端之前它的状态是「已卸载」，「已启用」开关不可用。新建插件后重启一次后端。
:::

插件没出现时，问题在发现阶段：`keywords` 不对的工程直接跳过，不留日志；缺少 `pyproject.toml` 或 `src/`、`src/` 下顶层包数量不对、`readme` 文件不存在时，只在后端控制台输出 DEBUG 级日志，`debug/app.log` 只记 INFO 及以上，看不到。

状态是「加载失败」时，插件中心的「错误」列和 `debug/app.log` 里有导入报错。常见原因：`PLUGIN` 没导出，代码引用了包外的模块。「加载失败」的插件修好代码后重启后端。

## 改了源码后怎么生效

后端监视 `plugins/` 目录。你保存 `.py` 或 `.pyi` 文件，停止改动约 0.75 秒后，插件中心里这个插件展开详情中的「代码状态」会变成 `needs_reload`。**宿主不会自动换代码**，你要在这一行的「更多操作」菜单里手动点「重载」。

- 重载时，已启用的插件依次执行：旧实例的 `on_reload`，`on_disable`，重新导入源码，新实例的 `on_enable`。本地源码导入时不使用 `.pyc` 缓存，每次都读最新文件。
- 只改 `pyproject.toml` 里的版本号、包名等元数据，同样会标成 `needs_reload`。
- 重载失败时，插件进入「运行异常」。修好后再点「重载」。这时直接打开「已启用」开关，用的是上次成功导入的旧代码。
- 只有 `.py`、`.pyi`、`.toml`、`.json` 文件的变化会触发重新发现，只改 README 不会。
- 监视会忽略 `.git`、`__pycache__`、`build`、`dist`、`node_modules`、`pypi`、`site-packages` 以及 `.egg-info`、`.dist-info`、`.data` 目录。

## 添加依赖

把依赖写进 `pyproject.toml` 的 `dependencies`，然后**重启后端**。启动时后端把主程序依赖和所有插件的依赖放在一起解析，安装到 `plugins/pypi/site-packages`。

- 所有插件共用这一个环境，插件没有独立的虚拟环境。
- 主程序当前安装的依赖版本是固定的，插件不能改动。和主程序依赖无法同时满足时，后端初始化失败，日志报「插件自动升级失败：目标主程序与插件的依赖无法同时满足」，调整你的版本范围。
- 解析依赖需要联网。所有镜像都没能完成解析时，后端同样初始化失败，日志报「兼容性检查未完成，请检查网络或稍后重试」。
- 别删掉 `auto-mas-core` 这一行。

## 二次开发别人的插件

想改别人的插件，或给它贡献代码：

1. 到插件仓库页面 fork 一份。没有原仓库写权限时必须走 fork。
2. 把你的 fork 克隆到 `plugins/<插件ID>/`。

   ```bash
   cd plugins
   git clone https://github.com/<你的用户名>/AUTO-MAS-Whimbox-Adapter.git whimbox_adapter
   ```

   上面以奇想盒适配器为例，目录名换成你要改的插件的 ID。
3. 重启后端，在插件中心启用，改完代码按上一节重载。
4. 代码的提交、推送和发版都在插件自己的仓库里做。

要注意：

- 同一个插件同时存在市场安装的版本和 `plugins/` 下的源码时，用源码。要验证「从市场安装」的真实效果，先停后端，把源码目录移出 `plugins/`，再启动并从市场安装。
- 不要改 `auto-mas-id:` 的值。如果你想把改过的版本当成独立插件发布，必须同时换包名、插件 ID 和 `src/` 下的包名（导入名）。同一个插件 ID 对应不同包名，或两个插件用了相同的包名、导入名，插件发现都会直接报冲突，后端启动时会因此初始化失败。
- MAA 和奇想盒是真实的完整样例，带自己的配置、执行器和接口，见[独立专项适配](../adapter/standalone)。

插件写好后，到[发布插件](./publish)把它发布出去。插件的配置文件存在哪里，见[配置文件](./config)。
