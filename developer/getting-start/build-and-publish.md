# 构建与发布

::: warning 提示
本页部分内容仍在持续整理更新中，具体流程与参数请以主仓库最新的工作流和脚本为准。
:::

## 本地构建

执行本地构建前，请先完成[环境配置](./environment.md)。

在仓库根目录执行以下命令：

```powershell
cd frontend
vp install --frozen-lockfile
vp run build
```

该命令仅编译 Windows x64 桌面包，不内置 Runtime，也不会触发发布。构建完成后，可直接运行的程序位于 `frontend/dist/win-unpacked/`。

## CI 构建

当 PR 创建、更新或重新打开，以及代码推送到 `dev` 或 `main` 时，[CI 构建产物][ci-package]工作流会自动生成未签名的 Windows x64 便携 ZIP 包（仅修改 Markdown 或 `docs/` 时会自动跳过）。你也可以在 GitHub Actions 中手动触发该工作流，并在运行详情页的 Artifacts 区域下载构建产物。

| 构建模式 | 适用场景 | 包含内容与后端来源 |
| --- | --- | --- |
| 普通验证包 | 验证 PR 或指定分支的代码 | 包含本次构建的前端与 Electron，包内 `repo/` 预置同一提交的后端代码；Runtime 需另行准备。 |
| Alpha 测试包 | 在 `dev` 手动运行并勾选 `alpha` | 附带校验过的 Runtime，启动时从 `dev` 准备后端。桌面端版本号为 `vX.Y.Z-alpha.<运行编号>`，包内 `build-source.json` 会记录前端提交、后端分支、Runtime 版本和 SHA-256。 |

如果改动涉及 Electron 或打包配置，建议优先下载对应提交的验证包实测。其他 PR 验证方式可参考[测试与验证](./testing.md)和[主仓 CI 检查说明][ci-skill]。

## 维护者发版

主程序发版由维护者在 GitHub Actions 分两步手动触发：先运行[准备发版][prepare-release]生成发版 PR，审核合并后再运行[构建并发布应用程序][build-app]，由 CI 自动完成构建、签名、创建 GitHub Release 与多渠道分发。

### 1. 准备并审核发版 PR

日常改动按[版本记录](./development-specifications.md#版本记录)提交更新日志碎片。[更新日志入账][absorb-changelog]工作流每天北京时间 23:30 会汇总 `dev` 上已合并的碎片，其余待入账碎片会在「准备发版」时统一处理，版本号与版本文件也在发版 PR 中一并更新。

在 GitHub Actions 中从 `dev` 或目标发布线最新正式版对应的 `release/<版本>` 分支触发[准备发版][prepare-release]后，工作流会根据可达 tag 计算版本号、汇总剩余碎片、同步版本文件，并自动创建 `Release <版本>` PR。

| `kind` 参数 | 用途 | 版本计算规则 |
| --- | --- | --- |
| `beta` | 在 `dev` 发布公测版 | 已有公测版时递增 `beta.N`；若上一版本为正式版，则开启下个版本的 `beta.1`。 |
| `stable` | 在 `dev` 准备正式版 | 将最近的 `vX.Y.Z-beta.N` 转为正式版 `vX.Y.Z`。 |
| `patch` | 在最新正式版分支发布补丁 | 在该正式版基础上递增修订号 `Z`。 |
| `explicit` | 手动指定版本号 | 需填写 `version`，且必须大于当前分支可达的最新版本，不能与已有 tag 冲突。 |

合并发版 PR 前，请仔细核对版本号、发布公告、更新亮点以及工作流摘要中提示需人工确认的提交：

- **正式版**：由维护者按[主仓分支规范][branch-rules]将已审核的 `dev` 合入 `main`，再从 `main` 发布。
- **公测版**：直接从 `dev` 发布。
- **补丁版**：从已合并发版 PR 的 `release/*` 分支发布。

### 2. 构建、签名与分发

在已合并发版 PR 的分支上手动触发[构建并发布应用程序][build-app]。工作流会先校验版本号、公告、待入账碎片、重复 tag 和同名发布分支（若合并后又有新改动进入「未发布」，需先重新运行「准备发版」），随后依次完成以下流程（具体配置以所选分支上的工作流为准）：

1. **打包内置 Runtime**：读取 `res/runtime-version.txt`，从独立发行版下载对应 Runtime 可执行文件，校验 SHA-256 后打入桌面包（详见 [Runtime 版本说明][local-build]；完整初始化时仍会检查线上版本）。
2. **签名与制作安装包**：通过 SignPath 为 `AUTO-MAS.exe` 签名，再使用 Inno Setup 打包安装程序。正式版与补丁版的安装程序也会经 SignPath 签名，公测版仅签名主程序。
3. **归档与多渠道分发**：生成便携包 `AUTO-MAS-<版本>-x64.zip` 与安装包 `AUTO-MAS-Setup-<版本>-x64.zip`，自动创建版本 tag 与 `release/<版本>` 分支，并上传至 GitHub Release；随后触发 CNB 分发，并将安装包推送到自建源与 Mirror 酱。

补充说明：

- **CNB 分发**：使用发版所选分支的 [CNB 配置][cnb-config]。目前 `dev` 中的 `.cnb.yml` 为 `main` 和 `dev` 定义了发布入口；从 `release/*` 发补丁时，请先确认该分支的 CNB 配置及各渠道发布情况。
- **自建源上传**：当前 `dev` 的上传参数为 `datacenter_upload_mode`（默认 `r2`，可选 `tus`）。公测版仅更新公测渠道，正式版与补丁版会同时更新正式与公测渠道。
- **Runtime 发版**：Runtime 在 `AUTO-MAS-Runtime` 仓库独立发版；主仓的[构建并签名 Runtime][runtime-signing]工作流负责核对源码提交与 tag、完成构建签名并将产物交回 Runtime 发版流程，主程序发版直接复用已发布的 Runtime。

### 3. 已发布版本的后端热更新

已安装的发行版会通过 Runtime 跟踪自身对应的 `release/<版本>` 分支，继续沿用安装包内的前端。

- **纯后端小范围修复**：先合入 `dev`，再按[主仓分支规范][branch-rules]通过 Cherry Pick PR 同步到对应 `release/<版本>` 分支，避免带入无关改动。
- **涉及前端或 Electron 的修复**：无法通过热更新下发，必须随新的桌面包发布；若需推出新版本号的补丁，请从该发布线最新的正式版分支重新走完准备与发布流程。

[local-build]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/scripts/README.md
[ci-package]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/ci-package.yml
[ci-skill]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.agents/skills/mas-ci-checks/SKILL.md
[absorb-changelog]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/absorb-changelog.yml
[prepare-release]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/prepare-release.yml
[branch-rules]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/AGENTS.md
[cnb-config]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.cnb.yml
[build-app]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/build-app.yml
[runtime-signing]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/build-sign-runtime.yml
