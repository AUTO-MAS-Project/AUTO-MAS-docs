# 配置声明

::: warning
本文以 `dev` 分支 2026-11-18 的实现为准。插件系统仍在变化，接口以你安装的 AUTO-MAS 源码为准。
:::

插件的配置用 Python 类声明，宿主据此完成校验、持久化，并生成设置页面。你不写任何前端代码。

::: tip 和旧版的区别
旧版文档里的 `Config(BaseModel)`、`PluginField()`、`mas.plugin_config`、`ctx.config.get()` 都已不存在，运行时 schema 字典和 `string`/`key_value`/`table` 这些类型名也不再使用。现在配置是宿主统一的配置框架（`ConfigEntry` / `ConfigGroup`），和脚本、用户、计划配置是同一套。
:::

## 最小示例

```python
from typing import Annotated, Literal

from pydantic import Field

from auto_mas_core import (
    BasePlugin,
    ConfigGroup,
    PluginConfig,
    Trigger,
    UiVisibility,
    encrypted,
    trigger_field,
    ui_visibility,
)


class DemoConfig(PluginConfig):
    class General(ConfigGroup):
        enabled: bool = Field(default=True, description="启用")
        interval: int = Field(default=5, ge=1, le=60, description="检查间隔（分钟）")
        mode: Literal["always", "failed_only", "never"] = Field(
            default="always", description="通知策略"
        )
        token: Annotated[str, encrypted()] = Field(default="", description="访问令牌")

    class Action(ConfigGroup):
        send_test: Trigger = Field(default=False, description="发送测试通知")

    general: General = Field(default_factory=General, description="常规")
    action: Action = Field(default_factory=Action, description="操作")

    @ui_visibility("action.send_test")
    def _vis_send_test(self) -> UiVisibility:
        # 没启用时按钮置灰
        return UiVisibility.SHOW if self.general.enabled else UiVisibility.DISABLE

    @trigger_field("action.send_test", cancellable=False)
    async def on_send_test(self) -> None:
        from auto_mas_core.notify import Notify, NotifyPayload

        await Notify.send_global(
            NotifyPayload(title="测试", text=f"策略：{self.general.mode}"),
            include_system=True,
        )


class PLUGIN(BasePlugin):
    config_class = DemoConfig

    async def on_enable(self) -> None:
        assert isinstance(self.config, DemoConfig)
        interval = self.config.general.interval
```

要点：

- 配置类继承 `PluginConfig`，在 `BasePlugin` 子类上用 `config_class` 指定。
- **分组**：配置类里每个注解为 `ConfigGroup` 子类的字段是一个分组，上例的 `general`、`action`。设置页按分组分块。
- **字段**：分组里是普通的 pydantic 字段，类型、默认值、`ge`/`le` 等约束都由 pydantic 校验。
- 分组名 `ui` 被宿主占用，不要使用。
- 读取用 `self.config.general.interval`。写入是赋值后 `await self.config.commit()`，见[核心功能](./core)。

## 配置存在哪

- 文件：`config/plugins/<插件名>.yaml`，在 AUTO-MAS 的工作目录下。
- 启用时从文件构建并激活；停用时先提交、写盘再释放；卸载时删除文件。
- 配置对象的 `uid` 与插件注册项的 `uid` 是两个不同的值。

## 界面控件

设置页的控件由字段类型推导，也可以用注解覆盖。后端把推导结果放在配置的 `ui.hints` 里，前端 `DeclarativeControl.vue` 负责渲染。

