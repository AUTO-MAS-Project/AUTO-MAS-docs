# 风味专项开发

MFW 是运行 MaaFramework 项目的通用引擎。项目已有 `interface.json`，只需要在 AUTO-MAS 中运行和配置任务时，直接使用 [MFW 项目指南](/docs/script-guide/maafw) 即可。只有项目确实需要自己的托管类型、文案或少量运行前规则时，才需要做“风味专项”（代码中称为 flavor）。

本页是操作指南：照着步骤做一个风味。风味为什么这样设计、钩子与契约的全集、M9A 与 MSS 的逐项说明，见设计页 [风味](./maafw/flavor.md)；MFW 整体设计见 [MFW 设计总览](./maafw/index.md)。

M9A 和 MSS 是现有例子：它们复用 MFW 的项目导入、运行环境、更新、任务执行、通知与周期任务；各自只声明项目识别规则和项目独有行为。开始前先读主仓的 [`app/task/MaaFW/AGENTS.md`](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/app/task/MaaFW/AGENTS.md) 与 [`MaaFWFlavor` 契约](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/app/task/MaaFW/tools/embedded/flavor.py)（模块说明里写着全部可选钩子的契约）。

## 先判断要不要做风味

| 需求 | 放在哪里 |
| --- | --- |
| 项目已有的任务、控制器、资源和选项 | 项目的 `interface.json`；MFW 直接读取 |
| 所有 MaaFramework 项目都需要的能力 | MFW 通用引擎 |
| 某个项目独有的身份、提示、运行前任务编排、运行前登录、少量独有表单 | 风味配置与钩子 |
| 原生编辑器会话、按游戏语义组织的专属界面 | 专项，不是风味 |
| 游戏内任务怎样执行、任务结果怎样判定 | 项目自身；AUTO-MAS 不重写项目逻辑 |

要接入的不是 MaaFramework 项目，而是有自己运行方式的脚本软件时，去看 [独立专项适配](../plugin/adapter/standalone)。

风味是主程序中的一种托管类型，不是放进项目发行包的插件。一个普通 MFW 托管导入被某个风味认领的项目后，会原地切换成该类型；换成其他项目时也会重新识别。托管 ID 保持不变，但项目相关的任务队列需要重新检查。

`app/task/MaaFW/` 里不能出现具体风味的名字；风味的一切逻辑只经 `FLAVOR` 钩子进入。

## 第一步：后端配置类与登记

下面以新风味 `X` 为例。M9A、MSS 在这些位置都出现过，漏一处的后果在各条里写明。

1. 在 `app/models/config/script.py` 里，紧挨着 `MSSConfig` 建立 `XUserConfig(MaaFWUserConfig)` 与 `XConfig(MaaFWConfig)`。类属性 `DEFAULT_SCRIPT_NAME`、`USER_CONFIG_CLASS`、`FLAVOR` 必写。`FLAVOR` 保持字符串，避免配置模型在导入时直接加载任务模块。账号字段没有差别时，`XUserConfig` 可以只有一行文档字符串。

   ```python
   class XUserConfig(MaaFWUserConfig):
       """X 账号配置。"""


   class XConfig(MaaFWConfig):
       """X 托管配置：MaaFW 的特调类型。"""

       DEFAULT_SCRIPT_NAME: ClassVar = "新 X 托管"
       USER_CONFIG_CLASS: ClassVar = XUserConfig
       FLAVOR: ClassVar = "app.task.X.flavor:FLAVOR"

       class InfoFields(MaaFWConfig.InfoFields):
           Name: str = Field(
               default_factory=lambda: "新 X 托管", description="MaaFW 托管名称"
           )

       # 独立配置类型有意覆盖分组及集合，保留专项默认值与准确字段类型。
       Info: InfoFields = Field(  # pyright: ignore[reportIncompatibleVariableOverride]
           default_factory=InfoFields
       )
       UserData: ConfigCollection[XUserConfig] = collection(  # pyright: ignore[reportIncompatibleVariableOverride]
           XUserConfig
       )
   ```

   `script.py` 里已经导入了 `ClassVar`、`Field`、`ConfigCollection`、`collection`。`Info.Name` 的默认值要与 `DEFAULT_SCRIPT_NAME`、前端描述对象的 `defaultScriptName` 写成同一个名字。

   要改通用字段的默认值时，照 `M9AConfig` 的写法，在子类里重写那个分组，其余字段继承：

   - 内置单任务时限：同时写类属性 `DEFAULT_TASK_TIME_LIMIT_OVERRIDES`（JSON 字符串，任务名 → 分钟）和 `Run.TaskTimeLimitOverrides` 的 `default_factory`，两处用同一个常量。通用 MFW 托管因导入项目换成该风味时，托管的 `Run.TaskTimeLimitOverrides` 还是空的才会补上类属性的值。只放比全局默认（`Run.TaskTimeLimit`，45 分钟）更长的任务。

     ```python
     _X_TASK_TIME_LIMIT_OVERRIDES = '{"某个耗时任务": 60}'


     class XConfig(MaaFWConfig):
         # 下面这些合进上面的 XConfig，其余成员不变
         DEFAULT_TASK_TIME_LIMIT_OVERRIDES: ClassVar = _X_TASK_TIME_LIMIT_OVERRIDES

         class RunFields(MaaFWConfig.RunFields):
             TaskTimeLimitOverrides: JsonDictString = Field(
                 default_factory=lambda: _X_TASK_TIME_LIMIT_OVERRIDES,
                 description="按任务名覆盖的单任务时限（分钟），值 0 表示该任务不限。",
             )

         Run: RunFields = Field(  # pyright: ignore[reportIncompatibleVariableOverride]
             default_factory=RunFields
         )
     ```

   - 游戏启动等待时间：`Game.WaitTime` 通用默认 120 秒，M9A 改成 15。写法是继承 `MaaFWConfig.GameFields` 只重写 `WaitTime`，再用 `Game: GameFields = Field(default_factory=GameFields)` 覆盖分组。
   - 风味独有的配置项（M9A 的 `Run.LoginMode`、MSS 账号上的 `Info.PlanMode`）同样加在继承的分组里。

   同文件末尾还有两处要补：`__all__` 里加 `"XUserConfig"`、`"XConfig"`；类型联合 `ScriptConfigType`、`UserConfigType` 里各加一项。

