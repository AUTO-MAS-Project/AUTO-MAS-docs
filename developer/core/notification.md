# 通知系统

通知系统是主程序的核心组件，入口为 `from app.core import Notify`。`app/core/notify/__init__.py` 创建全局实例，`app/core/notify/manager.py` 根据全局或账号配置确定投递目标，并组织渲染、发送和结果汇总。业务负责决定发送时机和组织消息内容。用户配置见[推送通知](../../docs/advanced-features/notification.md)。

## 发送通知

在异步业务函数中，先构造通用消息，再选择与业务场景对应的发送入口。

```python
from app.core import Notify
from app.models.notification import NotifyPayload

<通知内容> = NotifyPayload(
    title=<通知标题>,
    text=<纯文本正文>,
)

<分发结果> = await Notify.send_global(
    <通知内容>,
    include_system=<是否纳入系统通知>,
)
```

- **消息参数**

  - `title=`：消息标题。
  - `text=`：可独立阅读的纯文本正文。支持富文本的目标可以使用同一消息中提供的其他正文。

- **投递参数**

  - `<通知内容>`：要发送的 `NotifyPayload`。
  - `include_system=`：默认 `False`。设为 `True` 时，允许将已启用的系统通知渠道纳入本次投递。
  - `<分发结果>`：`DispatchResult`，用于读取实际投递结果。

### 选择发送入口

- **按单个账号的配置发送**

  ```python
  <分发结果> = await Notify.send_user(<通知内容>, <账号配置>)
  ```

  - `<账号配置>`：本次投递所依据的账号配置。宿主按该账号启用的渠道确定收件目标。

- **向业务选定的目标发送**

  ```python
  <分发结果> = await Notify.dispatch(
      payload=<通知内容>,
      targets=<投递目标列表>,
      attempts=<总尝试次数>,
      retry_delay=<重试间隔秒数>,
      skip_channel_ids=<已送达的目标 ID>,
  )
  ```

  - `targets=`：由 `Notify.global_target()`、`Notify.user_target(<账号配置>)` 等入口构造的目标列表。构造目标本身不会发送消息。
  - `attempts=`：包含首次发送在内的总尝试次数，默认 `1`。
  - `retry_delay=`：两次发送尝试之间的秒数，默认 `0`。
  - `skip_channel_ids=`：本次要跳过的稳定目标 ID，默认空集合。补发时可传入上一轮的 `succeeded_ids`。

- **直接发送系统提示或测试 Webhook**

  ```python
  await Notify.send_notice(<本地提示>)
  await Notify.test_webhook(<通知内容>, <Webhook 目标快照>)
  ```

  - `<本地提示>`：`LocalNotice`，由调用方提供标题、正文、提示文本和显示时长。
  - `<Webhook 目标快照>`：`WebhookTargetSnapshot`，包含本次测试使用的 URL、请求方法、模板和请求头等信息。设置页可测试尚未启用的草稿。

## 消息与图片

消息类型定义在 `app/models/notification/`。业务可以同时提供纯文本、Markdown、HTML、摘要和图片。宿主按具体目标的能力选择正文，并处理图片引用。

### 提供正文与摘要

```python
from app.models.notification import NotificationSummary, NotifyPayload

<通知内容> = NotifyPayload(
    title=<通知标题>,
    text=<纯文本正文>,
    markdown=<Markdown 正文>,
    html=<HTML 正文>,
    summary=NotificationSummary(
        text=<紧凑摘要>,
        title=<摘要标题>,
        overflow_text=<正文超限时的备用内容>,
    ),
)
```

- **可选正文**

  - `markdown=`、`html=`：默认 `None`，表示未提供该格式。空字符串会被视为已提供的正文。
  - `append_signature=`：默认 `True`。消息与目标均允许时，渲染器附加统一签名。
  - `body_title=`：可选的正文标题，由目标的标题策略决定是否使用。

- **摘要**

  - `text=`：目标需要紧凑显示时使用的正文。
  - `title=`：目标允许替换标题时使用，默认 `None`。
  - `overflow_text=`：目标启用了超限回退且正文超过字节预算时使用。省略或为空时回退到紧凑摘要。

模板、图片等生成代价较高的可选内容可以通过 `DeferredContent(<内容生成函数>)` 提供。渲染器在目标需要该内容时生成，后续目标和重试复用生成结果。可选正文生成失败时按目标配置的格式顺序尝试下一种表达。

### 在正文中引用图片

```python
from app.models.notification import NotificationImage, image_reference

<图片资源> = NotificationImage(
    id=<图片 ID>,
    data=<图片原始字节>,
    url=<图片 URL>,
    alt=<替代文字>,
    mime_type=<图片 MIME 类型>,
)
<图片引用> = image_reference(<图片 ID>)
```

- **图片资源**

  - `id=`：一条消息中唯一的图片标识。以字母或数字开头，后续可用字母、数字、下划线、点和连字符。
  - `data=`、`url=`：至少提供一项。同时提供时，两者应表示同一张图片。
  - `alt=`、`mime_type=`：默认分别为空字符串和 `"image/png"`。

- **正文引用**

  - 将 `<图片资源>` 加入 `NotifyPayload.images`，在 HTML 的 `img src` 或 Markdown 的图片地址中使用 `<图片引用>`。
  - 邮件可将原始字节作为内嵌图片发送，Markdown 图片使用已有 URL。图片无法按目标能力呈现时，渲染器保留可读文字。
  - 动态账号名、错误信息等插入 HTML 前需要转义。渠道所需的 Base64、MIME 或其他协议格式由渠道实现处理。

