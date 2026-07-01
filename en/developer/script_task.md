# Specialized Script Adaptation

Specialized script adaptation must connect configuration, schema, API, task scheduling, frontend entry points, and runtime verification. The detailed engineering rules for the current project are defined by the Skill built into the main repository at `.agents/skills/mas-script-specialized-adapter`. This document keeps only the workflow entry points most likely to be missed by developers.

## Confirm the Architecture First

Before adding or refactoring a `ScriptType`, identify which architecture line the external script belongs to:

| Architecture line | Typical shape | Reference in this repository |
|-------------------|---------------|------------------------------|
| MAA line | MAA-style configuration sessions, plans, stage/sanity configuration | `MAA` |
| SRC line | Alas / SRC-style large forms and sections | `SRC` |
| MXU line | MaaEnd + MXU, `mxu-*.json`, ScriptConfig overlay | `MaaEnd` |
| MFAA line | M9A / MFAA task queue JSON without ScriptConfig wrapper | `M9A` |
| General | Generic paths, processes, and log monitoring | `General` |
| ok-script line | `-t` / `-e` CLI plus form-based configuration editor | `Okww` |

Ask the user for the upstream script or GUI wrapper repository URL first. Then determine the architecture line based on README, launch arguments, configuration persistence method, and release artifacts. If no repository is available, confirm the script shape, formal `ScriptType`, display text, icon source, startup method, and configuration persistence method with the user.

## Implementation Order

Specialized adaptation should follow the order "frontend surface first, then backend completion":

1. **Architecture confirmation**
   - Confirm upstream repository, formal `ScriptType`, route segment, user-visible text, and icon.
   - Confirm whether automatic tasks run through CLI arguments, JSON writes, or an external configuration session.

2. **Frontend surface**
   - `Scripts.vue` and `ScriptTable.vue`: Hub entry, cards, icons, and action buttons.
   - `router/index.ts`: add a route segment consistent with the Hub.
   - `frontend/src/types/script.ts`: add `ScriptType` and default structure.
   - `frontend/src/composables/useScriptApi.ts`: complete script type branches and `UserConfig -> users[]` branch.
   - `EditView/Script/`, `EditView/User/`: add or adjust script edit pages, user edit pages, and sections.

3. **Backend registration**
   - `app/models/config.py`: add `XxxConfig` / `XxxUserConfig` and register them to the corresponding configuration collections.
   - `app/models/schema.py`: complete API schema.
   - `app/core/config.py`, `app/api/scripts.py`, `app/utils/constants.py`: complete configuration, API, display text, and type mapping.
   - After backend schema changes, start the current local backend and run `yarn openapi` in `frontend`. Do not manually edit the generated `frontend/src/api` directory.

4. **Task module**
   - `app/task/Xxx/`: implement `Manager` and `AutoProxy` according to the architecture line, and add `config_schema.py` or `ScriptConfig.py` when needed.
   - `Manager` and task execution classes must implement `final_task` / `on_crash`.
   - `check()` return messages should tell users how to resolve the issue, not just provide technical descriptions.

5. **Verification and cleanup**
   - If `General` was used first to verify startup, logs, and configuration feasibility, remove the temporary entry after specialized adaptation lands.
   - Before committing, self-check against `.agents/skills/mas-script-specialized-adapter/references/adapter-code-norms.md` in the main repository.
   - Confirm that Hub, routes, edit pages, API, task scheduling, logs, and history records all work.

## Code Quality Baseline

- Do not copy `app/task/general` and globally replace variable names. Align with the closest architecture line and existing surface template.
- Configuration writes and restores should use an atomic mindset to avoid corrupting configuration after task interruption.
- Recovery logic shared by `final_task` and `on_crash` should be extracted and reused.
- Side effects such as process cleanup and file cleanup should catch exceptions separately, preventing one failure from blocking later cleanup.
- Instance attributes should be explicitly initialized with type annotations in `__init__`, not patched with `hasattr()` fallbacks.
- Status, logs, process management, and pre/post scripts should align with `General` first. Introduce specialized differences only after the architecture is confirmed.

## Further Reading

When an AI assistant performs specialized adaptation, it should load `.agents/skills/mas-script-specialized-adapter/SKILL.md` from the main repository and read the following as needed:

- `references/script-frontend-architectures.md`
- `references/adapter-code-norms.md`
- `references/examples-frontend-surfaces.md`
- `examples-*.md` for the corresponding architecture line
