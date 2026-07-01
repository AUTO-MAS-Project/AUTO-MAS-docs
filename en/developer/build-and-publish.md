# Build and Release

## Build a Development Package

```bash
cd frontend
yarn build
```

The build output is located at:

```text
frontend/dist/
```

## Release Process

1. Update version numbers:
   - `version` in `res/version.json`
   - `version` in `frontend/package.json`
   - `AppConfig.VERSION` in `app/core/config.py`

2. Check `version_info` in `res/version.json`:
   - User-visible changes must be registered in the corresponding version entry.
   - Do not keep appending entries to an already released version. Add a new version entry instead.

3. Commit changes to the repository.

4. Notify release reviewer **DLmaster_361** to prepare the release:
   - Do not skip this step. Release publishing requires signature approval from **DLmaster_361**.

5. Trigger the `构建并发布应用程序` workflow.
