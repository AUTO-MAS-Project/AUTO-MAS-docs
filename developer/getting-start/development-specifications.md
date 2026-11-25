# 开发规范

本页汇总了参与 AUTO-MAS 开发时需遵守的仓库、Git 协作、测试、更新日志及代码风格约定。

## 仓库一览

| 项目 | 仓库 |
| --- | --- |
| 主程序 | [AUTO-MAS](https://github.com/AUTO-MAS-Project/AUTO-MAS) |
| 文档站 | [AUTO-MAS-docs](https://github.com/AUTO-MAS-Project/AUTO-MAS-docs) |
| 数据站前端 | [AUTO-MAS-data-center-frontend](https://github.com/AUTO-MAS-Project/AUTO-MAS-data-center-frontend)（私有） |
| 数据站后端 | [AUTO-MAS-data-center-backend](https://github.com/AUTO-MAS-Project/AUTO-MAS-data-center-backend)（私有） |
| 数据同步与插件 | `AUTO-MAS-Xxx` / `AUTO-MAS-<适配对象>-Adapter` |

## 使用 AI 助手

使用 AI 编程助手参与开发前，请先确认主仓库中存在并已加载 `.agents/skills/mas-skills/SKILL.md`，让 AI 遵循项目既定的工作流与规范。

## Git 分支

### 1. 主分支（`main`）

- **用途**：存放稳定版代码，保证所有功能稳定且可直接运行。
- **规范**：禁止直接 Push 或 Force Push，仅接受来自 `dev` 分支的 Pull Request。

### 2. 主开发分支（`dev`）

- **用途**：存放包含当前最新功能的代码，要求能够正常启动和运行。
- **规范**：
  - 项目维护者可直接维护 `dev`，但较大功能仍建议新建开发分支，测试通过后再合并；
  - 外部贡献者请从最新的上游 `dev` 拉出开发分支，完成后向 `AUTO-MAS-Project/AUTO-MAS:dev` 提交 Pull Request。
- **提示**：创建开发分支前请先执行 `git fetch`，基于**最新**的上游 `dev` 切出分支，尽量减少合并冲突。

### 3. 发行版分支（`release/<version>`）

- **用途**：存放各个已发布版本的最新后端代码，由发版工作流自动创建。用户端的发行版软件会自动从对应分支拉取更新，实现后端功能与问题修复的热更新。
- **规范**：在 `dev` 分支完成修复后，可通过 Cherry Pick 同步到对应的 `release/<version>` 分支。注意：**仅允许同步改动较小、且完全不涉及前端逻辑的纯后端修复**；将包含前端改动的提交 Cherry Pick 到发行版分支属于违规操作。

  _只要同一提交同时改动了前端路径（如 `frontend/`）与后端路径（如 `app/`），即视为包含前端逻辑。_

  如果误操作触发违规，将按以下方式处理：

  1. 关闭相关 Pull Request 并 Revert 对应 Commit。
  2. 若违规提交已随发行版同步给用户，须立即回退发行版分支代码并向用户公告说明。

### 4. 开发分支

- **命名**：自定义，不与其他分支或标签重名即可（推荐使用 `feat/xxx`、`fix/xxx` 等清晰名称）。
- **用途**：用于开发中的功能或问题修复，开发过程中不要求随时可运行，完成后再合并到 `dev` 分支。
- **规范**：功能合并完成后，请及时删除不再使用的开发分支。

## Git 提交信息

提交信息采用 Conventional Commits 规范，格式如下：

```text
<type>(<scope>): <subject>
```

### 1. type（提交类型）

| 类型 | 说明 |
| --- | --- |
| **feat** | 新功能（feature） |
| **fix** | 修复 Bug |
| **docs** | 文档修改（例如 README、使用/开发文档） |
| **style** | 代码格式调整（不影响逻辑，如空格、缩进） |
| **refactor** | 代码重构（既非新增功能，也非修复 Bug） |
| **perf** | 性能优化 |
| **test** | 新增或修改测试 |
| **chore** | 杂项维护、依赖或工具配置调整 |
| **build** | 影响构建流程或外部依赖的变更 |
| **ci** | CI/CD 工作流配置修改 |

### 2. scope（影响范围）

1. **只修改单个文件时**：填写去掉扩展名后的文件名，保留原有大小写。
   - 示例：修改 `logger.py` 时，使用 `logger`：
     `fix(logger): 修改 logger 日志等级`
2. **修改多个文件时**：优先填写它们共同所属的**父目录名**；若跨越多个目录，则选择最能代表本次变更意图的业务模块名，不要把零散目录生硬拼在一起。
   - 示例：修改 `app/api/a.py` 和 `app/api/b.py` 时，使用 `api`：
     `fix(api): 修改 api 模块`

### 3. subject（修改摘要）

用一句话概括本次改动；如需补充详细说明，可空一行后分条列出：

```text
feat(core): 改进数据库连接模块

- 优化连接池初始化逻辑
- 减少启动时间约 30%
- 修复可能导致连接泄漏的问题
```

## Issue 与 PR 正文

### Issue 正文规范

提交 Issue 时，请重点描述**用户能观察到的问题或需求**，包括实际现象、预期表现、复现步骤与运行环境，并尽量附上日志或截图；若无法稳定复现，请写明已知的触发条件。具体的实现方案、内部接口与代码位置由开发者在排查和设计时确定即可。

### PR 正文规范

- 在 `## 摘要` 下用 1 至 4 条简明概括本次解决的问题与改动内容；如有对应 Issue，使用 `Closes #编号` 关联。
- 按需补充实际执行的验证步骤和结果，未运行的检查请如实注明。
- 涉及用户可见变化时，按[版本记录](#版本记录)补充更新日志碎片；普通 PR 请不要修改版本号或版本文件。

## 测试提交与维护规范

测试用来验证代码改动是否真正生效且未破坏原有功能。能够稳定重复运行、保护核心行为或公开接口的自动化回归测试，应随代码改动在同一个 PR 中提交并长期维护。

### 测试放在哪里

- **长期自动化测试**：可稳定隔离、重复运行的单元测试与回归测试，放在对应的测试目录或被测模块附近，随代码一起提交。断言时优先验证用户可感知的结果或公开接口，避免耦合私有函数调用顺序、内部数据结构等实现细节。
- **本机真实环境验证**：需要真实账号、游戏、脚本或模拟器的场景，由 PR 作者按主仓库的 `docs/testing/e2e.md` 在本机运行，并在 PR 中提供命令与脱敏后的验证证据。切勿提交账号凭据、个人数据、游戏资源或本机专属配置。
- **临时排查脚本**：仅供一次性定位问题的脚本请留在本地，或放在被 Git 忽略的 `.dev/` 目录中；只有确实需要长期复用的工具才放入 `scripts/`。

### PR 验证记录

- 记录实际运行的测试命令、通过/失败结果、对应的 commit SHA、运行环境以及未覆盖的范围；涉及真实环境运行时，还应补充前置条件与关键证据。
- 如果 Bug 可以稳定复现，新增的回归测试应满足「修复前失败、修复后通过」，并随 PR 保留。若确实无法稳定复现或无法自动化，请在 PR 中说明原因及替代验证手段，不要只贴本地临时测试的通过数量。
- 若本次未新增或修改自动化测试，请说明原因与替代验证方式；纯文档、CI 配置或纯测试维护改动可注明「不适用」。

可参考以下模板填写 PR 验证记录（按实际执行情况填写即可）：

```markdown
- 验证场景：<用户操作或公开接口>
- 命令：<实际执行的命令>
- 提交与环境：<完整 commit SHA、工作区状态、运行环境与前置条件>
- 结果：<通过、失败或未运行，以及实际观察结果>
- 证据：<相关日志、截图或运行摘要位置>
- 未覆盖范围：<未验证的分支或环境限制>
```

### 测试的修改、删除与治理

- 当功能行为或公开接口调整时，同步更新对应的测试预期。如需删除已有测试，须说明原因（例如对应功能已移除、已被更强测试覆盖、用例重复或维护成本过高）及替代保障。
- 遇到偶发失败的不稳定测试，应关联跟踪 Issue、明确负责人与复查日期并临时隔离，之后尽快修复、替换或移除。不要靠自动重试掩盖问题，也不要让长期跳过或 `xfail` 的用例无人跟进。
- 优化测试时关注失败率、定位成本和运行耗时，不以测试行数或测试代码占比作为指标；耗时上升时优先排查慢测试与公共初始化开销。

### 本地与 CI 命令

完整的本地测试命令与 CI 检查范围见[测试与验证](./testing.md)。

## 版本记录

日常 PR 涉及**用户可见的变化**时，请在 `changelog.d/` 目录下新增一条更新日志碎片，不要直接修改 `CHANGELOG.md`、`res/version.json` 或版本号。

- **命名与格式**：文件名为 `<PR 号或分支名>.<分类>.md`。首行填写 `project: <项目键>`（仅 `dev` 分类可省略），正文用一句话描述用户能感知的变化（不超过 50 字），不加项目名前缀、PR 号、署名或内部实现细节。
- **快捷生成**：在仓库根目录运行：

  ```powershell
  python scripts/changelog.py add <分类> <项目键> "<一句话描述>"
  ```

- **无需写碎片的情况**：纯文档、CI、测试和用户无感知的重构，为 PR 打上 `skip-changelog` 标签即可。本发版周期内刚引入的新功能，在同一个 beta 内的微调不写碎片；跨 beta 的调整添加 `beta-only: true`（转正式版汇总时会自动忽略）。完整分类、项目键与细则见[更新日志碎片说明](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/changelog.d/README.md)。
- **自动汇总**：GitHub Actions 每天北京时间 23:30 会将已合并的碎片自动汇总到 `CHANGELOG.md` 的「未发布」小节，发版时再统一写入版本号。

## 后端注释

::: tip 总览
后端代码注释统一采用 **Google Python Style Guide – Docstring Conventions**（简称 **Google Docstring**）。
:::

### 方法注释

```python
def add_numbers(a: int, b: int) -> int:
    """计算两个整数之和并返回结果。

    Args:
        a: 第一个整数。
        b: 第二个整数。

    Returns:
        两个整数之和。

    Raises:
        ValueError: 当传入的参数不是整数时抛出。
    """

    if not all(isinstance(x, int) for x in (a, b)):
        raise ValueError("形参必须是整数。")
    return a + b
```

### 配置类注释

配置对象继承 `ConfigEntry`，按语义分组的字段块继承 `ConfigGroup`。通过类型注解声明字段类型，用 `Field` 补充默认值、约束条件和中文说明；嵌套分组、列表等可变对象统一通过 `default_factory` 创建，避免实例间共享引用。

```python
from typing import Literal

from pydantic import Field

from app.config import ConfigEntry, ConfigGroup


class GlobalConfig(ConfigEntry):
    """全局配置的声明示例。"""

    class FunctionFields(ConfigGroup):
        HistoryRetentionTime: Literal[7, 15, 30, 60, 90, 180, 365, 0] = Field(
            default=0, description="历史记录保留天数，0 表示不清理"
        )
        IfAllowSleep: bool = Field(default=False, description="是否允许睡眠")
        IfSilence: bool = Field(default=False, description="是否启用静默模式")

    class NotifyFields(ConfigGroup):
        IfSendStatistic: bool = Field(default=False, description="是否发送运行结果")

    Function: FunctionFields = Field(default_factory=FunctionFields)
    Notify: NotifyFields = Field(default_factory=NotifyFields)
```

配置字段与校验规则只在配置模型中声明一份，API 的输入输出类型均由此派生；运行时临时状态不作为持久化字段。详细写法见[配置管理](../core/config.md)。

## 后端函数调用

::: tip 总览
能一眼看懂含义的用位置参数，否则用关键字参数；布尔参数与多参数调用强制使用关键字参数。
:::

为提高代码可读性并减少传参顺序出错，调用函数时请遵循以下原则：

### 1. 参数少且含义直观 → 使用位置参数

当参数不多（通常不超过 2 个）且含义清晰时，可直接传位置参数：

```python
add(1, 2)
math.sqrt(4)
```

### 2. 含义不够直观或容易混淆 → 使用关键字参数

显式写出参数名更易读，也不受参数顺序变化影响：

```python
UserItem(user_id=str(uid), name=config.get("Info", "Name"), status="等待")
```

### 3. 所有布尔参数 → 必须使用关键字参数

避免阅读代码时猜 `True` / `False` 分别代表什么：

```python
# 不推荐
send_notification(True, False)

# 推荐
send_notification(enable=True, urgent=False)
```

### 4. 参数超过 2 个 → 优先使用关键字参数

便于维护并降低传错顺序的风险：

```python
connect(host="localhost", port=3306, timeout=5)
```
