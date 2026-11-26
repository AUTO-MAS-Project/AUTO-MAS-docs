# Testing and Verification

AUTO-MAS uses four layers of testing and checks. Choose the layers that match your changes:

| Layer | Covers | Local Command | In CI |
| --- | --- | --- | --- |
| **Pytest (Backend)** | Backend contracts, config round-trips, scheduling, and plugin loading | `uv run --no-sync python -m pytest tests -q` | Yes; full suite on Windows and Ubuntu for PRs targeting `dev` |
| **Frontend Checks & Unit Tests (`vp`)** | Linting, formatting, and Vitest unit tests | `vp check`, `vp test` (in `frontend/`) | Yes; format and check workflow |
| **Playwright (Browser E2E)** | End-to-end UI flows through real HTTP/WebSocket | `vp run e2e` (in `frontend/`) | Launcher contract and seed self-check only |
| **Local Real Run (`@real`)** | Real scripts, emulators, accounts, and live scheduling | `vp run e2e:real` (in `frontend/`) | Author's machine only |

## Pytest (Backend Tests)

**Pytest** drives backend automated testing in isolated environments and displays detailed actual-vs-expected diffs on failure.

### Scope

Covers deterministic backend behavior: configuration persistence and reloads, API requests and responses, scheduling state transitions, path validation, and plugin loading. PRs targeting `dev` run the full suite on Windows and Ubuntu after preparing locked plugin packages (see the [Python Tests workflow](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/python-tests.yml)).

### Running Pytest

Run these commands from the repository root.

Before your first run, install dev dependencies and prepare locked test plugin packages:

```powershell
uv sync --locked --group dev
uv run --no-sync python scripts/prepare_test_plugins.py
```

Then run either the full suite or a single test file:

```powershell
# Full regression suite
uv run --no-sync python -m pytest tests -q

# Single module (replace with the target test file)
uv run --no-sync python -m pytest tests/api/test_config_roundtrip.py -q
```

### Organization & Guidelines

