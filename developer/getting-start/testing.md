# 测试与验证

AUTO-MAS 的本地测试与校验分为四层，你可以按改动范围选用：

- **后端单元测试（Pytest）**：验证后端配置、接口、调度和插件加载等核心逻辑。
- **前端检查与单元测试（vp）**：使用前端统一工具链 `vp` 执行代码检查、格式化与前端单元测试。
- **端到端测试（Playwright）**：同时拉起前端和真实后端，在隔离环境下验证完整界面交互。
- **本机真实环境测试（`@real`）**：在开发者本机用真实账号、脚本与模拟器跑通完整调度流程。

## Pytest（后端测试）

**Pytest** 是后端自动化测试的执行引擎，可在隔离环境中快速验证业务逻辑，并在测试失败时直接展示实际值与预期值的差异，方便定位问题。

### 适用范围

适合可在确定环境下验证的后端行为，例如配置读写、API 请求与响应、调度状态流转、路径校验、插件加载与配置解析等。

提交到 `dev` 的 PR 会自动在 Windows 和 Ubuntu 上运行全量 Pytest（CI 会先准备好锁定版本的插件发行包），具体配置见 [Python 测试工作流](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/python-tests.yml)。

### 运行方法

以下命令均在主仓库根目录执行。

首次运行前，请先安装开发依赖并准备锁定版本的测试插件包：

```powershell
uv sync --locked --group dev
uv run --no-sync python scripts/prepare_test_plugins.py
```

准备好环境后，即可按需运行：

- **全量测试**

  ```powershell
  uv run --no-sync python -m pytest tests -q
  ```

- **单模块测试**（将路径替换为目标测试文件；涉及插件集成测试时同样需先准备插件包）

  ```powershell
  uv run --no-sync python -m pytest tests/api/test_config_roundtrip.py -q
  ```

### 代码组织与编写建议

