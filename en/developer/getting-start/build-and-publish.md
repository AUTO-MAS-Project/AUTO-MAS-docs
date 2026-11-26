# Build and Release

::: warning Note
Parts of this page are still being updated. Refer to the latest workflows and scripts in the main repository for authoritative details.
:::

## Local Build

Complete [Environment Setup](./environment.md) before building locally.

From the repository root, run:

```powershell
cd frontend
vp install --frozen-lockfile
vp run build
```

This builds only the Windows x64 desktop bundle, without embedding Runtime or publishing anything. The runnable output is written to `frontend/dist/win-unpacked/`.

## CI Builds

When a PR is opened, updated, or reopened—or when commits are pushed to `dev` or `main`—the [CI Package][ci-package] workflow builds an unsigned Windows x64 portable ZIP (skipped automatically for changes that only touch Markdown files or `docs/`). You can also trigger it manually in GitHub Actions and download the ZIP from the workflow run's Artifacts section.

| Build Mode | When to Use | Contents & Backend Source |
| --- | --- | --- |
| Standard verification package | Verify a PR or specific branch | Contains the frontend and Electron from that commit, with the matching backend pre-bundled in `repo/`. Runtime must be prepared separately. |
| Alpha test package | Trigger manually on `dev` with `alpha` checked | Includes a verified Runtime and prepares the backend from `dev` on startup. Versioned as `vX.Y.Z-alpha.<run-number>`, with `build-source.json` recording frontend commit, backend branch, Runtime version, and SHA-256. |

When a change touches Electron or packaging configuration, download and test the verification package for that commit. For other verification steps, see [Testing and Verification](./testing.md) and the [main repository CI check guide][ci-skill].

## Maintainer Releases

Releasing the main application requires two manual GitHub Actions runs by a maintainer: first run [Prepare Release][prepare-release] to open a release PR; once reviewed and merged, run [Build and Publish App][build-app] to build, sign, create the GitHub Release, and distribute packages across channels.

### 1. Prepare and Review the Release PR

Ordinary changes add fragments as described in [Version Records](./development-specifications.md#version-records). The [Absorb Changelog][absorb-changelog] workflow compiles merged fragments on `dev` daily at 23:30 Beijing time, while remaining fragments are compiled during release preparation. Version numbers and version files are updated together in the release PR.

Trigger [Prepare Release][prepare-release] from `dev` or the latest stable `release/<version>` branch of the target release line. The workflow computes the next version from reachable tags, compiles remaining fragments, updates version files, and opens a `Release <version>` PR.

| `kind` | Purpose | Versioning Rule |
| --- | --- | --- |
| `beta` | Publish a beta from `dev` | Increments `beta.N` if a beta already exists; otherwise starts `beta.1` for the next version after a stable release. |
| `stable` | Promote to stable from `dev` | Converts the latest `vX.Y.Z-beta.N` into `vX.Y.Z`. |
| `patch` | Publish a patch from a stable release branch | Increments patch version `Z` on that stable release. |
| `explicit` | Set a version manually | Supply `version`; it must be greater than the latest reachable version on the branch and must not conflict with any existing tag. |

Before merging the release PR, verify the version number, release announcement, highlights, and any commits flagged for manual review in the workflow summary:

- **Stable releases**: Merge the reviewed `dev` branch into `main` per the [branch rules][branch-rules], then publish from `main`.
- **Beta releases**: Publish directly from `dev`.
- **Patch releases**: Publish from the `release/*` branch with the merged release PR.

### 2. Build, Sign, and Distribute

Trigger [Build and Publish App][build-app] on the branch containing the merged release PR. The workflow validates the version, announcement, pending fragments, duplicate tags, and release branch names (if new changes landed under Unreleased after merging, rerun Prepare Release first), then performs:

1. **Bundle Runtime**: Reads `res/runtime-version.txt`, downloads the corresponding Runtime executable, verifies its SHA-256 checksum, and embeds it in the desktop package (see [Runtime version notes][local-build]; full initialization still checks online versions).
2. **Sign and package**: Signs `AUTO-MAS.exe` via SignPath and builds the installer with Inno Setup. Stable and patch installers are also signed via SignPath, while beta releases sign only the main executable.
3. **Publish and distribute**: Produces `AUTO-MAS-<version>-x64.zip` (portable) and `AUTO-MAS-Setup-<version>-x64.zip` (installer), creates the version tag and `release/<version>` branch, uploads both ZIPs to GitHub Release, triggers CNB distribution, and pushes packages to the self-hosted mirror and MirrorChyan.

Additional notes:

- **CNB distribution**: Uses the [CNB configuration][cnb-config] on the selected release branch. Currently `.cnb.yml` on `dev` defines entry points for `main` and `dev`; when patching from `release/*`, verify that branch's CNB configuration and published outputs.
- **Self-hosted upload**: The upload parameter on `dev` is `datacenter_upload_mode` (defaults to `r2`; `tus` is also supported). Beta releases update only the beta channel, while stable and patch releases update both stable and beta channels.
- **Runtime releases**: Runtime is released independently in `AUTO-MAS-Runtime`. The main repository's [Build and Sign Runtime][runtime-signing] workflow verifies the source commit and tag, builds and signs the binary, and hands the artifact back to the Runtime release flow; main app releases consume already-published Runtime binaries.

### 3. Backend Hot Updates for Released Versions

Installed releases use Runtime to track their corresponding `release/<version>` branch while keeping the frontend bundled inside the installer.

- **Small pure-backend fixes**: Merge into `dev` first, then cherry-pick into the target `release/<version>` branch via PR following the [branch rules][branch-rules], avoiding unrelated commits.
- **Fixes involving frontend or Electron**: Cannot be delivered via hot updates and must ship in a new desktop package. To issue a new versioned patch, run the full prepare-and-publish flow from the latest stable branch of that release line.

[local-build]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/scripts/README.md
[ci-package]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/ci-package.yml
[ci-skill]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.agents/skills/mas-ci-checks/SKILL.md
[absorb-changelog]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/absorb-changelog.yml
[prepare-release]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/prepare-release.yml
[branch-rules]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/AGENTS.md
[cnb-config]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.cnb.yml
[build-app]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/build-app.yml
[runtime-signing]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/build-sign-runtime.yml