| 字段类型 | 控件 |
| --- | --- |
| `bool` | 开关 |
| `int` | 数字输入，`ge`/`gt`/`le`/`lt` 作为最小、最大值（约束值为 `0` 时不会下发给控件） |
| `str` | 单行输入框 |
| `Annotated[str, encrypted()]` | 密码框，加密存储 |
| `Literal[...]`、`Enum` | 下拉框 |
| `list[Literal[...]]`、`list[Enum]` | 多选下拉框 |
| `datetime.date` | 日期选择器 |
| `FilePath`、`FolderPath`、`ExecutablePath` | 路径输入框和「选择」按钮 |
| `Annotated[str, select(endpoint=...)]` | 下拉框，选项来自宿主接口，见下 |
| `Trigger` | 按钮，见[触发器](#触发器) |

没有对应控件的类型会退化成单行文本输入。已知的情况：

- `float`、`time`、`datetime` 渲染成单行输入框，输入值由后端校验。
- `list[str]`、`dict` 等没有编辑器。不要把它们当界面字段用，需要的话改成 `Literal` 列表，或把数据放在隐藏分组里。
- 选择文件和目录的按钮靠桌面端的系统对话框；浏览器里单独打开前端时不可用。

控件宽度由类型决定，前端固定：按钮、开关占 6 列，下拉框、数字占 8 列，其余占 12 列（共 24 列）。

### 覆盖推导：`ui()`

```python
from auto_mas_core import ui

# 放在 Annotated 里
note: Annotated[str, ui(widget="input")] = Field(default="")
```

| 参数 | 作用 |
| --- | --- |
| `widget` | 指定控件名，覆盖推导。前端认识 `input`、`path`、`number`、`select`、`date`、`button`、`dropdown-button`、`switch`（`bool`、`boolean`、`checkbox`、`toggle-button` 同 `switch`，`ref-select` 同 `select`）；其他名称按单行输入处理 |
| `visibility` | 显隐，见下 |
| `secret` | 标记为密码输入 |
| `format`、`deps` | 写入 hint，`deps` 用于选项联动；`format` 前端目前不使用，不会产生 URL 或邮箱校验 |

### 下拉选项来自宿主接口：`select()`

```python
from auto_mas_core import select

game: Annotated[str, select(endpoint="/api/plugin/options/games")] = Field(default="-")
```

前端只认下面这些 `endpoint`，写其他值会在页面上提示「尚未接入选项接口」并附上该地址：

| endpoint | 选项 | 联动 |
| --- | --- | --- |
| `/api/plugin/options/games` | 宿主已登记的游戏 | 无 |
| `/api/plugin/options/devices` | 某个游戏的设备 | `ui(deps=["<分组>.<游戏字段>"])` |
| `/api/plugin/options/plans` | 计划 | 无 |
| `/api/info/combox/emulator` | 模拟器 | 无 |
| `/api/info/combox/emulator/devices` | 某个模拟器的设备实例 | `ui(deps=["<分组>.<模拟器字段>"])` |
| `/api/info/combox/plan` | 通用计划 | 无 |

带 `endpoint` 的下拉框会多出一项「不关联」，对应值 `"-"`，字段默认值写 `"-"`。依赖字段的值为 `"-"` 或空时，设备下拉框不请求接口，只有「不关联」一项。

## 显隐与只读

`UiVisibility` 有四个值：

| 值 | 效果 |
| --- | --- |
| `SHOW` | 显示，可编辑 |
| `HIDE` | 不显示，接口输出里也没有这个字段 |
| `DISABLE` | 显示，但只读 |
| `MASKED` | 值被占位符替换，原样提交占位符表示不修改。只能用在具体字段上 |

两种声明方式，同一字段只能选一种，重复声明在类创建时抛 `TypeError`：

```python
# 常量：放在 Annotated 里，字段或整个分组都可以
debug: Annotated[bool, ui(visibility=UiVisibility.HIDE)] = Field(default=False)
cache: Annotated[CacheFields, ui(visibility=UiVisibility.HIDE)] = Field(default_factory=CacheFields)

# 动态：配置类上的同步方法，按其他字段的值决定
@ui_visibility("general.token")
def _vis_token(self) -> UiVisibility:
    return UiVisibility.SHOW if self.general.mode != "never" else UiVisibility.HIDE
```

- 动态方法必须是同步函数，返回 `UiVisibility`；写成 `async def`，求值时抛 `TypeError`。
- 分组被 `HIDE` 时，组内字段全部隐藏；分组 `DISABLE` 时，组内字段全部只读。
- 配置被锁定（例如有任务在使用）时，所有字段都只读。
- 显隐在后端求值。前端在保存或点按钮之后重新读取配置，编辑过程中不会实时重算。
- `HIDE` 和 `DISABLE` 的字段，前端提交的修改会被忽略。程序内部调用 `await config.update(patch, check_visibility=False)` 可以绕过这一限制。

## 触发器

触发器是「点一下执行」的字段，值不落盘，读取永远是空闲值（`Trigger` 是 `False`，下文的 `Literal` 触发器是 `None`）。

1. 在分组里声明 `Trigger` 字段。
2. 在配置类上用 `@trigger_field("<分组>.<字段>")` 绑定处理方法。
3. 需要时用 `@ui_visibility` 控制按钮显隐。

```python
from auto_mas_core import trigger_field

@trigger_field("action.send_test", cancellable=False)
async def on_send_test(self) -> None:
    ...
```

- 处理方法可以是同步或异步，没有参数。
- `cancellable` 默认 `True`，声明这个操作是否允许被取消。
- 绑定必须一一对应：`Trigger` 字段没有 `@trigger_field` 时，对它赋值抛 `AttributeError`；同一字段绑定两次抛 `TypeError`。
- 处理方法作为插件操作在后台任务组里运行，不阻塞调用方。同一个触发器正在运行时，再次以相同取值触发会合并到同一个操作，以不同取值触发会被拒绝。
- `HIDE` 或 `DISABLE` 的触发器，赋值 `True` 会被静默忽略；通过 `Operations.start` 启动则被拒绝。

前端按钮点击后，调用宿主的命令接口，等操作结束再刷新页面；执行期间整页表单被禁用。

在代码里触发有两种方式：

```python
# 1) 赋值。不返回操作 ID
self.config.action.send_test = True

# 2) 通过 Operations 启动，拿到操作 ID 用于查询和取消
from auto_mas_core.services import Operations

operation_id = await Operations.start(
    config_uid=self.config.uid, group="action", field="send_test", value=True
)
```

`Operations.start` 在配置被锁定时拒绝，值与空闲值相同（例如 `False`）时也拒绝。HTTP 入口是 `/api/operations/start`。

### 多按钮触发器

一个字段写成 `Literal`，界面显示为下拉按钮，选中的值会传给处理方法。

```python
from typing import Annotated, Literal

from app.config import TriggerDecl

class Action(ConfigGroup):
    export: Annotated[Literal["json", "yaml"] | None, TriggerDecl()] = None
```

处理方法接收一个参数：

```python
@trigger_field("action.export")
async def on_export(self, value: str) -> None:
    ...
```

::: warning
`TriggerDecl` 目前没有从 `auto_mas_core` 导出，只能从 `app.config` 导入。同一个字段不能把 `bool` 和 `Literal` 混用。
:::

## 界面文案

界面上的标签不取 pydantic 的 `description`，而是从插件包的语言文件里取；取不到就直接显示字段名或分组名。

语言文件放在包内 `i18n/` 目录：`zh_CN.json`、`en.json`、`ja.json`（也接受 `zh-CN.json`、`en-US.json`、`ja-JP.json`）。文件是扁平的 JSON 对象，键必须以 `plugins.<插件名>.` 开头，值必须是字符串。

| 键 | 用途 |
| --- | --- |
| `plugins.<插件名>.config.<分组>.label` | 分组标题 |
| `plugins.<插件名>.config.<分组>.<字段>.label` | 字段标签；按钮的文字也用它 |
| `plugins.<插件名>.config.<分组>.<字段>.tip` | 字段旁的问号提示 |
| `plugins.<插件名>.config.<分组>.<字段>.options.<值>` | 下拉选项的显示文字 |

```json
{
  "plugins.demo.config.general.label": "常规",
  "plugins.demo.config.general.interval.label": "检查间隔（分钟）",
  "plugins.demo.config.general.mode.label": "通知策略",
  "plugins.demo.config.general.mode.tip": "决定什么时候发通知",
  "plugins.demo.config.general.mode.options.always": "总是",
  "plugins.demo.config.general.mode.options.failed_only": "仅失败时",
  "plugins.demo.config.general.mode.options.never": "从不",
  "plugins.demo.config.action.label": "操作",
  "plugins.demo.config.action.send_test.label": "发送测试通知"
}
```

文件为空对象、键重复、键不带前缀都会导致语言资源加载失败。

## 保存与校验

- 设置页的「保存」把当前所有分组的值提交到 `/api/plugin_config/update`，后端用你的 `config_class` 重新校验；校验失败时返回错误，不写入。
- 前端只做控件级的范围限制，后端校验才算数。
- 插件没有启用，或没有声明 `config_class` 时，设置页显示空态，保存接口返回「插件未启用或无自有配置」。

## 开发建议

1. 枚举用 `Literal[...]`，界面自动是下拉框。
2. 密钥、令牌用 `Annotated[str, encrypted()]`。
3. 只给内部使用的状态放进 `HIDE` 的分组，不要出现在界面上。
4. 动态显隐的条件写在 `@ui_visibility` 方法里，保持同步、不做 IO。
5. 界面文案一律写进语言文件，`description` 只当代码注释。
