# Environment Setup

Core and plugin development share this environment. Start the local AUTO-MAS host before developing or debugging a plugin.

## Technology Stack

### Backend

- **Python**: 3.12.x
- **Environment and dependency management**: uv (`pyproject.toml` + `uv.lock`)
- **Web framework**: FastAPI
- **Realtime communication**: WebSockets 15.0.1
- **Logging**: Loguru 0.7.3
- **Image processing**: OpenCV, Pillow, PyAutoGUI
- **OCR**: RapidOCR (ONNX Runtime)

### Frontend

- **Framework**: Vue 3 + TypeScript
- **UI component library**: Ant Design Vue 4.x
- **Desktop**: Electron
- **Build tool**: Vite Plus (Vite)
- **Routing**: Vue Router 4
- **State management**: Pinia 3.x

## Environment Requirements

### Required Software

1. **uv**
   - Follow the [official uv installation guide](https://docs.astral.sh/uv/getting-started/installation/). On Windows, run `winget install --id=astral-sh.uv -e`, then reopen your terminal.
   - The root `.python-version` file selects the Python version (currently 3.12.13). uv downloads it if it is missing, so a separate Python installation is not required.
2. **Node.js 22.18+**
   - Download: https://nodejs.org/
   - Node.js 24 LTS is recommended, matching the current frontend checks.
3. **Yarn** (recommended)
   - The project uses Yarn 4. Enable it through Corepack: `corepack enable`
   - After entering `frontend`, you can directly use the Yarn version declared by the repository.
4. **Git**
   - Download: https://git-scm.com/downloads
5. **VC runtime**
   - Download: [Latest supported Visual C++ Redistributable](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170)

## Quick Start

Commands below are provided for both PowerShell and Bash. Select the tab matching your terminal. Run commands from the repository root unless a step explicitly enters `frontend`.

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

Before making changes, create a development branch from the latest upstream `dev` as described in the [development specifications](./development-specifications.md).

::: tip Agent Development Prerequisite

If an AI assistant participates in development, first confirm that the main repository contains and loads `.agents/skills/mas-skills/SKILL.md`. If project-owned Skills are missing, complete the main repository content first. Do not let the AI assistant start work directly.

:::

### 2. Backend Environment Setup

**Sync the Python environment and dependencies**

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

uv selects Python from `.python-version`, creates or reuses `.venv` in the repository root, and installs dependencies from `uv.lock`. The development dependency group, including Ruff and pytest, is installed by default. `--locked` checks that the lockfile matches `pyproject.toml` without rewriting it.

Use `pyproject.toml` and `uv.lock` for development. You do not need to create or activate a virtual environment manually or run `pip install -r requirements.txt`. Select `.venv\Scripts\python.exe` as your IDE's Python interpreter. Run `uv sync --locked` again after pulling code to sync dependencies.

### 3. Frontend Environment Setup

**Enter the frontend directory and install dependencies**

::: code-group

```powershell [PowerShell]
cd frontend
yarn install
```

```bash [Bash]
cd frontend
yarn install
```

:::

If the Electron binary download is slow or fails, select the npmmirror source before installing:

::: code-group

```powershell [PowerShell]
$env:ELECTRON_MIRROR = "https://npmmirror.com/mirrors/electron/"
yarn install
```

```bash [Bash]
ELECTRON_MIRROR="https://npmmirror.com/mirrors/electron/" yarn install
```

:::

The PowerShell setting lasts for the current terminal session; the Bash setting affects only this command. Neither option requires `cross-env` to have been installed or changes global configuration.

### 4. Start the App

::: tip Administrator Privileges

Run the terminal or IDE as administrator when debugging features that require elevated access, such as game or emulator control. The development backend does not elevate itself.

If you use a JetBrains IDE or VS Code, start the IDE as administrator so terminals opened inside it also have administrator privileges.

:::

Both options below run backend code from your current local checkout. Choose one.

#### 4.1 Start Backend and Frontend Separately

1. **Start the backend service from the repository root**

::: code-group

```powershell [PowerShell]
uv run --locked python main.py
```

```bash [Bash]
uv run --locked python main.py
```

:::

The backend defaults to development mode when launched from source and does not read `.env`. It listens on `http://127.0.0.1:36164` by default. Open `http://127.0.0.1:36164/docs` to inspect the API and keep the terminal running. The packaged application uses port `36163` by default.

2. **Start the frontend service**

Open a new terminal at the repository root:

::: code-group

```powershell [PowerShell]
cd frontend
yarn dev:frontend
```

```bash [Bash]
cd frontend
yarn dev:frontend
```

:::

The frontend server starts at `http://localhost:5173`. The Electron window connects to the development backend running in the first terminal.

`yarn dev:frontend` starts only the frontend. Synchronize Python dependencies and start the backend first.

#### 4.2 Start Backend and Frontend Together

Open a terminal at the repository root and run:

```powershell
cd frontend
yarn dev:fullstack
```

On Windows, this option starts the frontend and uses Runtime to supervise the local development backend. Use the two-terminal method for backend breakpoints or on systems without Runtime.

## Debugging in the IDE

In VS Code, select the repository’s `.venv` Python interpreter and debug `main.py` with the repository root as the working directory. In a second terminal, run `yarn dev:frontend` from `frontend/`. Once the host is running, see [Local Plugin Development and Debugging](/en/developer/plugins/fundamentals/develop) for plugin entry points and reload.

To save the backend launch settings, add this configuration to `.vscode/launch.json`. Start VS Code as administrator if the features you are debugging require elevated access.

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

## Project Structure

See [Project Structure](/en/developer/core/project-structure) for the repository layout and module responsibilities.

## FAQ

### Q: Do I still need to activate the Python virtual environment?

**A**: No. Start the backend in the project environment with `uv run --locked python main.py`. Select `.venv\Scripts\python.exe` as your IDE's interpreter.

### Q: The uv command cannot be found

**A**: Reopen your terminal after installing uv, then run `uv --version`. If it is still unavailable, check that its installation directory is on PATH using the [official uv installation guide](https://docs.astral.sh/uv/getting-started/installation/).

### Q: uv sync --locked says the lockfile needs updating

**A**: First confirm that `pyproject.toml`, `uv.lock`, and `.python-version` come from the same checkout. If you intentionally changed dependency declarations, run `uv lock` from the repository root, then verify with `uv sync --locked`. Commit the dependency declarations and lockfile together. Do not delete `uv.lock` to bypass the error.

### Q: yarn install fails

**A**: Try:

1. Clear cache: `yarn cache clean`
2. Delete `node_modules`, keeping the repository's `yarn.lock`
3. Install again: `yarn install`
4. If the Electron binary download is slow or fails, select the mirror as shown in [Frontend Environment Setup](#_3-frontend-environment-setup), then retry.

### Q: Backend startup fails due to insufficient permissions

**A**: AUTO-MAS requires administrator privileges. Run as administrator:

- Open PowerShell or Git Bash as administrator
- Or start VS Code as administrator

### Q: Electron window cannot open

**A**: Check:

1. Whether the frontend dev server is running normally at `http://localhost:5173`
2. Confirm the backend is running at `http://127.0.0.1:36164/docs`. For a separate frontend, use `yarn dev:frontend`.
3. Error output in the terminal

## License

This project is open source under the [AGPL-3.0 License](https://www.gnu.org/licenses/agpl-3.0.txt).

When using, distributing, or modifying this software, comply with AGPL-3.0 and the supplementary terms in the project README.

---

Thank you for contributing to AUTO-MAS.
