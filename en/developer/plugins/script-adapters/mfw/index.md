# MFW Design

MFW is the generic engine in AUTO-MAS for running MaaFramework projects, located at `app/task/MaaFW/`. It reads tasks, resources, and controllers from a project's `interface.json` and runs inside each managed script's own project view. M9A and MSS add project-specific behavior through flavor configurations and hooks.

## Reading Order

1. **Building a flavor**: Start with [Flavor Development](./develop.md) to complete config registration, backend hooks, and frontend descriptor objects. See [Flavor](./flavor.md) for optional hooks and existing flavor designs.
2. **Understanding import and updates**: Review module responsibilities in [MFW Architecture](./architecture.md), then read [MFW Import and Payload Views](./import-and-payload.md) and [MFW Project Updates](./update.md) to see how project versions are maintained.
3. **Understanding task execution**: Trace the flow from task snapshots to workers in [MFW Runs](./run.md), and learn about interpreters, native libraries, and dependency installation in [MFW Runtime Environments](./runtime-env.md).
4. **Maintaining user entry points**: See [MFW Frontend and Interaction](./frontend.md) and [MFW Shell Import and Issue Reports](./shell-import-and-issue-report.md).
5. **Checking design rationale**: See [MFW Design Tradeoffs](./tradeoffs.md) for constraints covering concurrency, version synchronization, task retries, and flavor extensions.

Throughout these pages, `MaaFW/` refers to `app/task/MaaFW/` in the main program repository. Read `AGENTS.md` in that directory before making changes. When modifying M9A or MSS, also consult the notes in their respective task directories.

## Glossary

- **Projects and Import**
  - **Project interface**: The `interface.json` file in the project root directory, following the ProjectInterface (PI) protocol.
  - **Shell**: A project's GUI or CLI launcher, such as MFAAvalonia, MXU, MFW-PyQt6, or MaaPiCli. MAS runs projects directly through the MFW engine instead of launching a shell.
  - **Source directory**: The directory chosen by the user for import, recorded in `Info.Path`.
  - **Projection**: Selecting files needed for import or update based on the project interface and projection rules.
- **Payloads and Views**
  - **Lineage**: Project identity, resolved in order from `mirrorchyan_rid`, `github`, or `name`.
  - **Group**: The set of managed scripts sharing the same lineage and update channel; members of a group synchronize to the same payload.
  - **Payload**: An immutable project file tree registered by lineage, version, and content hash.
  - **View**: The complete project directory used by a managed script. Shareable files are reused via hard links; in the UI it is referred to as a "copy".
- **Execution**
  - **Flavor**: A `MaaFWConfig` subclass plus a project hook object that recognizes and runs a specific project on top of the generic engine.
  - **Managed tasks**: Tasks whose execution and ordering are controlled by a flavor, such as M9A's game startup and exit tasks.
  - **Runtime pool**: The runner's base virtual environment together with versioned Python bindings and native libraries.
  - **Binding**: The `maafw` Python package used to call MaaFramework; its version must match the native library loaded at run time.
  - **Worker**: The runner subprocess that executes in the runtime pool and depends on the engine core package.
  - **Agent**: The project-supplied process providing custom recognitions or actions.
  - **Precheck**: Preparing a runtime environment in a staging directory before registering an updated payload.
