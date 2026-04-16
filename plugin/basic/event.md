# 事件系统

事件系统是MAS插件系统的重要组成部分，可以说没有事件系统就没有插件系统。

本文将带你了解MAS事件系统的构造和使用，帮助你快速编写一个使用事件系统的插件。

## 事件模型

### 事件对象结构

标准事件对象包含以下结构：

```json
{
  "event": "task.start",
  "event_version": "1",
  "source": "core.task_manager",
  "timestamp": "2026-03-22T01:23:45+08:00",
  "data": {
    "task_id": "task-001"
  }
}
```

字段分层说明：

- 元信息层（顶层）：event、event_version、source、timestamp。
- 业务数据层（data）：业务侧字段，按事件类型变化。

### task.* 与 script.* 的关键差异

task.* 事件通常把任务字段放在 data 内；script.* 事件会把脚本上下文字段提升到顶层。

script.exit 示例：

```json
{
  "event": "script.exit",
  "event_version": "1",
  "source": "core.task_manager",
  "timestamp": "2026-03-22T01:23:59+08:00",
  "task_id": "task-001",
  "script_id": "script-001",
  "script_name": "日常任务",
  "mode": "AutoProxy",
  "status": "完成",
  "error": null,
  "result": "script.success",
  "data": {
    "queue_id": "queue-001",
    "queue_name": "每日轮询"
  }
}
```

开发建议：

- 处理 task.* 时，优先从 payload["data"] 读取 task_id 等字段。
- 处理 script.* 时，优先从 payload 顶层读取 task_id、script_id、script_name、result。

## 系统提供的标准事件

### 任务维度事件

- task.start：任务启动。
- task.progress：任务进度快照。
- task.log：当前脚本日志快照。
- task.exit：任务结束收口。

### 脚本维度事件

- script.start：脚本开始。
- script.success：脚本成功完成。
- script.error：脚本失败。
- script.cancelled：脚本被取消。
- script.exit：脚本生命周期收口事件（推荐优先监听）。

::: tip 有什么区别？

task是script的集合，一个task任务中可以包含多个script。

比如说我创建一个MAA任务（task），有两个账号，每个账号都是一个script

:::

推荐策略：

- 需要对每个脚本做统一收口时，优先监听 script.exit，再根据 result 分支处理。

## 监听器声明与签名规范

### 装饰器参数

示例：

```python
from app.core.plugins import on_event

@on_event("script.exit", scope="global", priority=0, once=False)
async def on_script_exit(payload, ctx):
    ...
```

参数说明：

- event：事件名，不能为空。
- scope：global 或 instance。
- priority：数值越大，越先执行。
- once：触发一次后自动解绑。
- error_policy：continue 或 raise，None 表示继承事件级策略。

### 处理函数允许的签名

加载器会自动注入上下文，建议使用以下任一形式：

- handler(payload)
- handler(payload, ctx)
- handler(payload, *, ctx=...)

如果签名不匹配，会在注册阶段报错。

### 同步监听器注意事项

事件总线对同步函数会放到线程执行。若你在同步监听器内直接调用 asyncio.create_task，常见报错为 no running event loop。

建议：

- 需要创建异步任务时，将监听器定义为 async def。

## 作用域、优先级、错误策略

### 作用域路由

- 事件以 global 广播时，只会命中 scope=global 的监听器。
- 事件以 instance 广播时，只会命中同一实例的 scope=instance 监听器。

核心事件（如 task.*、script.*）默认按 global 发送，因此插件监听核心事件时通常应使用 scope="global"。

### 执行顺序

- 先按 priority 从高到低分组。
- 同优先级内并发执行。
- 当前优先级组完成后，再执行下一组。

### 错误策略

- continue：记录异常后继续分发。
- raise：该监听器异常会向上抛出并参与分发异常聚合。

实践建议：

- 插件内优先自行捕获异常，避免因策略或版本差异影响事件流稳定性。

## payload 读取范式

### task.* 读取范式

```python
def read_task_payload(payload: dict) -> tuple[str, str]:
    data = payload.get("data") if isinstance(payload, dict) else None
    if not isinstance(data, dict):
        return "", ""
    task_id = str(data.get("task_id") or "").strip()
    mode = str(data.get("mode") or "").strip()
    return task_id, mode
```

### script.* 读取范式

```python
def read_script_payload(payload: dict) -> tuple[str, str, str]:
    if not isinstance(payload, dict):
        return "", "", ""
    script_name = str(payload.get("script_name") or "").strip()
    script_id = str(payload.get("script_id") or "").strip()
    result = str(payload.get("result") or "").strip()
    return script_name, script_id, result
```

### 常见字段错位问题

- 错误方式：在 script.exit 的 data 中找 script_name。
- 正确方式：先读顶层 script_name、script_id、result，再读 data 扩展字段。

## 最小可用实现模板