2. 在 `app/models/config/__init__.py` 的 `CLASS_BOOK` 里登记 `"X": XConfig`。键就是创建时的类型名，也必须等于钩子对象的 `type_key`。引擎按 `CLASS_BOOK` 的登记顺序选择第一个认领项目的风味，所以新类型放在谁前面谁后面是有意义的。
3. 在 `app/utils/common/constants.py` 的 `TYPE_BOOK` 里加 `"XConfig": "X"`（M9A 还登记了 `M9AUserConfig`）。托管下拉框按它取类型名（`app/core/config/manager.py` 的 `get_script_combox`），漏了会在取下拉框时抛 `KeyError`。
4. 在 `app/task/MaaFW/registration.py` 里：从 `app.models.config` 导入 `XConfig`，并把它加进 `ScriptRegistration` 的 `run_types`。宿主按类型精确匹配运行索引，子类不会自动继承父类的登记；漏了，宿主构建专项登记时会抛 `ValueError`（“专项能力漏登: XConfig”）。`app/core/config/manager.py` 里 `isinstance(…, MaaFWConfig)` 的分支已经覆盖子类，不要再加风味分支。
5. 补齐接口的请求和响应类型。要改的是下面这几处，`app/models/schema.py` 不用动：
   - `app/api/scripts/management.py`：
     - 配置载荷与输入别名：`XConfig = ConfigPayload[_config_models.XConfig]`、`XConfigInput = ConfigInput[_config_models.XConfig]`；
     - `ScriptIndexItem.type` 的枚举里加类名 `"XConfig"`；
     - `ScriptCreateIn.type` 的枚举里加 `"X"`，`description` 里的类型清单也补上；
     - 联合类型：`ScriptCreateOut.data`、`ScriptGetOut.data` 加 `XConfig`，`ScriptUpdateIn.data` 加 `XConfigInput`。
   - `app/api/scripts/users.py`（账号侧）：
     - `XUserConfig`、`XUserConfigInput` 别名；
     - `UserIndexItem.type` 的枚举里加 `"XUserConfig"`；
     - 联合类型：`UserGetOut.data`、`UserCreateOut.data` 加 `XUserConfig`，`UserUpdateIn.data` 加 `XUserConfigInput`。
6. 可选：`app/core/config/manager.py` 的 `add_script`、`add_user` 返回类型标注里加上新类（只是标注）；风味对应的游戏有游戏日历提醒时，在 `app/core/game_calendar.py` 的 `SCRIPT_CALENDAR_GAMES` 里加 `"X": "<日历里的游戏键>"`。
7. 改完后端，起开发后端重新生成前端 API（见下面“收尾”）；不要手改 `frontend/src/api/`。

## 第二步：后端钩子对象

在 `app/task/X/flavor.py` 中导出 `FLAVOR` 对象。它至少要满足 `MaaFWFlavor` 协议：一个 `type_key`、一个 `matches_project(interface_model)`、一个 `decorate_selection(...)`。最小示例：

