# 环境配置

开发主程序或调试插件前，请先配置好本地 AUTO-MAS 开发环境。

## 技术栈

### 后端

- **Python**：3.12.x
- **环境与依赖管理**：uv（`pyproject.toml` + `uv.lock`）
- **Web 框架**：FastAPI
- **实时通信**：WebSockets

### 前端

- **框架**：Vue 3 + TypeScript
- **UI 组件库**：Ant Design Vue 4.x
- **桌面端**：Electron
- **统一工具链**：Vite+（`vp` CLI，集成 Vite、Node.js 与包管理器环境管理、构建、代码检查、格式化与测试）
- **路由**：Vue Router 4
- **状态管理**：Pinia 3.x

## 环境要求

开始之前，只需安装以下 4 个基础工具。Python、Node.js 和 Yarn 都会由 `uv` 与 `vp` 自动管理，无需逐个手动安装配置。

### 1. uv（后端工具链）

- **安装方式**：参考 [uv 官方安装文档](https://docs.astral.sh/uv/getting-started/installation/)。
  - Windows（PowerShell）：运行 `winget install --id=astral-sh.uv -e`，安装完成后重新打开终端。
- **推荐版本**：0.12+
- **说明**：uv 会按仓库 `.python-version` 自动配置 Python 3.12（当前为 `3.12.13`），无需单独安装 Python。

### 2. Vite+ / `vp`（前端统一工具链）

- **安装方式**：参考 [Vite+ 官方文档](https://viteplus.dev/)。
  - Windows（PowerShell）：运行 `irm https://vite.plus/ps1 | iex`（也可直接从官网下载 `vp-setup.exe` 安装），安装完成后重新打开终端。
  - macOS / Linux：运行 `curl -fsSL https://vite.plus | bash`。
- **说明**：`vp` 会自动接管前端所需的 Node.js 运行环境与包管理器（Yarn），并统一执行依赖安装、脚本运行、代码检查、格式化和测试，**无需单独安装 Node.js 或手动执行 `corepack enable`**。

### 3. Git

- **安装方式**：前往 [Git 官网](https://git-scm.com/downloads) 下载安装包并安装。

### 4. Visual C++ 运行库（Windows）

- **安装方式**：下载并安装[最新受支持的 Visual C++ 可再发行程序包](https://learn.microsoft.com/zh-cn/cpp/windows/latest-supported-vc-redist?view=msvc-170)。

## 快速开始

除明确说明进入 `frontend` 目录的步骤外，其余命令均在仓库根目录执行。

### 1. 克隆项目

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

准备提交改动时，请从最新的上游 `dev` 新建开发分支，详见[开发规范](./development-specifications.md#git-分支)。

### 2. 后端环境搭建

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

`uv sync --locked` 会按 `.python-version` 准备好 Python，在仓库根目录创建或复用 `.venv`，并严格按 `uv.lock` 安装运行依赖和开发依赖（包含 Ruff、pytest 等），不会改动锁文件。

日常开发中，不需要手动创建或激活虚拟环境，也无需执行 `pip install -r requirements.txt`：

- 在 IDE 中，将 Python 解释器选择为仓库内的 `.venv\Scripts\python.exe`。
- 每次拉取最新代码后，重新执行 `uv sync --locked` 即可同步依赖。

### 3. 前端环境搭建

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

`vp install` 会自动准备前端所需的 Node.js 与 Yarn 环境，并按仓库配置安装所有依赖。

如果 Electron 二进制文件下载缓慢或失败，可在安装前临时指定 npmmirror 镜像：

::: code-group

```powershell [PowerShell]
$env:ELECTRON_MIRROR = "https://npmmirror.com/mirrors/electron/"
vp install
```

```bash [Bash]
ELECTRON_MIRROR="https://npmmirror.com/mirrors/electron/" vp install
```

:::

PowerShell 的设置在当前终端窗口内生效，关闭后自动失效；Bash 的写法只影响这一次命令，两者都不会修改系统全局配置。

#### 前端统一工具链 `vp` 常用命令

前端已统一使用 `vp`（Vite+）管理运行环境与开发任务。日常开发在 `frontend/` 目录下常用命令如下：

| 用途 | 命令 | 说明 |
| --- | --- | --- |
| 安装 / 同步依赖 | `vp install` | 自动调用项目声明的 Yarn 安装依赖；CI 或需严格按锁文件安装时使用 `vp install --frozen-lockfile` |
| 运行项目脚本 | `vp run <脚本名>` | 执行 `package.json` 中的脚本，如 `vp run dev:frontend`、`vp run dev:fullstack`、`vp run build` |
| 代码检查与格式化 | `vp check` | 一键执行代码检查与格式校验（也可按需单独运行 `vp lint` 或 `vp fmt`） |
| 运行前端单元测试 | `vp test` | 运行 Vitest 单元测试 |
| 调用本地依赖工具 | `vp exec <命令>` | 使用项目本地依赖中的命令行工具，如 `vp exec playwright ...` |
| 查看当前环境 | `vp env` | 查看当前由 `vp` 接管的 Node.js 和包管理器版本 |

### 4. 启动软件

::: tip 管理员权限

调试需要管理员权限的游戏或模拟器控制功能时，请以管理员身份运行终端或 IDE。开发环境下，后端不会自动提权。

如果使用 JetBrains IDE 或 VS Code，可以直接以管理员身份启动 IDE，这样在 IDE 内打开的终端也会自动具备管理员权限。

:::

下面两种启动方式都会运行你本地的后端代码，按需要选择一种即可。

#### 4.1 分别启动后端与前端

这种方式适合同时调试前后端代码，或在 IDE 中为后端打断点。

1. **启动后端服务**

   在仓库根目录执行：

   ::: code-group

   ```powershell [PowerShell]
   uv run --locked python main.py
   ```

   ```bash [Bash]
   uv run --locked python main.py
   ```

   :::

   从源码启动时，后端默认处于开发模式并监听 `http://127.0.0.1:36164`（可访问 `http://127.0.0.1:36164/docs` 查看接口文档；正式打包版默认使用 `36163` 端口）。保持该终端运行即可。

2. **启动前端服务**

   在仓库根目录新开一个终端，执行：

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

   前端开发服务器会在 `http://localhost:5173` 启动，随后打开的 Electron 窗口会自动连接上面运行中的后端。注意 `vp run dev:frontend` 只启动前端，运行前请先确认后端已启动。

#### 4.2 一次性启动后端与前端

如果不需要调试后端断点，只想快速把整个程序跑起来，可以在仓库根目录执行：

```powershell
cd frontend
vp run dev:fullstack
```

在 Windows 上，该命令会自动获取 **AUTO-MAS Runtime** 来启动并托管本地后端。

若 Runtime 自动下载失败，可以手动下载并指定路径：

1. 打开 `res/runtime-version.txt`，查看当前需要的 Runtime 版本号。
2. 前往 [AUTO-MAS-Runtime 发布页](https://github.com/AUTO-MAS-Project/AUTO-MAS-Runtime/releases)，下载对应版本的 `auto-mas-runtime-<版本>.exe`。
3. 在终端设置环境变量 `AUTO_MAS_RUNTIME_EXE` 后再启动前后端：

   ```powershell
   $env:AUTO_MAS_RUNTIME_EXE = 'D:\Tools\AUTO-MAS-Runtime\auto-mas-runtime.exe'
   cd frontend
   vp run dev:fullstack
   ```

如需恢复自动获取 Runtime，在当前终端执行 `Remove-Item Env:AUTO_MAS_RUNTIME_EXE` 后重新启动即可。

::: info 注意
一次性启动前后端时无法在 IDE 中命中后端断点。如需断点调试后端，请使用[4.1 分别启动后端与前端](#_4-1-分别启动后端与前端)。
:::

## 在 IDE 中调试

在 VS Code 中，将 Python 解释器选为仓库的 `.venv\Scripts\python.exe`，以仓库根目录为工作目录启动调试 `main.py`；另开一个终端，在 `frontend/` 下运行 `vp run dev:frontend`。

宿主启动后，插件入口与热重载调试方式见[插件开发与调试](/developer/plugins/fundamentals/develop)。

你可以把下面的后端调试配置保存到 `.vscode/launch.json`（如需调试涉及提权的功能，请以管理员身份启动 VS Code）：

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "调试 AUTO-MAS 后端",
      "type": "debugpy",
      "request": "launch",
      "program": "${workspaceFolder}/main.py",
      "cwd": "${workspaceFolder}",
      "console": "integratedTerminal"
    }
  ]
}
```

## 常见问题

### Q: 还需要手动激活 Python 虚拟环境吗？

**A**：不需要。直接用 `uv run --locked python main.py` 就会自动在项目虚拟环境中运行后端；在 IDE 中选择 `.venv\Scripts\python.exe` 即可。

### Q: 提示找不到 `uv` 命令？

**A**：安装完成后请先关闭并重新打开终端，再运行 `uv --version` 检查。如果仍然提示找不到，请参考 [uv 官方安装文档](https://docs.astral.sh/uv/getting-started/installation/) 检查安装路径是否已加入系统 `PATH`。

### Q: 提示找不到 `vp` 命令？

**A**：安装 Vite+ 后请重新打开终端，并运行 `vp --version` 检查。如果依然找不到，请确认 Vite+ 安装目录已加入系统 `PATH`，或参考 [Vite+ 官方文档](https://viteplus.dev/) 重新安装。运行 `vp env` 可进一步检查当前接管的 Node.js 与包管理器状态。

### Q: `uv sync --locked` 提示锁文件需要更新？

**A**：先确认 `pyproject.toml`、`uv.lock` 和 `.python-version` 来自同一份代码。如果是你主动修改了依赖声明，请在仓库根目录执行 `uv lock` 更新锁文件，再运行 `uv sync --locked` 校验，并将修改后的依赖声明与锁文件一并提交。不要为了绕过报错删除 `uv.lock`。

### Q: `vp install` 安装前端依赖失败？

**A**：可以按以下步骤排查：

1. 删除 `frontend/node_modules`（保留仓库中的 `yarn.lock`）。
2. 重新执行 `vp install`（如怀疑缓存异常，可先在 `frontend/` 下清理 Yarn 缓存 `yarn cache clean` 再重试）。
3. 如果卡在 Electron 二进制下载，请按上文[前端环境搭建](#_3-前端环境搭建)设置 `ELECTRON_MIRROR` 镜像后重试。

### Q: 后端启动或操作模拟器时提示权限不足？

**A**：涉及游戏或模拟器控制等功能时需要管理员权限。请尝试：

- 以管理员身份打开 PowerShell 或 Git Bash 后再启动后端；
- 或以管理员身份启动 VS Code / JetBrains IDE。

### Q: Electron 窗口无法打开？

**A**：请依次检查：

1. 前端开发服务器是否正常启动（`http://localhost:5173`）。
2. 后端服务是否已经运行（可访问 `http://127.0.0.1:36164/docs` 确认）。单独启动前端时请使用 `vp run dev:frontend`。
3. 终端中是否输出了具体报错信息。

---

**感谢您对 AUTO-MAS 项目的贡献！** 🎉
