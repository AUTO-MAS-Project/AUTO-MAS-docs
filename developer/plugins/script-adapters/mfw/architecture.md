# 总体架构

MFW 位于 `app/task/MaaFW/`，根据项目接口导入、更新并执行 MaaFramework 项目。托管类型 `MaaFWConfig`、`M9AConfig` 和 `MSSConfig` 在 `MaaFW/registration.py` 的 `run_types` 分别登记，共用 `MaaFWEmbeddedManager`。账号页保存的任务队列是运行时的任务来源。

## 代码职责

1. **接收接口请求**：`MaaFW/api.py` 定义路由，由 `app/api/scripts/__init__.py` 统一挂载。端点解析输入、调用业务并构造响应。视图导入、预览、更新、环境准备和外壳配置导入分别由 `MaaFW/api_service/` 下的模块实现。
2. **接入宿主**：`MaaFW/embedded_manager.py` 与 `MaaFW/tools/embedded/` 负责配置锁、托管视图、调度、通知与历史记录。`MaaFW/tools/restore_service.py` 接入配置备份恢复。项目独有行为通过 `tools/embedded/flavor.py` 定义的[风味机制](./flavor.md)调用。
3. **执行通用引擎逻辑**：`MaaFW/tools/core/` 处理接口解析、任务快照、运行计划、解释器管理、项目载荷和更新。worker 在独立虚拟环境内运行，其 `app.*` 导入闭包限制在 `tools/core/`，第三方依赖限于 `maafw` 和 `BASE_RUNTIME_PACKAGES`，相关约束由 `tests/task/test_maafw_core.py` 检查。
4. **提供操作界面**：`frontend/src/views/EditView/` 下的 MFW 托管页、账号页和风味页面展示导入、配置与运行状态。风味登记在 `frontend/src/composables/useMaaFWFlavor.ts`。Electron 问题包由 `frontend/electron/services/maafwIssueReportService.ts` 导出。

API 业务单向依赖引擎和宿主接入模块。更新器在宿主进程中运行；引擎核心中供 worker 使用的代码需保持独立。worker 的工作目录是应用根目录，`PYTHONSAFEPATH` 只设置给 worker，项目 agent 仍按项目配置解析同级模块。

## 一次导入

1. **验证来源目录**：托管页调用 `/api/scripts/maafw/embedded/reimport`。宿主先使用 `Info.Path` 的校验规则检查来源。
2. **投影项目文件**：按项目接口与投影规则挑选文件，过滤可识别的外壳，将必要的运行状态文件放入视图私有区域。源码形态改走本地私有渠道。
3. **登记载荷与切换视图**：将文件归入载荷，把当前托管的视图切到同组版本，再同步同组空闲托管。导入进度通过 WebSocket 推送。
4. **识别托管类型**：按登记顺序选中第一个认领该项目的风味，否则使用通用 MFW。类型变化时就地切换，保留托管 UID。

投影规则、磁盘布局与复用流程见[导入与载荷视图](./import-and-payload.md)。

## 一次运行

1. **准备托管**：`MaaFWEmbeddedManager` 预约视图，确保项目已导入，再执行组同步及投影补齐。托管和账号集合加锁后，持票使用运行配置。
2. **准备版本与环境**：按托管设置检查更新。切换版本或环境需要补确认时，在账号运行前完成确认。见[项目更新](./update.md)。
3. **运行账号任务**：`tools/embedded/runner_task.py` 根据账号快照生成计划，处理设备准备并启动 worker。worker 连接控制器，逐项下发任务，回传进度和结果。风味钩子、任务时限及重试规则见[运行](./run.md)。
4. **落盘并释放资源**：记录账号状态、日志、失败截图和历史结果。执行器保存运行状态并解锁，随后维护视图与汇总报告。正常收尾时可跟进同组就绪版本。

## 并发与锁

- **视图预约**：`tools/embedded/project_path.py` 的 `try_reserve_project_path()` 为单个视图提供进程内独占。运行、更新、准备环境和切换均受预约约束，抢占失败时由调用方跳过或提示。
- **谱系锁**：`tools/core/project_update/state.py` 中的 `DurableFileLock` 跨进程串行化同一项目的更新、登记和回收复核。
- **配置锁**：托管执行期间锁定托管和账号配置，避免编辑写回覆盖运行状态。资源恢复后按调度层的票据规则释放。

每个托管拥有自己的视图，项目在运行中写入的私有状态按托管隔离。硬链接只用于已判定可共享的内容，具体写穿巡检见[设计取舍](./tradeoffs.md)。