```python
from __future__ import annotations

from typing import Any

from app.task.MaaFW.tools.core.interface.models import MaaFWInterface
from app.task.MaaFW.tools.embedded.flavor_support import MaaFWSelectionContext


class XFlavor:
    type_key = "X"

    def matches_project(self, interface_model: MaaFWInterface) -> bool:
        return str(interface_model.mirrorchyan_rid or "").strip().casefold() == "x"

    def decorate_selection(
        self,
        interface_model: MaaFWInterface,
        task_ids: list[str],
        task_options: dict[str, Any],
        *,
        context: MaaFWSelectionContext,
    ) -> tuple[list[str], dict[str, Any]]:
        return list(task_ids), dict(task_options)


FLAVOR = XFlavor()
```

- `matches_project` 只根据项目的 `interface.json` 识别身份。优先选稳定的项目标识，例如 `mirrorchyan_rid` 或仓库地址；名称适合作为兼容性判据。判据不能把其他项目误认进来。抛异常只会告警，不会挡住导入。
- `decorate_selection` 在运行计划生成前接收**已经选中的任务实例 ID**和选项，返回新的 `(task_ids, task_options)`。可以插入、删去、排序项目独有任务，或按账号配置调整选项；不要在这里重做更新、重试、超时、游戏启停等通用流程。
  - 每个账号建运行计划时调用一次，重试时不再调。传进来的列表已经剔掉了不可选任务和今天不在运行日的实例；钩子自己补回的同一实例照常运行。
  - 签名的最后一个参数是只读的 `context`（`MaaFWSelectionContext`，在 `flavor_support.py`），除它之外没有别的关键字参数：

    | 字段 | 内容 |
    | --- | --- |
    | `context.script_config` | 脚本（托管）配置 |
    | `context.user_config` | 本次运行的账号配置，账号字段如 `Info.Account` 从这里读 |
    | `context.resource_name` | 运行时实际生效的资源。脚本资源留空时引擎已按控制器回落，不要自己再推一遍 |
    | `context.excluded_task_ids` | 引擎判定本轮不跑的实例 ID（`frozenset`）：用户设成红灯的、今天不在运行日的 |
    | `context.log` | 写一行账号运行日志；钩子在工作线程里跑，这个回调可以直接调 |

  - 钩子另读账号配置自动补任务时，**不要把 `excluded_task_ids` 里的实例补回来**，也不要自己再解析快照去判红灯或星期。核心建计划时还会按红灯再滤一遍装饰结果，补回的红灯实例照样不跑。
  - 任务实例 ID 不一定等于任务名（同一任务重复入队时是 `<任务名>__MAS_DUP__<后缀>`），用 `resolve_task_instance_name`（`app.task.MaaFW.tools.core.interface.models`）解析；按 interface 任务的 `entry` 找任务，不按任务名。`flavor_support.py` 的 `task_name_for_entry(interface_model, entry)` 返回第一个 `entry` 相同的任务名，找不到返回 `None`。
  - 返回新列表和新字典，不要改写传入对象；项目缺少预期任务或选项时用 `context.log` 写一行可读日志后跳过。
  - 它在工作线程里运行，没有事件循环。
  - 抛异常等于这个账号本次建运行计划失败：该账号记“异常”、发错误通知、不运行、不重试，其他账号照常；异常原文接在“无法构建 MaaFW 运行计划”后面给用户看。只在确实不能运行时这样做，文案要能直接给用户看。

补一个项目独有任务的写法：

```python
from __future__ import annotations

from typing import Any

from app.task.MaaFW.tools.core.interface.models import MaaFWInterface
from app.task.MaaFW.tools.embedded.flavor_support import (
    MaaFWSelectionContext,
    task_name_for_entry,
)

DAILY_ENTRY = "X_Daily"


class XFlavor:
    type_key = "X"

    def matches_project(self, interface_model: MaaFWInterface) -> bool:
        return str(interface_model.mirrorchyan_rid or "").strip().casefold() == "x"

    def decorate_selection(
        self,
        interface_model: MaaFWInterface,
        task_ids: list[str],
        task_options: dict[str, Any],
        *,
        context: MaaFWSelectionContext,
    ) -> tuple[list[str], dict[str, Any]]:
        ids = list(task_ids)
        options = dict(task_options)
        daily = task_name_for_entry(interface_model, DAILY_ENTRY)
        if daily is None:
            context.log(f"[X] interface 里没有 entry 为 {DAILY_ENTRY} 的任务，未自动加入")
        elif daily not in ids and daily not in context.excluded_task_ids:
            ids.append(daily)
        return ids, options


FLAVOR = XFlavor()
```

### 可选钩子与声明

只在需要时实现。它们都按 `getattr` 探测，**不要加进 `MaaFWFlavor` 协议**——协议是 `runtime_checkable` 的，加进去后没实现它们的风味会整个退回通用 MFW。

