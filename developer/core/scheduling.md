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
  - `TaskMode.AUTO_PROXY`：运行代理任务
  - `TaskMode.SCRIPT_CONFIG`：打开软件配置会话
  - `TaskMode.SCRIPT_VIEW` 打开软件查看会话。
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

后端执行链已持有任务节点时，可以按[配置框架](./config.md#配置节点-node)直接读取节点的字段和子集合。前端通过本节的 HTTP 接口获取运行状态和结果。

### 任务信息树

任务信息树定义在 `app/models/task.py`，保存单次任务的执行清单、状态、结果和日志。各节点使用配置对象和有序集合，任务根直接包含本次执行的托管。

```text
TaskItem <任务 UUID>
└─ scripts | ConfigCollection[TaskScriptItem]
   └─ TaskScriptItem <托管运行 UUID>
      └─ users | ConfigCollection[TaskUserItem]
         └─ TaskUserItem <账号运行 UUID>
            └─ logs | ConfigCollection[LogRecord]
               └─ LogRecord <日志记录 UUID>
```

- **任务 | `TaskItem`**
  - `info`：本次任务的名称、模式、触发原因和整体状态。
  - `execution`：受理时确定的运行前后脚本和完成后操作。
  - `current.index`：当前托管的运行节点 UUID，尚未选定时为 `None`。
  - `current.log`：当前正在运行的脚本日志，
- **托管执行 | `TaskScriptItem`**
  - `info`：原托管 `script_id`、专项类型、名称、本次执行状态和参数。
  - `current.index`：当前账号的运行节点 UUID，尚未选定时为 `None`。
  - `current.log`：托管级流程日志，例如等待配置锁或设备的提示。
- **账号执行 | `TaskUserItem`**
  - `info`：原账号 `user_id`、名称、本次执行状态和参数。共享配置会话使用 `"default"`。
  - `logs`：本次账号执行产生的日志记录集合。
- **日志记录 | `LogRecord`**
  - `info`：日志记录的开始时间、阶段和状态说明。
  - `info.content`：按行保存的日志正文。

`info.result` 是逐级计算的运行结果摘要，账号汇总日志记录，托管汇总账号结果，任务根汇总托管结果和账号完成数。

### 任务信息接口

接口定义在 `app/api/dispatch.py`。指定任务时使用创建任务接口返回的 `taskId`。完整任务树和状态摘要均可查询内存中保留的近期终态，最多保留 200 条，单条自任务结束起最多保留 30 分钟。

#### 读取完整任务信息树

`POST /api/dispatch/get`

**请求体 | `TaskGetIn`**

```json
{ "taskId": "<任务 UUID>" }
```

- `taskId`：可选，省略或传入 `null` 表示查询当前运行的所有任务

  - `<任务 UUID>`：查询该任务的完整信息树，含各节点的日志正文。

**返回体 | `TaskGetOut`**

- `index`：查询结果的索引，每项的 `uid` 是任务 UUID。
- `data`：以任务 UUID 为键，值为对应的任务信息树。
- `code`：业务状态码。
  - `200`：查询成功。
  - `400`：任务 UUID 格式错误。
  - `404`：指定的任务已不在运行列表和近期终态中。

#### 读取单个任务状态摘要

`GET /api/dispatch/task/<任务 UUID>`

**请求参数**

- `<任务 UUID>`：必填，指示要查询任务。

**返回体 | `TaskStatusOut`**

- `taskId`：本次查询的任务 UUID。
- `status`：本次任务的状态。
  - `running`：任务已受理或正在运行。树中的 `pending` 也映射为此值。
  - `success`：任务成功结束。
  - `error`：任务以异常结束。查无任务时也返回此值，通过 `code` 区分。
  - `cancelled`：任务已取消。
- `mode`、`trigger`：任务的运行模式和触发原因。
- `detail`：任务树根节点的结果摘要。
- `code`：业务状态码。
  - `200`：查询成功。
  - `404`：找不到该任务。

#### 读取当前任务快照

调度台在首次连接或恢复连接时使用该接口取得当前运行任务和日志基线。

`GET /api/dispatch/runtime-snapshot`

**请求参数**

无。

**返回体 | `TaskRuntimeSnapshot`**

- `tasks`：当前运行任务的快照列表，每项的主要字段如下
  - `taskId`：任务 UUID。
  - `task`：完整的任务信息树。
  - `mode`、`trigger`：任务的运行模式和触发原因。
  - `stopping`：任务是否正在停止。
  - `scripts`：任务关联的托管列表，每项包含原托管 UUID `scriptId` 和专项类型 `scriptType`。
  - `log`：当前日志正文。当前账号已有日志时可取其最近一条日志记录，也可显示当前托管的流程日志。
  - `logSeq`：当前日志的基线序号，`null` 表示当前正文无法作为追加基线。
  - `logFirstLine`：当前日志在来源中的起始行号。
