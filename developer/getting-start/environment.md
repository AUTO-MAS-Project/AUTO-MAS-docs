# 环境配置

开发主程序或调试插件之前，需要先配置本地 AUTO-MAS 开发环境。

## 技术栈

### 后端

- **Python**: 3.12.x
- **环境与依赖管理**: uv（`pyproject.toml` + `uv.lock`）
- **Web 框架**: FastAPI
- **实时通信**: WebSockets

### 前端

- **框架**: Vue 3 + TypeScript
- **UI 组件库**: Ant Design Vue 4.x
- **桌面端**: Electron
- **构建工具**: Vite Plus（Vite）
- **路由**: Vue Router 4
- **状态管理**: Pinia 3.x

## 环境要求

### 必需软件

#### 1. uv

- **安装方式**： [uv 官方安装文档](https://docs.astral.sh/uv/getting-started/installation/)

  - Windows：运行 `winget install --id=astral-sh.uv -e`，安装后重新打开终端。

- **推荐版本**：0.12+

- **提示**：uv 会自动配置 `python 3.12.13`，无需单独安装。

#### 2. Node.js

- **安装方式**: 访问 [Node.js 官网](https://nodejs.org/) 下载并运行安装程序。
- **推荐版本**： 22.18+（推荐 Node.js 24 LTS）

#### 3. Yarn

- **安装方式**：安装 Node.js 后运行 `corepack enable`
- **提示**：运行该命令后，Node.js 会自动使用仓库声明的 Yarn 版本，无需额外操作。

#### 4. Git

- **安装方式**：前往 [Git 下载页](https://git-scm.com/downloads) 下载安装包后安装。

#### 5. VC运行库

- **安装方式**: 下载 [最新受支持的 Visual C++ 可再发行程序包](https://learn.microsoft.com/zh-cn/cpp/windows/latest-supported-vc-redist?view=msvc-170) 并运行安装。

## 快速开始

除明确进入 `frontend` 的步骤外，其余命令均在仓库根目录执行。

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

uv 会按 `.python-version` 选择 Python，在仓库根目录创建或复用 `.venv`，并按 `uv.lock` 安装依赖。开发依赖组默认一并安装，包含 Ruff、pytest 等工具。`--locked` 会检查锁文件与 `pyproject.toml` 是否一致，不会自动改写锁文件。

日常开发以 `pyproject.toml` 和 `uv.lock` 为准，无需再手动创建、激活虚拟环境或运行 `pip install -r requirements.txt`。IDE 的 Python 解释器请选择仓库中的 `.venv\Scripts\python.exe`。拉取代码后重新执行 `uv sync --locked`，即可同步依赖。

### 3. 前端环境搭建

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

如果 Electron 二进制下载缓慢或失败，可以在安装前指定 npmmirror 镜像：

::: code-group

```powershell [PowerShell]
$env:ELECTRON_MIRROR = "https://npmmirror.com/mirrors/electron/"
yarn install
```

```bash [Bash]
ELECTRON_MIRROR="https://npmmirror.com/mirrors/electron/" yarn install
```

:::

PowerShell 的设置在当前终端内生效，关闭终端后失效；Bash 的设置只影响本次命令。两种方式都不依赖尚未安装的 `cross-env`，也不修改全局配置。

### 4. 启动软件

::: tip 管理员权限

调试需要管理员权限的游戏或模拟器控制功能时，请以管理员身份运行终端或 IDE。开发环境中，后端不会自动提权。

如果你使用 jetbrains IDE 或 VScode，你可以直接以管理员权限启动 IDE，这样打开的命令行程序也拥有管理员权限。

:::

以下两种方式都运行当前检出的本地后端代码，任选一种即可。

#### 4.1 分别启动后端与前端

1. **启动后端服务**

::: code-group

```powershell [PowerShell]
uv run --locked python main.py
```

```bash [Bash]
uv run --locked python main.py
```

:::

从源码启动后端默认使用开发环境，默认监听 `http://127.0.0.1:36164`，可打开 `http://127.0.0.1:36164/docs` 查看接口。打包后的发行版默认使用端口 `36163`。

2. **启动前端服务**

请在仓库根目录打开一个新的终端并执行

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

前端开发服务器在 `http://localhost:5173` 启动，Electron 窗口会连接第一个终端中已运行的开发后端。

`yarn dev:frontend` 只启动前端，运行前请先确保后端已启动。

#### 4.2 一次性启动后端与前端

```powershell
cd frontend
yarn dev:fullstack
```

在 Windows 中，该命令会让前端自动获取 **AUTO-MAS Runtime**，以启动并管理后端。

若下载失败，可以手动下载并指定 Runtime：

1. 查看 `res/runtime-version.txt` 确认所需 Runtime 版本号
2. 前往[AUTO-MAS-Runtime 发布页](https://github.com/AUTO-MAS-Project/AUTO-MAS-Runtime/releases)，下载对应版本的 `auto-mas-runtime-<版本>.exe`。
3. 在调试终端设置 `AUTO_MAS_RUNTIME_EXE`，再启动前后端，参考命令如下：

   ```powershell
   $env:AUTO_MAS_RUNTIME_EXE = 'D:\Tools\AUTO-MAS-Runtime\auto-mas-runtime.exe'
   cd frontend
   yarn dev:fullstack
   ```

要恢复自动选择 Runtime，在调试终端执行 `Remove-Item Env:AUTO_MAS_RUNTIME_EXE` 后重新启动。

一次性启动后端与前端时无法调试后端断点，如果需要在 IDE 中调试后端断点，请[分别启动后端与前端](#_4-1-分别启动后端与前端)。

## 在 IDE 中调试

在 VS Code 中选择仓库 `.venv` 的 Python 解释器，使用仓库根目录作为工作目录调试 `main.py`。另开一个终端，在 `frontend/` 下执行 `yarn dev:frontend`。宿主启动后，插件入口和重载流程见[插件开发与调试](/developer/plugins/fundamentals/develop)。

可以把后端启动设置保存到 `.vscode/launch.json`。需要调试要求提权的功能时，以管理员身份运行 VS Code。

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

### Q: 还需要手动激活 Python 虚拟环境吗

**A**: 不需要。使用 `uv run --locked python main.py` 即可在项目环境中启动后端；IDE 请选择 `.venv\Scripts\python.exe`。

### Q: 提示找不到 uv 命令

**A**: 安装 uv 后重新打开终端，并运行 `uv --version` 检查。如果仍找不到命令，按 [uv 官方安装文档](https://docs.astral.sh/uv/getting-started/installation/) 检查安装路径是否已加入 PATH。

### Q: uv sync --locked 提示锁文件需要更新

**A**: 先确认 `pyproject.toml`、`uv.lock` 和 `.python-version` 来自同一份代码。如果是自己有意修改了依赖声明，在仓库根目录运行 `uv lock` 更新锁文件，再运行 `uv sync --locked` 验证，并将依赖声明与锁文件一起提交。不要为绕过报错删除 `uv.lock`。

### Q: yarn install 失败

**A**: 尝试以下方法:

1. 清理缓存: `yarn cache clean`
2. 删除 `node_modules`，保留仓库中的 `yarn.lock`
3. 重新安装: `yarn install`
4. Electron 二进制下载缓慢或失败时，按上文[前端环境搭建](#_3-前端环境搭建)设置镜像后重试。

### Q: 后端启动失败提示权限不足

**A**: AUTO-MAS 需要管理员权限运行。请以管理员身份运行:

- 以管理员身份打开 PowerShell 或 Git Bash
- 或在 VS Code 中以管理员身份启动

### Q: Electron 窗口无法打开

**A**: 检查:

1. 前端开发服务器是否正常运行（`http://localhost:5173`）
2. 确认后端已启动（`http://127.0.0.1:36164/docs`）。单独启动前端时使用 `yarn dev:frontend`。
3. 查看终端输出的错误信息

---

**感谢您对 AUTO-MAS 项目的贡献！** 🎉