| 名字 | 用途 |
| --- | --- |
| `sanitize_task_snapshot(...)` | 保存账号任务快照前整理风味接管的任务，可一并写入账号 `Info` 字段。仅在能读到项目、写入包含 `Task.TaskSnapshot` 时调用；运行前仍应按同一规则处理旧快照。M9A 的受管任务是参考实现 |
| `async ensure_game_updated(...)` | 需要检查游戏客户端版本时才实现。仅在 ADB 控制、托管启用游戏更新时，在模拟器启动后、首个任务前调用；返回 `app.utils.common.game_update.GameUpdateResult`（字段 `status`、`message`、`resource_version`；`status` 取 `Skipped` / `UpToDate` / `Updated` / `NeedManualUpdate`）。项目发行包的更新仍由 MFW 通用流程管理，这两个“更新”不是同一件事 |
| `async ensure_logged_in(...)` | 运行前登录，见下面“运行前登录” |
| `abort_round_entries = {entry: 说明}` | 关键任务：它失败或超时就结束本轮（照常重试） |
| `nonfatal_entries = {entry: 说明}` | 收尾任务：它失败、超时或被强停都不算本轮失败、不重试 |
| `skip_on_retry_entries = frozenset({entry, ...})` | 重试时本次运行里已经完成过的这些任务不再补跑。只列重跑纯属浪费、且不依赖本轮前面步骤的任务；逐个写出，新任务默认照旧补跑 |
| `unselectable_entries = {entry: 原因}` | 不可选任务：账号页不让选，运行时剔掉并记一行。用于 MAS 无人值守跑不了的任务 |
| `no_default_fallback_inputs = frozenset({(选项名, 字段名), ...})` | 字符串输入没填时，引擎默认回落 interface 里的 `default`；列在这里的字段不回落，没填就下发空串。用于 `default` 是项目留下的字面量占位、真下发会出错的字段（M9A 自定义兑换码的“占位”）。`/maafw/preview` 会把这些字段的 `default` 置空，账号页不会把它显示成没填时的值。能跑的 `default` 别列 |
| `retype_notice_lines(*, entering, has_users) -> list[str]` | 项目导入后脚本换成 / 换出本风味时，接在系统通知“已按项目识别为 …”下面的说明行。换入以 `entering=True`、换出以 `entering=False` 各调一次；`has_users` 是脚本此刻有没有账号，只对已有账号说的话没有账号时不要返回 |

签名、调用时机与优先级见 [`flavor.py` 的模块说明](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/app/task/MaaFW/tools/embedded/flavor.py)，设计说明见 [风味](./maafw/flavor.md)。

### 运行前登录：`ensure_logged_in`

游戏需要在任务开始前由 MAS 完成登录时才实现。它在 ADB 控制器、MAS 启动模拟器（连带拉起游戏）之后、第一个任务下发之前调用，紧跟在 `ensure_game_updated` 之后，在事件循环里 `await`。要不要真正做事由钩子自己按配置判断，不做事就返回空串。

```python
from collections.abc import Awaitable, Callable
from typing import Any

from app.task.MaaFW.tools.embedded.flavor import MaaFWLoginRequired, StartupTask


class XFlavor:
    # 其余成员同上

    async def ensure_logged_in(
        self,
        *,
        script_config: Any,
        user_config: Any,
        resource_name: str | None,
        package_name: str | None,
        adb_path: str | None,
        adb_address: str,
        progress: Callable[[str], Awaitable[None]] | None,
        host_adb: Any = None,
    ) -> str | StartupTask:
        if not adb_address:
            raise MaaFWLoginRequired("这种登录方式只支持 ADB 控制器")
        # ……截图、识别、点击，完成登录
        return "X 登录完成"
```

- 参数：`user_config.Info.Account` 是账号字段；`package_name` 是这次随模拟器拉起的包名（空串表示没拉起，钩子可自行按包名拉起）；`host_adb` 非空时，截图和命令必须走这条私有通道，否则按 `adb_path` 连接；`progress` 是写用户运行日志的异步回调。Win32 控制器会以空的 `adb_address` 调用，钩子据此拒绝不支持的登录模式。
- 返回值：字符串会写进运行日志，空串不写。登录**成功**后才记完成；失败时下一次尝试会重新调用，重试关闭游戏 / 模拟器后也会重新验证。
- 失败：抛 `MaaFWLoginRequired`，文案直接给用户看。本账号本次尝试判失败，按普通失败走重试；抛其它异常同样按失败处理，引擎把它包成 `MaaFWLoginRequired`，文案是“运行前登录出错：<异常原文>”。
- 取消：用户停止时的 `CancelledError` 必须照常向上抛，不能吞掉。
- 选号后还要由项目启动游戏时，返回 `StartupTask(deadline, entry)`，而不是字符串：`deadline` 是 `time.monotonic()` 的绝对时刻（剩余登录时限的终点），`entry` 是首项公开启动任务的 entry（M9A 返回 `StartupTask(deadline, "StartUp")`）。要求：
  - 运行计划的第一个任务的 entry 必须等于它，否则本次判失败（所以 `decorate_selection` 要把这个任务放在队首）；
  - 完整计划仍由一个 worker 执行，首项任务的单任务时限会收紧到剩余登录时限以内；首项失败即结束本轮，不做启动后的核验；
  - 返回 `StartupTask` 时引擎不写日志文案，也先不记“已登录”；首项启动任务出现在 worker 回报的已完成任务里，才记已登录。

