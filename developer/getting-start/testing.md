# 测试与验证

## Pytest

**Pytest** 是本项目后端自动化回归测试的核心执行引擎。通过函数级隔离与依赖注入机制，将复杂的业务状态拆解为可复用的测试单元；同时利用断言重写，在测试失败时直接输出变量实际值与预期值的差异，降低排查成本。

### 适用范围

适用于能在确定环境中验证的后端行为，例如：配置保存和重新加载、API 请求和响应、调度状态转换、路径校验、插件加载和配置读取等。

面向 `dev` 的 PR 会在 Windows 和 Ubuntu 上运行全量 Pytest。CI 会先准备锁定版本的插件发行包，再执行测试。 [Python 测试工作流](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/python-tests.yml)

### 运行方法

在主仓库根目录执行。

- 全量测试

  ```powershell
  uv run --no-sync python -m pytest tests -q
  ```

  首次运行前，执行 `uv sync --locked --group dev` 安装测试依赖，再执行 `uv run --no-sync python scripts/prepare_test_plugins.py` 准备锁定版本的插件发行包。

- 单模块测试

  ```powershell
  uv run --no-sync python -m pytest tests/api/test_config_roundtrip.py -q
  ```

  将示例路径替换为目标测试文件。首次运行前先安装测试依赖，涉及专项集成时还需准备插件发行包。

### 代码组织规范

后端模块 `app/<模块名>/` 对应的测试须放在 `tests/<模块名>/`。

测试插件相关代码时，插件自身的行为在插件仓库验证；主程序与插件如何协作在主仓库通过加载指定的已发布插件验证。

### 开发规范

