# 配置声明

本文说明 AUTO-MAS 插件配置 Schema 的推荐写法、后端支持范围和前端渲染规则。

## 推荐写法

插件配置推荐使用 Pydantic `BaseModel` 加 `PluginField`：

```python
from typing import Literal
from pydantic import BaseModel, ConfigDict
from mas.plugin_config import PluginField

class Config(BaseModel):
    model_config = ConfigDict(extra="allow")

    enabled: bool = PluginField(default=True, description="启用")
    host: str = PluginField(default="127.0.0.1", description="服务器地址")
    port: int = PluginField(default=8080, description="端口", ge=1, le=65535)
    token: str = PluginField(default="", description="访问令牌", format="password")
    homepage: str = PluginField(default="", description="主页", format="url")
    contact: str = PluginField(default="", description="邮箱", format="email")
    template: str = PluginField(default="", description="模板", format="textarea", rows=8)
    mode: Literal["always", "failed_only", "never"] = PluginField(
        default="always",
        description="通知策略",
    )
```

::: tip 小提示

不建议在普通插件配置里直接写 `Field(format="password")`。Pydantic 运行时虽然能接收这类额外参数，但 IDE 会按 `Field()` 类型签名检查，并提示“意外的实参”。`PluginField()` 是插件系统提供的 typed wrapper，会把这些 UI 参数转换成标准 `json_schema_extra`。推荐从轻量模块 `mas.plugin_config` 导入，避免 schema 加载时触发主程序重型初始化。

:::

## PluginField 参数

`PluginField()` 支持 Pydantic `Field()` 的常用参数，例如：

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

同时额外支持插件前端 UI 参数：

| 参数 | 类型 | 作用 |
| --- | --- | --- |
| `format` | `"password" | "url" | "email" | "textarea"` | 指定字符串字段的专用输入控件或校验方式 |
| `rows` | `int` | `textarea` 默认行数 |
| `placeholder` | `str` | 输入框占位提示 |
| `help` | `str` | 字段底部帮助说明 |
| `ui_type` | `str` | 覆盖前端控件类型，例如 `key_value`、`table` |
| `item_type` | `str` | 列表或表格元素类型提示 |
| `json_schema_extra` | `dict[str, Any]` | 高级扩展入口，普通场景不推荐直接使用 |

## 后端 Schema 输出

后端会把 `Config` 模型归一化成运行时 schema 字典。常见字段如下：

| 字段 | 类型 | 含义 |
| --- | --- | --- |
| `type` | `str` | 字段类型 |
| `required` | `bool` | 是否必填 |
| `default` | `Any` | 默认值 |
| `nullable` | `bool` | 是否允许 `None` |
| `description` | `str` | 字段说明 |
| `title` | `str` | 字段标题 |
| `examples` | `list` | 示例值 |
| `constraints` | `dict` | Pydantic 约束，例如 `ge`、`le`、`pattern` |
| `enum` | `list` | 枚举值 |
| `format` | `str` | 前端格式提示 |
| `item_type` | `str` | 列表元素类型提示 |

## 类型支持

后端支持以下基础类型：

| Schema type | Python 类型 | 前端渲染 |
| --- | --- | --- |
| `string` / `str` | `str` | 单行输入框 |
| `integer` / `int` | `int` | 数字输入框 |
| `number` / `float` | `float` | 数字输入框 |
| `boolean` / `bool` | `bool` | 开关 |
| `list` / `list[...]` | `list` | 列表编辑器 |
| `dict` | `dict[str, Any]` | 默认 JSON 编辑 |
| `key_value` | `dict[str, Any]` | 键值表格 |
| `table` | `list[dict[str, Any]]` | 表格编辑器 |

后端还支持 `Literal[...]`、`Optional[...]`、`T | None`、`dict[str, T]`、`list[T]` 等常见类型表达式。

## 枚举

推荐使用 `Literal`：

```python
mode: Literal["always", "failed_only", "never"] = PluginField(
    default="always",
    description="通知策略",
)
```

后端会把它输出为运行时 `enum` 数组，前端会渲染为下拉框。

多选枚举写法：

```python
channels: list[Literal["system", "mail", "webhook"]] = PluginField(
    default_factory=lambda: ["system"],
    description="启用渠道",
)
```

前端会渲染为多选下拉框。

插件运行时 schema 不进入 OpenAPI 强类型生成链路，因此这里可以安全使用中文枚举值。不要把插件配置字段改造成 OpenAPI 强类型模型，否则中文 enum 可能触发代码生成器的成员名冲突问题。

## 字符串格式

密码框：

```python
token: str = PluginField(default="", description="访问令牌", format="password")
```

多行文本：

```python
template: str = PluginField(
    default="",
    description="消息模板",
    format="textarea",
    rows=8,
)
```

URL：

```python
homepage: str = PluginField(default="", description="主页", format="url")
```

邮箱：

```python
contact: str = PluginField(default="", description="邮箱", format="email")
```

前端会对 `url`、`email`、`pattern`、长度和数字范围做即时提示；后端仍负责最终可信校验。

## 键值对和表格

键值对：

```python
headers: dict[str, str] = PluginField(
    default_factory=lambda: {"Content-Type": "application/json"},
    description="请求头",
    ui_type="key_value",
)
```

表格：

```python
class WebhookItem(BaseModel):
    model_config = ConfigDict(extra="allow")

    name: str = PluginField(default="默认通道", description="名称")
    enabled: bool = PluginField(default=True, description="启用")
    url: str = PluginField(default="", description="URL", format="url")


class Config(BaseModel):
    model_config = ConfigDict(extra="allow")

    webhooks: list[WebhookItem] = PluginField(
        default_factory=list,
        description="Webhook 列表",
        ui_type="table",
        item_type="object",
    )
```

## 前端当前支持

当前插件配置页支持：

- 布尔开关。
- 数字输入和基础范围约束。
- 单行字符串。
- 密码框。
- 多行文本框。
- URL 和邮箱格式提示。
- `Literal` 下拉框。
- `list[Literal[...]]` 多选下拉框。
- 基础列表编辑器。
- 键值对表格。
- 对象表格。
- 未识别复杂类型的 JSON 编辑入口。

## 开发建议

1. 普通插件配置字段优先使用 `PluginField()`。
2. 枚举优先使用 `Literal[...]`。
3. 密钥、令牌、密码使用 `format="password"`。
4. URL 和邮箱使用 `format="url"`、`format="email"`。
5. 大文本使用 `format="textarea"` 和 `rows`。
6. 复杂对象如果没有明确 UI，允许先走 JSON 编辑入口。
7. 前端校验只提升体验，不能替代后端校验。