## 第三步：前端描述对象

托管页、账号页、新建流程和路由都复用 MFW 的公共页面。新风味只写一个描述对象。下面的顺序与 `frontend/src/composables/useMaaFWFlavor.ts` 文件头的清单一致；文件头里“词表 zh-CN.ts”一项已过时，词条以后端 `res/i18n` 为准（见“收尾”）。

1. 后端写完并重新生成 OpenAPI 后，`ScriptType` 会自动多出 `'X'`：它由生成的 API 的创建类型派生（`frontend/src/types/script.ts`），不用另抄一份。`scriptLogos.ts` 的图标与展示名由下一条的 `SCRIPT_METADATA` 派生，`useScriptApi.ts` 的配置类名由风味注册表提供，这几处都不用再手改。
2. 在 `frontend/src/utils/scriptMetadata.ts` 的 `SCRIPT_METADATA` 里加 `X` 一项（该表 `satisfies Record<ScriptType, ScriptMetadata>`，漏了 typecheck 会报错）：`logo`（图片放 `frontend/src/assets/`，顶部 import）、`label`、`typeTagColor`、`routeSuffix`、`routeTitles`（`script` / `userAdd` / `userEdit`）、`createOrder`、`create`（`titleKey`、`descriptionKey`、搜索用 `keywords`、`group`）。
3. 在 `frontend/src/composables/maafwFlavorTypes.ts` 的 `MaaFWFlavorType` 里加上 `'X'`。
4. 新建目录 `frontend/src/views/EditView/MaaFWFlavor/x/`，在 `index.ts` 里用 `defineMaaFWFlavor` 导出描述对象 `X_FLAVOR`。**只写与 MFW 不同的部分**：身份字段、`routes.suffix`、`create.card` 必写；文案 key 只写要换的；没有独有区块就不写 `slots`。
5. 在 `frontend/src/composables/useMaaFWFlavor.ts` 的 `FLAVOR_REGISTRY` 里登记（漏登记 typecheck 会报错），并在文件顶部 import。
6. 帮助链接 `docUrl` 取 `frontend/src/utils/openExternal.ts` 的 `MAS_DOC_URLS`：通用的是 `MAS_DOC_URLS.scripts`，有专门文档页的在 `MAS_DOC_URLS.scriptTypes` 里加一项（M9A 就是这样）。
7. 风味在脚本或账号配置上有独有字段、且前端组件要读写它时，把字段按可选项加进 `frontend/src/types/script.ts` 的 `MaaFWScriptConfig` / `MaaFWUserConfig`（如 `Run.LoginMode?`、`Info.PlanMode?`）。
8. 补词条，见下面“收尾”。

最小示例（写法照 `mss/index.ts`：先取 `SCRIPT_METADATA.X`，再用 `metadata.label` 等）：

```ts
import { MAS_DOC_URLS } from '@/utils/openExternal'
import { SCRIPT_METADATA } from '@/utils/scriptMetadata'
import { defineMaaFWFlavor } from '../defineFlavor'

const metadata = SCRIPT_METADATA.X

export const X_FLAVOR = defineMaaFWFlavor({
  type: 'X',
  scriptConfigType: 'XConfig',
  userConfigType: 'XUserConfig',
  defaultScriptName: '新 X 托管',
  typeTagLabel: metadata.label,
  typeTagColor: metadata.typeTagColor,
  logo: metadata.logo,
  docUrl: MAS_DOC_URLS.scripts,
  routes: { suffix: metadata.routeSuffix, titles: metadata.routeTitles },
  create: {
    card: {
      ...metadata.create,
      after: null,
    },
  },
})
```

`create.card.after` 是新建对话框里这张卡片排在哪个类型之后，`null` 排到最后（M9A 是 `'MaaEnd'`）。

路由不用改：`frontend/src/router/maafwFlavorRoutes.ts` 会按注册表为每个类型生成托管编辑、项目引导、添加账号、编辑账号四条路由，页面宿主按托管实际类型选页面。

### 描述对象里可以写什么

先看哪些字段按“差异”合并。`scriptPage.text`、`userPage.text`、`userPage.managed` 按字段浅合并到 MFW 的默认值上：不写沿用 MFW，写 `null` 表示明确关掉（只有标注可为空的字段能写 `null`）。`page`、`sections`、`slots`、`prepare`（含 `create.sections` / `create.slots`）不继承，不写就是没有。

`scriptPage.text`（值都是词条 key）：