```python
from typing import Any
from app.core.plugins import on_event


class Plugin:
    def __init__(self, ctx):
        self.ctx = ctx
        self._seen: set[tuple[str, str, str]] = set()

    @on_event("script.exit", scope="global")
    async def on_script_exit(self, payload: Any, ctx) -> None:
        _ = ctx
        try:
            if not isinstance(payload, dict):
                return

            script_name = str(payload.get("script_name") or "").strip()
            task_id = str(payload.get("task_id") or "").strip()
            script_id = str(payload.get("script_id") or "").strip()
            result = str(payload.get("result") or "").strip()

            if not script_name:
                return

            key = (task_id, script_id, script_name)
            if key in self._seen:
                return
            self._seen.add(key)

            if result == "script.success":
                self.ctx.logger.info(f"脚本成功: {script_name}")
            elif result == "script.error":
                self.ctx.logger.warning(f"脚本失败: {script_name}")
            elif result == "script.cancelled":
                self.ctx.logger.info(f"脚本取消: {script_name}")

        except Exception as e:
            self.ctx.logger.warning(f"处理 script.exit 失败: {type(e).__name__}: {e}")
```

## 调试与验证流程

- 确认插件实例已加载且状态正常。
- 确认监听器事件名完全匹配（如 script.exit）。
- 确认 scope 设置正确（监听核心事件应为 global）。
- 在处理函数第一行打印 payload 结构，核对字段层级。
- 对 result 做完整分支（success/error/cancelled），避免漏处理。
- 增加去重签名，避免重复事件引起副作用。
- 对处理逻辑加总兜底异常捕获，保证总线稳定。

## 常见问题

### 监听器没有触发

可能原因：

- event 名拼写错误。
- 监听器用的是 scope="instance"，但核心事件按 global 发出。
- 函数签名不合法导致注册失败。

### payload 字段为空

可能原因：

- 把 task.* 当成 script.* 读取，或反过来。
- 未做 isinstance(payload, dict) 与 data 字段判空。

### 监听器内部起异步任务报错

典型报错：no running event loop。

原因：

- 监听器是同步函数，实际在线程执行。

处理：

- 将监听器改成 async def，再创建后台任务。

## 事件样例

### task.start

```json
{
  "event": "task.start",
  "event_version": "1",
  "source": "core.task_manager",
  "timestamp": "2026-03-22T01:23:45+08:00",
  "data": {
    "task_id": "task-001",
    "mode": "AutoProxy",
    "queue_id": "queue-001",
    "queue_name": "每日轮询",
    "script_total": 3,
    "scripts": [
      {
        "script_id": "script-001",
        "script_name": "日常任务",
        "status": "等待"
      }
    ],
    "primary_script_id": null,
    "primary_script_name": null,
    "actions": {
      "stop_task": {
        "api": "/api/dispatch/stop",
        "method": "POST",
        "body": {
          "taskId": "task-001"
        }
      },
      "stop_all_tasks": {
        "api": "/api/dispatch/stop",
        "method": "POST",
        "body": {
          "taskId": "ALL"
        }
      }
    }
  }
}
```

### task.progress

```json
{
  "event": "task.progress",
  "event_version": "1",
  "source": "core.task_manager",
  "timestamp": "2026-03-22T01:23:50+08:00",
  "data": {
    "task_id": "task-001",
    "mode": "AutoProxy",
    "queue_id": "queue-001",
    "queue_name": "每日轮询",
    "current_script_index": 0,
    "current_script_id": "script-001",
    "current_script_name": "日常任务",
    "script_total": 3,
    "script_completed": 1,
    "user_total": 12,
    "user_completed": 4,
    "current_script": {
      "script_id": "script-001",
      "script_name": "日常任务",
      "status": "运行",
      "current_user_index": 1,
      "user_count": 4
    }
  }
}
```

### task.log

```json
{
  "event": "task.log",
  "event_version": "1",
  "source": "core.task_manager",
  "timestamp": "2026-03-22T01:23:51+08:00",
  "data": {
    "task_id": "task-001",
    "mode": "AutoProxy",
    "queue_id": "queue-001",
    "queue_name": "每日轮询",
    "script_id": "script-001",
    "script_name": "日常任务",
    "script_status": "运行",
    "current_script_index": 0,
    "log": "...完整日志...",
    "log_tail": "...末尾日志...",
    "log_length": 12345,
    "truncated_for_tail": true
  }
}
```

### task.exit

```json
{
  "event": "task.exit",
  "event_version": "1",
  "source": "core.task_manager",
  "timestamp": "2026-03-22T01:24:20+08:00",
  "data": {
    "task_id": "task-001",
    "mode": "AutoProxy",
    "queue_id": "queue-001",
    "queue_name": "每日轮询",
    "scripts": [
      {
        "script_id": "script-001",
        "script_name": "日常任务",
        "status": "完成"
      }
    ],
    "final_script_id": "script-001",
    "final_script_name": "日常任务",
    "final_script_status": "完成",
    "result": "success",
    "error": null,
    "summary": "任务摘要..."
  }
}
```

### script.exit

```json
{
  "event": "script.exit",
  "event_version": "1",
  "source": "core.task_manager",
  "timestamp": "2026-03-22T01:23:59+08:00",
  "task_id": "task-001",
  "script_id": "script-001",
  "script_name": "日常任务",
  "mode": "AutoProxy",
  "status": "完成",
  "error": null,
  "result": "script.success",
  "data": {
    "queue_id": "queue-001",
    "queue_name": "每日轮询"
  }
}
```
