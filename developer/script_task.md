# 脚本专项适配

脚本专项适配需要同时打通配置、Schema、API、任务调度、前端入口与运行验证。当前项目的详细工程规则以内置在主仓 `.agents/skills/mas-script-specialized-adapter` 的 Skill 为准；本文档只保留开发者阅读时最容易遗漏的流程入口。

## 适配前先确认架构

新增或重构 `ScriptType` 前，先确认外部脚本属于哪条架构线：

| 架构线 | 典型形态 | 本仓参照 |
|--------|----------|----------|
| MAA 线 | MAA 系配置会话、计划表、关卡/理智类配置 | `MAA` |
| SRC 线 | Alas / SRC 风格的大表单与 Section | `SRC` |
| MXU 线 | MaaEnd + MXU、`mxu-*.json`、ScriptConfig 遮罩 | `MaaEnd` |
| MFAA 线 | M9A / MFAA 任务队列 JSON，不套 ScriptConfig 壳 | `M9A` |
| General | 通用路径、进程、日志监控 | `General` |
| ok-script 线 | `-t` / `-e` CLI + 表单化配置编辑器 | `Okww` |

推荐先让用户提供脚本或 GUI 壳的仓库 URL，再根据 README、启动参数、配置落盘方式和 Release 产物判断架构线。没有仓库时，应先向用户确认脚本形态、正式 `ScriptType`、展示文案、图标来源、自启动方式和配置落盘方式。

## 落地顺序

专项适配按“前端表面优先，然后补齐后端”的顺序推进：

1. **架构确认**
   - 确认上游仓库、正式 `ScriptType`、路由片段、用户可见文案与图标。
   - 确认自动跑任务靠 CLI 参数、写 JSON，还是启动外置配置会话。

2. **前端表面**
   - `Scripts.vue` 与 `ScriptTable.vue`：Hub 入口、卡片、图标、操作按钮。
   - `router/index.ts`：新增与 Hub 一致的路由片段。
   - `frontend/src/types/script.ts`：补充 `ScriptType` 与默认结构。
   - `frontend/src/composables/useScriptApi.ts`：补齐脚本类型分支和 `UserConfig -> users[]` 分支。
   - `EditView/Script/`、`EditView/User/`：新增或调整脚本编辑页、用户编辑页和 Section。

3. **后端注册**
   - `app/models/config.py`：新增 `XxxConfig` / `XxxUserConfig`，注册到对应配置集合。
   - `app/models/schema.py`：补齐 API Schema。
   - `app/core/config.py`、`app/api/scripts.py`、`app/utils/constants.py`：补齐配置、API、展示文案和类型映射。
   - 后端 schema 变更后，启动当前本地后端并在 `frontend` 运行 `yarn openapi`。禁止手改 `frontend/src/api` 生成目录。

4. **任务模块**
   - `app/task/Xxx/`：按架构线实现 `Manager`、`AutoProxy`，按需补 `config_schema.py` 或 `ScriptConfig.py`。
   - `Manager` 与任务执行类必须实现 `final_task` / `on_crash`。
   - `check()` 返回信息应告诉用户该怎么处理，不要只给技术描述。

5. **验证与清理**
   - 先用 `General` 跑通启动、日志和配置可行性时，专项落地后应删除临时入口。
   - 对照主仓 `.agents/skills/mas-script-specialized-adapter/references/adapter-code-norms.md` 做提交前自检。
   - 确认 Hub、路由、编辑页、API、任务调度、日志和历史记录都已打通。

## 代码质量底线

- 不要简单复制 `app/task/general` 后全局替换变量名；应对齐最接近的架构线和现有表面模板。
- 配置写入和恢复使用原子化思路，避免任务中断后损坏配置。
- `final_task` 和 `on_crash` 共用的恢复逻辑应提取复用。
- 进程清理、文件清理等副作用操作应分别捕获异常，避免一个失败阻塞后续清理。
- 实例属性在 `__init__` 中显式初始化并带类型注记，不依赖 `hasattr()` 兜底。
- 状态、日志、进程管理、pre/post 脚本等优先对齐 `General`，只有架构确认后才引入专项差异。

## 进一步阅读

AI 助手执行专项适配时，应加载主仓 `.agents/skills/mas-script-specialized-adapter/SKILL.md`，并按需阅读其中的：

- `references/script-frontend-architectures.md`
- `references/adapter-code-norms.md`
- `references/examples-frontend-surfaces.md`
- 对应架构线的 `examples-*.md`