## 投递流程

`app/core/notify/manager.py` 负责读取配置和装配发送器。`app/core/notify/content.py` 按目标能力生成内容，`app/core/notify/dispatch.py` 负责投递与汇总。

1. **准备目标**：宿主遍历当前作用域内已启用的渠道，按配置构造 `ChannelTarget`。目标固定本次的接收位置、有效内容能力和所需传输配置。用于补发的目标 ID 和用于展示的名称分别保存。
2. **渲染内容**：按目标声明的格式顺序，从消息提供的正文中选择第一种可用格式，再处理标题、签名、图片和摘要。正文超限时，根据目标策略使用业务提供的备用摘要。渲染失败会记录该目标的失败。
3. **执行发送**：由 `app/core/notify/channels/` 中的适配器完成协议编码和单次发送。可重试的发送失败按调用方设置的次数重试，同一目标复用已渲染内容与目标快照。配置事务内按顺序在同一个执行协程中投递。
4. **汇总结果**：按输入顺序汇总各目标。渠道发现或目标准备失败也会记录在结果中，其他已解析目标继续投递。

### 读取分发结果

- **投递计数**

  - `attempted`：计入本轮投递处理的目标数。重试同一个目标不会增加计数。
  - `succeeded`、`failed`：成功目标和失败项的显示名称。要判断是否有实际送达，读取 `succeeded` 或 `succeeded_ids`。

- **目标标识和失败原因**

  - `succeeded_ids`、`failed_ids`：已解析目标的稳定 ID，可用于去重或补发。
  - `failures`：包含失败阶段、类别、可重试性和实际发送次数等信息。渠道发现和目标准备失败可能没有目标 ID。
  - `queued`：任务通知已经由宿主接管，仍需等待实际投递结果。

- **空收件人策略 | `empty_policy`**

  - `"send"`：继续交给渠道处理。
  - `"warn"`：发出警告并计为失败。
  - `"skip"`：跳过该目标。

## 任务报告与社区通知

任务报告的入口位于 `app/core/notify/manager.py`。报告策略由 `app/core/notify/task_report.py` 固定，`app/core/notify/coordinator.py` 负责等待、合并和补发签到摘要。

```python
<分发结果> = await Notify.push_proxy_result(
    title=<报告标题>,
    report=<任务结果报告>,
    task_info=<任务信息节点>,
)
```

- **任务结果报告**

  - `report=`：`TaskResultReport`，由业务提供开始和结束时间、已完成和未完成账号数、结果正文。专项模板所需的扩展数据可放在 `template_fields`。
  - `task_info=`：当前 `TaskItem`，用于关联本次任务的通知状态。省略时使用现有通知批次上下文。
  - `result_template=`：可选 HTML 模板名，默认 `"general_result.html"`。

- **统计与即时提示**

  - `Notify.push_statistics_report()`：任务内已有统计正文时，按全局和账号统计开关构造目标，并由宿主接管投递和失败反馈。
  - `Notify.send_statistics()`：独立发送统计消息时使用，返回本次分发结果。
  - `Notify.dispatch_task_notification()`：任务内已有消息内容和投递目标时使用，沿用宿主的通知生命周期。

任务报告会在短暂等待窗口内尝试合并签到摘要。若签到稍后才结束，协调器只向尚未收到摘要的目标补发。独立的社区签到通知由 `Notify.community` 处理，使用已有的签到结果。业务继续通过各自入口提交结果，宿主统一处理摘要投递状态。

## 扩展内置渠道

渠道实现放在 `app/core/notify/channels/<渠道 ID>/`。各渠道负责构造自己的目标、声明内容能力、编码协议并执行单次发送。目录中的 `README.md` 记录清单和注册契约。

1. **声明清单**：添加 `manifest.json`，声明接口版本、渠道 ID、注册入口和所需依赖。入口格式见下方示例。
2. **注册渠道**：在渠道包提供 `register() -> NotifyChannel`，声明设置元数据、作用域、启用字段，以及 `target_builder`、`send_adapter` 和 `sender_factory`。注册阶段只装配声明。
3. **构造目标**：`target_builder` 读取宿主提供的 `ChannelConfig`，按显式启用的设置生成具有稳定 ID 的 `ChannelTarget`，并在目标上声明内容能力。
4. **实现单次发送**：通过宿主注入的传输依赖，将 `RenderedNotification` 转换成渠道请求并判断送达结果。返回 `True` 或 `None` 表示成功，返回 `False` 或抛出异常表示失败。协议适配可用 `NotificationDeliveryError` 明确报告可否重试。
5. **更新配置与验证**：增加配置字段或对外接口时，按[配置框架](./config.md)和[前端开发](./api.md)完成接入。验证投递逻辑时，复用 `tests/core/test_notify.py` 的替代发送器，检查启用范围、目标解析、内容选择和失败结果。

```json
{
  "interface_version": 1,
  "id": "<渠道 ID>",
  "entrypoint": "app.core.notify.channels.<渠道 ID>:register",
  "requires": ["<必需的第三方模块名>"]
}
```

- **清单参数**

  - `id`：稳定的渠道标识，与注册声明的 `key` 一致。
  - `entrypoint`：渠道包的注册函数，包目录名与 `id` 相同。
  - `requires`：必须安装的第三方顶层模块名。没有依赖时填空列表。
