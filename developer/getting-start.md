
# 开发起步

欢迎参与 AUTO-MAS 项目开发！本文档将帮助您快速搭建开发环境并了解项目结构。

---

## 🛠 技术栈

### 后端
- **Python**: 3.12+（推荐3.13）
- **Web 框架**: FastAPI 0.116.1
- **异步运行时**: Uvicorn 0.35.0
- **数据验证**: Pydantic 2.11.7
- **日志系统**: Loguru 0.7.3
- **实时通信**: WebSockets 15.0.1
- **图像处理**: OpenCV, Pillow, PyAutoGUI
- **OCR**: RapidOCR (ONNX Runtime)

### 前端
- **框架**: Vue 3 + TypeScript
- **UI 组件库**: Ant Design Vue 4.x
- **桌面端**: Electron 37.x
- **构建工具**: Vite 7.x
- **路由**: Vue Router 4
- **状态管理**: Pinia 3.x

---

##  环境要求

### 必需软件

1. **Python 3.12+（推荐3.13）**
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
5. VC运行库
	- 下载地址: [最新受支持的 Visual C++ 可再发行程序包](https://learn.microsoft.com/zh-cn/cpp/windows/latest-supported-vc-redist?view=msvc-170)

---

##  快速开始

### 1. 克隆项目

```bash
git clone https://github.com/AUTO-MAS-Project/AUTO-MAS.git
cd AUTO-MAS
```

### 2. 后端环境搭建

#### 2.1 创建虚拟环境 (推荐)

```powershell
# Windows PowerShell
python -m venv .venv
.\.venv\Scripts\Activate.ps1
```

#### 2.2 安装 Python 依赖

```bash
pip install -r requirements.txt
```

#### 2.3 启动后端服务（非必要，仅后端）

```bash
python main.py
```

::: tip 启动逻辑

MAS无需先启动后端，在启动前端后（见后文），*会自动从**Github**上拉取**dev**分支上的后端*

后端服务将在 `http://localhost:36163` 启动。

> **注意**: 程序需要管理员权限运行。如果没有管理员权限，程序会提示重新启动。
>
> 如果你使用jetbrains IDE，你可以直接以管理员权限启动IDE，这样打开的命令行程序也具有管理员权限

:::

### 3. 前端环境搭建

#### 3.1 进入前端目录

```bash
cd frontend
```

#### 3.2 安装依赖

```bash
yarn install
```

#### 3.3 启动开发服务器

**启动环境**（无需启动后端）:

```bash
yarn dev
```

前端开发服务器将在 `http://localhost:5173` 启动，Electron 窗口会自动打开。

此后，进行开发，只需要启动`yarn dev`即可

---

## 📁 项目结构

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

---

##  常见开发方向

#### 日志记录

```python
from app.utils import get_logger

logger = get_logger("模块名")

logger.info("信息日志")
logger.warning("警告日志")
logger.error("错误日志")
logger.debug("调试日志")
```

#### 配置管理

```python
from app.core import Config

# 获取配置
config = await Config.get_config()

# 保存配置
await Config.save_config(new_config)
```

### 前端开发

#### 页面开发流程

1. **在 `frontend/src/views/` 创建页面组件**

```vue
<template>
  <div class="my-page">
    <h1>{{ title }}</h1>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const title = ref('My Page');
</script>

<style scoped>
.my-page {
  padding: 20px;
}
</style>
```

2. **在 `frontend/src/router/index.ts` 注册路由**

```typescript
{
  path: '/my-page',
  name: 'MyPage',
  component: () => import('@/views/MyPage.vue')
}
```

---

##  调试与测试

### 后端调试

#### 查看日志

日志文件位于 `debug/app.log

### 前端调试

#### 浏览器开发者工具

- 按 `F12` 打开开发者工具
- 或在应用菜单中选择"开发者工具"

#### Electron 主进程调试

```bash
# 查看主进程日志
yarn dev
```

主进程日志会在终端输出。

---

##  代码规范

请见[开发者规范](/developer/development-specifications.md)

---

##  构建与发布

### 开发构建

```bash
cd frontend
yarn build
```

构建产物位于 `frontend/dist/`

### 发布流程

1. 更新版本号
   - `pyproject.toml` 中的 `version`
   - `frontend/package.json` 中的 `version`
   - `app/__init__.py` 中的 `__version__`

2. 更新 `res/version.json`

3. 提交并打 tag
   ```bash
   git add .
   git commit -m "chore: release v5.0.0"
   git tag v5.0.0
   git push origin main --tags
   ```

5. 在 GitHub 上创建 Release 并上传构建产物

---

## ❓ 常见问题

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

---

## 🤝 贡献指南

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'feat: Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 提交 Pull Request

### Pull Request 检查清单

- [ ] 代码遵循项目规范
- [ ] 已运行代码检查和格式化
- [ ] 已测试新功能/修复
- [ ] 已更新相关文档
- [ ] 提交信息清晰明确

---

## 📞 联系与支持

- **QQ 交流群**: [957750551](https://qm.qq.com/q/bd9fISNoME)
- **GitHub Issues**: [提交问题](https://github.com/AUTO-MAS-Project/AUTO-MAS/issues)
- **官方文档**: [https://doc.auto-mas.top](https://doc.auto-mas.top)
- **官方网站**: [https://auto-mas.top](https://auto-mas.top)

---

## 📄 许可证

本项目采用 [GPL-3.0 License](LICENSE) 开源。

在使用、分发或修改本软件时，请务必遵守 GPL-3.0 协议和项目 README 中的补充条款。

---

**感谢您对 AUTO-MAS 项目的贡献！** 🎉
