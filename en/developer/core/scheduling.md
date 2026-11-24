# Task Scheduling

AUTO-MAS treats a queue run, a direct managed-script run, or a configuration session as an independent task. The host accepts, dispatches, and finalizes tasks. Specialized plugins prepare shared resources for a managed script and execute individual accounts.

## Create a Task

Once accepted, a task immediately returns its UUID and continues executing in the background.

```python
from app.core import TaskManager
from app.models.task import TaskMode, TaskTriggerReason

<task UUID> = await TaskManager.start(
    mode=<execution mode>,
    tasks=<selection tree>,
    trigger=<trigger reason>,
)
```

- `mode=`
  - `TaskMode.AUTO_PROXY` runs an automated task.
  - `TaskMode.SCRIPT_CONFIG` opens a configuration session.
  - `TaskMode.SCRIPT_VIEW` opens a view session.
- `tasks=` defines the targets and order for this run. See [Select the Run Scope](#select-the-run-scope) for each mode's selection tree.
- `trigger=`
  - `TaskTriggerReason.MANUAL` is the default and indicates a manual start.
  - `TaskTriggerReason.STARTUP` indicates an app-startup trigger.
  - `TaskTriggerReason.TIMED` indicates a scheduled trigger.
  - `TaskTriggerReason.CYCLE` indicates a recurring interval trigger.

### Select the Run Scope

The top level of the selection tree contains exactly one key, `0`. At each level, child keys begin at `0` and increase consecutively in selection order.

- **Node fields**

  - `uid` identifies the node. The examples show what it represents for each scope.
  - `params` is an optional dictionary passed to the execution level that handles the node.
  - `children` selects the child nodes. Explicit selections must preserve their configured order and parent relationships.

- **Run a queue | `TaskMode.AUTO_PROXY`**

  ```python
  <selection tree> = {
      0: {
          "uid": "<queue UUID>",
          "children": {
              0: {
                  "uid": "<queue item UUID>",
                  "params": <managed script execution parameters>,
                  "children": {0: {"uid": "<account UUID>"}},
              },
          },
      },
  }
  ```

  - `<queue UUID>` identifies the queue. Omit its `children` or pass an empty dictionary to select all eligible queue items.
  - `<queue item UUID>` identifies a queue item. The host resolves it to the corresponding managed script. Omit its `children` or pass an empty dictionary to select all accounts for that managed script.
  - `<managed script execution parameters>` is an optional dictionary for this execution.
  - `<account UUID>` identifies an account belonging to the managed script.

- **Run managed scripts directly | `TaskMode.AUTO_PROXY`**

  ```python
  <selection tree> = {
      0: {
          "uid": "-",
          "children": {
              0: {
                  "uid": "<managed script UUID>",
                  "children": {0: {"uid": "<account UUID>"}},
              },
          },
      },
  }
  ```

  - `"-"` selects a direct run. The root `children` must specify at least one managed script.
  - `<managed script UUID>` identifies the managed script to run.
  - `<account UUID>` identifies an account belonging to that managed script. Omit the script node's `children` or pass an empty dictionary to select all its accounts.

- **Open a configuration or view session | `TaskMode.SCRIPT_CONFIG` / `TaskMode.SCRIPT_VIEW`**

  ```python
  <selection tree> = {
      0: {
          "uid": "<managed script UUID>",
          "children": {0: {"uid": "<session target>"}},
      },
  }
  ```

  - `<managed script UUID>` identifies the managed script for the session.
  - `<session target>` is one account UUID from that managed script, or `"default"` for the shared configuration.

## Task Execution Flow

The full execution chain has four levels. L1 accepts the task and starts L2, which dispatches L3, and L3 dispatches L4. Each parent runs its child with `await self.spawn(child)` and waits for the child's cleanup. Cancellation propagates along this await chain. `TaskManager.stop(task_id)` returns after top-level cleanup finishes.

### L1 Accept the Task and Build the Runtime Tree

`TaskManager.start()` and `_launch()` in `app/core/task_manager.py` validate the request, build the runtime tree, and start execution. L1 returns once it launches L2. The run continues in the background.

1. **Validate the request.** In a short configuration transaction, check the selected targets, their ownership, and their order. Resolve queue items to managed scripts and expand the account selection.
2. **Build the runtime tree.** Create managed-script and account nodes for the selected scope, copying node parameters and queue-level actions. Later queue edits affect subsequent runs.
3. **Record the trigger time.** Write the corresponding timestamp in the same transaction for an automatic trigger. Skip this step for a manual start.
4. **Start execution.** After the transaction succeeds, register the runtime tree, publish the task creation event, start L2, and return the task UUID.

### L2 Dispatch Managed Scripts and Manage Devices

`ScriptExpander` in `app/task/script_expander.py` dispatches managed scripts in order, holds their locks, manages game sessions, and finalizes the overall run.

1. **Start the run.** Run the queue's pre-run script if configured. Then visit managed scripts in runtime tree order, using the current weekday to evaluate any queue-item day restrictions.
2. **Acquire the managed-script lock.** Check that the configuration is still the object selected at acceptance, then take an exclusive lock on the managed script itself. Resolve the configuration again on each retry. Mark the runtime node as failed if the configuration has been deleted or replaced.
3. **Construct L3.** With the lock held, find the enabled plugin declaration for the configuration type, verify the mode, and construct the expander with the runtime node and locked configuration. The plugin provides a `ScriptTypeDecl` through `ScriptAdapterPlugin.decl`, using `script_type`, `user_type`, `expander_class`, and `supported_modes`.
4. **Acquire the device and run L3.** The host acquires the game instance and device from the binding, then waits for L3. The specialized plugin can provide emulator requirements through `expect_builder`. The [game adapter plugin](../plugins/game-adapters.md) implements device control and restoration.
5. **Finish the managed-script run.** Once L3 finishes cleanup, release the game session and then the managed-script lock. Device restoration failures affect the managed-script result, and information needed for a future restoration attempt is retained.
6. **Finish the overall run.** After traversing the managed scripts, run the queue's post-run script if configured. During cleanup, archive execution history, hand off any applicable completion action, and publish the task completion event.

### L3 Prepare the Managed Script and Dispatch Accounts

`UserExpander` in `app/task/user_expander.py` provides account dispatch. A specialized plugin imports it from `auto_mas_core` and subclasses `UserExpander[MyScriptEntry]` to implement shared preparation, account-worker construction, and managed-script cleanup.

1. **Check and prepare the managed script.** Call the plugin's `check()` to verify prerequisites. A nonempty message ends this managed-script run. If the check passes, call `prepare()` to set up the environment or resources shared by the accounts.
2. **Validate each account.** Resolve the account again in runtime tree order and take its cascading exclusive lock. Mark a deleted or replaced account as failed. Under the lock, read the latest configuration and check eligibility for the current mode. Skip accounts that are ineligible for automatic execution. Select the account's bound device when required.
3. **Construct and run L4.** Call the plugin's `build_worker(user, config)`. The worker receives the current expander, the account runtime node, and the locked configuration. For the shared `default` configuration, `config` is `None` and L2's managed-script lock protects the session. _For example, the MAA plugin registers `MaaManager` as L3. It constructs the automatic, configuration, and view workers._
4. **Release account resources.** Wait for L4 to execute and finish cleanup, then release any account-specific device reservation and the account lock before moving on.
5. **Complete managed-script cleanup.** Call the plugin's `finish()` to save and clean up shared state, such as restoring configuration used by several accounts. L2 still holds the managed-script lock.

### L4 Execute an Account in the Selected Mode

`ModeWorker` in `app/task/mode_worker.py` provides the lifecycle. Specialized plugins import and subclass `AutoProxyWorker`, `ScriptConfigWorker`, or `ScriptViewWorker` from `auto_mas_core` to execute one account or shared configuration session and record the outcome.

1. **Enter execution.** The base class checks the mode and the supplied account configuration and marks the runtime node as running. The plugin can use `check()` to verify that the account can run and `prepare()` to set up account-specific configuration or resources.
2. **Perform the operation.** In `main_logic()`, call the script's public entry point to run automation, open a configuration session that can save changes, or open a view session.
3. **Record the outcome.** Update the account's runtime status based on the script's result. Use `self.user_info.set_state(...)` to commit it.
4. **Finish account cleanup.** In `finish()`, save configuration or restore resources modified during the operation. If normal cleanup completes while the account still shows as running, the framework records a failure. L3 releases the account lock after cleanup.

Both the managed-script and account levels follow the [configuration framework](./config.md) commit rules. Commit staged changes in the coroutine that staged them. Cleanup only commits changes staged during cleanup.

## Automatic Triggers

The **main timer** in `app/core/timer.py` checks whether a queue is due on startup, at a scheduled time, or at a recurring interval. When a condition is met, the timer starts a task. If multiple conditions apply to one queue, each starts a separate task.

For an automatically triggered task, L1 records the corresponding queue trigger time in the acceptance transaction. That timestamp is used for later scheduling decisions and remains recorded if the task is still waiting or is later cancelled.

## Query Status and Results

The task manager provides running task data and recent terminal states. The history service provides archived execution records. Use the UUID returned when the task was created to query a run. Use the record or attachment UUID returned by the history service to query archived details and files.

### Query Running Tasks

Import `TaskManager` with `from app.core import TaskManager`. The HTTP routes are in `app/api/dispatch.py`.

- **Read the runtime snapshot**

  ```python
  <runtime snapshot> = TaskManager.get_runtime_snapshot()
  ```

  - `<runtime snapshot>` is a `TaskRuntimeSnapshot`. Its `tasks` field holds the active tasks, their task information trees, and log baselines.
  - Use `GET /api/dispatch/runtime-snapshot` over HTTP. The scheduler reads the snapshot on its first connection and when reconnecting.

- **Read the complete task information tree**

  ```python
  <task information tree> = TaskManager.get_task(<task UUID>)
  ```

  - `<task UUID>` identifies this run. Pass a UUID object or a string.
  - `<task information tree>` is a dictionary containing all nodes and logs. The task manager checks active runs first, then recent terminal states retained in memory. It returns `None` if neither contains the task.
  - Use `POST /api/dispatch/get` over HTTP. Specify `taskId` in the body to query one task, or send `{}` for the active tasks. The response's `index` lists UUIDs, and `data` maps each UUID to its information tree.

- **Read a task status summary**

  ```python
  <task status> = TaskManager.get_task_status(<task UUID string>)
  ```

  - `<task UUID string>` is the run UUID as a string.
  - `<task status>` is a `TaskStatusOut` containing status, mode, trigger reason, and a result summary. The method returns `None` if the task cannot be found.
  - `running` means accepted or running.
  - `success` means the task completed successfully.
  - `error` means the task failed.
  - `cancelled` means the task was cancelled.
  - Use `GET /api/dispatch/task/{task_id}` to poll status or read a recent terminal state. The summary omits logs. An unknown task produces application `code` `404`.

### Receive Runtime Updates

`frontend/src/composables/useTaskRuntimeState.ts` manages the scheduler's runtime data. It reads an HTTP snapshot on connection or reconnection and receives changes through the main WebSocket.

- **Discover new tasks**

  - `task.created` uses `TaskManager` as the subscription ID. The scheduler uses the UUID in the message to start tracking that run.

- **Follow a specific task**

  - `task.info.updated` uses the task UUID as the subscription ID and updates tree structure and status.
  - `task.log.updated` uses the task UUID and appends or replaces logs by sequence number. Read a new snapshot if the baseline is missing or the sequence has a gap.
  - `task.notice` uses the task UUID and delivers messages about this run.
  - `task.completed` uses the task UUID and delivers the terminal status and result summary.

### Query Historical Results

Import `History` with `from app.core import History` to read persistent history. The history page's HTTP routes are in `app/api/history.py`.

- **Query record indexes and statistics by date**

  ```python
  <grouped results> = await History.search(
      start_date=<start date>,
      end_date=<end date>,
      mode=<grouping mode>,
  )
  ```

  - `start_date=` and `end_date=` take `date` objects. The range includes both dates.
  - `mode=` selects how to group results.
  - `"DAILY"` groups by day.
  - `"WEEKLY"` groups by week, using the Monday date as the key.
  - `"MONTHLY"` groups by month, using the first day of the month as the key.
  - `<grouped results>` are organized by group date and account name. Each `index` lists records, and `data` supplies aggregate statistics. Use the index entry's `id` to read the details.
  - Use `POST /api/history/search` over HTTP with `start_date`, `end_date`, and `mode` in the body. Pass dates as `YYYY-MM-DD` strings.

- **Read details by record UUID**

  ```python
  <history details> = await History.get_detail(record_id=<history record UUID>)
  ```

  - `record_id=` is the UUID in the history index item's `id` field.
  - `<history details>` is a `HistoryDetail` containing the execution result, statistics, the log or the reason it is unavailable, and the list of attachments.
  - Use `POST /api/history/detail` over HTTP and pass the history record UUID as `recordId`.

- **Read a file by attachment UUID**

  ```python
  <attachment file> = await History.get_attachment(attachment_id=<attachment UUID>)
  ```

  - `attachment_id=` is the UUID in the attachment's `id` field in the history details.
  - `<attachment file>` is a `HistoryAttachmentFile` containing attachment metadata and the local file path.
  - Use `GET /api/history/attachments/{attachment_id}` over HTTP. Images and videos are returned inline by default. Use `download=true` to download them.

- **Read statistics for overview pages**

  ```python
  <latest managed script results> = await History.get_latest_script_results(script_names=<managed script names>)
  <account execution overview> = await History.get_overview(day=<query date>)
  ```

  - `script_names=` looks up the latest record by managed script name and returns its execution time and status. `GET /api/info/scripts/status` provides both schedule configuration and latest results.
  - `day=` takes a `date` and summarizes run counts, failure records, and the latest execution time by account. `POST /api/info/get/overview` returns the automation overview.

### Task Information Tree

The task information tree is defined in `app/models/task.py`. It holds the selected execution list, current status, and logs. At acceptance, the host resolves queue items to managed scripts. `TaskItem` directly contains their individual executions. The nodes reuse configuration objects, ordered collections, and virtual fields from the [configuration framework](./config.md) and live in memory.

```text
TaskItem <task run UUID>
└─ scripts | ConfigCollection[TaskScriptItem]
   └─ TaskScriptItem <managed script run UUID>
      └─ users | ConfigCollection[TaskUserItem]
         └─ TaskUserItem <account run UUID>
            └─ logs | ConfigCollection[LogRecord]
               └─ LogRecord <log record UUID>
```

- **Task | `TaskItem`**

  - `info` holds the task name, mode, trigger reason, and overall status.
  - `execution` holds the queue's pre-run and post-run scripts, completion action, and delay captured for this run.
  - `current.index` points to the current managed script by its run UUID in `scripts`. It is `None` until a managed script is selected.

- **Managed-script execution | `TaskScriptItem`**

  - `info` holds the original `script_id`, adapter type, name, current status, and node parameters.
  - `current.index` points to the current account by its run UUID in `users`. It is `None` until an account is selected.
  - `current.log` holds managed-script lifecycle messages, such as waiting for a configuration lock or a device.

- **Account execution | `TaskUserItem`**

  - `info` holds the original `user_id`, name, current status, and node parameters. A shared configuration session uses `"default"` as its `user_id`.
  - `logs` contains the log records produced for the account run.

- **Log record | `LogRecord`**

  - `info.start_time`, `info.phase`, and `info.status` hold the start time, phase, and status description.
  - `info.content` stores the log line by line. Use `append_content()` to append lines.

`scripts`, `users`, and `logs` are keyed by each runtime node's `uid`. Use `info.script_id` or `info.user_id` to look up its original configuration. If a managed script appears twice in a queue, it produces two `TaskScriptItem` nodes. They share `info.script_id` and each has its own `uid`, status, and logs.

Execution code updates `info.status`. The virtual `info.result` summarizes child data. Accounts summarize their log records, managed scripts summarize their accounts, and the task summarizes managed-script results and account completion counts. In the plugin execution chain, the root's `current.log` follows both `current.index` pointers to display the most recent log for the current account. When a managed script has been selected and there is no account log yet, it displays the managed script's lifecycle log.