| 字段 | 作用 | MFW 默认 |
| --- | --- | --- |
| `titleKey` | 脚本编辑页标题；可为 `null`，`null` 时是“<项目名> 项目配置”。引导形态一律是“<项目名> 项目引导” | `null` |
| `sourceDirectoryKey` | 项目目录字段的标签 | `core.edit.localProjectDirectory` |
| `sourceHintKey` | 项目目录标签上的问号提示（导入后字段锁死，提示换成统一的“已导入”那句） | `core.edit.pickMfwProjectDirectory` |
| `sourcePlaceholderKey` | 项目目录输入框占位 | `core.edit.pickActualMfwProject` |

M9A、MSS 只改了 `sourceHintKey`，在目录问号里点出项目名。

`userPage.text`：

| 字段 | 作用 | MFW 默认 |
| --- | --- | --- |
| `accountPlaceholderKey` | 账号字段的占位。账号另有用处时写清作用 | `core.edit.localNoteOnly` |
| `accountTooltipKey` | 账号标签上的问号提示；可为 `null`，`null` 时不挂问号 | `null` |
| `queueHintKey` | 任务队列区顶部的提示，`\n` 分行，一行一个框；可为 `null`，`null` 时不显示 | `null` |

`userPage.managed`：风味把某些任务收归后端全权控制（受管任务）时才写，没有就沿用 MFW 的空值。字段必须与后端的整理规则是同一组 entry。

| 字段 | 作用 | MFW 默认 |
| --- | --- | --- |
| `entries` | 不许用户自己加的任务（interface 里任务的 `entry`）：账号页“添加任务”与预设模板里都不出现；已经在队列里的照常显示（能看能删） | `[]` |
| `accountTask` | 受管的切号任务 `{ entry, resources }`：资源在 `resources` 里、队列里它的有效实例（目标账号非空）≥ 2 时，后端拒绝运行该账号、要求拆成多个账号。可为 `null` | `null` |
| `warningKey` | 需要拆账号时的警告词条，插值 `count`（个数）、`accounts`（各目标账号）；可为 `null` | `null` |
| `noticeKey` | 其余受管任务残留在队列里时的提示词条，插值 `tasks`（任务名），运行照常；可为 `null` | `null` |

M9A 的写法见 `m9a/index.ts`：`entries: ['StartUp', 'SwitchAccount', 'Close1999']`，`accountTask: { entry: 'SwitchAccount', resources: ['官服'] }`。

`prepare`（`scriptPage.prepare` / `userPage.prepare`）：`async () => { ... }`，页面加载期间与其他读取并行调用，用来给独有区块预取数据，避免区块先空一下再冒出来。失败不要往外抛，自己兜底。`scriptPage.prepare` 在读到脚本详情后调用，导入后类型变成本风味时再调一次；`userPage.prepare` 与读取 interface 并行。新建流程没有 `prepare`。把取数逻辑放在按需加载的模块里，注册表会被脚本列表、路由等处引入，不要带上 API 依赖。MSS 的写法：

```ts
prepare: async () => {
  const { loadMSSPlanComboxItems } = await import('./planModeOptions')
  await loadMSSPlanComboxItems()
},
```

### 需要独有界面时

由浅到深，能用浅的就别用深的：

1. **插入点**：用 `defineMaaFWLazyComponent(() => import('./XField.vue'))` 写进 `scriptPage.slots` / `userPage.slots` / `create.slots`，值是组件数组，按顺序渲染。组件以一个 `context` prop 接收上下文，改动按页面类型交回：脚本页发 `change(category, key, value)`，账号页发 `save(key, value)`，新建流程的插入点只读。现有插入点：
   - 托管页：`afterBasicInfo`、`beforeControl`、`besidePackageName`、`afterControl`、`afterUpdate`、`afterRun`；
   - 账号页：`afterBasicInfo`、`beforeTaskQueue`、`afterTaskQueue`（都不进任务队列两栏）；
   - 新建流程：`afterSourceStep`（只读）。

   `besidePackageName` 比较特殊：它不是页面直接放的位置，而是由**控制方式分节**（`control`）提供的同名作用域 slot——ADB 下排在“游戏包名”“等待时间”之后、截图 / 输入策略表之前，每个组件占一格。页面按登记的组件数逐格转发。所以只用插入点时不用管；**替换 `control` 分节时，要自己在新分节里保留这个 slot**（按 `besidePackageNameCount` 每格转发一次，带上 `{ index }`），否则这些组件不会显示。

   账号页插入点组件示例（写法照 `mss/MSSPlanModeField.vue`）：

   ```vue
   <!-- eslint-disable vue/no-mutating-props -- Slot components edit the parent-owned reactive draft; persistence stays in the parent. -->
   <template>
     <a-form-item label="X 选项">
       <a-select
         v-model:value="context.formData.Info.Notes"
         :options="options"
         :disabled="context.loading"
         @change="emit('save', 'Info.Notes', context.formData.Info.Notes)"
       />
     </a-form-item>
   </template>

   <script setup lang="ts">
   import type { MaaFWUserSlotContext } from '@/composables/maafwFlavorTypes'

   defineProps<{
     context: MaaFWUserSlotContext
   }>()

   const emit = defineEmits<{
     save: [key: string, value: unknown]
   }>()

   const options = [
     { label: '甲', value: 'a' },
     { label: '乙', value: 'b' },
   ]
   </script>
   ```

   示例拿 `Info.Notes` 当被保存的字段；实际用时换成你在 `types/script.ts` 里加的字段。文件顶部那行 `eslint-disable vue/no-mutating-props` 不能省，现有插入点组件都这样写：配置草稿归页面所有，组件直接改草稿，落盘由页面完成。例如 MSS 在 `userPage.slots.beforeTaskQueue` 放计划表下拉与活动优先开关，M9A 在 `scriptPage.slots.besidePackageName` 放“登录方式”和“游戏更新”两个下拉（`M9ALoginModeField.vue`、`M9AGameUpdateField.vue`）。