优先复用能守住当前行为的用例。需要新增时，从可复现的故障或公共契约出发，断言用户可观察的结果，将真实账号、网络和外部设备留在隔离边界之外。验证 Bug 的回归用例应能在修复前失败、修复后通过。是否长期保留、如何记录结果及如何修改或移除测试，按[测试提交与维护规范](./development-specifications.md#测试提交与维护规范)执行。

## Playwright

### 概述

**Playwright** 是本项目端到端（E2E）测试的执行引擎，负责验证从用户界面到后端服务的完整链路。Playwright 能够启动开发前端与真实 MAS 后端，测试完整的用户操作路径。

### 适用范围

适用于通过界面操作验证前后端协作的场景，例如页面导航、配置保存与失败回滚、队列编辑、历史记录筛选和 WebSocket 断线重连。Windows 桌宠的窗口交互由 `@mascot` 用例覆盖。 [前端测试说明](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/frontend/TESTING.md)

隔离的浏览器用例可以在能启动前后端并安装浏览器的 CI 环境中运行。主仓面向 `dev` 的 PR 目前只检查 E2E 启动器、测试配置隔离和种子数据。开发者在本机按改动选择相关端到端用例。 [前端检查工作流](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/format-check.yml)

### 运行方法 {#playwright-运行方法}

1. 准备环境

```powershell
uv sync --locked --group dev
Set-Location frontend
yarn install --immutable
yarn playwright install chromium
```

2. 执行测试

| 操作                    | 命令                                   |
| ----------------------- | -------------------------------------- |
| 检查启动器与 E2E 类型   | `yarn e2e:check`                       |
| 运行所有非 `@real` 用例 | `yarn e2e`                             |
| 运行导航冒烟测试        | `yarn e2e:smoke`                       |
| 选择相关标签            | `yarn e2e --grep '@queue\|@scheduler'` |
| 验证 Windows 桌宠窗口   | `yarn e2e --grep '@mascot'`            |
| 在 Playwright UI 中调试 | `yarn e2e:ui`                          |
| 打开浏览器测试报告      | `yarn e2e:report`                      |

常用标签包括 `@init` 冷启动、`@queue` 队列、`@scheduler` 调度台、`@settings` 设置、`@history` 历史、`@recovery` 重连和 `@notify` 通知。Windows 系统通知用例会弹出真实通知。按实际改动查找已有用例和标签。 [E2E 用例目录](https://github.com/AUTO-MAS-Project/AUTO-MAS/tree/dev/frontend/e2e)

启动器默认创建临时数据根，运行数据和后端日志写入该目录，后端和前端退出后随目录清理。普通隔离用例使用测试种子和默认配置。失败截图、Trace、视频和 HTML 报告分别保存在 `frontend/test-results/browser/` 与 `frontend/playwright-report/browser/`。

| 排查场景                 | 用法                                                                                                                                                                                             |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 检查配置、历史或后端日志 | 在当前终端设置 `$env:AUTO_MAS_E2E_KEEP_ROOT = '1'` 后运行。终端会打印数据根，可查看其中的 `debug/app.log`。                                                                                      |
| 复用调试专用的数据根     | 传 `--in-place` 或设置 `AUTO_MAS_E2E_IN_PLACE=1`，用 `AUTO_MAS_E2E_DATA_ROOT` 指向该目录。未指定目录时使用源码检出。就地模式会写入对应的 `config/`、`data/`、`history/` 和 `debug/` 并保留结果。 |

### 代码组织规范

用例位于 `frontend/e2e/*.pw.ts`，按用户场景命名，通过 `test.describe('@标签', ...)` 支持选择运行。复用 `fixtures.ts` 的页面入口、断言辅助与证据采集。`run.mjs` 统一创建数据根、准备种子并启动 Playwright，运行用例时用上表的 Yarn 命令。

用例所需的配置优先通过 UI 或公共 API 创建。必须在后端启动前存在的数据由启动器准备。适合多个场景复用的初始状态可提取为最小测试配置。

### 开发规范

从用户动作和最小可观察结果出发，等待可见状态、API 响应或 WebSocket 事件。验证保存时可以比较隔离目录中写入前后的配置副本，或在页面中离开再返回，确认能读回修改。需要验证重启后的持久化时，应实际重启后端再检查。新增或修改场景时，要验证被守行为发生故障后用例确实能失败，然后还原故障并重跑。

一次性排查流程留在本地。适合长期保留的用例和 PR 记录按[测试提交与维护规范](./development-specifications.md#测试提交与维护规范)执行。需要协助选择、运行与交接 E2E 时，参考 [mas-e2e](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.agents/skills/mas-e2e/SKILL.md)。

## `@real` {#real}

### 概述

`@real` 是在 PR 作者本机执行的 E2E。它使用作者已经配置好的托管、账号，以及实际安装的脚本、游戏和所需设备，通过 MAS 的公开入口执行调度。

### 适用范围

改动涉及任务调度逻辑、游戏或托管适配插件时，按主仓库要求对受影响的路径附 `@real` 证据。纯前端交互按相关 E2E 标签验证。验证以 MAS 公开的任务终态、目标账号结果、对应历史记录和收尾状态为准。每次运行的结论只适用于实际执行的目标和断言。 [E2E 流程清单](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/docs/testing/e2e.md)

### 运行方法

按 [Playwright 的运行方法](#playwright-运行方法) 准备依赖，并确保 PowerShell 7 的 `pwsh` 可用。预先在 AUTO-MAS 配好可运行的托管和启用账号。托管引用计划或绑定模拟器时，配置来源也需要有相应条目。

```powershell
cd frontend                       # 工作目录为 frontend
yarn e2e:real                     # 运行所选目标的 @real 用例
yarn e2e:real --grep complete     # 只验证执行完成、重连和历史
yarn e2e:real --grep stop-restart # 只验证停止、再次启动和收尾
```

默认从主仓检出的 `config/scripts.yaml` 选择第一个有启用账号的托管及其首个启用账号。入口会打印实际选中的目标。迁移完成后缺少该文件时，先在 MAS 中恢复配置。使用 Git worktree 时，可将 `AUTO_MAS_E2E_TEMPLATE_ROOT` 设为实际保存配置的主检出目录。

| 环境变量                    | 指定内容         |
| --------------------------- | ---------------- |
| `AUTO_MAS_E2E_SCRIPT_ID`    | 托管 UUID        |
| `AUTO_MAS_E2E_USER_ID`      | 账号 UUID        |
| `AUTO_MAS_E2E_ACCOUNT_NAME` | 配置中的账号名称 |

找不到指定目标时启动器会报错。需要对多个账号验证时，可以先列出目标或按专项筛选。

```powershell
yarn e2e:real:all --list
yarn e2e:real:all --only MaaConfig,SrcConfig
yarn e2e:real:all
```

批量入口按启用账号串行执行，为每个目标创建独立数据根并按托管绑定设备。单个目标失败后会继续运行其他目标。单次运行的精选截图和 `real-e2e-summary.json` 在 `frontend/test-results/real/`。在 `frontend/` 下用 `yarn playwright show-report playwright-report/real` 打开报告。批量结果按目标保存在 `frontend/test-results/real-batch/`，HTML 报告对应最后一次运行。

### 代码组织规范

真实用例和其他端到端用例同在 `frontend/e2e/`，真实流程必须带 `@real` 标签，并可再加专项标签。`run-real.ps1` 使用 `run.mjs` 启动单次真实运行，`real-batch.mjs` 负责串行分发多个目标。新增专项路径时写明要求的安装目录、托管、账号和设备条件。需要复制额外脚本数据时，先更新启动器的种子规则和自检。

### 开发规范

默认隔离运行只复制所选目标需要的配置和数据，并清理复制配置中的账号通知字段。脚本安装、游戏和模拟器仍使用本机真实环境，可能改变游戏状态或消耗资源。批量运行要求每个目标独立隔离。确实需要单次就地排查时，按 E2E 的调试方法使用专用数据根。

同一台机器一次运行一个 `@real`，启动器通过机器级锁限制并发。强制结束进程后若锁残留，确认真实用例已停止再处理锁文件。用例应等待任务终止、确认运行时空闲，并只关闭本次测试启动的模拟器。任务身份或空闲状态无法确认时，需要在结果中保留收尾失败的信息并人工排查。

按[测试提交与维护规范](./development-specifications.md#pr-验证记录)记录命令、被测提交、环境和实际结果，并附审核过的摘要关键字段与精选截图。提交前检查可识别的账号信息、路径和凭据，避免直接附完整报告。作者本机验证的执行与交接约定见 [mas-e2e](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.agents/skills/mas-e2e/SKILL.md)。
