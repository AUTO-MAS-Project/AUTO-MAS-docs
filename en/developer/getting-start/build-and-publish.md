# Build and Release

## Build locally

From the `frontend/` directory, run `yarn build` to build the renderer, compile the Electron main process, and run Electron Builder without publishing. The renderer output is in `frontend/dist/`; that directory is not the desktop installer.

For an unsigned Windows test package, use the `CI 构建产物` workflow. Select `dev` and enable `alpha` for a package that includes Runtime and follows the `dev` backend. Download it from the workflow artifacts. The package's `build-source.json` identifies the frontend source and Runtime used for the build.

## Prepare and publish a release

1. Keep ordinary changes in [changelog fragments](./development-specifications.md#version-records). Do not manually bump versions in development PRs.
2. Maintainers run `准备发版` on `dev` or the applicable `release/*` branch. Choose beta, stable, or patch as appropriate. The workflow compiles the remaining fragments, advances the version, and opens a release PR.
3. Review the release notes and merge that PR. Then a maintainer runs `构建并发布应用程序` and obtains the required approval for signing.
