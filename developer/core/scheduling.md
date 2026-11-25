# 调度逻辑

AUTO-MAS 把一次队列运行、直接运行托管或打开配置会话组织成独立任务。主程序负责受理、派发和收尾。专项插件提供托管级准备和账号级执行能力。

## 创建任务

任务受理后立刻返回本次任务的 UUID，实际执行在后台继续。

```python
from app.core import TaskManager
from app.models.task import TaskMode, TaskTriggerReason

<任务 UUID> = await TaskManager.start(
    mode=<运行模式>,
    tasks=<选择树>,
    trigger=<触发原因>,
)
```

- `mode=`：运行模式
  - `TaskMode.AUTO_PROXY`：运行代理任务。
  - `TaskMode.SCRIPT_CONFIG`：打开软件配置会话。
  - `TaskMode.SCRIPT_VIEW`：打开软件查看会话。
- `tasks=`：选择树，用于指定本次执行的目标和顺序。各模式的填法见[选择运行范围](#选择运行范围)。
- `trigger=`：触发原因
  - `TaskTriggerReason.MANUAL`：手动触发，默认值。
  - `TaskTriggerReason.STARTUP`：启动时触发。
  - `TaskTriggerReason.TIMED`：定时触发。
  - `TaskTriggerReason.CYCLE`：循环触发。

### 选择运行范围

选择树顶层有且只有索引 `0`。每层子节点的索引按选择顺序从 `0` 连续递增。

- **节点字段**

  - `uid`：当前节点的标识，具体取值见下列示例。
  - `params`：传给当前节点对应执行层的参数字典，可以省略。
  - `children`：当前节点下选择的子节点。显式选择须保持原配置的顺序和归属。

- **运行队列 | `TaskMode.AUTO_PROXY`**

  ```python
  <选择树> = {
      0: {
          "uid": "<队列 UUID>",
          "children": {
              0: {
                  "uid": "<队列项 UUID>",
                  "params": <托管执行参数>,
                  "children": {0: {"uid": "<账号 UUID>"}},
              },
          },
      },
  }
  ```

  - `<队列 UUID>`：要运行的队列。省略该节点的 `children` 或传入空字典时，展开全部有效队列项。
  - `<队列项 UUID>`：要运行的队列项，由主程序解析为对应的托管。省略该节点的 `children` 或传入空字典时，展开该托管的全部账号。
  - `<托管执行参数>`：传给这次托管执行的可选字典。
  - `<账号 UUID>`：该托管下要运行的账号。

- **直接运行托管 | `TaskMode.AUTO_PROXY`**

  ```python
  <选择树> = {
      0: {
          "uid": "-",
          "children": {
              0: {
                  "uid": "<托管 UUID>",
                  "children": {0: {"uid": "<账号 UUID>"}},
              },
          },
      },
  }
  ```

  - `"-"`：表示直接运行托管。根节点的 `children` 中至少指定一个托管。
  - `<托管 UUID>`：要运行的托管。
  - `<账号 UUID>`：该托管下要运行的账号。托管节点省略 `children` 或传入空字典时，展开全部账号。

- **打开配置或查看会话 | `TaskMode.SCRIPT_CONFIG` / `TaskMode.SCRIPT_VIEW`**

  ```python
  <选择树> = {
      0: {
          "uid": "<托管 UUID>",
          "children": {0: {"uid": "<会话目标>"}},
      },
  }
  ```

  - `<托管 UUID>`：要打开会话的托管。
  - `<会话目标>`：填写该托管下的一个账号 UUID，或者填写 `"default"` 打开共享配置。

## 任务执行流程

一个标准的完整任务执行链有四个层级，L1 完成受理后启动 L2，后续逐级派发至 L3 和 L4。下层通过 `await self.spawn(child)` 执行，父层等待下层完整收尾。停止任务时，取消沿这条等待链传递，`TaskManager.stop(task_id)` 等顶层收尾完成后返回。

### L1 受理并建立运行树

L1 由 `app/core/task_manager.py` 中的 `TaskManager.start()` 和 `_launch()` 实现，负责校验请求、建立本轮运行树并启动执行，该层级不会等待任务执行完成，发布下级任务后就会直接返回。

1. **校验请求**：在短配置事务内检查选择树中的目标、归属和顺序，解析队列项对应的托管，并展开所选账号。
2. **建立运行树**：按本轮范围生成托管和账号节点，写入各节点参数及队列级动作，之后编辑队列不会影响本次运行。
3. **记录触发时间**：自动触发时，在同一事务内写入对应的触发时间；手动运行跳过本步骤。
4. **启动执行**：事务成功后登记运行树并发送创建通知，再启动 L2，向调用方返回本次任务的 UUID。

### L2 派发托管并管理设备

L2 由 `app/task/script_expander.py` 中的 `ScriptExpander` 实现，负责按顺序派发托管、持有托管锁，并管理游戏会话和整轮收尾。

1. **开始本轮执行**：如果配置了队列运行前脚本，先执行该脚本。随后按运行树顺序访问各托管，遇到队列项的星期限制时按当前星期决定是否跳过。
2. **取得托管锁**：确认配置仍是受理时选中的对象，再取得托管本体的排他锁。等待重试时重新查找配置，已删除或替换的节点记为异常。
3. **构造 L3**：持锁后，按当前托管配置类型查找已启用的插件声明，核对任务模式，再用托管运行节点和已锁配置构造展开器。专项插件通过 `ScriptAdapterPlugin.decl` 提供 `ScriptTypeDecl`，在其中填写 `script_type`、`user_type`、`expander_class` 和 `supported_modes`。
4. **取得设备并执行 L3**：宿主按绑定取得游戏实例和设备，再等待 L3 执行。专项插件可以通过 `expect_builder` 提供所需的模拟器配置，设备控制和恢复由[游戏适配插件](../plugins/game-adapters.md)实现。
5. **结束本次托管**：等 L3 收尾后释放游戏会话，再归还托管锁。设备恢复失败会计入托管结果，并保留恢复所需的信息。
6. **结束整轮任务**：托管遍历结束后，执行已配置的队列运行后脚本。收尾时归档历史、按条件交接完成后操作，并发布任务完成消息。

### L3 准备托管并派发账号

L3 的账号派发流程由 `app/task/user_expander.py` 中的 `UserExpander` 实现，专项插件从 `auto_mas_core` 导入并继承 `UserExpander[MyScriptEntry]`，实现托管级准备、账号执行器构造和托管级清理。

1. **检查并准备托管**：先调用插件的 `check()` 检查运行前提，返回非空文本时表示检查未通过，结束这个托管的执行。检查通过后调用 `prepare()`，准备多个账号共用的运行环境或资源。
2. **逐个确认账号**：按运行树顺序重新查找账号并取得级联排他锁。账号被删除或替换时记为异常。取锁后读取最新配置，按运行模式判断准入。不满足自动代理条件的账号记为跳过。需要使用设备时，再按账号绑定选择设备。
3. **构造并执行 L4**：调用插件的 `build_worker(user, config)`，按当前模式构造执行器，并将当前展开器、账号运行节点和已锁配置传给执行器。共享配置 `default` 的 `config` 为 `None`，会话由 L2 的托管锁保护。_以 MAA 为例，插件将 `MaaManager` 注册为 L3，由它构造自动代理、配置或查看执行器。_
4. **释放账号资源**：等待 L4 完成执行和收尾，再释放账号覆盖的设备占用和账号锁，继续派发下一个账号。
5. **完成托管级清理**：调用插件的 `finish()`，完成托管级保存和清理，例如恢复各账号共用的脚本配置。此时托管锁仍由 L2 持有。

### L4 执行单个账号的模式任务

L4 的生命周期由 `app/task/mode_worker.py` 中的 `ModeWorker` 提供，专项插件从 `auto_mas_core` 导入并继承 `AutoProxyWorker`、`ScriptConfigWorker` 或 `ScriptViewWorker`，实现相应模式任务，负责执行单个账号任务或共享级配置会话，并记录结果。

1. **进入执行**：基类核对当前模式和传入的账号配置，将账号运行节点标记为运行中。插件可以在 `check()` 中确认单个账号能否执行，再在 `prepare()` 中准备该账号需要的脚本配置或资源。
2. **执行本次操作**：插件在 `main_logic()` 中调用脚本公开入口，运行自动代理、打开可保存的配置会话，或打开查看会话。
3. **记录明确结果**：按脚本提供的结果更新账号运行状态，可用 `self.user_info.set_state(...)` 提交。
4. **完成账号级收尾**：插件在 `finish()` 中保存配置或恢复本次操作期间修改的资源。正常路径完成收尾时，账号仍停留在「运行」状态会被框架记为异常。L3 等收尾完成后才归还账号锁。

托管和账号两级都遵循[配置框架](./config.md)的提交规则。主流程在产生暂存的协程内提交修改，收尾只提交在收尾阶段自己产生的暂存。

## 自动触发

由位于 `app/core/timer.py` 的**主定时器**负责判断调度队列的自动运行条件。任务可以有启动时、定时和循环三种自动运行条件。条件满足时，主定时器会自动发起任务。同一个队列同时满足多种条件时，每种触发各自提交任务。

对于自动触发的调度任务，L1 在受理事务中自动更新对应调度队列的各条件触发时间记录，用于后续排期判断。即使任务仍在等待执行或之后被取消，记录也会保留。

## 查询状态与结果

任务管理器提供当前运行信息和近期终态，历史服务提供已经归档的执行记录。查询当前任务时使用创建任务返回的任务 UUID，查询历史详情和附件时使用历史服务返回的记录 UUID 或附件 UUID。

### 查询运行任务

后端通过 `from app.core import TaskManager` 使用以下方法。HTTP 入口位于 `app/api/dispatch.py`。

- **读取运行快照**

  ```python
  <运行快照> = TaskManager.get_runtime_snapshot()
  ```

  - `<运行快照>`：`TaskRuntimeSnapshot`，其中 `tasks` 包含当前运行的任务、各自的任务信息树和日志基线。
  - HTTP 使用 `GET /api/dispatch/runtime-snapshot`。调度台在首次连接和重连时读取快照。

- **读取完整任务信息树**

  ```python
  <任务信息树> = TaskManager.get_task(<任务 UUID>)
  ```

  - `<任务 UUID>`：本次运行的 UUID，可传 UUID 对象或字符串。
  - `<任务信息树>`：包含完整节点和日志的字典。先查运行中的任务，再查内存保留的近期终态。两处都找不到时返回 `None`。
  - HTTP 使用 `POST /api/dispatch/get`。请求体含 `taskId` 时查询指定任务，传入空对象 `{}` 时查询当前运行的任务。响应的 `index` 列出任务 UUID，`data` 按任务 UUID 提供对应信息树。

- **读取单次任务的状态摘要**

  ```python
  <任务状态> = TaskManager.get_task_status(<任务 UUID 字符串>)
  ```

  - `<任务 UUID 字符串>`：本次运行的 UUID 字符串。
  - `<任务状态>`：`TaskStatusOut`，包含运行状态、模式、触发原因和结果摘要。未找到指定任务时返回 `None`。
    - `running`：任务已受理或正在运行。
    - `success`：任务成功结束。
    - `error`：任务以异常结束。
    - `cancelled`：任务已取消。
  - HTTP 使用 `GET /api/dispatch/task/{task_id}`，适合轮询状态或读取近期终态。状态摘要省去日志内容，找不到任务时响应的业务 `code` 为 `404`。

### 接收运行更新

调度台的运行数据由 `frontend/src/composables/useTaskRuntimeState.ts` 统一维护。它在首次连接和重连时读取 HTTP 快照，通过主 WebSocket 持续接收变化。

- **发现新任务**

  - `task.created`：以 `TaskManager` 为订阅 ID。收到新任务的 UUID 后，开始跟踪该次运行。

- **跟踪指定任务**

  - `task.info.updated`：以任务 UUID 为订阅 ID，更新任务信息树的状态和结构。
  - `task.log.updated`：以任务 UUID 为订阅 ID，按序号追加或替换日志。缺少日志基线或序号不连续时重新读取快照。
  - `task.notice`：以任务 UUID 为订阅 ID，接收本次运行中的提示。
  - `task.completed`：以任务 UUID 为订阅 ID，接收任务终态和结果摘要。

### 查询历史结果

后端通过 `from app.core import History` 读取持久化历史。历史页面的 HTTP 入口位于 `app/api/history.py`。

- **按日期查询记录索引和统计**

  ```python
  <分组结果> = await History.search(
      start_date=<开始日期>,
      end_date=<结束日期>,
      mode=<分组方式>,
  )
  ```

  - `start_date=`、`end_date=`：传入 `date` 对象，起止日期均包含在查询范围内。
  - `mode=`：指定结果的分组方式。
    - `"DAILY"`：按日分组。
    - `"WEEKLY"`：按周分组，以该周周一的日期作为键。
    - `"MONTHLY"`：按月分组，以该月第一天的日期作为键。
  - `<分组结果>`：按分组日期和账号名称组织，每组的 `index` 列出记录，`data` 提供汇总统计。后续读取详情时使用索引项的 `id`。
  - HTTP 使用 `POST /api/history/search`，在请求体中提供 `start_date`、`end_date` 和 `mode`，日期使用 `YYYY-MM-DD` 字符串。

- **按记录 UUID 读取详情**

  ```python
  <历史详情> = await History.get_detail(record_id=<历史记录 UUID>)
  ```

  - `record_id=`：历史索引项的 `id`，类型为 UUID。
  - `<历史详情>`：`HistoryDetail`，包含执行结果、统计、日志正文或不可用原因，以及附件列表。
  - HTTP 使用 `POST /api/history/detail`，请求体中的 `recordId` 填历史记录 UUID。

- **按附件 UUID 读取文件**

  ```python
  <附件文件> = await History.get_attachment(attachment_id=<附件 UUID>)
  ```

  - `attachment_id=`：历史详情的附件列表中，目标附件的 `id`，类型为 UUID。
  - `<附件文件>`：`HistoryAttachmentFile`，包含附件信息和本地文件路径。
  - HTTP 使用 `GET /api/history/attachments/{attachment_id}` 获取文件。图片和视频默认以内联方式返回，可传入 `download=true` 下载。

- **读取概览使用的统计**

  ```python
  <最近托管结果> = await History.get_latest_script_results(script_names=<托管名称列表>)
  <账号执行概览> = await History.get_overview(day=<查询日期>)
  ```

  - `script_names=`：按托管名称查找最近一条历史记录，返回对应的执行时间和结果状态。托管状态概览通过 `GET /api/info/scripts/status` 同时提供排期配置和最近结果。
  - `day=`：传入 `date` 对象，按账号汇总当天的执行次数、异常记录和最近执行时间。代理概览通过 `POST /api/info/get/overview` 返回。

### 任务信息树

任务信息树定义在 `app/models/task.py`，用于保存本次任务的执行清单、当前状态和日志。受理时，队列项被解析成托管，`TaskItem` 直接持有各次托管执行。树中的节点复用[配置框架](./config.md)的配置对象、有序集合和虚拟字段，在内存中维护。

```text
TaskItem <任务运行 UUID>
└─ scripts | ConfigCollection[TaskScriptItem]
   └─ TaskScriptItem <托管运行 UUID>
      └─ users | ConfigCollection[TaskUserItem]
         └─ TaskUserItem <账号运行 UUID>
            └─ logs | ConfigCollection[LogRecord]
               └─ LogRecord <日志记录 UUID>
```

- **任务 | `TaskItem`**

  - `info`：保存本次任务的名称、运行模式、触发原因和整轮状态。
  - `execution`：保存受理时确定的队列运行前脚本、运行后脚本、完成后操作和延迟时间，供本轮执行使用。
  - `current.index`：当前托管运行节点的 UUID，可以用它在 `scripts` 中查找该节点。尚未选定托管时为 `None`。

- **托管执行 | `TaskScriptItem`**

  - `info`：保存原托管的 `script_id`、专项类型、名称、本次执行状态和传入该节点的参数。
  - `current.index`：当前账号运行节点的 UUID，可以用它在 `users` 中查找该节点。尚未选定账号时为 `None`。
  - `current.log`：托管级流程日志，例如等待配置锁或设备时的提示。

- **账号执行 | `TaskUserItem`**

  - `info`：保存原账号的 `user_id`、名称、本次执行状态和传入该节点的参数。打开共享配置时，`user_id` 为 `"default"`。
  - `logs`：本次账号执行产生的日志记录集合，可以包含多条记录。

- **日志记录 | `LogRecord`**

  - `info.start_time`、`info.phase`、`info.status`：记录的开始时间、阶段和状态说明。
  - `info.content`：按行保存的日志正文，使用 `append_content()` 追加。

`scripts`、`users` 和 `logs` 都用运行节点的 `uid` 索引。回查原配置时使用 `info.script_id` 或 `info.user_id`。例如，同一个托管在队列中出现两次，会生成两个 `TaskScriptItem`。它们的 `info.script_id` 相同，各自有独立的 `uid`、状态和日志。

`info.status` 由执行层更新。`info.result` 是根据下层信息计算的摘要：账号从日志记录生成结果，托管汇总账号结果，任务再汇总各托管结果与账号完成数。专项插件执行链中，任务根的 `current.log` 根据两级 `current.index` 选择当前账号最近一条日志。已选定托管且账号尚无日志时，显示该托管的流程日志。
