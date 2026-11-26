# Environment Setup

Set up a local AUTO-MAS development environment before working on the core application or debugging plugins.

## Technology Stack

### Backend

- **Python**: 3.12.x
- **Environment & dependency management**: uv (`pyproject.toml` + `uv.lock`)
- **Web framework**: FastAPI
- **Realtime communication**: WebSockets

### Frontend

- **Framework**: Vue 3 + TypeScript
- **UI component library**: Ant Design Vue 4.x
- **Desktop**: Electron
- **Unified toolchain**: Vite+ (`vp` CLI — integrates Vite, Node.js and package manager environment management, building, linting, formatting, and testing)
- **Routing**: Vue Router 4
- **State management**: Pinia 3.x

## Environment Requirements

You only need to install the 4 tools below. Python, Node.js, and Yarn are managed automatically by `uv` and `vp`, so you do not need to install or configure them separately.

### 1. uv (Backend Toolchain)

- **Installation**: Follow the [official uv installation guide](https://docs.astral.sh/uv/getting-started/installation/).
  - Windows (PowerShell): Run `winget install --id=astral-sh.uv -e`, then reopen your terminal.
- **Recommended version**: 0.12+
- **Note**: uv automatically installs and manages Python 3.12 (currently `3.12.13`) based on `.python-version`, so a separate Python installation is not required.

### 2. Vite+ / `vp` (Frontend Unified Toolchain)

- **Installation**: Follow the [official Vite+ documentation](https://viteplus.dev/).
  - Windows (PowerShell): Run `irm https://vite.plus/ps1 | iex` (or download and run `vp-setup.exe` from the website), then reopen your terminal.
  - macOS / Linux: Run `curl -fsSL https://vite.plus | bash`.
- **Note**: `vp` automatically manages the Node.js runtime and package manager (Yarn) required by the frontend, and provides unified commands for dependency installation, scripts, linting, formatting, and testing. **You do not need to install Node.js separately or run `corepack enable`.**

### 3. Git

- **Installation**: Download and install from the [Git website](https://git-scm.com/downloads).

### 4. Visual C++ Redistributable (Windows)

- **Installation**: Download and install the [latest supported Visual C++ Redistributable](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170).

## Quick Start

Run all commands from the repository root unless a step explicitly switches to `frontend`.

### 1. Clone the Project

::: code-group

```powershell [PowerShell]
git clone --branch dev https://github.com/AUTO-MAS-Project/AUTO-MAS.git
cd AUTO-MAS
```

```bash [Bash]
git clone --branch dev https://github.com/AUTO-MAS-Project/AUTO-MAS.git
cd AUTO-MAS
```

:::

When you are ready to make changes, create a development branch from the latest upstream `dev`. See [Git Branches](./development-specifications.md#git-branches).

### 2. Backend Environment Setup

::: code-group

```powershell [PowerShell]
uv --version
uv sync --locked
```

```bash [Bash]
uv --version
uv sync --locked
```

:::

`uv sync --locked` selects Python from `.python-version`, creates or reuses `.venv` in the repository root, and installs both runtime and development dependencies (including Ruff and pytest) strictly according to `uv.lock` without rewriting the lockfile.

In day-to-day development, you do not need to create or activate a virtual environment manually or run `pip install -r requirements.txt`:

- Select `.venv\Scripts\python.exe` as your IDE's Python interpreter.
- Run `uv sync --locked` again whenever you pull new changes to keep dependencies in sync.

### 3. Frontend Environment Setup

::: code-group

```powershell [PowerShell]
cd frontend
vp --version
vp install
```

```bash [Bash]
cd frontend
vp --version
vp install
```

:::

`vp install` automatically prepares the Node.js and Yarn environment required by the repository and installs all frontend dependencies.

If downloading the Electron binary is slow or fails, temporarily set the npmmirror mirror before installing:

::: code-group

```powershell [PowerShell]
$env:ELECTRON_MIRROR = "https://npmmirror.com/mirrors/electron/"
vp install
```

```bash [Bash]
ELECTRON_MIRROR="https://npmmirror.com/mirrors/electron/" vp install
```

:::

The PowerShell variable only lasts for the current terminal session, and the Bash prefix only applies to that single command. Neither modifies global system settings.

#### Common `vp` Commands for the Frontend

The frontend uses `vp` (Vite+) as its unified toolchain. Common commands (run from `frontend/`):

| Task | Command | Notes |
| --- | --- | --- |
| Install / sync dependencies | `vp install` | Delegates to the repository's Yarn version automatically; use `vp install --frozen-lockfile` in CI or for strict lockfile installs |
| Run project scripts | `vp run <script>` | Runs scripts defined in `package.json`, such as `vp run dev:frontend`, `vp run dev:fullstack`, and `vp run build` |
| Check & format code | `vp check` | Runs lint and format checks together (or run `vp lint` / `vp fmt` individually) |
| Run frontend unit tests | `vp test` | Runs Vitest unit tests |
| Run local dependency binaries | `vp exec <command>` | Invokes locally installed CLI tools, such as `vp exec playwright ...` |
| Inspect managed environment | `vp env` | Shows the Node.js and package manager versions currently managed by `vp` |

### 4. Start the App

::: tip Administrator Privileges

Run your terminal or IDE as administrator when debugging features that require elevated access, such as game or emulator control. The development backend does not elevate itself automatically.

If you use a JetBrains IDE or VS Code, launching the IDE as administrator grants elevated privileges to terminals opened inside it as well.

:::

Both options below run your local backend code. Choose whichever fits your workflow.

#### 4.1 Start Backend and Frontend Separately

Recommended when debugging both frontend and backend code, or when setting backend breakpoints in your IDE.

1. **Start the backend service**

   From the repository root:

   ::: code-group

   ```powershell [PowerShell]
   uv run --locked python main.py
   ```

   ```bash [Bash]
   uv run --locked python main.py
   ```

   :::

   When launched from source, the backend starts in development mode and listens on `http://127.0.0.1:36164` (open `http://127.0.0.1:36164/docs` to inspect the API; packaged builds use port `36163` by default). Keep this terminal running.

2. **Start the frontend service**

   Open a second terminal at the repository root and run:

   ::: code-group

   ```powershell [PowerShell]
   cd frontend
   vp run dev:frontend
   ```

   ```bash [Bash]
   cd frontend
   vp run dev:frontend
   ```

   :::

   The frontend dev server starts at `http://localhost:5173`, and the Electron window automatically connects to the running backend. Note that `vp run dev:frontend` only starts the frontend—make sure the backend is already running first.

#### 4.2 Start Backend and Frontend Together

If you do not need backend breakpoints and just want to launch the whole app quickly, run from the repository root:

```powershell
cd frontend
vp run dev:fullstack
```

On Windows, this command automatically downloads **AUTO-MAS Runtime** to start and supervise your local backend.

If downloading Runtime fails, you can download and specify it manually:

1. Check `res/runtime-version.txt` for the required Runtime version.
2. Download `auto-mas-runtime-<version>.exe` from the [AUTO-MAS-Runtime releases page](https://github.com/AUTO-MAS-Project/AUTO-MAS-Runtime/releases).
3. Set `AUTO_MAS_RUNTIME_EXE` in your terminal before starting the app:

   ```powershell
   $env:AUTO_MAS_RUNTIME_EXE = 'D:\Tools\AUTO-MAS-Runtime\auto-mas-runtime.exe'
   cd frontend
   vp run dev:fullstack
   ```

To restore automatic Runtime selection, run `Remove-Item Env:AUTO_MAS_RUNTIME_EXE` in that terminal and restart.

::: info Note
IDE backend breakpoints are not available in fullstack mode. Use [4.1 Start Backend and Frontend Separately](#_4-1-start-backend-and-frontend-separately) when debugging backend code with breakpoints.
:::

## Debugging in the IDE

In VS Code, select `.venv\Scripts\python.exe` as the Python interpreter and debug `main.py` with the repository root as the working directory. In a second terminal, run `vp run dev:frontend` inside `frontend/`.

Once the host is running, see [Local Plugin Development and Debugging](/en/developer/plugins/fundamentals/develop) for plugin entry points and hot reload.

You can save the backend launch configuration below to `.vscode/launch.json` (start VS Code as administrator if debugging features that require elevated privileges):

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Debug AUTO-MAS backend",
      "type": "debugpy",
      "request": "launch",
      "program": "${workspaceFolder}/main.py",
      "cwd": "${workspaceFolder}",
      "console": "integratedTerminal"
    }
  ]
}
```

## FAQ

### Q: Do I still need to activate the Python virtual environment?

**A**: No. `uv run --locked python main.py` runs inside the project virtual environment automatically. In your IDE, simply select `.venv\Scripts\python.exe`.

### Q: The `uv` command cannot be found?

**A**: Close and reopen your terminal after installing uv, then run `uv --version`. If it is still not found, check that its installation directory is in `PATH` using the [official uv installation guide](https://docs.astral.sh/uv/getting-started/installation/).

### Q: The `vp` command cannot be found?

**A**: Reopen your terminal after installing Vite+ and run `vp --version`. If it is still unavailable, verify that the Vite+ installation directory is on `PATH` or reinstall from the [official Vite+ documentation](https://viteplus.dev/). You can also run `vp env` to inspect the managed Node.js and package manager state.

### Q: `uv sync --locked` says the lockfile needs updating?

**A**: First verify that `pyproject.toml`, `uv.lock`, and `.python-version` come from the same checkout. If you intentionally updated dependency declarations, run `uv lock` at the repository root, verify with `uv sync --locked`, and commit both files together. Never delete `uv.lock` to bypass the error.

### Q: `vp install` fails when installing frontend dependencies?

**A**: Try the following steps:

1. Delete `frontend/node_modules` (keep the repository's `yarn.lock`).
2. Run `vp install` again (if you suspect a corrupted cache, run `yarn cache clean` inside `frontend/` first).
3. If the Electron binary download is slow or fails, configure `ELECTRON_MIRROR` as shown in [Frontend Environment Setup](#_3-frontend-environment-setup) and retry.

### Q: Backend startup or emulator control fails with insufficient permissions?

**A**: Features that control games or emulators require administrator privileges. Try:

- Opening PowerShell or Git Bash as administrator before starting the backend; or
- Starting VS Code / JetBrains IDE as administrator.

### Q: The Electron window fails to open?

**A**: Check the following in order:

1. Confirm the frontend dev server is running at `http://localhost:5173`.
2. Confirm the backend is running at `http://127.0.0.1:36164/docs`. Use `vp run dev:frontend` when starting the frontend separately.
3. Inspect the terminal output for error messages.

---

**Thank you for contributing to AUTO-MAS!** 🎉
