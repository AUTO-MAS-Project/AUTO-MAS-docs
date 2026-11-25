# MFW Runtime Environments

`MaaFW/tools/core/runtime_pool/` maintains the runner's base virtual environment, Python bindings, and native libraries. `MaaFW/tools/core/agent_env/` chooses how to launch each project's own agent. The binding version used by both the runner and the agent must match the MaaFramework native library loaded at run time.

## Choosing the Agent Interpreter

Determined by `MaaFW/tools/core/agent_env/planner.py`.

| Launch mode | When used | Notes |
| --- | --- | --- |
| `project_python` | Project ships `python/python.exe` | A bundled interpreter may be a specific version and its `site-packages` may contain packages omitted from dependency manifests |
| `isolated_venv` | Bundled Python is declared but missing, or bare `python` / `python3` / `pythonw` is specified ([#793][793]) | Windows `CreateProcess` searches the parent process's directory and parent `PATH` first, ignoring the child's `PATH`; bare `python` would always resolve to an interpreter without `maa` installed |
| `project_binary` | Native agent (Go / C++ executable) | — |
| `external` | Everything else (such as `node`) | — |

- Having no Python in the project directory is not an error.
- Isolated venvs are keyed by a hash of the project path and install only what `requirements.txt` lists; **when `requirements.txt` is absent, a single `maafw` entry matching the native library version is added** ([#884][884]), whereas manifests that exist without listing `maafw` are not appended to. This supports MFW-PyQt6 "embedded agent" projects that freeze `maa` into the shell and ship no dependency manifest.
- When a bundled Python is embeddable and lacks pip, `uv pip install --python <exe>` is used instead.
- The agent environment strips host-specific variables (`ISOLATED_HOST_KEYS` in `runtime_pool/host_environment.py`), sets `PYTHONUTF8` and `PYTHONUNBUFFERED=1`, and sets `PYTHONPYCACHEPREFIX` to `.pycache` under the project root (skipped when doing so would exceed `MAX_PATH`).

## Pinning Dependency Versions

During environment preparation, `runner/environment.py` determines the MaaFramework version.

- **Pinning isolated environments**: `maafw` in an isolated venv is pinned to the **exact version** of the native library loaded by the runner rather than copying the range from `requirements.txt` (`resolve_project_maafw_requirement` in `runner/environment.py`). Because `maafw` 5.13.0 bumped the Agent protocol version from 7 to 8, letting pip choose freely could land on an incompatible protocol version.
- **Matching host-supplied native libraries**: Projects without bundled native libraries are also pinned to the exact version ([#1294][1294]) to prevent protocol mismatches; such projects rebuild their isolated venv once after upgrading.
- **Recording the dependency manifest**: The pinned version is included in the venv manifest hash so any change triggers an automatic rebuild, sparing users from manually deleting broken venvs.
- **Handling bundled interpreters**: The binding inside `project_python` is also aligned to the native library version, strictly for **embedded views and update staging directories**; user-owned directories only log a note and are never modified ([#885][885]), preventing project deployment scripts from upgrading `maafw` to PyPI latest and breaking protocol compatibility.
- **Handling duplicate installation metadata**: When multiple `maafw-*.dist-info` directories remain in `site-packages`, non-highest versions are moved aside before adjustment and deleted only after pip succeeds and the re-read version matches; otherwise they are moved back intact (`_stale_maafw_dist_infos` in `agent_env/env.py`, [#1024][1024]). Because pip reads the first directory entry while MAS picks the highest version, leaving duplicates causes disagreement; restoring on failure keeps offline preparation working.
- **Handling alignment failures**: Any failure during alignment only logs a warning and does not block preparation, allowing offline runs to proceed.

## Native Library Selection

| Design | Why |
| --- | --- |
| `project_maafw_runtime_path` in `runner/environment.py`: prefers project-bundled libraries, prioritizing the host RID and verifying architecture via PE headers; if multiple host-loadable copies exist, picks the highest version ([#1024][1024], [#1140][1140]) | Sorting by name could select `arm64` on x64 hosts, and leftover legacy layouts would otherwise win |
| Single source of truth for host architecture is `host_architecture()` in `runtime_pool/architecture.py`, based on the interpreter's build platform; non-x64 hosts emit a single pre-run notice ("only x64 is currently supported") with no arm64 adaptation ([#1102][1102], [#1117][1117]) | `platform.machine()` on Python 3.12 can report the wrong architecture under emulation |
| Plugin directories skip DLLs whose PE header does not match the worker's architecture; explicitly named plugins with mismatched architecture fail immediately without retry ([#1102][1102]) | One incompatible DLL in a multi-arch release should not fail the entire round |
| Runner and agent use the same native library: when bundled Python has no `maa/bin`, MAS injects `MAAFW_BINARY_PATH` pointing to the runner's copy and prepends it to the agent's `PATH` (`project_python_agent_binary_path` in `agent_env/env.py`, [#1143][1143]) | When two native libraries coexist in a project, MAS and the project agent could pick them in opposite order and hit a protocol mismatch |
| Native agents always load from `<project>/maafw` and cannot be redirected: if the runner picked a different library with a different version, MAS reports a mixed-library error before starting the agent | Avoids waiting out the full connection timeout |
| `Protocol version mismatch` in agent output fails immediately, reporting both sides' versions and protocol numbers ([#1140][1140]) | Previously waited the full 600 seconds |
| Health checks on bundled Python use the exact same native library the agent uses when running ([#1082][1082]) | Some releases keep libraries outside `maa/bin` and point to them in the agent; otherwise health checks always produce false failures |
| When a health check fails, the UI shows only the last line of the traceback, while full output goes to detail logs ([#1088][1088]) | Task results and notifications display only the first line |

## Runtime Pool

Located at `config/maafw_runtime_pool/`.

| Design | Why |
| --- | --- |
| The runner has a single base venv keyed by the constant package set `BASE_RUNTIME_PACKAGES` (excluding `maafw`) + host Python identity; **never installs a project's `requirements.txt`** | The worker does not execute project code; previously the same `maafw` version fragmented into multiple venvs due to differing dependencies |
| Bindings are stored per version in `bindings/maafw-<version>/` (pure Python, with only a placeholder in `maa/bin`); projects without bundled native libraries use official native libraries under `native/maafw-<version>/` (`ensure_binding` in `runtime_pool/binding.py`) | The worker selects the binding matching the project's native library version |
| Binding resolution order: official platform wheel from PyPI / mirrors → self-built package from the MaaFramework Git tag source (meaningful only for projects with bundled native libraries) → skip upgrading if neither works; if all candidates fail to connect, report a network error without falling back to source ([#901][901]) | Some projects shipped a `maafw` version absent from PyPI |
| Self-built wheels must include a `maa/bin` placeholder and keep the `maaagentbinary` dependency | `import maa` opens `<maa>/bin`, and `MaaAgentBinary` is the default ADB controller agent path whose removal silently drops touch and screencap components |
| Selecting versions for ranges or bare `maafw`: remembers each project's last selected version and reuses it if still compatible; otherwise picks the highest compatible local version, then the highest compatible index version (prereleases considered only when explicitly allowed by the range) | Prevents silent drift when sibling projects or indexes publish newer versions |
| `MAAFW_BINARY_PATH` must come from the same source as the native library actually opened by the runner | Bindings reset the path before every initialization |
| GC: computes the active version set across all MFW managed scripts; unused bindings/native libraries are deleted after a 24-hour grace period; in-process reference counts span entire runs; deletions always rename first; the entire pass skips if the managed-script table did not load completely | Deleting lazily loaded modules mid-run breaks execution, and an empty table would treat everything as orphaned |
| When the host is embeddable Python, never use it to bootstrap a venv ([#531][531]) | Copying embeddable Python into a venv omits `._pth`, causing registry fallback to mix in system Python extension modules |
| Under Runtime supervision, reuses its injected `uv`, cache, and mirror list without clearing the shared cache | The main program also depends on that cache |
| Proxy: managed-script `Update.ProxyAddress` falls back to global when empty and also governs `uv` / `pip` installs; loopback addresses bypass the proxy ([#943][943]) | Proxies configured in MAS must be respected |

### Isolated Environment Sharing

`agent_env/venv_share.py` derives an environment key from the resolved dependency manifest, MaaFramework version, and host interpreter identity. Views with identical dependencies share one isolated environment. Manifests referencing local files stay isolated per view to prevent identically named relative paths or wheels from colliding across projects.

## Environment Confirmation and Caching

| Design | Why |
| --- | --- |
| Preparation results are cached by project fingerprint (`MaaFW/tools/embedded/env_cache.py`): includes `interface.json`, dependency manifests, lockfiles, bundled native library version, binding metadata names, and host interpreter identity; cache hits also require the recorded interpreter and venv to still exist ([#610][610]) | Re-confirming on every page visit wastes work, while swapping the bundled Python must invalidate old results |
| Cache checks existence only, not contents; safety nets are a pre-run interpreter ABI check and the edit page's "Prepare runtime environment" button forcing a full run | Deliberate cost tradeoff |
| Environment confirmation runs after pre-run updates (**even when the update fails**) and after every version switch; when normal wrap-up syncs to a group version, confirmation runs after the report is sent without starting new downloads during wrap-up | Rebuilding an isolated venv takes minutes and requires network; deferring it to run time leaves the emulator and game running while tasks look stuck |
| Edit page classifies outcomes into "Ready / Updated / Up to date / Failed" | Conclusion is highlighted on the last log line |

## Limitations

- Only x64 is supported.
- If a project's agent installs dependencies itself on first run, it may exceed the runner's connection timeout window.
- Packages manually installed by the user into a bundled interpreter at import time are cleared on the first full-package update; MAS does not reinstall them (see [MFW Project Updates](./update.md)).

[531]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/531
[610]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/610
[793]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/793
[884]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/884
[885]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/885
[901]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/901
[943]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/943
[1024]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1024
[1082]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1082
[1088]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1088
[1102]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1102
[1117]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1117
[1140]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1140
[1143]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1143
[1294]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1294
