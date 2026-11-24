# 构建与发布

本页已过时，等待重写。

## 本地构建

执行本地构建前，请先完成[环境配置](./environment.md)。

```powershell
Set-Location frontend
yarn install --immutable
yarn build
```

仅编译 Windows x64 发行版，不预置 Runtime，不会自动发布。相关产物位置：

- Windows x64
  - 可直接运行的软件包：`frontend/dist/win-unpacked/`

## CI 构筑

「CI 构建产物」会在 PR 打开、更新或重新打开，以及推送到 `dev` 或 `main` 时生成未签名的 Windows x64 便携 ZIP。仅修改 Markdown 或 `docs/` 的自动触发会跳过构建。也可以在 GitHub Actions 手动运行，并在运行详情的 Artifacts 下载 ZIP。[CI 构建产物][ci-package]

| 模式         | 何时使用                        | 包内内容与后端来源                                                                                                                                             |
| ------------ | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 普通验证包   | 验证 PR 或指定分支的代码        | 前端和 Electron 来自本次构建，包内 `repo/` 预置同一提交的后端。Runtime 需另行准备。                                                                            |
| Alpha 测试包 | 在 `dev` 手动运行并勾选 `alpha` | 随附校验过的 Runtime，启动时从 `dev` 准备后端。桌面端使用 `vX.Y.Z-alpha.<运行编号>`，包内 `build-source.json` 记录前端提交、后端分支、Runtime 版本和 SHA-256。 |

涉及 Electron 或打包时，优先下载对应提交的验证包检查。其余 PR 验证按改动范围选择，参见[测试与验证](./testing.md)和[主仓 CI 检查说明][ci-skill]。

## 维护者发版

主程序发版需要维护者在 GitHub Actions 手动触发两次。先运行「准备发版」，由 CI 生成发版 PR。审核并合并该 PR 后，再运行「构建并发布应用程序」，由 CI 完成构建、签名、创建 GitHub Release 和分发。常规 PR 和推送会按上一节的条件生成验证包，定时任务负责入账更新日志碎片。[准备发版][prepare-release] · [构建并发布应用程序][build-app]

### 1. 准备并审核发版 PR

日常改动按[版本记录](./development-specifications.md#版本记录)提交碎片。`dev` 每天北京时间 23:30 入账已合并的碎片，发布线上的剩余碎片由「准备发版」处理。版本号和生成的版本文件统一在发版 PR 中更新。[更新日志入账][absorb-changelog]

在 GitHub Actions 运行「准备发版」，从 `dev` 或目标发布线最新已发布版本对应的 `release/<版本>` 分支启动。工作流根据所选分支可达的 tag 计算版本，编译剩余碎片、同步版本文件，并向所选分支创建 `Release <版本>` PR。[准备发版][prepare-release]

| `kind`     | 用途                                 | 版本规则                                                                        |
| ---------- | ------------------------------------ | ------------------------------------------------------------------------------- |
| `beta`     | 在 `dev` 发布公测                    | 已有公测时递增 `beta.N`。上次为正式版时，开启下一次版本的 `beta.1`。            |
| `stable`   | 在 `dev` 准备转正                    | 将最后一个 `vX.Y.Z-beta.N` 转为 `vX.Y.Z`。                                      |
| `patch`    | 从目标发布线的最新正式版分支发布补丁 | 在该正式版上递增 `Z`。                                                          |
| `explicit` | 手动指定版本                         | 填写 `version`。版本须大于当前分支可达的最新版本号，且不能与仓库已有 tag 重号。 |

合并发版 PR 前，核对版本号、公告、亮点，以及工作流摘要中需要人工确认的提交。转正时由维护者按分支规范把已审核的 `dev` 版本合入 `main`，再从 `main` 发布。公测从 `dev` 发布，补丁从已合并发版 PR 的 `release/*` 发布。[主仓分支规范][branch-rules]

### 2. 构建、签名与分发

在包含发版 PR 的发布分支上手动运行「构建并发布应用程序」。工作流会校验版本和公告，检查待入账的碎片、重复 tag 以及同名发布分支。若发版 PR 合并后又有改动进入「未发布」，先重新运行「准备发版」。工作流配置与参数以所选分支上的版本为准。[发布工作流][build-app]

1. 读取 `res/runtime-version.txt`，从 Runtime 的独立发行版下载 exe，校验 SHA-256 后放入桌面包。这个版本是安装包内置的 Runtime，完整初始化还会检查线上版本。[Runtime 版本说明][local-build]
2. 通过 SignPath 签名 `AUTO-MAS.exe`，再用 Inno Setup 制作安装程序。正式版和补丁版的安装程序也经 SignPath 签名，公测版只签名主程序。工作流等待签名完成后继续。[发布工作流][build-app]
3. 生成 `AUTO-MAS-<版本>-x64.zip`（便携包）和 `AUTO-MAS-Setup-<版本>-x64.zip`（安装包），创建版本 tag 与 `release/<版本>`，并把两个 ZIP 上传至 GitHub Release。工作流另行触发 CNB 分发，随后把安装包推送到自建源与 Mirror 酱。[发布工作流][build-app]

CNB 分发使用发版所选分支的配置。当前 `dev` 中的 `.cnb.yml` 为 `main` 和 `dev` 定义了发布入口。从 `release/*` 发补丁时，请查看所选分支的 CNB 配置，确认各渠道的实际发布结果。[CNB 配置][cnb-config]

当前 `dev` 的自建源上传参数为 `datacenter_upload_mode`，默认 `r2`，也可选 `tus`。自建源的公测版只更新公测渠道，正式版和补丁版同时更新正式与公测渠道。[自建源发布配置][build-app]

Runtime 由 `AUTO-MAS-Runtime` 仓库独立发版。主仓的「构建并签名 Runtime」接收该仓的发版请求，核对源码提交和 tag 后构建并签名，再将产物交回 Runtime 发版流程。主程序发版直接使用已发布的 Runtime。[Runtime 签名工作流][runtime-signing]

### 3. 已发布版本的后端热更新

已安装的发行版通过 Runtime 跟踪自身的 `release/<版本>`，并继续使用安装包内的前端。小范围纯后端修复先合入 `dev`，再通过 cherry-pick PR 合入对应发行版分支，避免带入其他 `dev` 提交。涉及前端或 Electron 的修复须随新的桌面包发布。需要推出新版本号的补丁时，从该发布线上最新的正式版分支重新走准备与发布流程。[主仓分支规范][branch-rules]

[frontend-package]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/frontend/package.json
[local-build-script]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/scripts/build-local-package.ps1
[local-build]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/scripts/README.md
[ci-package]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/ci-package.yml
[ci-skill]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.agents/skills/mas-ci-checks/SKILL.md
[absorb-changelog]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/absorb-changelog.yml
[prepare-release]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/prepare-release.yml
[branch-rules]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/AGENTS.md
[cnb-config]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.cnb.yml
[build-app]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/build-app.yml
[runtime-signing]: https://github.com/AUTO-MAS-Project/AUTO-MAS/blob/dev/.github/workflows/build-sign-runtime.yml
