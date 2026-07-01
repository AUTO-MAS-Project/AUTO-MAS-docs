# Configuration Schema

This document explains the recommended way to define AUTO-MAS plugin configuration schemas, the backend support scope, and frontend rendering rules.

## Recommended Pattern

Plugin configuration should use Pydantic `BaseModel` with `PluginField`:

```python
from typing import Literal
from pydantic import BaseModel, ConfigDict
from mas.plugin_config import PluginField

class Config(BaseModel):
    model_config = ConfigDict(extra="allow")

    enabled: bool = PluginField(default=True, description="Enable")
    host: str = PluginField(default="127.0.0.1", description="Server address")
    port: int = PluginField(default=8080, description="Port", ge=1, le=65535)
    token: str = PluginField(default="", description="Access token", format="password")
    homepage: str = PluginField(default="", description="Homepage", format="url")
    contact: str = PluginField(default="", description="Email", format="email")
    template: str = PluginField(default="", description="Template", format="textarea", rows=8)
    mode: Literal["always", "failed_only", "never"] = PluginField(
        default="always",
        description="Notification strategy",
    )
```

::: tip Tip

Do not write `Field(format="password")` directly in normal plugin configuration. Pydantic can accept these extra parameters at runtime, but IDEs check against the `Field()` type signature and report "unexpected argument". `PluginField()` is a typed wrapper provided by the plugin system. It converts these UI parameters into standard `json_schema_extra`. Import it from the lightweight module `mas.plugin_config` to avoid triggering heavy main-program initialization during schema loading.

:::

## PluginField Parameters

`PluginField()` supports common Pydantic `Field()` parameters, such as:

- `default`
- `default_factory`
- `title`
- `description`
- `examples`
- `gt`
- `ge`
- `lt`
- `le`
- `multiple_of`
- `min_length`
- `max_length`
- `pattern`

It also supports additional plugin frontend UI parameters:

| Parameter | Type | Purpose |
| --- | --- | --- |
| `format` | `"password" | "url" | "email" | "textarea"` | Specifies a special string input control or validation mode |
| `rows` | `int` | Default row count for `textarea` |
| `placeholder` | `str` | Input placeholder |
| `help` | `str` | Help text shown below the field |
| `ui_type` | `str` | Overrides the frontend control type, such as `key_value` or `table` |
| `item_type` | `str` | Type hint for list or table elements |
| `json_schema_extra` | `dict[str, Any]` | Advanced extension entry point, not recommended for ordinary cases |

## Backend Schema Output

The backend normalizes the `Config` model into a runtime schema dictionary. Common fields are:

| Field | Type | Meaning |
| --- | --- | --- |
| `type` | `str` | Field type |
| `required` | `bool` | Whether the field is required |
| `default` | `Any` | Default value |
| `nullable` | `bool` | Whether `None` is allowed |
| `description` | `str` | Field description |
| `title` | `str` | Field title |
| `examples` | `list` | Example values |
| `constraints` | `dict` | Pydantic constraints, such as `ge`, `le`, and `pattern` |
| `enum` | `list` | Enum values |
| `format` | `str` | Frontend format hint |
| `item_type` | `str` | List element type hint |

## Supported Types

The backend supports the following basic types:

| Schema type | Python type | Frontend rendering |
| --- | --- | --- |
| `string` / `str` | `str` | Single-line input |
| `integer` / `int` | `int` | Number input |
| `number` / `float` | `float` | Number input |
| `boolean` / `bool` | `bool` | Toggle |
| `list` / `list[...]` | `list` | List editor |
| `dict` | `dict[str, Any]` | Default JSON editor |
| `key_value` | `dict[str, Any]` | Key-value table |
| `table` | `list[dict[str, Any]]` | Table editor |

The backend also supports common type expressions such as `Literal[...]`, `Optional[...]`, `T | None`, `dict[str, T]`, and `list[T]`.

## Enums

Using `Literal` is recommended:

```python
mode: Literal["always", "failed_only", "never"] = PluginField(
    default="always",
    description="Notification strategy",
)
```

The backend outputs it as a runtime `enum` array, and the frontend renders it as a dropdown.

Multi-select enum example:

```python
channels: list[Literal["system", "mail", "webhook"]] = PluginField(
    default_factory=lambda: ["system"],
    description="Enabled channels",
)
```

The frontend renders it as a multi-select dropdown.

The plugin runtime schema does not enter the OpenAPI strongly typed generation chain, so enum values in Chinese are safe here. Do not convert plugin configuration fields into OpenAPI strongly typed models, or Chinese enum values may trigger member-name conflicts in the code generator.

## String Formats

Password input:

```python
token: str = PluginField(default="", description="Access token", format="password")
```

Multiline text:

```python
template: str = PluginField(
    default="",
    description="Message template",
    format="textarea",
    rows=8,
)
```

URL:

```python
homepage: str = PluginField(default="", description="Homepage", format="url")
```

Email:

```python
contact: str = PluginField(default="", description="Email", format="email")
```

The frontend provides instant hints for `url`, `email`, `pattern`, length, and number range validation. The backend is still responsible for final trusted validation.

## Key-Value Pairs and Tables

Key-value pairs:

```python
headers: dict[str, str] = PluginField(
    default_factory=lambda: {"Content-Type": "application/json"},
    description="Request headers",
    ui_type="key_value",
)
```

Table:

```python
class WebhookItem(BaseModel):
    model_config = ConfigDict(extra="allow")

    name: str = PluginField(default="Default channel", description="Name")
    enabled: bool = PluginField(default=True, description="Enable")
    url: str = PluginField(default="", description="URL", format="url")


class Config(BaseModel):
    model_config = ConfigDict(extra="allow")

    webhooks: list[WebhookItem] = PluginField(
        default_factory=list,
        description="Webhook list",
        ui_type="table",
        item_type="object",
    )
```

## Current Frontend Support

The current plugin configuration page supports:

- Boolean toggles.
- Number inputs and basic range constraints.
- Single-line strings.
- Password inputs.
- Multiline textareas.
- URL and email format hints.
- `Literal` dropdowns.
- `list[Literal[...]]` multi-select dropdowns.
- Basic list editor.
- Key-value table.
- Object table.
- JSON editor entry point for unrecognized complex types.

## Development Recommendations

1. Prefer `PluginField()` for ordinary plugin configuration fields.
2. Prefer `Literal[...]` for enums.
3. Use `format="password"` for keys, tokens, and passwords.
4. Use `format="url"` and `format="email"` for URLs and emails.
5. Use `format="textarea"` and `rows` for large text.
6. For complex objects without a clear UI, allow the JSON editor first.
7. Frontend validation improves UX but cannot replace backend validation.