- Place tests for `app/<module>/` under `tests/<module>/`.
- Test plugin-internal behavior in the plugin's own repository; test host–plugin integration in the main repository using locked released plugin packages.
- Prefer reusing existing tests. When adding tests, start from a reproducible bug or public contract, assert user-observable results, and keep real accounts, networks, and devices outside the isolation boundary. Regression tests for bugs should fail before the fix and pass after.
- See [Test Submission and Maintenance](./development-specifications.md#test-submission-and-maintenance) for retention and PR evidence rules.

## Frontend Checks & Unit Tests (`vp`)

The frontend uses `vp` (Vite+) as its unified toolchain for code quality checks, formatting, and unit tests. Run these commands from `frontend/`:

| Task | Command | Notes |
| --- | --- | --- |
| Lint & format check | `vp check` | Runs code quality and format checks in one step |
| Lint or format individually | `vp lint` / `vp fmt` | Run linting or formatting on demand |
| Frontend unit tests | `vp test` | Runs Vitest unit tests (`*.test.ts` files live next to the modules they test) |

Before opening a PR with frontend changes, run `vp check` and `vp test` locally.

## Playwright (End-to-End Tests)

**Playwright** launches both the dev frontend and the real AUTO-MAS backend to verify complete user workflows end to end.

### Scope

Covers UI-to-backend flows such as page navigation, configuration save and failure rollback, queue editing, history filtering, and WebSocket reconnection. Windows desktop mascot interactions are covered by `@mascot` (see [frontend/TESTING.md](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/frontend/TESTING.md)).

On `dev` PRs, CI checks the E2E launcher, configuration isolation, and seed rules (see the [Format Check workflow](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/format-check.yml)); developers run relevant browser E2E tags locally.

### Setup & Running {#playwright-setup}

1. **Install dependencies**

   ```powershell
   uv sync --locked --group dev
   cd frontend
   vp install --frozen-lockfile
   vp exec playwright install chromium
   ```

2. **Run E2E tests** (from `frontend/`)

   | Task | Command |
   | --- | --- |
   | Check launcher & E2E types | `vp run e2e:check` |
   | Run all non-`@real` tests | `vp run e2e` |
   | Run navigation smoke tests | `vp run e2e:smoke` |
   | Filter by tags | `vp run e2e --grep '@queue\|@scheduler'` |
   | Verify Windows mascot window | `vp run e2e --grep '@mascot'` |
   | Debug in Playwright UI | `vp run e2e:ui` |
   | Open browser HTML report | `vp run e2e:report` |

   Common tags include `@init` (cold start), `@queue`, `@scheduler`, `@settings`, `@history`, `@recovery` (reconnection), and `@notify` (displays a real Windows system notification). Browse [frontend/e2e](https://github.com/AUTO-MAS-Project/AUTO-MAS/tree/dev/frontend/e2e) for all available specs and tags.

3. **Logs & Debugging**

   By default, the launcher creates a temporary data root for runtime data and backend logs, deleting it when the run finishes. Failure screenshots, traces, videos, and HTML reports are saved under `frontend/test-results/browser/` and `frontend/playwright-report/browser/`.

   | Debugging Need | How to Use |
   | --- | --- |
   | Keep configs, history, or backend logs | Set `$env:AUTO_MAS_E2E_KEEP_ROOT = '1'` in your terminal before running. The launcher prints the temporary root path so you can inspect `debug/app.log`. |
   | Reuse a dedicated debug data directory | Pass `--in-place` or set `AUTO_MAS_E2E_IN_PLACE=1`, and point `AUTO_MAS_E2E_DATA_ROOT` to that directory (defaults to the current checkout). In-place mode writes and keeps `config/`, `data/`, `history/`, and `debug/`. |

### Organization & Guidelines

- E2E specs live in `frontend/e2e/*.pw.ts`, grouped with `test.describe('@tag', ...)`. Reuse page entries, assertions, and evidence helpers from `fixtures.ts`.
- Always invoke tests through the `vp run` scripts above (which route through `run.mjs`) rather than calling `playwright test` directly.
- Build test state through the UI or public APIs first; rely on launcher seeding only for data that must exist before backend startup.
- Assert user-observable states, API responses, or WebSocket events. Verify persistence by comparing config copies, navigating away and back, or restarting the backend when required. After adding or modifying a test, temporarily break the guarded behavior to verify the test fails, then restore and rerun.
- Follow [Test Submission and Maintenance](./development-specifications.md#test-submission-and-maintenance) for long-lived tests and PR records. For AI-assisted E2E selection and handoff conventions, see [mas-e2e](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.agents/skills/mas-e2e/SKILL.md).

## `@real` (Author-Local Real Run) {#real}

`@real` runs E2E flows on the PR author's own machine using pre-configured managed scripts, enabled accounts, and locally installed scripts, games, and emulators through AUTO-MAS's public entry points.

### Scope

Required when changes touch task scheduling, game adapters, or script adapters (see the [E2E checklist](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/docs/testing/e2e.md)). Pure frontend changes only require the corresponding standard E2E tags. Results are judged by the public task terminal state, account outcome, history entry, and cleanup state, and only cover the targets actually executed.

### Running `@real`

Complete the dependency steps under [Setup & Running](#playwright-setup), ensure PowerShell 7 (`pwsh`) is installed, and configure a runnable managed script with an enabled account (plus any referenced plan or emulator) in AUTO-MAS beforehand.

1. **Run a single target**

   ```powershell
   cd frontend                           # work from frontend/
   vp run e2e:real                       # run @real for the selected target
   vp run e2e:real --grep complete       # verify completion, reconnect, and history only
   vp run e2e:real --grep stop-restart   # verify stop, restart, and cleanup only
   ```

   By default, the launcher reads `config/scripts.yaml` from the checkout, picks the first managed script with an enabled account, and prints the chosen target. If that file is missing, restore configuration in AUTO-MAS first. In a Git worktree, set `AUTO_MAS_E2E_TEMPLATE_ROOT` to the main checkout holding your config.

   To target a specific entry, set these environment variables (non-existent targets fail immediately):

   | Environment Variable | Targets |
   | --- | --- |
   | `AUTO_MAS_E2E_SCRIPT_ID` | Managed script UUID |
   | `AUTO_MAS_E2E_USER_ID` | Account UUID |
   | `AUTO_MAS_E2E_ACCOUNT_NAME` | Account name in config |

2. **Batch runs across multiple targets**

   ```powershell
   vp run e2e:real:all --list                    # list runnable targets without executing
   vp run e2e:real:all --only MaaConfig,SrcConfig # run only specified adapters
   vp run e2e:real:all                           # run all enabled accounts sequentially
   ```

   Batch runs execute enabled accounts one by one with separate isolated data roots and bound devices; a failure on one target does not stop the rest.

3. **Reports & Summaries**

   - Single-run selected screenshots and `real-e2e-summary.json` are written to `frontend/test-results/real/`. Open the report from `frontend/` with `vp exec playwright show-report playwright-report/real`.
   - Batch outputs are saved per target under `frontend/test-results/real-batch/`; the HTML report reflects the last run.

### Guidelines & Safeguards

- **File structure**: Real specs live alongside other E2E specs in `frontend/e2e/` and must carry the `@real` tag. `run-real.ps1` invokes `run.mjs` for single runs, and `real-batch.mjs` orchestrates batch runs. Document installation, script, account, and device prerequisites when adding new specialized flows.
- **Isolation & side effects**: Default isolated mode copies only the required target config and strips account notification fields, but scripts, games, and emulators still execute in your real environment and **may modify game state or consume resources**. Batch runs always enforce per-target isolation.
- **Machine lock & cleanup**: Only one `@real` run can execute at a time per machine via a machine-wide lock. If a force-kill leaves a stale lock, confirm no real run is active before removing it. Wait for task termination, verify runtime idle state, and only close emulators launched by the test.
- **Sanitized PR evidence**: Record commands, commit SHA, environment, and outcomes per [PR Verification Record](./development-specifications.md#pr-verification-record), attaching reviewed summary fields and selected screenshots. Strip account identifiers, private paths, and credentials before sharing—never upload full raw reports. See [mas-e2e](https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.agents/skills/mas-e2e/SKILL.md) for handoff conventions.
