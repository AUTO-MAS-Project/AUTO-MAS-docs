# Development Standards

This page covers the repository layout, Git workflow, testing policy, changelog conventions, and backend coding style for contributing to AUTO-MAS. For detailed repository division and AI assistant rules, see [Repository and Agent Rules](./agent-and-repository-rules.md).

## Repositories

| Project | Repository |
| --- | --- |
| Main Application | [AUTO-MAS](https://github.com/AUTO-MAS-Project/AUTO-MAS) |
| Documentation Site | [AUTO-MAS-docs](https://github.com/AUTO-MAS-Project/AUTO-MAS-docs) |
| Data Center Frontend | [AUTO-MAS-data-center-frontend](https://github.com/AUTO-MAS-Project/AUTO-MAS-data-center-frontend) (private) |
| Data Center Backend | [AUTO-MAS-data-center-backend](https://github.com/AUTO-MAS-Project/AUTO-MAS-data-center-backend) (private) |
| Data Sync & Plugins | `AUTO-MAS-Xxx` / `AUTO-MAS-<target>-Adapter` |

## Using AI Assistants

Before using an AI coding assistant, confirm that the main repository contains and loads `.agents/skills/mas-skills/SKILL.md` so the assistant follows project workflows and conventions.

## Git Branches

### 1. Main Branch (`main`)

- **Purpose**: Stores stable release code. All features must be stable and runnable.
- **Rules**: Direct `push` and `force push` are forbidden. `main` only accepts Pull Requests from `dev`.

### 2. Main Development Branch (`dev`)

- **Purpose**: Stores the latest feature code and must remain buildable and runnable.
- **Rules**:
  - Maintainers may maintain `dev` directly, though larger features should still use dedicated development branches and be merged after testing.
  - External contributors should branch off the latest upstream `dev` and submit Pull Requests to `AUTO-MAS-Project/AUTO-MAS:dev`.
- **Tip**: Run `git fetch` before creating a development branch so you start from the **latest** upstream `dev` and minimize merge conflicts.

### 3. Release Branch (`release/<version>`)

- **Purpose**: Stores the latest backend code for each shipped release and is created automatically by the release workflow. Installed apps pull from their corresponding release branch to receive backend hot updates.
- **Rules**: After landing a fix on `dev`, synchronize it to the target `release/<version>` branch via `cherry-pick`. **Only small, pure-backend fixes unrelated to frontend code may be cherry-picked**; cherry-picking any commit that includes frontend logic is a **violation**.

  _Any commit that touches both frontend paths (such as `frontend/`) and backend paths (such as `app/`) counts as carrying frontend logic—judge by the touched files, not by the commit message._

  If a violation occurs:

  1. Close the related Pull Request and revert the offending commit.
  2. If the violating commit has already shipped to users, roll back the release branch and notify users.

### 4. Development Branches

- **Naming**: Any descriptive name that does not conflict with existing branches or tags (for example `feat/xxx` or `fix/xxx`).
- **Purpose**: Used for work-in-progress features or bug fixes; it does not have to be runnable at every intermediate commit, and is merged into `dev` when complete.
- **Rules**: Delete the branch once merged.

## Git Commit Messages

Commits follow the Conventional Commits format:

```text
<type>(<scope>): <subject>
```

### 1. type

| Type | Description |
| --- | --- |
| **feat** | New feature |
| **fix** | Bug fix |
| **docs** | Documentation changes (such as README or guides) |
| **style** | Formatting changes that do not affect logic (spacing, indentation, etc.) |
| **refactor** | Refactoring that is neither a feature nor a bug fix |
| **perf** | Performance optimization |
| **test** | Adding or updating tests |
| **chore** | Maintenance, dependency, or tooling adjustments |
| **build** | Changes affecting the build system or external dependencies |
| **ci** | CI/CD workflow configuration changes |

### 2. scope

1. **Single file modified**: Use the file name without its extension, preserving original casing.
   - Example: modifying `logger.py` -> `fix(logger): adjust logger level`
2. **Multiple files modified**: Prefer the common **parent directory name**. If changes span multiple directories, choose the business module name that best represents the intent instead of concatenating unrelated paths.
   - Example: modifying `app/api/a.py` and `app/api/b.py` -> `fix(api): adjust api module`

### 3. subject

Summarize the change in one concise line. If more detail is helpful, add a blank line followed by bullet points:

```text
feat(core): improve database connection module

- Optimize connection pool initialization
- Reduce startup time by about 30%
- Fix a possible connection leak
```

## Issue and PR Bodies

### Issue Body

Describe the **user-observable problem or request**: actual behavior, expected behavior, reproduction steps, environment details, and relevant logs or screenshots. If the issue does not reproduce reliably, describe known trigger conditions. Leave implementation steps, internal interfaces, and code locations to investigation and design.

### PR Body

- Under `## Summary`, provide 1 to 4 concise bullets describing the problem and changes. Link related issues with `Closes #<number>`.
- Record actual verification commands and results; clearly note any checks that were not run.
- For user-visible changes, add a changelog fragment per [Version Records](#version-records). Do not edit version numbers or version files in ordinary PRs.

## Test Submission and Maintenance

Tests provide repeatable proof that changes work and do not break existing behavior. Deterministic, isolated automated regression tests protecting user-visible behavior or public contracts should be submitted in the same PR as the code change and maintained alongside it.

### Test Categories

- **Long-lived automated tests**: Place deterministic, isolated, repeatable unit, contract, and regression tests in the appropriate test directory or alongside the code under test, and submit them with the implementation. Prefer asserting user-visible behavior or public contracts over private call order or internal data layouts.
- **Author-local real flows**: Run flows requiring real accounts, games, script installations, or emulators locally following `docs/testing/e2e.md` in the main repository, and include the commands and sanitized evidence in the PR. Never commit credentials, account data, game assets, or machine-specific settings.
- **Temporary diagnostics**: Keep one-off diagnostic scripts local or in the Git-ignored `.dev/` directory. Only promote tools with clear ongoing reuse value into `scripts/`.

### PR Verification Record

- Record the commands run, pass/fail outcomes, tested commit SHA, environment, and uncovered areas. For real runs, include preconditions and selected evidence.
- When a bug reproduces reliably, its regression test should fail before the fix, pass after the fix, and remain in the PR. If reliable reproduction or automation is not feasible, explain why and provide alternative verification—do not report pass counts from unsubmitted local tests alone.
- If no automated test is added or updated, explain why and describe alternative verification. Pure documentation, CI, or test-maintenance PRs may mark this as not applicable.

You can use the following template in PR descriptions:

```markdown
- Scenario: <user action or public contract>
- Command: `<actual command run>`
- Commit & environment: <full commit SHA, worktree state, OS/preconditions>
- Result: <pass / fail / not run and observed outcome>
- Evidence: <relevant log, screenshot, or summary path>
- Uncovered scope: <untested branches or environment limitations>
```

### Revising, Removing, and Quarantining Tests

- Update test expectations whenever behavior or a public contract changes. When deleting a test, explain why (behavior removed, replaced by stronger coverage, redundant, or unsustainable maintenance cost) and note replacement coverage.
- Track flaky tests with an issue, an owner, and a review date, then quarantine them temporarily until fixed, replaced, or removed. Do not hide failures with automatic retries or leave skipped / `xfail` tests unowned.
- Maintain tests based on failure rate, diagnosis cost, and runtime—not line counts or test-to-code ratios. When runtime grows, investigate slow tests and shared setup first.

### Local and CI Commands

See [Testing and Verification](./testing.md) for local commands and CI coverage.

## Version Records

For PRs with **user-visible changes**, add a changelog fragment under `changelog.d/` instead of editing `CHANGELOG.md`, `res/version.json`, or version numbers directly.

- **Naming & format**: Name the file `<PR number or branch>.<category>.md`. Put `project: <key>` on the first line (optional only for `dev` fragments), followed by a single user-facing sentence (max 50 characters) without project prefixes, PR numbers, attribution, or internal details.
- **Helper command**: From the repository root, run:

  ```powershell
  python scripts/changelog.py add <category> <project> "<one-line summary>"
  ```

- **When to skip**: Pure documentation, CI, test, and internal refactor PRs should use the `skip-changelog` PR label. For a feature first introduced in the current release cycle, follow-up tweaks within the same beta do not need a fragment; changes across betas should add `beta-only: true` (automatically omitted when promoting to stable). See the [fragment guide](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/changelog.d/README.md) for project keys, categories, and exceptions.
- **Compilation**: GitHub Actions compiles merged fragments into the Unreleased section of `CHANGELOG.md` daily at 23:30 Beijing time, and release preparation handles remaining fragments and version numbers.

## Backend Comments

::: tip Overview
Backend docstrings follow the **Google Python Style Guide – Docstring Conventions** (**Google Docstring**).
:::

### Method Comments

```python
def add_numbers(a: int, b: int) -> int:
    """Add two integers and return their sum.

    Args:
        a: First integer.
        b: Second integer.

    Returns:
        Sum of the two integers.

    Raises:
        ValueError: Raised when either argument is not an integer.
    """

    if not all(isinstance(x, int) for x in (a, b)):
        raise ValueError("arguments must be integers.")
    return a + b
```

### Configuration Class Comments

Configuration objects inherit from `ConfigEntry`, and semantic field groups inherit from `ConfigGroup`. Declare field types via type annotations and document defaults, constraints, and descriptions with `Field`. Use `default_factory` for nested groups and mutable fields so instances do not share state.

```python
from typing import Literal

from pydantic import Field

from app.config import ConfigEntry, ConfigGroup


class GlobalConfig(ConfigEntry):
    """Example declaration of global configuration."""

    class FunctionFields(ConfigGroup):
        HistoryRetentionTime: Literal[7, 15, 30, 60, 90, 180, 365, 0] = Field(
            default=0, description="History retention in days; 0 means no cleanup"
        )
        IfAllowSleep: bool = Field(default=False, description="Whether sleep is allowed")
        IfSilence: bool = Field(default=False, description="Whether silent mode is enabled")

    class NotifyFields(ConfigGroup):
        IfSendStatistic: bool = Field(default=False, description="Whether to send runtime results")

    Function: FunctionFields = Field(default_factory=FunctionFields)
    Notify: NotifyFields = Field(default_factory=NotifyFields)
```

Declare configuration fields and validation rules once in the configuration model—API input/output contracts are derived from it, and runtime-only state is never persisted. See [Configuration Framework](../core/config.md) for details.

## Backend Function Calls

::: tip Overview
Use positional arguments only when the meaning is obvious at a glance; otherwise use keyword arguments. Boolean and multi-argument calls must use keyword arguments.
:::

Follow these rules to keep function calls readable and safe against argument-order mistakes:

### 1. Simple functions with obvious meaning -> positional arguments

Acceptable when there are at most two parameters and their roles are unambiguous:

```python
add(1, 2)
math.sqrt(4)
```

### 2. Less obvious or easily confused calls -> keyword arguments

Explicit parameter names improve readability and prevent ordering bugs:

```python
UserItem(user_id=str(uid), name=config.get("Info", "Name"), status="waiting")
```

### 3. All boolean parameters -> keyword arguments required

Avoid opaque `True` / `False` flags:

```python
# Not recommended
send_notification(True, False)

# Recommended
send_notification(enable=True, urgent=False)
```

### 4. More than two parameters -> prefer keyword arguments

Improves maintainability and reduces argument-order errors:

```python
connect(host="localhost", port=3306, timeout=5)
```
