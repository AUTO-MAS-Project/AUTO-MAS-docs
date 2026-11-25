# Project Structure

The main AUTO-MAS repository contains the Python backend, the Vue renderer, and the Electron desktop shell. The backend starts from `main.py` at the repository root, with application code in `app/`. The frontend lives in `frontend/`, with renderer code in `frontend/src/` and desktop process code in `frontend/electron/`. All paths on this page are relative to the main repository root.

## Backend

### Source Directories

| Path            | Responsibility                                                                                                                                                                                              |
| --------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `main.py`       | Backend process entry point. Initializes and starts the FastAPI application.                                                                                                                                |
| `app/api/`      | HTTP and WebSocket layer. Validates requests, calls application entry points, and builds responses and API errors. Endpoint-specific models live in the corresponding API file.                             |
| `app/core/`     | Core components that coordinate configuration, task scheduling, game instances, notifications, WebSocket delivery, and plugin management.                                                                   |
| `app/models/`   | Shared data structures, type contracts, and field constraints. Shared HTTP and WebSocket contracts live in `schema.py`. Business configuration declarations live in `config/`.                              |
| `app/config/`   | General configuration framework for nodes, fields, transactions, locks, serialization, and signals. `wire.py` derives API types from configuration declarations.                                            |
| `app/services/` | Cross-platform services organized by feature. Each service keeps its implementation in its own module and can call other components' public entry points.                                                   |
| `app/utils/`    | General cross-platform utilities that take arguments and return results. Imports within the project are limited to `utils` itself.                                                                          |
| `app/tools/`    | Standalone tools such as community sign-in. Each tool keeps its private models, contracts, and platform implementations together. Its project dependencies are limited to its own package and `app/utils/`. |
| `app/task/`     | Task execution in the main app, including managed scripts, accounts, and specialized adapters still maintained in the host.                                                                                 |
| `app/plugin/`   | Base plugin contracts and runtime mechanisms. Host integration and orchestration live in `app/core/plugin/`. The public SDK lives in `plugins/auto_mas_core/`.                                              |
| `app/mystery/`  | Reserved for advanced features.                                                                                                                                                                             |

### Where to Put Code

- **Group code by responsibility.** Routes handle input and output. Application orchestration belongs in `core` or the owning service. Shared models define structures and constraints. Task execution belongs in `task` and the corresponding adapter. Start with one file in the owning layer and introduce a package within that layer as needed. A large core component can introduce a separate foundation package under `app/` when its underlying mechanisms need to be reused independently of application orchestration.
- **Use public component entry points.** Core singletons are owned by their modules and available on demand from `app.core`, for example `Config` and `Notify`. A simple component can define its implementation and singleton in one file. A complex component can organize its responsibilities across files in its package. Each task owns the objects created for that run.
- **Keep configuration mechanisms, declarations, and application management in their respective modules.** General mechanisms live in `app/config/`, business fields in `app/models/config/`, and startup loading, application reads and writes, upgrades, and backups in `app/core/config/`. Use `Config` path properties to locate resources and user files. Callers pass any required paths to standalone tools.
- **Keep platform differences inside the owning feature.** `services`, `utils`, and `tools` each provide a common public entry point. Callers supply context and configuration to standalone tools and handle any configuration updates, scheduling, or notifications.

## Frontend

### Vue Renderer

The renderer starts in `frontend/src/main.ts` and mounts the app through `App.vue`. Pages use Vue 3, TypeScript, and Ant Design Vue. Vue Router manages navigation.

| Path                                                                 | Responsibility                                                                                                                                                                                                            |
| -------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `frontend/src/router/`                                               | Page routes, navigation, and specialized editor entry points.                                                                                                                                                             |
| `frontend/src/views/`                                                | Route pages and their workflows. A feature page usually lives at `views/<feature>/index.vue` or in a single file. Feature-specific components and logic stay with the page, for example in `views/<feature>/components/`. |
| `frontend/src/components/`                                           | Components shared by multiple pages.                                                                                                                                                                                      |
| `frontend/src/composables/`                                          | Composables shared by pages. `useXxxApi.ts` wraps application requests. The page handles navigation, dialogs, and local state after submission.                                                                           |
| `frontend/src/api/`                                                  | Request clients, services, and types generated from the backend OpenAPI definition. Regenerate them when a backend contract changes.                                                                                      |
| `frontend/src/services/`                                             | Renderer infrastructure, including Vue Query configuration, WebSocket subscriptions, and configuration synchronization.                                                                                                   |
| `frontend/src/stores/`                                               | Pinia stores for reactive state owned by the client and shared across pages or components.                                                                                                                                |
| `frontend/src/types/`, `frontend/src/utils/`                         | Handwritten domain and desktop bridge types, plus standalone utility functions. Generated API types live in `src/api/`.                                                                                                   |
| `frontend/src/i18n/`, `frontend/src/styles/`, `frontend/src/assets/` | Internationalization, global styles, and renderer assets. `src/style.css` is also a global stylesheet entry point.                                                                                                        |

Keep components and logic with the page that uses them. Move them to a shared directory when multiple modules reuse them. The page or component owns form drafts and temporary interaction state. Vue Query caches backend query results that need to be shared.

### Electron Desktop App

| Path                           | Responsibility                                                                                                      |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------- |
| `frontend/electron/main.ts`    | Desktop main-process entry point. Organizes windows and the application lifecycle.                                  |
| `frontend/electron/preload.ts` | Exposes `window.electronAPI` to bridge access to desktop features. Types are in `frontend/src/types/electron.d.ts`. |
| `frontend/electron/ipc/`       | Main-process IPC handlers for entry points exposed by preload.                                                      |
| `frontend/electron/services/`  | Desktop implementations for backend process management, Runtime, updates, and files.                                |

Pages send HTTP requests to the Python backend through generated clients wrapped by business composables. They receive updates through the shared WebSocket service. For desktop features, pages call the preload interface and the Electron main process handles the request.

## Development Directories

| Path                                                                     | Contents                                                                                     |
| ------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------- |
| `res/`                                                                   | Resources and version information shipped with the app.                                      |
| `tests/`, `frontend/e2e/`                                                | Python and Playwright tests. Frontend `*.test.ts` files live next to the modules they cover. |
| `pyproject.toml`, `uv.lock`, `.python-version`                           | Python project and dependencies, lockfile, and interpreter version.                          |
| `frontend/package.json`, `frontend/yarn.lock`, `frontend/vite.config.ts` | Frontend dependencies and scripts, lockfile, and Vite+ (`vp`) configuration.                 |