- 后端模块 `app/<模块名>/` 对应的测试请放在 `tests/<模块名>/`。
- 插件自身的功能测试应在对应插件仓库完成；主程序与插件的协作则在主仓库中通过加载指定的已发布插件来验证。
- 优先复用已有测试。需要新增用例时，从可复现的问题或公开接口出发，断言用户可感知的结果，将真实账号、外网和外部设备留在隔离边界之外。修复 Bug 的回归用例应满足「修复前失败、修复后通过」。
- 测试的留存策略与 PR 验证记录要求，见[测试提交与维护规范](./development-specifications.md#测试提交与维护规范)。

## 前端检查与单元测试（vp）

前端统一通过 `vp`（Vite+）执行代码质量检查、格式化和单元测试。以下命令均在 `frontend/` 目录下执行：

| 操作 | 命令 | 说明 |
| --- | --- | --- |
| 综合检查与格式校验 | `vp check` | 一键检查代码规范与格式问题 |
| 单独执行代码检查 / 格式化 | `vp lint` / `vp fmt` | 按需单独运行 Lint 或自动格式化 |
| 运行前端单元测试 | `vp test` | 运行 Vitest 单元测试（前端 `*.test.ts` 与被测模块就近放置） |

提交涉及前端的 PR 前，建议先执行一次 `vp check` 和 `vp test`，提前发现格式、类型或逻辑问题。

## Playwright（端到端测试）

**Playwright** 负责端到端（E2E）测试，会自动启动开发前端和真实 AUTO-MAS 后端，验证从界面操作到后端响应的完整链路。

### 适用范围

适合通过界面验证前后端协作的场景，例如页面跳转、配置保存与失败回滚、队列编辑、历史记录筛选以及 WebSocket 断线重连；Windows 桌宠窗口交互由 `@mascot` 用例覆盖。更多细节见主仓 [前端测试说明](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/frontend/TESTING.md)。

目前面向 `dev` 的 PR 在 CI 中主要校验 E2E 启动器、测试配置隔离与种子数据（详见 [前端检查工作流](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/format-check.yml)），具体的端到端场景由开发者在本机按改动范围挑选执行。

### 运行方法

1. **准备环境**

   ```powershell
   uv sync --locked --group dev
   cd frontend
   vp install --frozen-lockfile
   vp exec playwright install chromium
   ```

2. **执行测试**（在 `frontend/` 目录下执行）

   | 操作 | 命令 |
   | --- | --- |
   | 检查启动器与 E2E 类型 | `vp run e2e:check` |
   | 运行所有非 `@real` 用例 | `vp run e2e` |
   | 运行导航冒烟测试 | `vp run e2e:smoke` |
   | 按标签筛选运行 | `vp run e2e --grep '@queue\|@scheduler'` |
   | 验证 Windows 桌宠窗口 | `vp run e2e --grep '@mascot'` |
   | 在 Playwright UI 中交互调试 | `vp run e2e:ui` |
   | 打开浏览器测试报告 | `vp run e2e:report` |

   常用标签包括 `@init`（冷启动）、`@queue`（队列）、`@scheduler`（调度台）、`@settings`（设置）、`@history`（历史）、`@recovery`（重连）和 `@notify`（通知，Windows 下会弹出真实系统通知）。可按实际改动在 [E2E 用例目录](https://github.com/AUTO-MAS-Project/AUTO-MAS/tree/dev/frontend/e2e) 中查找已有用例与标签。

3. **查看日志与排查问题**

   启动器默认会创建一个临时数据目录来保存运行数据和后端日志，前后端退出后自动清理。失败时的截图、Trace、录屏和 HTML 报告分别输出在 `frontend/test-results/browser/` 与 `frontend/playwright-report/browser/`。

   | 排查场景 | 操作方式 |
   | --- | --- |
   | 保留配置、历史或后端日志 | 运行前在当前终端执行 `$env:AUTO_MAS_E2E_KEEP_ROOT = '1'`。终端会打印临时目录路径，可进入查看 `debug/app.log`。 |
   | 复用指定的调试数据目录 | 传入 `--in-place` 或设置 `AUTO_MAS_E2E_IN_PLACE=1`，并用 `AUTO_MAS_E2E_DATA_ROOT` 指定目标目录（未指定时直接使用当前代码仓库目录）。就地模式会读写并保留对应的 `config/`、`data/`、`history/` 和 `debug/`。 |

### 代码组织与编写建议

- E2E 用例位于 `frontend/e2e/*.pw.ts`，按用户场景命名，并通过 `test.describe('@标签', ...)` 支持按标签运行；请复用 `fixtures.ts` 提供的页面入口、断言辅助与证据采集能力。
- `run.mjs` 统一负责创建数据目录、准备测试种子并启动 Playwright，运行测试请直接使用上表中的 `vp run` 命令。
- 用例所需配置尽量通过界面操作或公开 API 创建，仅在后端启动前必须存在的数据由启动器预置；多个场景共用的初始状态可抽成最小测试配置。
- 编写断言时，从用户操作和最小可观察结果出发，等待界面状态、API 响应或 WebSocket 事件。验证持久化时，可通过比对隔离目录中的配置文件、离开页面再返回，或在需要时重启后端来确认。新增或修改用例后，先制造对应故障确认用例确实会失败，再还原代码重跑通过。
- 一次性排查脚本留在本地即可；长期保留的用例与 PR 验证记录请遵循[测试提交与维护规范](./development-specifications.md#测试提交与维护规范)。如需协助挑选、运行或整理 E2E 结果，可参考 [mas-e2e](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.agents/skills/mas-e2e/SKILL.md)。

## `@real`（本机真实环境测试）

`@real` 是由 PR 作者在自己电脑上运行的端到端测试。它会使用本机已配置好的托管、账号以及实际安装的脚本、游戏和模拟器，通过 AUTO-MAS 公开入口跑通真实调度。

### 适用范围

当改动涉及任务调度逻辑、游戏适配或脚本托管适配时，请按主仓 [E2E 流程清单](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/docs/testing/e2e.md) 对受影响路径提供 `@real` 验证证据（纯前端交互只需运行对应的普通 E2E 标签）。

验证结论以 AUTO-MAS 公开的任务终态、目标账号结果、历史记录及收尾状态为准，且仅代表实际执行过的目标与断言。

### 运行方法

开始前，请先完成 [Playwright 运行方法](#运行方法-1) 中的依赖准备，确保系统已安装 PowerShell 7（`pwsh`），并提前在 AUTO-MAS 中配好可正常运行的托管与启用账号（若托管关联了计划或模拟器，也需一并配好）。

1. **运行单个目标**

   ```powershell
   cd frontend                           # 进入 frontend 目录
   vp run e2e:real                       # 运行所选目标的 @real 用例
   vp run e2e:real --grep complete       # 只验证执行完成、重连和历史记录
   vp run e2e:real --grep stop-restart   # 只验证停止、重新启动和收尾状态
   ```

   默认会从主仓的 `config/scripts.yaml` 中选取第一个包含启用账号的托管及其首个启用账号，并在终端打印实际选中的目标。如果该配置文件缺失，请先在 AUTO-MAS 中恢复配置；使用 Git worktree 时，可通过 `AUTO_MAS_E2E_TEMPLATE_ROOT` 指定保存配置的主检出目录。

   如需指定特定目标，可在运行前设置以下环境变量（找不到目标时启动器会直接报错）：

   | 环境变量 | 指定内容 |
   | --- | --- |
   | `AUTO_MAS_E2E_SCRIPT_ID` | 托管 UUID |
   | `AUTO_MAS_E2E_USER_ID` | 账号 UUID |
   | `AUTO_MAS_E2E_ACCOUNT_NAME` | 配置中的账号名称 |

2. **批量验证多个目标**

   ```powershell
   vp run e2e:real:all --list                    # 列出所有可测目标
   vp run e2e:real:all --only MaaConfig,SrcConfig # 只运行指定专项
   vp run e2e:real:all                           # 串行运行所有启用账号
   ```

   批量入口会按启用账号依次串行执行，为每个目标创建独立数据目录并按托管绑定设备；某个目标失败不会中断后续目标。

3. **查看报告与摘要**

   - 单次运行的精选截图与 `real-e2e-summary.json` 保存在 `frontend/test-results/real/`，可在 `frontend/` 下运行 `vp exec playwright show-report playwright-report/real` 查看报告。
   - 批量结果按目标分别保存在 `frontend/test-results/real-batch/`，HTML 报告对应最后一次执行结果。

### 注意事项与编写规范

- **文件组织**：真实用例与其他 E2E 用例同在 `frontend/e2e/`，必须带有 `@real` 标签（可再加专项标签）。单次运行由 `run-real.ps1` 调用 `run.mjs` 启动，批量分发由 `real-batch.mjs` 负责。新增专项路径时需写明所需的安装目录、托管、账号与设备前提；如需复制额外脚本数据，请同步更新启动器的种子规则与自检。
- **隔离边界与真实影响**：默认隔离模式只复制目标所需的配置和数据，并清理副本中的账号通知设置；但脚本、游戏与模拟器运行在你的真实环境中，**可能会修改游戏状态或消耗资源**。批量运行要求每个目标独立隔离；如确需就地排查，请参照上文 E2E 调试方式使用专用数据目录。
- **并发锁与收尾**：同一台机器同一时间只能运行一个 `@real`（由机器级锁控制）。如果中途强杀进程导致锁残留，请先确认真实测试进程已全部退出后再清理锁文件。用例结束后应等待任务终止、确认运行时恢复空闲，并只关闭本次测试启动的模拟器；若无法确认收尾状态，须在结果中保留异常信息以便人工排查。
- **提交脱敏证据**：按[测试提交与维护规范](./development-specifications.md#pr-验证记录)填写命令、commit SHA、环境与结果，附上审核后的摘要关键字段与精选截图。提交前务必检查并剔除个人账号信息、敏感路径和凭据，不要直接上传完整报告。更多交接约定见 [mas-e2e](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.agents/skills/mas-e2e/SKILL.md)。
