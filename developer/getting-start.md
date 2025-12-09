
# 开发起步

欢迎参与 AUTO-MAS 项目开发！本文档将帮助您快速搭建开发环境并了解项目结构。

## 重要事项

您通过任意方式提交代码到 **AUTO-MAS-Project** 下属任意仓库，即代表您理解并同意以下条款：

- 您授权 **AUTO-MAS-Project** 的开发团队 **AUTO-MAS Team** 使用您所提交代码的部分权利：

    1. **版权许可**  
    您授予 **AUTO-MAS Team** 一项永久的、全球性的、不可撤销的、免版税的、非独占的版权许可，允许其使用、复制、修改、分发、公开演示和展示您的贡献，并基于您的贡献创作衍生作品。

    2. **专利许可**  
    您同时授予 **AUTO-MAS Team** 一项永久的、全球性的、不可撤销的、免版税的专利许可，覆盖您贡献中所必需的专利权利。

    3. **保留权利**  
    您仍保有您原创贡献的版权和署名权。您的姓名将作为作者保留在 Git 提交历史及项目致谢中。

- **AUTO-MAS Team** 的所有权利均授权给 [DLmaster (@DLmaster361)](https://github.com/DLmaster361)，仅该被授权人能代表 **AUTO-MAS Team** 行使全部权利。补充细则如下：

    1. 无法联系到 [DLmaster (@DLmaster361)](https://github.com/DLmaster361) 时，**AUTO-MAS Team** 相关事务由 [AUTO-MAS 主要开发者](https://github.com/orgs/AUTO-MAS-Project/teams/core) 依据社区共识处理。
    
    2. [DLmaster (@DLmaster361)](https://github.com/DLmaster361) 退出项目时，必须将 **AUTO-MAS Team** 的所有授权移交给另一项目开发者。

    3. [AUTO-MAS 主要开发者](https://github.com/orgs/AUTO-MAS-Project/teams/core) 绝对多数通过时，允许取消对 [DLmaster (@DLmaster361)](https://github.com/DLmaster361) 的授权，由 [AUTO-MAS 主要开发者](https://github.com/orgs/AUTO-MAS-Project/teams/core) 重新授权给其他项目开发者。

以上条款用于保证 **AUTO-MAS-Project** 项目拥有清晰的责任主体，能够对侵权者采取法律手段进行法律维权，并确保 **AUTO-MAS Team** 的权利不会被滥用。

##  技术栈

### 后端
- **Python**: 3.12.x
- **Web 框架**: FastAPI 0.116.1
- **实时通信**: WebSockets 15.0.1
- **日志系统**: Loguru 0.7.3
- **图像处理**: OpenCV, Pillow, PyAutoGUI
- **OCR**: RapidOCR (ONNX Runtime)

### 前端
- **框架**: Vue 3 + TypeScript
- **UI 组件库**: Ant Design Vue 4.x
- **桌面端**: Electron 37.x
- **构建工具**: Vite 7.x
- **路由**: Vue Router 4
- **状态管理**: Pinia 3.x


##  环境要求

### 必需软件

1. **Python 3.12.x**
   - 下载地址: https://www.python.org/downloads/
   - 确保勾选 "Add Python to PATH"
2. **Node.js 18+**
   - 下载地址: https://nodejs.org/
   - 推荐使用 LTS 版本
3. **Yarn**(推荐)
   - 安装命令: `npm install -g yarn`
   - 使用 Corepack: `corepack enable`
4. **Git**
   - 下载地址: https://git-scm.com/downloads
5. **VC运行库**
	- 下载地址: [最新受支持的 Visual C++ 可再发行程序包](https://learn.microsoft.com/zh-cn/cpp/windows/latest-supported-vc-redist?view=msvc-170)


##  快速开始

### 1. 克隆项目

```bash
git clone https://github.com/AUTO-MAS-Project/AUTO-MAS.git
cd AUTO-MAS
```

### 2. 后端环境搭建

1. **创建虚拟环境 (推荐)**

```powershell
# Windows PowerShell
python -m venv .venv
.\.venv\Scripts\Activate.ps1
```

2. **安装 Python 依赖**

```bash
pip install -r requirements.txt
```

### 3. 前端环境搭建

**进入前端目录并安装依赖**

```bash
cd frontend
yarn install
```


### 4. 启动软件

::: tip 管理员权限

AUTO-MAS 前后端都需要以管理员权限运行。如果没有管理员权限，程序会提示重新启动。

如果你使用 jetbrains IDE 或 VScode，你可以直接以管理员权限启动 IDE，这样打开的命令行程序也具有管理员权限。

您也可以使用 `sudo` 命令以管理员权限运行。

:::

#### 4.1 使用来自远端的后端代码

**进入前端目录并启动开发服务器**

```bash
cd frontend
yarn dev
```

前端开发服务器将在 `http://localhost:5173` 启动，Electron 窗口会自动打开。

前端将自动完成后端初始化，后端来自 Github 的 **dev** 分支，后端本地文件位于 `frontend/node_modules/electron/dist` 下。

#### 4.2 使用本地的后端代码

1. **启动后端服务**

```bash
python main.py
```

后端服务将在 `http://localhost:36163` 启动，可以访问 `http://localhost:36163/docs` 查看接口文档。

2. **启动前端服务**

您需要开启一个新的终端窗口用于运行前端服务。

```bash
cd frontend
yarn dev
```

3. **跳过启动后端**

由于您已经打开用于开发的后端，前端初始化时将在 `启动后端` 步骤失败，此时直接选择 `跳过启动后端` 即可正常进入应用主界面，此时前端所使用的后端服务将是您之前终端窗口中启动的后端。


##  项目结构

```
AUTO-MAS/
├── app/                          # 后端核心代码
│   ├── api/                      # FastAPI 路由和接口
│   │   ├── core.py              # 核心 API
│   │   ├── dispatch.py          # 任务调度 API
│   │   ├── emulator.py          # 模拟器管理 API
│   │   ├── history.py           # 历史记录 API
│   │   ├── ocr.py               # OCR 相关 API
│   │   ├── plan.py              # 计划管理 API
│   │   ├── queue.py             # 队列管理 API
│   │   └── ...
│   ├── core/                     # 核心业务逻辑
│   │   ├── broadcast.py         # 广播系统
│   │   ├── config.py            # 配置管理
│   │   ├── emulator_manager.py  # 模拟器管理器
│   │   ├── task_manager.py      # 任务管理器
│   │   └── timer.py             # 定时器
│   ├── models/                   # 数据模型
│   │   ├── config.py            # 配置模型
│   │   ├── emulator.py          # 模拟器模型
│   │   ├── schema.py            # API Schema
│   │   └── task.py              # 任务模型
│   ├── services/                 # 外部服务
│   │   ├── matomo.py            # 统计服务
│   │   ├── notification.py      # 通知服务
│   │   ├── system.py            # 系统服务
│   │   └── update.py            # 更新服务
│   ├── task/                     # 任务实现
│   │   ├── general/             # 通用任务
│   │   └── MAA/                 # MAA 相关任务
│   └── utils/                    # 工具函数
│       ├── logger.py            # 日志工具
│       ├── ImageUtils.py        # 图像处理工具
│       ├── ProcessManager.py    # 进程管理
│       ├── emulator/            # 模拟器工具
│       ├── LogMonitor/          # 日志监控
│       └── OCR/                 # OCR 工具
│
├── frontend/                     # 前端代码
│   ├── src/                     # 源代码
│   │   ├── views/               # 页面组件
│   │   ├── components/          # 公共组件
│   │   ├── api/                 # API 调用，由插件生成
│   │   ├── router/              # 路由配置
│   │   ├── composables/         # 组合式函数
│   │   └── types/               # TypeScript 类型
│   ├── electron/                # Electron 主进程代码
│   │   ├── main.ts              # 主进程入口
│   │   ├── preload.ts           # 预加载脚本
│   │   └── ipc/                 # IPC 通信
│   └── public/                  # 静态资源
│
├── res/                          # 资源文件
│   ├── images/                  # 图片资源
│   ├── icons/                   # 图标
│   └── sounds/                  # 音效
│
├── main.py                       # 后端入口
├── requirements.txt              # Python 依赖
├── pyproject.toml               # Python 项目配置
└── README.md                    # 项目说明
```


##  常见问题

### Q: Python 虚拟环境激活失败

**A**: 如果在 PowerShell 中遇到执行策略错误:

```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### Q: yarn install 失败

**A**: 尝试以下方法:

1. 清理缓存: `yarn cache clean`
2. 删除 `node_modules` 和 `yarn.lock`
3. 重新安装: `yarn install`

### Q: 后端启动失败提示权限不足

**A**: AUTO-MAS 需要管理员权限运行。请以管理员身份运行:

- 右键点击 CMD/PowerShell，选择"以管理员身份运行"
- 或在 VS Code 中以管理员身份启动

### Q: Electron 窗口无法打开

**A**: 检查:

1. 前端开发服务器是否正常运行（`http://localhost:5173`）
2. 后端服务是否正常启动（`http://localhost:36163`）
3. 查看终端输出的错误信息


##  许可证

本项目采用 [GPL-3.0 License](https://www.gnu.org/licenses/gpl-3.0.txt) 开源。

在使用、分发或修改本软件时，请务必遵守 GPL-3.0 协议和项目 README 中的补充条款。

---

**感谢您对 AUTO-MAS 项目的贡献！** 🎉
