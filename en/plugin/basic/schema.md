# Configuration Schema

::: warning
This page follows the `dev` branch implementation as of 2026-11-18. The plugin system is still changing; the interfaces are whatever your installed AUTO-MAS source says.
:::

A plugin's configuration is declared with a Python class. The host uses it to validate, persist, and generate the settings page. You write no frontend code.

::: tip Differences from the old version
`Config(BaseModel)`, `PluginField()`, `mas.plugin_config`, and `ctx.config.get()` from the old docs no longer exist, and the runtime schema dict and the type names `string`/`key_value`/`table` are no longer used. Config is now the host's unified configuration framework (`ConfigEntry` / `ConfigGroup`), the same one used for script, user, and plan configuration.
:::

## Minimal Example

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

Key points:

- The config class extends `PluginConfig`, and you set it on the `BasePlugin` subclass with `config_class`.
- **Groups:** each field in the config class annotated with a `ConfigGroup` subclass is a group; in the example, `general` and `action`. The settings page is divided into blocks by group.
- **Fields:** inside a group are ordinary pydantic fields. Types, defaults, and constraints such as `ge`/`le` are all validated by pydantic.
- The group name `ui` is taken by the host; do not use it.
- Read with `self.config.general.interval`. To write, assign and then `await self.config.commit()`; see [Core Capabilities](./core).

## Where Config Is Stored

- File: `config/plugins/<plugin name>.yaml`, under AUTO-MAS's working directory.
- It is built from the file and activated on enable; on disable it is committed and written to disk before release; on uninstall the file is deleted.
- The `uid` of the config object and the `uid` of the plugin's registry entry are two different values.

## UI Controls

The controls on the settings page are inferred from the field type, and you can override them with annotations. The backend puts the inferred result in the config's `ui.hints`, and the frontend `DeclarativeControl.vue` renders it.

