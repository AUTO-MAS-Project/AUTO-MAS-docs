# Getting Started

Welcome to AUTO-MAS development. This document helps you quickly set up the development environment and understand the project structure.

## Technology Stack

### Backend

- **Python**: 3.12.x
- **Web framework**: FastAPI 0.116.1
- **Realtime communication**: WebSockets 15.0.1
- **Logging**: Loguru 0.7.3
- **Image processing**: OpenCV, Pillow, PyAutoGUI
- **OCR**: RapidOCR (ONNX Runtime)

### Frontend

- **Framework**: Vue 3 + TypeScript
- **UI component library**: Ant Design Vue 4.x
- **Desktop**: Electron 37.x
- **Build tool**: Vite 7.x
- **Routing**: Vue Router 4
- **State management**: Pinia 3.x

## Environment Requirements

### Required Software

1. **Python 3.12.x**
   - Download: https://www.python.org/downloads/
   - Make sure **Add Python to PATH** is selected.
2. **Node.js 18+**
   - Download: https://nodejs.org/
   - LTS is recommended.
3. **Yarn** (recommended)
   - The project uses Yarn 4. Enable it through Corepack: `corepack enable`
   - After entering `frontend`, you can directly use the Yarn version declared by the repository.
4. **Git**
   - Download: https://git-scm.com/downloads
5. **VC runtime**
   - Download: [Latest supported Visual C++ Redistributable](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170)

## Quick Start

### 1. Clone the Project

```bash
git clone https://github.com/AUTO-MAS-Project/AUTO-MAS.git
cd AUTO-MAS
```

::: tip Agent Development Prerequisite

If an AI assistant participates in development, first confirm that the main repository contains and loads `.agents/skills/mas-skills/SKILL.md`. If project-owned Skills are missing, complete the main repository content first. Do not let the AI assistant start work directly.

:::

### 2. Backend Environment Setup

1. **Create a virtual environment (recommended)**

```powershell
# Windows PowerShell
python -m venv .venv
.\.venv\Scripts\Activate.ps1
```

2. **Install Python dependencies**

```bash
pip install -r requirements.txt
```

### 3. Frontend Environment Setup

**Enter the frontend directory and install dependencies**

```bash
cd frontend
yarn install
```

### 4. Start the App

::: tip Administrator Privileges

Both the AUTO-MAS backend and frontend need to run with administrator privileges. Without administrator privileges, the app will ask to restart.

If you use a JetBrains IDE or VS Code, start the IDE as administrator so terminals opened inside it also have administrator privileges.

You can also use `sudo` where available.

:::

#### 4.1 Use Remote Backend Code

**Enter the frontend directory and start the development server**

```bash
cd frontend
yarn dev
```

The frontend dev server starts at `http://localhost:5173`, and the Electron window opens automatically.

The frontend automatically initializes the backend. The backend comes from the GitHub **dev** branch, and local backend files are under `frontend/node_modules/electron/dist`.

#### 4.2 Use Local Backend Code

Method 1: start backend and frontend separately.

1. **Start the backend service**

```bash
python main.py
```

The backend starts at `http://localhost:36163`. Visit `http://localhost:36163/docs` to view API documentation.

2. **Start the frontend service**

Open a new terminal window for the frontend service:

```bash
cd frontend
yarn dev
```

3. **Skip backend startup**

Because you already opened a backend for development, frontend initialization will fail at the `Start backend` step. Select `Skip backend startup` to enter the application normally. The frontend will use the backend service started in your earlier terminal window.

Method 2: use the frontend script to start local backend and frontend together.

```bash
cd frontend
yarn dev:fullstack
```

This command runs local `main.py`, the Vite dev server, and the Electron development window. It is suitable when you need to debug frontend-backend interaction.

## Project Structure

```text
AUTO-MAS/
├── app/                          # Backend core code
│   ├── api/                      # FastAPI routes and interfaces
│   │   ├── core.py               # Core API
│   │   ├── dispatch.py           # Task scheduling API
│   │   ├── emulator.py           # Emulator management API
│   │   ├── history.py            # History API
│   │   ├── ocr.py                # OCR-related API
│   │   ├── plan.py               # Plan management API
│   │   ├── queue.py              # Queue management API
│   │   └── ...
│   ├── core/                     # Core business logic
│   │   ├── broadcast.py          # Broadcast system
│   │   ├── config.py             # Configuration management
│   │   ├── emulator_manager.py   # Emulator manager
│   │   ├── task_manager.py       # Task manager
│   │   └── timer.py              # Timer
│   ├── models/                   # Data models
│   │   ├── config.py             # Configuration model
│   │   ├── emulator.py           # Emulator model
│   │   ├── schema.py             # API schema
│   │   └── task.py               # Task model
│   ├── services/                 # External services
│   │   ├── matomo.py             # Analytics service
│   │   ├── notification.py       # Notification service
│   │   ├── system.py             # System service
│   │   └── update.py             # Update service
│   ├── task/                     # Task implementations
│   │   ├── general/              # General tasks
│   │   └── MAA/                  # MAA-related tasks
│   └── utils/                    # Utility functions
│       ├── logger.py             # Logging utility
│       ├── ImageUtils.py         # Image processing utility
│       ├── ProcessManager.py     # Process management
│       ├── emulator/             # Emulator utilities
│       ├── LogMonitor/           # Log monitoring
│       └── OCR/                  # OCR utilities
│
├── frontend/                     # Frontend code
│   ├── src/                      # Source code
│   │   ├── views/                # Page components
│   │   ├── components/           # Shared components
│   │   ├── api/                  # API calls, generated by tooling
│   │   ├── router/               # Router configuration
│   │   ├── composables/          # Composables
│   │   └── types/                # TypeScript types
│   ├── electron/                 # Electron main process code
│   │   ├── main.ts               # Main process entry
│   │   ├── preload.ts            # Preload script
│   │   └── ipc/                  # IPC communication
│   └── public/                   # Static assets
│
├── res/                          # Resource files
│   ├── images/                   # Image assets
│   ├── icons/                    # Icons
│   └── sounds/                   # Sound effects
│
├── main.py                       # Backend entry point
├── requirements.txt              # Python dependencies
└── README.md                     # Project description
```

## FAQ

### Q: Python virtual environment activation fails

**A**: If PowerShell reports an execution policy error:

```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### Q: yarn install fails

**A**: Try:

1. Clear cache: `yarn cache clean`
2. Delete `node_modules` and `yarn.lock`
3. Install again: `yarn install`

### Q: Backend startup fails due to insufficient permissions

**A**: AUTO-MAS requires administrator privileges. Run as administrator:

- Right-click CMD/PowerShell and select **Run as administrator**
- Or start VS Code as administrator

### Q: Electron window cannot open

**A**: Check:

1. Whether the frontend dev server is running normally at `http://localhost:5173`
2. Whether the backend service is running normally at `http://localhost:36163`
3. Error output in the terminal

## License

This project is open source under the [AGPL-3.0 License](https://www.gnu.org/licenses/agpl-3.0.txt).

When using, distributing, or modifying this software, comply with AGPL-3.0 and the supplementary terms in the project README.

---

Thank you for contributing to AUTO-MAS.