2. **替换分节**：`defineMaaFWSection(part, key, load)` 写进 `sections`。替换组件必须接得住 `MaaFWFlavor/sectionContracts.ts` 里该分节的 props（接不住 typecheck 会报错），并按该分节的 `…Emits` 发出事件（或把 `$attrs` 透传给它包的 MFW 分节），否则改动不会保存。
3. **整页替换**：`scriptPage.page` / `userPage.page`，用 `views/EditView/Script/MaaFWScriptEdit/pageKit.ts`、`views/EditView/User/MaaFWUserEdit/pageKit.ts` 导出的分节与编排层拼，不要复制 MFW 页面。

需要一个现在没有的插入点：在 `maafwFlavorTypes.ts` 的 `MaaFWFlavorSlotContextMap` 加名字，并在公共页面对应位置放 `<MaaFWFlavorSlot>`。不要在公共页面里写 `type === 'X'` 分支，也不要往描述对象里加布尔开关；这两条没有自动检查拦着，靠评审把关。

## 需要计划表时（以 MSS 为例） {#plan-table}

风味要让账号引用一张按星期或轮换日出不同配置的计划表时，除了上面的步骤，还要多做下面几步。逐项都能在 MSS 里找到对应代码。

后端：

1. 账号配置里加引用字段，并让它拿得到计划表集合。`MSSUserConfig` 的写法（`script.py`）：类属性 `related_config: ClassVar = {}`，`Info.PlanMode` 用 `ref("PlanConfig", default="Fixed", allow_values=("Fixed",), entry_types=(MSSPlanConfig,), display_field="Info.Name")`，默认值 `"Fixed"`。`related_config` 由 `app/core/config/manager.py` 在初始化时按列表填入，而账号配置类不在 `CLASS_BOOK` 里，所以要把 `XUserConfig` 也加进那处列出 `MSSUserConfig`、`BAAHUserConfig` 的元组，并在 `manager.py` 顶部从 `app.models.config` 导入它。
2. 计划表配置类：在 `app/models/config/plan.py` 建 `XPlanConfig`（MSS 继承 `WeeklyKeyPlan`），每个日期槽位存一个 key 模型。key 模型 `XPlanKey` 写在 `app/models/schema.py`（`MSSPlanKey` 为例，`extra="forbid"`），旧数据归一化函数写在 `app/models/config/shared.py`（`normalize_mss_plan_key`），并在 `XPlanConfig.normalize_payload` 里调用。`plan.py` 里要把新类加进 `__all__` 与 `PlanConfigType` 联合。
3. 在 `app/models/config/__init__.py` 的 `PLAN_BOOK` 里登记，键是计划表类名：`create_type`（创建时用的类型名，如 `"MSSPlan"`）、`config_class`、`consumer`（取 `PLAN_CONSUMER_VALUES[下标]`）、`script_class`（托管类 `XConfig`）、`field_group` 与 `field_name`（账号上引用计划表的字段，如 `"Info"`、`"PlanMode"`）。删除计划表时，管理器靠后三项找出引用它的账号（先按 `script_class` 筛托管，再比对账号上这个字段），把引用改回 `"Fixed"`（`app/core/config/manager.py` 的删除计划逻辑），所以账号上的引用字段要有 `"Fixed"` 这个取值。
4. 在 `app/utils/common/constants.py` 的 `PLAN_CONSUMER_VALUES` 里**只能往末尾追加**新消费方：`PLAN_BOOK` 按下标引用它，往中间插会把已有计划表类型指到别的消费方（该常量的文档字符串写明了；`"mss"` 排在 `"baah"` 之后，下标就是这么来的）。
5. 在 `app/models/schema.py` 的 `PlanComboxConsumer`（`Literal`）里加同样的字符串，计划下拉框接口（`app/api/info.py`）按它校验。
6. 在 `app/api/plan.py` 里：`XPlanConfig` / `XPlanConfigInput` 别名、`PlanCreateType`（`Literal`）、联合类型 `PlanConfigData` 与 `PlanConfigDataInput` 都加上。计划表索引与创建接口的类型枚举由 `PLAN_BOOK` 自动生成，不用手写。
7. 风味钩子里取当天槽位：`type(user_config).related_config["PlanConfig"][uuid.UUID(mode)]`，再调计划表的 `get_current_key()`（见 `app/task/MSS/flavor.py` 的 `_current_plan_key`）。引用的计划表被删或类型不对时，按“固定”处理并写日志。