| Field type | Control |
| --- | --- |
| `bool` | Switch |
| `int` | Number input; `ge`/`gt`/`le`/`lt` become the min and max (a constraint value of `0` is not passed to the control) |
| `str` | Single-line input |
| `Annotated[str, encrypted()]` | Password input, stored encrypted |
| `Literal[...]`, `Enum` | Dropdown |
| `list[Literal[...]]`, `list[Enum]` | Multi-select dropdown |
| `datetime.date` | Date picker |
| `FilePath`, `FolderPath`, `ExecutablePath` | Path input with a "Select" button |
| `Annotated[str, select(endpoint=...)]` | Dropdown whose options come from a host endpoint; see below |
| `Trigger` | Button; see [Triggers](#triggers) |

A type with no matching control falls back to a single-line text input. Known cases:

- `float`, `time`, and `datetime` render as a single-line input, and the entered value is validated by the backend.
- `list[str]`, `dict`, and similar have no editor. Do not use them as UI fields; if you need them, change to a `Literal` list or put the data in a hidden group.
- The buttons for choosing files and directories rely on the desktop app's system dialogs; they are unavailable when the frontend is opened on its own in a browser.

The control width is determined by type and fixed by the frontend: buttons and switches take 6 columns, dropdowns and numbers take 8, and the rest take 12 (out of 24 columns).

### Overriding the Inference: `ui()`

```python
from auto_mas_core import ui

# 放在 Annotated 里
note: Annotated[str, ui(widget="input")] = Field(default="")
```

| Parameter | Purpose |
| --- | --- |
| `widget` | Specifies the control name, overriding the inference. The frontend knows `input`, `path`, `number`, `select`, `date`, `button`, `dropdown-button`, and `switch` (`bool`, `boolean`, `checkbox`, and `toggle-button` are the same as `switch`, and `ref-select` is the same as `select`); other names are treated as a single-line input |
| `visibility` | Show/hide; see below |
| `secret` | Marks it as a password input |
| `format`, `deps` | Written into the hint; `deps` is used for option linkage; `format` is currently unused by the frontend and produces no URL or email validation |

### Dropdown Options from a Host Endpoint: `select()`

```python
from auto_mas_core import select

game: Annotated[str, select(endpoint="/api/plugin/options/games")] = Field(default="-")
```

The frontend only recognizes the `endpoint` values below. Any other value shows "option endpoint not wired up yet" on the page, with that address:

| endpoint | Options | Linkage |
| --- | --- | --- |
| `/api/plugin/options/games` | Games registered with the host | None |
| `/api/plugin/options/devices` | Devices of a game | `ui(deps=["<group>.<game field>"])` |
| `/api/plugin/options/plans` | Plans | None |
| `/api/info/combox/emulator` | Emulators | None |
| `/api/info/combox/emulator/devices` | Device instances of an emulator | `ui(deps=["<group>.<emulator field>"])` |
| `/api/info/combox/plan` | General plans | None |

A dropdown with an `endpoint` gets an extra "Not linked" (「不关联」) option, whose value is `"-"`; make the field's default `"-"`. When the dependency field's value is `"-"` or empty, the device dropdown does not request the endpoint and has only the "Not linked" option.

## Visibility and Read-only

`UiVisibility` has four values:

| Value | Effect |
| --- | --- |
| `SHOW` | Shown and editable |
| `HIDE` | Not shown, and the field is not in the API output either |
| `DISABLE` | Shown but read-only |
| `MASKED` | The value is replaced by a placeholder; submitting the placeholder unchanged means no modification. Only usable on specific fields |

There are two ways to declare it. A field may use only one; declaring both raises `TypeError` when the class is created:

```python
# 常量：放在 Annotated 里，字段或整个分组都可以
debug: Annotated[bool, ui(visibility=UiVisibility.HIDE)] = Field(default=False)
cache: Annotated[CacheFields, ui(visibility=UiVisibility.HIDE)] = Field(default_factory=CacheFields)

# 动态：配置类上的同步方法，按其他字段的值决定
@ui_visibility("general.token")
def _vis_token(self) -> UiVisibility:
    return UiVisibility.SHOW if self.general.mode != "never" else UiVisibility.HIDE
```

- A dynamic method must be a synchronous function that returns `UiVisibility`; written as `async def`, it raises `TypeError` when evaluated.
- When a group is `HIDE`, all fields in it are hidden; when a group is `DISABLE`, all fields in it are read-only.
- When the config is locked (for example, a task is using it), all fields are read-only.
- Visibility is evaluated in the backend. The frontend re-reads the config after a save or a button click; it does not recompute in real time while editing.
- Changes submitted by the frontend for `HIDE` and `DISABLE` fields are ignored. Program-internal code can bypass this restriction with `await config.update(patch, check_visibility=False)`.

## Triggers

A trigger is a "click to run" field. Its value is not persisted, and reading it always returns the idle value (a `Trigger` is `False`; the `Literal` trigger below is `None`).

1. Declare a `Trigger` field in a group.
2. Bind a handler method on the config class with `@trigger_field("<group>.<field>")`.
3. If needed, control the button's visibility with `@ui_visibility`.

```python
from auto_mas_core import trigger_field

@trigger_field("action.send_test", cancellable=False)
async def on_send_test(self) -> None:
    ...
```

- The handler can be sync or async and takes no parameters.
- `cancellable` defaults to `True` and declares whether this operation may be cancelled.
- The binding must be one-to-one: assigning to a `Trigger` field without `@trigger_field` raises `AttributeError`; binding the same field twice raises `TypeError`.
- The handler runs as a plugin operation in the background task group and does not block the caller. While the same trigger is running, triggering it again with the same value merges into the same operation, and triggering with a different value is rejected.
- For a `HIDE` or `DISABLE` trigger, assigning `True` is silently ignored; starting it through `Operations.start` is rejected.

After a frontend button click, the frontend calls the host's command endpoint and refreshes the page when the operation ends; the whole form is disabled while it runs.

There are two ways to trigger it from code:

```python
# 1) 赋值。不返回操作 ID
self.config.action.send_test = True

# 2) 通过 Operations 启动，拿到操作 ID 用于查询和取消
from auto_mas_core.services import Operations

operation_id = await Operations.start(
    config_uid=self.config.uid, group="action", field="send_test", value=True
)
```

`Operations.start` is rejected when the config is locked, and also when the value equals the idle value (for example `False`). The HTTP entry is `/api/operations/start`.

### Multi-button Triggers

Write a field as a `Literal` and the UI shows a dropdown button; the selected value is passed to the handler.

```python
from typing import Annotated, Literal

from app.config import TriggerDecl

class Action(ConfigGroup):
    export: Annotated[Literal["json", "yaml"] | None, TriggerDecl()] = None
```

The handler takes one parameter:

```python
@trigger_field("action.export")
async def on_export(self, value: str) -> None:
    ...
```

::: warning
`TriggerDecl` is currently not exported from `auto_mas_core`; it can only be imported from `app.config`. A single field cannot mix `bool` and `Literal`.
:::

## UI Text

Labels in the UI do not come from pydantic's `description`; they come from the plugin package's language files. If not found, the field name or group name is shown directly.

Language files go in the package's `i18n/` directory: `zh_CN.json`, `en.json`, and `ja.json` (`zh-CN.json`, `en-US.json`, and `ja-JP.json` are also accepted). A file is a flat JSON object. Every key must start with `plugins.<plugin name>.`, and every value must be a string.

| Key | Purpose |
| --- | --- |
| `plugins.<plugin name>.config.<group>.label` | Group title |
| `plugins.<plugin name>.config.<group>.<field>.label` | Field label; button text uses it too |
| `plugins.<plugin name>.config.<group>.<field>.tip` | The question-mark tooltip beside the field |
| `plugins.<plugin name>.config.<group>.<field>.options.<value>` | Display text of a dropdown option |

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

An empty object, duplicate keys, or keys without the prefix make the language resources fail to load.

## Saving and Validation

- "Save" on the settings page submits the values of all current groups to `/api/plugin_config/update`, and the backend re-validates them with your `config_class`. On a validation failure it returns an error and writes nothing.
- The frontend only applies control-level range limits; the backend validation is what counts.
- When the plugin is not enabled or has not declared `config_class`, the settings page shows an empty state, and the save endpoint returns "plugin not enabled or has no config of its own" (「插件未启用或无自有配置」).

## Development Tips

1. Use `Literal[...]` for enumerations; the UI automatically becomes a dropdown.
2. Use `Annotated[str, encrypted()]` for secrets and tokens.
3. Put state that is only for internal use in a `HIDE` group so it does not appear in the UI.
4. Write the conditions for dynamic visibility in `@ui_visibility` methods, keeping them synchronous and free of IO.
5. Always put UI text in the language files; treat `description` as a code comment only.
