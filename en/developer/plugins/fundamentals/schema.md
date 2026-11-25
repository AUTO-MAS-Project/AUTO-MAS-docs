# Configuration Schema

::: warning
This guide describes the host contract after the Game Management migration. Check the SDK that ships with your target AUTO-MAS version.
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
    trigger,
    trigger_field,
    ui_visibility,
)


class DemoConfig(PluginConfig):
    class General(ConfigGroup):
        enabled: bool = Field(default=True, description="Enable")
        interval: int = Field(default=5, ge=1, le=60, description="Check interval (minutes)")
        mode: Literal["always", "failed_only", "never"] = Field(
            default="always", description="Notification policy"
        )
        token: Annotated[str, encrypted()] = Field(default="", description="Access token")

    class Action(ConfigGroup):
        send_test: Trigger = trigger(timeout=120, description="Send test notification")

    general: General = Field(default_factory=General, description="General")
    action: Action = Field(default_factory=Action, description="Actions")

    @ui_visibility("action.send_test")
    def _vis_send_test(self) -> UiVisibility:
        # Disable the button when the feature is not enabled
        return UiVisibility.SHOW if self.general.enabled else UiVisibility.DISABLE

    @trigger_field("action.send_test", finish_on_cancel=True)
    async def on_send_test(self) -> None:
        from auto_mas_core.notify import Notify, NotifyPayload

        await Notify.send_global(
            NotifyPayload(title="Test", text=f"Policy: {self.general.mode}"),
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
- **Fields:** inside a group are ordinary Pydantic fields. Types, defaults, and constraints such as `ge`/`le` are all validated by Pydantic.
- The group name `ui` is taken by the host; do not use it.
- Read with `self.config.general.interval`. To write, assign and then `await self.config.commit()`; see [Core Capabilities](./index.md).

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

# Place inside Annotated
note: Annotated[str, ui(widget="input")] = Field(default="")
```

| Parameter | Purpose |
| --- | --- |
| `widget` | Specifies the control name, overriding the inference. The frontend knows `input`, `path`, `number`, `select`, `date`, `button`, and `switch` (`bool`, `boolean`, `checkbox`, and `toggle-button` are the same as `switch`, and `ref-select` is the same as `select`); other names are treated as a single-line input |
| `visibility` | Show/hide; see below |
| `secret` | Marks it as a password input |
| `format`, `deps` | Written into the hint; `deps` is used for option linkage; `format` is currently unused by the frontend and produces no URL or email validation |

### Dropdown Options from a Host Endpoint: `select()`

```python
from auto_mas_core import select, ui

game: Annotated[str, select(endpoint="/api/plugin/options/games")] = Field(default="-")
device: Annotated[
    str,
    select(endpoint="/api/plugin/options/devices"),
    ui(deps=["general.game"]),
] = Field(default="-")
```

The frontend only recognizes the `endpoint` values below. Any other value shows "option endpoint not wired up yet" on the page, with that address:

| endpoint | Options | Linkage |
| --- | --- | --- |
| `/api/plugin/options/games` | Games registered with the host | None |
| `/api/plugin/options/devices` | Devices of a game | `ui(deps=["<group>.<game field>"])` |
| `/api/plugin/options/plans` | Plans | None |
| `/api/info/combox/plan` | General plans | None |

A dropdown with an `endpoint` gets an extra "Not linked" option, whose value is `"-"`; make the field's default `"-"`. When the dependency field's value is `"-"` or empty, the device dropdown does not request the endpoint and has only the "Not linked" option.

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
# Static: place inside Annotated on a field or an entire group
debug: Annotated[bool, ui(visibility=UiVisibility.HIDE)] = Field(default=False)
cache: Annotated[CacheFields, ui(visibility=UiVisibility.HIDE)] = Field(default_factory=CacheFields)

# Dynamic: a synchronous method on the config class that depends on other fields
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

A trigger runs a command independently of ordinary configuration saves. Reading its field gives `None` while idle or the task UUID while busy. Trigger state lives in memory and is excluded from configuration files and backups.

1. Declare a `Trigger` field in a group. Use `= None` for the default 900-second deadline, or `trigger(timeout=seconds)` for a finite positive deadline specific to that field.
2. Bind one instance method without arguments using `@trigger_field("group.field")`. Give each separate button its own field. Save action inputs through regular configuration fields before invoking the command.
3. Use `@ui_visibility` when a button’s visibility or editability depends on backend state.

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

| Value written | Effect |
| --- | --- |
| `None` / JSON `null` | Skip and return no UUID. |
| `True` / JSON `true` | Start an eligible action on committed config and return its UUID. If the field is already busy, return the existing UUID. |
| `False` / JSON `false` | Request cancellation and return the existing UUID, or no UUID if idle. |

Call `config.write_trigger("action", "send_test", True)` in Python. A config lock, disabled field, or plugin shutdown can prevent a new invocation. A repeated request for the same field reuses its active task. Actions on different fields need their own shared resource admission checks when they affect the same device.

The configuration manager uses its background task group. Synchronous handlers run in a worker thread. Async handlers run on the event loop and can offload blocking calls with `auto_mas_core.services.run_in_worker`. A task completion callback clears the field and records its outcome. When a deadline expires, the manager requests cancellation and records `timed_out` once the call settles. A running worker thread must actually return before the field clears. Give external calls their own I/O or process deadlines. Use `finish_on_cancel=True` on `@trigger_field` when that handler must run to completion even after cancellation or timeout. Use `finally` for temporary resource cleanup.

Query the UUID to distinguish success, failure, cancellation, and timeout. See the request examples and reconnect flow in [Configuration Trigger API](./frontend.md#configuration-trigger-api).

## UI Text

Labels in the UI do not come from Pydantic's `description`; they come from the plugin package's language files. If not found, the field name or group name is shown directly.

Language files go in the package's `i18n/` directory: `zh_CN.json`, `en.json`, and `ja.json` (`zh-CN.json`, `en-US.json`, and `ja-JP.json` are also accepted). A file is a flat JSON object. Every key must start with `plugins.<plugin name>.`, and every value must be a string.

| Key | Purpose |
| --- | --- |
| `plugins.<plugin name>.config.<group>.label` | Group title |
| `plugins.<plugin name>.config.<group>.<field>.label` | Field label; button text uses it too |
| `plugins.<plugin name>.config.<group>.<field>.tip` | The question-mark tooltip beside the field |
| `plugins.<plugin name>.config.<group>.<field>.options.<value>` | Display text of a dropdown option |

```json
{
  "plugins.demo.config.general.label": "General",
  "plugins.demo.config.general.interval.label": "Check Interval (minutes)",
  "plugins.demo.config.general.mode.label": "Notification Policy",
  "plugins.demo.config.general.mode.tip": "Decides when to send notifications",
  "plugins.demo.config.general.mode.options.always": "Always",
  "plugins.demo.config.general.mode.options.failed_only": "On failure only",
  "plugins.demo.config.general.mode.options.never": "Never",
  "plugins.demo.config.action.label": "Actions",
  "plugins.demo.config.action.send_test.label": "Send Test Notification"
}
```

An empty object, duplicate keys, or keys without the prefix make the language resources fail to load.

## Saving and Validation

- "Save" on the settings page submits the values of all current groups to `/api/plugin_config/update`, and the backend re-validates them with your `config_class`. On a validation failure it returns an error and writes nothing.
- The frontend only applies control-level range limits; the backend validation is what counts.
- When the plugin is not enabled or has not declared `config_class`, the settings page shows an empty state, and the save endpoint returns an error indicating that the plugin is not enabled or has no config of its own.

## Development Tips

1. Use `Literal[...]` for enumerations; the UI automatically becomes a dropdown.
2. Use `Annotated[str, encrypted()]` for secrets and tokens.
3. Put state that is only for internal use in a `HIDE` group so it does not appear in the UI.
4. Write the conditions for dynamic visibility in `@ui_visibility` methods, keeping them synchronous and free of I/O.
5. Always put UI text in the language files; treat `description` as a code comment only.
