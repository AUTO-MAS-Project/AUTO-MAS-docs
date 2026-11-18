# 独立专项适配

::: warning
插件系统还在快速变化。本页以主程序 `dev` 分支 2026-11-18 的实现为准。
:::

专项适配插件把一款脚本软件接进 MAS：登记一种托管类型，提供它的配置、执行器和专用接口。MAA 和奇想盒已经从主程序迁成这种插件，本页用它们做例子。

## 先判断该不该做独立适配

| 你要接入的东西 | 做法 |
| --- | --- |
| 有自己的运行方式、配置格式和更新渠道的脚本软件（MAA、奇想盒这类） | 独立专项适配，看本页 |
| 一个 MaaFramework `interface.json` 项目，只是想在 MFW 通用能力上加几处特调 | [风味专项开发](../../developer/maafw-flavor) |

## 两个样例

| | MAA | 奇想盒 |
| --- | --- | --- |
| 插件 ID / 导入名 | `maa_adapter` | `whimbox_adapter` |
| PyPI 包名 | `auto-mas-maa-adapter` | `auto-mas-whimbox-adapter` |
| 仓库 | [AUTO-MAS-MAA-Adapter](https://github.com/AUTO-MAS-Project/AUTO-MAS-MAA-Adapter) | [AUTO-MAS-Whimbox-Adapter](https://github.com/AUTO-MAS-Project/AUTO-MAS-Whimbox-Adapter) |
| 托管类型键 | `MAA` | `Whimbox` |
| 计划表类型 | 有（`MaaPlanConfig`） | 没有 |
| 支持的运行模式 | 自动代理、配置、查看 | 自动代理、配置 |
| 插件自有配置 | 有（关卡缓存） | 没有 |
| 对外提供服务 | 关卡信息三项 | 没有 |

## 代码放哪

- **后端 adapter 放独立仓库。** 主程序仓库只跟踪 SDK `plugins/auto_mas_core`，`plugins/` 下其余目录被 Git 忽略。开发时把插件仓库克隆到主程序的 `plugins/<插件ID>/`，发布后用户从插件中心安装。
- **专项页面目前仍随宿主构建。** 主程序还不能加载插件自带的前端。托管编辑页、账号编辑页、计划表放在主仓 `frontend/src/views/<专项>/`，随桌面应用一起发版。更新插件的 wheel 不会替换已经打包的页面。

所以一次专项改动常常是两个仓库各一个提交：后端进插件仓库，页面进主仓。

## 插件仓库的骨架

以奇想盒为例。`pyproject.toml`：

```toml
[build-system]
requires = ["setuptools>=77", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "auto-mas-whimbox-adapter"
version = "0.1.1"
readme = "README.md"
requires-python = ">=3.12,<3.13"
keywords = ["auto-mas-plugin", "auto-mas-id:whimbox_adapter"]
dependencies = ["auto-mas-core>=5,<6"]

[tool.setuptools.packages.find]
where = ["src"]
```

包入口 `src/whimbox_adapter/__init__.py` 导出 `PLUGIN`：

```python
from auto_mas_core import PLUGIN_API_VERSION

from .plugin import Plugin as PLUGIN

__all__ = ["PLUGIN", "PLUGIN_API_VERSION"]
```

`src/whimbox_adapter/plugin.py` 是全部声明：

```python
from auto_mas_core import Config, ScriptAdapterPlugin, ScriptTypeDecl, TaskMode
from fastapi import APIRouter

from .api import router as task_router
from .backup_api import router as backup_router
from .config import WhimboxConfig, WhimboxUserConfig
from .manager import WhimboxManager, check_prefetch

router = APIRouter()
router.include_router(backup_router)
router.include_router(task_router, prefix="/api/scripts")


class Plugin(ScriptAdapterPlugin):
    script_type_key = "Whimbox"
    api_router = router
    decl = ScriptTypeDecl(
        script_type=WhimboxConfig,
        user_type=WhimboxUserConfig,
        expander_class=WhimboxManager,
        supported_modes=(TaskMode.AUTO_PROXY, TaskMode.SCRIPT_CONFIG),
        prefetch_check=check_prefetch,
        activity_game="infinitynikki",
    )

    async def pre_enable(self):
        refs = {"PlanConfig": Config.PlanConfig}
        WhimboxConfig.related_config = refs
        WhimboxUserConfig.related_config = refs
```

### `ScriptTypeDecl` 各项

| 字段 | 含义 |
| --- | --- |
| `script_type` | 托管配置类，继承 `ScriptEntry`。必须有 `info` 和 `users` |
| `user_type` | 账号配置类，继承 `UserEntry`，必须有 `info`。`script_type.users` 必须用 `collection(user_type)` 收窄成只含这一种 |
| `expander_class` | 执行器，继承 `UserExpander` |
| `plan_type` | 计划表配置类，不用计划表就不写。写了的话 `user_type.info` 必须有 `plan_id` |
| `supported_modes` | 支持的运行模式，不能为空。不写时默认自动代理、查看、配置三种都支持 |
| `expect_builder` | 从托管配置解析模拟器期望（`EmulatorExpect`）。宿主目前只校验它可调用，还没有读取它的地方 |
| `prefetch_check` | 运行前预下载声明：异步函数，传入托管配置，返回 `DownloadRequest` 列表，空列表表示不用下载 |
| `activity_game` | 首页活动展示和活动提醒用的游戏标识 |

一个插件只声明一种托管类型。

### 配置类

配置类只从 `auto_mas_core` 取宿主能力，不直接 `import app`：

```python
from auto_mas_core import ScriptEntry, UserEntry
from auto_mas_core.host.config import ConfigGroup, collection, ui
```

托管和账号引用宿主的其他配置集合（计划表、模拟器）时，在 `pre_enable` 里绑定 `related_config`，像上面的例子那样。`pre_enable` 在领域配置激活之前执行，旧配置里的引用才能原样恢复。

### 专用接口

把 FastAPI 路由交给 `api_router`。路由按自己声明的路径挂进宿主，所以专项原有的 `/api/scripts/...` 路径可以原样保留；新接口放在 `/api/plugin/<插件ID>/...` 下。插件停用后这些路径随之卸载。

## 前端页面

在主仓加 `frontend/src/views/<专项>/entry.ts`。以奇想盒的 `frontend/src/views/whimbox/entry.ts` 为例：

```ts
import type { ScriptFrontendAdapter } from '@/plugins/scriptAdapters'
import whimboxIcon from './whimbox.png'

export default {
  type: 'Whimbox',
  metadata: {
    logo: whimboxIcon,
    label: 'Whimbox',
    typeTagColor: 'pink',
    routeSuffix: 'whimbox',
    routeTitles: {
      script: '编辑奇想盒脚本',
      userAdd: '添加奇想盒用户',
      userEdit: '编辑奇想盒用户',
    },
    createOrder: 12,
    create: {
      titleKey: 'core.scripts.type.Whimbox',
      descriptionKey: 'core.scripts.create.typeDesc.Whimbox',
      keywords: ['whimbox', '奇想盒', '无限暖暖', 'nikki'],
      group: 'specialized',
    },
  },
  editors: {
    script: () => import('./script/WhimboxScriptEdit.vue'),
    user: () => import('./WhimboxUserEdit.vue'),
  },
  routes: [],
} satisfies ScriptFrontendAdapter
```

再改两处登记：

1. `frontend/src/plugins/scriptAdapters.ts`：导入入口（`import whimbox from '@/views/whimbox/entry'`），加进 `adapters` 数组。
2. `frontend/src/utils/scriptMetadata.ts`：在 `SCRIPT_METADATA` 里加一行 `Whimbox: SCRIPT_PLUGIN_METADATA.Whimbox`。新建卡片和编辑页路由都从 `SCRIPT_METADATA` 读元数据，漏了这行就不会出现。

`type` 要和后端的 `script_type_key` 一致。前端的 `ScriptType` 来自生成的 `ScriptCreateIn` 枚举，所以要在插件启用时重新生成 OpenAPI，类型检查才认得这个键。新建托管的入口按后端当前启用的类型开放，插件没启用时卡片不出现。用计划表的专项再加一个 `plan` 段，写法见 `frontend/src/views/maa/entry.ts`。更多细节见[前端与后端的对接](./frontend)。

## 把旧的内置专项迁成插件

按这个顺序做。

### 1. 定版本边界

- 定下**最低宿主版本**：哪个主程序版本开始带插件系统并移除了这个内置专项。
- 定下 **SDK 下界**：`dependencies` 里 `auto-mas-core` 的范围。首发前按实际配套的 SDK 发行版本收紧下界，并用那个 SDK 发行包验证。版本号相同的开发源码可能含有还没发布的接口，不能据此认为旧 SDK 也兼容。

### 2. 保住旧数据的身份

- 配置类的类名不要改。配置文件里的 `type:` 存的就是类名（`WhimboxConfig`、`MaaUserConfig` 这类），类名不变，旧托管就能原样读回。
- UID、顺序、字段名保持不变。结构必须调整时在加载时转换，不重建：奇想盒在激活时把旧的 `UserData` 转成 `users`，账号还是原来那些。

### 3. 让宿主知道旧类型归哪个插件

在主程序 `app/core/config/plugin_upgrade.py` 的 `_ADAPTER_PLUGINS` 里加映射。键是索引里记录的配置类名，托管、计划表的类名（包括改过名的旧类名）都要列；值是插件 ID 和 PyPI 包名。当前内容：

```python
_ADAPTER_PLUGINS = {
    "WhimboxConfig": ("whimbox_adapter", "auto-mas-whimbox-adapter"),
    "MaaConfig": ("maa_adapter", "auto-mas-maa-adapter"),
    "MaaPluginConfig": ("maa_adapter", "auto-mas-maa-adapter"),
    "MaaPlanConfig": ("maa_adapter", "auto-mas-maa-adapter"),
    "MaaPluginPlanConfig": ("maa_adapter", "auto-mas-maa-adapter"),
}
```

v1.8 之前的配置按目录保存，没有集合索引。同一个文件的 `required_adapter_plugins` 里还要加一条目录判断，奇想盒加的是：

```python
    if any(config_path.glob("WhimboxConfig/*/config.json")):
        required["whimbox_adapter"] = "auto-mas-whimbox-adapter"
```

宿主每次启动加载配置时都会扫描 `ScriptConfig.json`、`PlanConfig.json`、`scripts.yaml`、`plans.yaml` 的索引（不只是版本升级那一次），发现这些类型就在领域配置激活**之前**安装、加载并启用对应插件。插件已经装着、或者有同 ID 的本地源码时不重新安装，只补加载和启用。装不上或启用失败就抛错，配置加载中止，托管不会在缺插件的情况下激活。

### 4. 清掉宿主里的内置实现

奇想盒迁出（提交 `e7970529a`，#2039）时主程序改了这些地方，可以当清单用：

| 位置 | 动作 |
| --- | --- |
| `app/task/<专项>/` | 整个删除 |
| `app/models/config/script.py`、`app/models/config/__init__.py` | 删配置类和登记 |
| `app/api/scripts/__init__.py`、`management.py`、`users.py` | 删路由挂载、类型枚举和联合类型 |
| `app/core/config/manager.py` | 删导入和返回类型 |
| `app/task/__init__.py`、`base.py`、`builtin_registration.py` | 删内置登记 |
| `app/core/task_manager.py` | 用宿主的类型表，不再写死类型 |
| `app/core/desktop_session/lifecycle.py`、`app/core/game_calendar.py`、`app/core/mcp.py` | 删按专项写死的分支和条目 |
| `app/core/notify/formats/<专项>.py` | 搬进插件 |
| `plugins/auto_mas_core/src/auto_mas_core/host/` | 插件要用、SDK 还没导出的宿主模块，在这里加再导出文件 |
| `frontend/src/views/<专项>/` | 页面集中到这里，加 `entry.ts`（见上文「前端页面」） |
| `frontend/src/plugins/scriptAdapters.ts`、`frontend/src/utils/scriptMetadata.ts` | 登记入口；`frontend/src/router/scriptEditorRoutes.ts` 里删掉原来写死的编辑页 |
| `frontend/src/api/` | 插件启用后重新生成 |
| `tests/requirements-plugins.txt` | 加上插件 wheel 的版本和 SHA256 |

### 5. 按顺序发布

**先发布插件 wheel，再交付移除内置实现的主程序版本。** 顺序反了，用户升级后装不到插件，升级会中止。

### 6. 用 PyPI 上的 wheel 验证旧托管升级

必须在**没有本地 adapter 源码**的环境里做。本地源码会覆盖同 ID 的发行包，留着它就验不到真实的安装路径。

1. 停掉后端，把 `plugins/<插件ID>/` 源码工程移出插件扫描目录。
2. 准备一份脱敏的旧版配置（升级前的 `config` 目录）。
3. 启动候选版本的主程序，确认首次启动自动从 PyPI 装上了插件。
4. 重启一次，核对托管、账号的 UID、顺序和各项设置都还在。

## 测试

- 插件自己的测试放在插件仓库，用匹配的主程序源码跑：设置 `AUTO_MAS_HOST_SOURCE` 指向主仓，再执行 `python -m pytest tests -q`。
- 主仓的集成回归用已发布的 wheel：版本和 SHA256 锁在 `tests/requirements-plugins.txt`，`scripts/prepare_test_plugins.py` 把它们装进被忽略的 `.test-plugins`。升级锁定的版本时两项一起改。

## 要提前告诉用户的事

- **禁用专项适配插件会删除这个专项的全部托管和账号**，声明了 `plan_type` 的还会删除这种计划表，重新启用不会恢复。
- 卸载还会删掉插件自己的设置文件 `config/plugins/<插件ID>.yaml`。

在插件 README 里写明，并提醒先备份。