前端：

1. `frontend/src/utils/planTypeRegistry.ts`：`PLAN_CONFIG_TYPES` 里加 `X: PlanIndexItem.type.…`（枚举名由 OpenAPI 生成），`PLAN_TYPE_REGISTRY` 里加描述：`configType`、`createType`、`displayNameKey`、`defaultName`（会被写进计划名并用来判断“还是默认名”，保持中文）、`selectorTag`、`reloadAfterSave`、`supportsSimpleView`、`tableComponent`。
2. 计划表组件 `frontend/src/views/plan/tables/XPlanTable.vue`，props 为 `tableData`、`currentMode`、`viewMode`、`planId`、`handlePlanChange`，照 `MSSPlanTable.vue` 写。
3. 账号页的计划表下拉：独有区块插入点 + `userPage.prepare` 预取。选项从 `Service.getPlanComboxApiInfoComboxPlanPost({ consumer: PlanComboxIn.consumer.X })` 取（见 `mss/planModeOptions.ts`），第一项固定是“固定”（值 `Fixed`）。
4. 词条 `core.plan.type.x`（见下面“收尾”）。

## 收尾

1. **补词条**。词条在后端资源 `res/i18n/zh_CN.json`、`en.json`、`ja.json`，是扁平的点分 key。中文是源语言，必须加；`en.json`、`ja.json` 允许缺 key，M9A、MSS 三份都补了，照着补上：
   - `core.scripts.type.X`：类型名；
   - `core.scripts.create.typeDesc.X`：新建对话框里的类型描述；
   - 描述对象和插入点组件里用到的文案 key（如 `core.edit.xFlavorSourceHint`）；
   - 有计划表的，再加 `core.plan.type.x`。
2. **重新生成 OpenAPI**。先起目标工作树的开发后端（`frontend/package.json` 里的 `openapi` 脚本从 `http://127.0.0.1:36164/openapi.json` 取 schema，后端要监听这个端口），再从该树的 `frontend` 目录运行 `corepack yarn openapi`。这条脚本在生成后会运行 `scripts/postprocess-openapi.mjs`，把生成物的 CRLF 换行统一成 LF；不要手改 `frontend/src/api/`。

## 检查清单

提交前逐条核对：

- [ ] 用目标项目和一个普通 MaaFramework 项目各导入一次，类型能正确切换，且没有误认。
- [ ] `app/task/MaaFW/` 里没有出现风味名字；公共前端页面里没有按类型分支；描述对象里没有布尔值。
- [ ] `decorate_selection` 处理过重复任务实例、空队列、缺失任务或选项、用户编辑后保存的队列；自动补任务时没有补回 `excluded_task_ids` 里的实例；有快照整理钩子时还核对过恢复配置后的结果。
- [ ] 风味接管的任务（如果有）在后端整理规则与前端描述对象里是同一组 entry。
- [ ] 新增的可选钩子没有加进 `MaaFWFlavor` 协议。
- [ ] 第一步里的登记位置都补齐了：`script.py`（含 `__all__`、类型联合）、`CLASS_BOOK`、`TYPE_BOOK`、`registration.py` 的 `run_types`、`management.py` 与 `users.py`。
- [ ] 验证过风味特有的运行顺序或选项，同时确认 MFW 通用的运行、项目更新和账号配置仍可使用。
- [ ] 在目标工作树用其本地依赖运行了 `tests/task/test_maafw_core.py` 与相关后端检查，以及前端 `typecheck`、`lint` 和相关测试；记录实际运行的命令与结果。改了 schema 的，从该树的开发后端重新生成了 OpenAPI。

只读参考：[M9A 风味](https://github.com/AUTO-MAS-Project/AUTO-MAS/tree/dev/app/task/M9A)、[MSS 风味](https://github.com/AUTO-MAS-Project/AUTO-MAS/tree/dev/app/task/MSS)、[前端风味目录](https://github.com/AUTO-MAS-Project/AUTO-MAS/tree/dev/frontend/src/views/EditView/MaaFWFlavor)。
