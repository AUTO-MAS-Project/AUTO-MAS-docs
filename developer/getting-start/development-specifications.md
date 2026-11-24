# 开发规范

## 仓库

| 项目           | 仓库                                                                                                       |
| -------------- | ---------------------------------------------------------------------------------------------------------- |
| 主程序         | [AUTO-MAS](https://github.com/AUTO-MAS-Project/AUTO-MAS)                                                   |
| 文档站         | [AUTO-MAS-docs](https://github.com/AUTO-MAS-Project/AUTO-MAS-docs)                                         |
| 数据站前端     | [AUTO-MAS-data-center-frontend](https://github.com/AUTO-MAS-Project/AUTO-MAS-data-center-frontend)（私有） |
| 数据站后端     | [AUTO-MAS-data-center-backend](https://github.com/AUTO-MAS-Project/AUTO-MAS-data-center-backend)（私有）   |
| 数据同步与插件 | `AUTO-MAS-Xxx` / `AUTO-MAS-<适配对象>-Adapter`                                                             |

## Agent

使用 AI 助手参与开发前，必须先确认主仓库存在并加载 `.agents/skills/mas-skills/SKILL.md`。

## Git 分支

### 1. 主分支

- **分支名**：`main`
- **描述**：用于存储稳定版代码，保证所有功能稳定，能够直接跑通。
- **规范**：主分支不允许 Force Push 或 Ppush，仅接受来自 `dev` 分支的 Pull Request。

### 2. 主开发分支

- **分支名**：`dev`
- **描述**：用于存储包含当前最新功能的代码，仅保证能够直接跑通。
- **规范**：项目维护者可以直接维护 `dev`，较大的功能开发应建立开发分支，测试完成后再合入；外部贡献者应从上游 `dev` 拉出开发分支，并通过 Pull Request 合入 `AUTO-MAS-Project/AUTO-MAS:dev`。
- **注意**：建开发分支前请先 `git fetch`，从**最新**的上游 `dev` 拉出，以减少合并冲突。

### 3. 发行版分支

- **分支名**：`release/<version>`
- **描述**：用于存储各个发行版的最新后端代码，由版本发布工作流自动创建，各个发行版软件自动从各自对应的发行版分支拉取最新后端代码，以实现后端新增功能与问题修复的热更。
- **规范**：在 `dev` 分支提交功能更新或问题修复后，通过 Cherry Pick 将更改同步到发行版分支。注意，只有**改动较小、且不关联任何前端**的 `dev` 修复才允许 Cherry Pick 到已发行的 `release/<version>` 分支；**Cherry Pick 带有前端逻辑的提交属于违规操作**。

  _只要同时触碰前端路径（如 `frontend/`）与后端路径（如 `app/`），即视为带前端逻辑_

  违反时按下列方式处理：

  1. 关闭相关的 Pull Request 并 Revert 相关 Commit。
  2. 违规提交已随发行版发布时，须回退发行版代码并向用户说明。

### 4. 开发分支

- **分支名**：`随意，别和其他分支或标签冲突就行~`
- **描述**：项目核心开发者用于存储开发中的功能，不一定能跑通，当功能开发完成后，将会合并到 `dev` 分支。
- **规范**：完成开发以及合并后，记得删除该分支。

## Git 提交信息

Git 提交采用Conventional Commits 标准，即：

```
<type>(<scope>): <subject>
```

### 1. type

| 类型         | 说明                                     |
| ------------ | ---------------------------------------- |
| **feat**     | 新功能（feature）                        |
| **fix**      | 修复 bug                                 |
| **docs**     | 文档修改（例如 README）                  |
| **style**    | 代码格式调整（不影响逻辑，如空格、缩进） |
| **refactor** | 代码重构（既不是新功能，也不是修复）     |
| **perf**     | 性能优化                                 |
| **test**     | 增加或修改测试                           |
| **chore**    | 构建流程或依赖管理上的调整               |
| **build**    | 影响构建系统或外部依赖的变更             |
| **ci**       | CI/CD 配置修改                           |

### 2. scope

1. **只修改了某个文件**

   填入去掉后缀的修正文件名称，保留大小写。

   - 例如：修改 `logger.py`，使用 `logger`

     `fix(logger): 修改logger日志等级 `

2. **修改了多个文件**

   优先填入这组修改共同所属的 **父文件夹名**；若涉及多个目录，选择最能代表本次变更意图的业务范围，避免把多个零散目录机械拼成 scope。

   - 例如：修改 `app/api/a.py`、`app/api/b.py`，使用 `api`

     `fix(api): 修改api模块 `

### 3. subject

**所做修改的一句话总结**，可隔一行书写详细修改内容

```
feat(core): 改进数据库连接模块

- 优化连接池初始化逻辑
- 减少启动时间约 30%
- 修复可能导致连接泄漏的问题
```

## Issue 与 PR 正文

### Issue 正文规范

Issue 应描述用户可观察的问题或需求，包括实际结果、预期结果、复现步骤及相关环境。可附日志或截图，无法稳定复现时写明已知触发条件。实现步骤、内部接口和代码位置由开发者在排查与设计时确定。

### PR 正文规范

在 `## 摘要` 下用 1 至 4 条概括问题和改动结果。有相关 Issue 时，用 `Closes #n` 关联。按需补充实际执行的验证和结果，未执行的检查应如实标明。

用户可见的改动按[版本记录](#版本记录)准备更新日志。普通 PR 保持版本号和版本文件原样。

## 测试提交与维护规范

测试是代码变更的可复核证据。可重复运行、守护用户行为或公共契约的自动化回归测试，应与被测改动放在同一仓库、同一个 PR 中，并随产品代码长期维护。

### 测试归属

- **长期自动化测试**：确定性、可隔离、可重复运行的单元、契约和回归测试放在对应测试目录或被测模块附近，随对应实现一起提交。优先断言用户可观察行为或公共契约，避免绑定私有函数调用顺序、内部数据布局等实现细节。
- **作者本机真实流程**：需要真实账号、游戏、脚本安装或模拟器的流程，按主仓 `docs/testing/e2e.md` 在作者环境运行，并在 PR 提供经过检查的命令与脱敏证据。不要提交凭据、账号数据、游戏资源或机器专属配置。
- **临时诊断**：只用于一次性排查的用例和脚本留在本地或被忽略的 `.dev/` 目录；只有确有重复使用价值的诊断工具才进入 `scripts/`。

### PR 验证记录

- 记录测试命令、通过/失败结果、测试的 commit SHA、运行环境和未覆盖范围；真实流程还应记录必要前置条件与精选证据。
- Bug 有稳定复现时，回归测试应在修复前失败、修复后通过，并随 PR 保留。若无法稳定复现或无法自动化，在 PR 中说明原因和替代验证，不要只报告未提交测试的通过数量。
- 没有新增或修改自动化测试时，说明原因和替代验证；纯文档、CI 或测试维护改动可注明不适用。

可将下面的模板用于 PR 的验证记录。按实际执行情况填写，证据和前置条件按需补充。

```markdown
- 验证场景：<用户操作或公共契约>
- 命令：<实际执行的命令>
- 提交与环境：<完整 commit SHA、工作区差异、运行环境与前置条件>
- 结果：<通过、失败或未运行，以及实际观察>
- 证据：<相关日志、截图或真实运行摘要的位置>
- 未覆盖范围：<未验证的分支或环境限制>
```

### 修改、删除与不稳定测试

- 行为或公共契约改变时，同步更新相关测试期望。删除测试需要说明被测行为已移除、已有更强测试替代、测试重复，或无法以合理成本维持可靠性的原因及替代覆盖。
- 不稳定测试应关联跟踪问题、负责人和复查日期，并临时隔离；之后必须修复、替换或删除。不要依赖自动重试掩盖失败，也不要让隔离或 `xfail` 长期没有负责人和复查时间。
- 按失败率、定位成本和运行耗时维护测试。测试行数、文件数或测试与业务代码的比例不作为质量门槛；耗时升高时先定位和优化慢测试及共享 setup。

### 本地与 CI 命令

锁定依赖、运行命令与主仓 CI 的覆盖范围见[测试与验证](./testing.md)。

## 版本记录

用户可见的改动随 PR 在 `changelog.d/` 下新增一个碎片，文件名为 `<PR 号或分支名>.<分类>.md`。首行写 `project: <项目键>`（只有 `dev` 碎片可以省略），正文一句、不超过 50 字。可运行 `python scripts/changelog.py add <分类> <项目键> "<一句话>"` 生成。描述用户能看到的变化，不加项目名前缀、PR 号、署名或内部实现细节。

纯文档、CI、测试和用户不可见的重构使用 PR 的 `skip-changelog` 标签。本发版周期才引入的功能，同一 beta 内的后续调整不写碎片，跨 beta 的调整加 `beta-only: true`，转正汇总时自动丢弃。项目键、分类、署名和例外见[更新日志碎片说明](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/changelog.d/README.md)。

工作流每天北京时间 23:30 把已合并的碎片编译到 `CHANGELOG.md` 的「未发布」段；准备发版时编译剩余碎片。日常 PR 不改 `CHANGELOG.md`、`res/version.json` 或版本号，版本号由发版 PR 写入。

## 后端注释

::: tip 总览
后端注释应该采用 **Google Python Style Guide – Docstring Conventions**，简称 **Google Docstring** 。
:::

### 方法注释

```
def add_numbers(a: int, b: int) -> int:
    """相加两个数字并返回它们的和。

    Args:
        a: 第一个数字。
        b: 第二个数字。

    Returns:
        数字之和。

    Raises:
        ValueError: 输入的参数不是整数。
    """

    if not all(isinstance(x, int) for x in (a, b)):
        raise ValueError("形参必须是整数。")
    return a + b

```

### 配置类注释

配置对象继承 `ConfigEntry`，按语义分组的字段块继承 `ConfigGroup`。类型注解定义字段类型，`Field` 描述默认值、约束和中文说明；分组、列表等通过 `default_factory` 为各实例独立创建。

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

配置字段及其校验规则只在配置模型声明一份，接口输入、输出契约从模型派生。运行时私有状态不作为持久化字段。完整定义方式、标准字段类型与调用约定见[配置管理](../core/config.md)。

## 后端函数调用

::: tip 总览
能一眼理解的用位置参数；否则用关键字参数。布尔和多参数强制使用关键字。
:::

为了提高代码可读性、减少误用、提升协作效率，项目中调用函数时应该遵循以下规则：

#### 1. 简单且含义明确的函数 → 使用位置参数

当参数少（通常 ≤2 个）且含义清晰时，可直接使用位置参数。

**示例：**

```
add(1, 2)
math.sqrt(4)
```

---

#### 2. 含义不够直观、参数较多、或容易混淆的函数 → 使用关键字参数

关键字参数能提升可读性，不依赖位置顺序，更安全。

**示例：**

```
UserItem(user_id=str(uid), name=config.get("Info", "Name"), status="等待")
```

---

#### 3. 所有布尔参数必须使用关键字参数

避免出现可读性极差的调用。

不推荐：

```
send_notification(True, False)
```

推荐：

```
send_notification(enable=True, urgent=False)
```

---

#### 4. 当参数数量超过 2 个时，优先使用关键字参数

提升可维护性，减少顺序错误风险。

**示例：**

```
connect(host="localhost", port=3306, timeout=5)
```
