# 打包与发布

插件开发者将发行包上传到 PyPI 后，用户才能在插件市场中发现并安装插件。为保证插件发行包可以被用户正确识别，需要按照特定的规范发布插件。

## 准备发布

在插件项目根目录按规范编撰 `pyproject.toml` 和 `README.md`，以确保相关信息可以被正确识别。

### Python 项目配置文件

以下模板使用 Setuptools 构建发行包，从 `src/` 发现插件的导入包，并将 `README.md` 写入发行元数据。

```toml
[build-system]
requires = ["setuptools>=77", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "<发行包名>"
version = "<发行版本>"
description = "<发行包简介>"
readme = { file = "README.md", content-type = "text/markdown" }
requires-python = "<Python 版本范围>"
license = "<SPDX 许可证标识>"
license-files = ["<许可证文件路径>"]
authors = [{ name = "<作者名称>" }]
keywords = ["auto-mas-plugin", "auto-mas-id:<插件 ID>"]
dependencies = ["auto-mas-core<SDK 版本范围>", "<其他运行依赖，可省略>"]

[project.urls]
Homepage = "<项目主页 URL>"
Repository = "<源码仓库 URL>"
Documentation = "<插件文档 URL>"
Issues = "<问题反馈 URL>"

[tool.setuptools.packages.find]
where = ["src"]
include = ["<导入包名>", "<导入包名>.*"]

[tool.setuptools.package-data]
"<导入包名>" = ["<包内资源匹配模式>"]
```

- `<发行包名>`：合法的 PyPI 项目名。要求包名规范化后包含 `automas` 或 `auto-mas`，建议采用 `auto-mas-<插件名>`。
- `keywords =`：用于识别插件发行包的关键词列表
  - `auto-mas-plugin`：必须存在。
  - `auto-mas-id:<插件 ID>`：有且仅有一个，插件 ID 须可用作文件名。
- `<发行版本>`：本次发布的 PEP 440 版本，例如 `1.2.0`。
- `<Python 版本范围>`：与目标宿主的 Python 版本匹配，建议采用 `>=3.12,<3.13`。
- `<SDK 版本范围>`：填写插件实际验证过的 SDK 版本约束，建议采用 `>=5,<6`。
- `[project.urls]`：该段落按需填写，允许省略。
- `<导入包名>`：与 `src/` 下包含 `__init__.py` 的顶层目录名一致。
- `<包内资源匹配模式>`：相对于导入包的资源路径或匹配模式，例如 `i18n/*.json`。可填写多个模式，无额外资源时可省略 `[tool.setuptools.package-data]`。

### 自述文件

由于 PyPI 元数据无法承载所有的插件描述信息，部分描述信息需要在 `README.md` 开头使用 `auto-mas:i18n` 声明，发行时随自述文件写入 PyPI 的项目长描述。

```markdown
<!-- auto-mas:i18n
{
  "display_name_i18n": {
    "<语言键>": "<插件市场显示名称>"
  },
  "description_i18n": {
    "<语言键>": "<插件市场简介>"
  },
  "tags_i18n": {
    "<语言键>": ["<市场标签>"]
  }
}
-->

# <插件名称>

<插件的用途和主要功能>

## 运行环境

- AUTO-MAS：<支持的宿主版本范围>
- <需要的外部软件或其他运行条件>

## 安装与使用

1. 在 AUTO-MAS 插件中心安装并启用「<插件市场显示名称>」。
2. <打开哪个页面以及需要填写的配置>
3. <运行插件主要功能的操作步骤>

## 问题反馈

<问题反馈入口和反馈时需要提供的信息>
```

## 构建与检查发行包

1. **构建发行包**：在插件仓库根目录使用目标 Python 版本运行以下命令，生成 wheel 和源码包，并检查发行元数据。

   ```powershell
   python -m pip install build twine
   python -m build
   python -m twine check --strict dist/*
   ```

2. **检查 wheel 内容**：打开 `dist/` 中本次构建的 wheel，确认顶层包入口会导出 `PLUGIN`，运行所需的资源均已打包。检查 `.dist-info/METADATA` 中的版本、依赖、身份关键词和长描述是否与本次发布声明一致。

## 发布到 PyPI

以下 GitHub Actions 工作流由 `v<发行版本>` 标签触发。构建任务检查标签与 `project.version` 是否一致，生成并校验 wheel 和源码包。上传任务取得该次构建的产物，通过 PyPI Trusted Publishing 上传。

### 工作流模板

```yaml
name: 构建并发布 PyPI 包

on:
  push:
    tags: ["v*"]

permissions:
  contents: read

jobs:
  build:
    name: 构建与校验
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@11d5960a326750d5838078e36cf38b85af677262
        with:
          persist-credentials: false
      - uses: actions/setup-python@a26af69be951a213d495a4c3e4e4022e16d87065
        with:
          python-version: "<构建用 Python 版本>"
      - name: 检查发布标签
        env:
          RELEASE_TAG: ${{ github.ref_name }}
        run: |
          python - <<'PY'
          import os
          import tomllib
          from pathlib import Path

          project = tomllib.loads(Path("pyproject.toml").read_text(encoding="utf-8"))["project"]
          expected_tag = f"v{project['version']}"
          if os.environ["RELEASE_TAG"] != expected_tag:
              raise SystemExit(f"发布标签应为 {expected_tag}")
          PY
      - name: 构建并校验发行包
        run: |
          python -m pip install build==1.6.1 twine==7.0.0
          python -m build
          python -m twine check --strict dist/*
      - uses: actions/upload-artifact@ea165f8d65b6e75b540449e92b4886f43607fa02
        with:
          name: python-distributions
          path: dist/
          if-no-files-found: error

  publish:
    name: 发布到 PyPI
    needs: build
    runs-on: ubuntu-latest
    environment:
      name: "<GitHub 发布环境名>"
      url: "https://pypi.org/project/<发行包名>/"
    permissions:
      id-token: write
    steps:
      - uses: actions/download-artifact@d3f86a106a0bac45b974a628896c90dbdf5c8093
        with:
          name: python-distributions
          path: dist/
      - name: 使用 Trusted Publishing 上传
        uses: pypa/gh-action-pypi-publish@7245906fb61fa33a13554f9d5fb856fe61758086
```

- `<构建用 Python 版本>`：与插件的 `requires-python` 和目标宿主匹配，例如 `3.12`。
- `<GitHub 发布环境名>`：与 GitHub 仓库中创建的环境一致，例如 `pypi`。
- `<发行包名>`：填写 `pyproject.toml` 的 `project.name`。

### 使用方法

1. **保存工作流**：替换占位符，将文件保存到插件仓库的 `.github/workflows/<工作流文件名>`。在仓库的 **Settings → Environments** 创建与模板一致的发布环境。
2. **配置可信发布者**：已有 PyPI 项目时，进入项目的 **Manage → Publishing**。首次发布时，进入 PyPI 账号的 **Publishing**，添加待使用的发布者并填写发行包名。选择 GitHub Actions，填写 GitHub 仓库所有者、仓库名、工作流文件名和发布环境名。
3. **推送发布标签**：确认当前提交包含工作流和本次发布版本，创建并推送对应版本标签。

   ```powershell
   git tag "v<发行版本>"
   git push "<插件仓库远端名>" "v<发行版本>"
   ```

4. **确认发布结果**：在 GitHub Actions 查看「构建与校验」和「发布到 PyPI」的结果，再到 PyPI 项目页核对版本和发行文件。

## 插件市场收录

[插件目录工作流](https://github.com/AUTO-MAS-Project/AUTO-MAS-Plugin-Index)每 30 分钟刷新已收录和手工补录的插件，每天 UTC 00:07（北京时间 08:07）额外扫描新候选，采集成功后发布变更到 [AUTO-MAS 数据分享中心](https://data.auto-mas.top/files/auto-mas/Client/plugins)。客户端优先使用来自数据中心的插件目录，无法获取时自行从 PyPI 搜索。

插件发布后，如需加快收录，可联系项目组手动触发目录刷新，首次发布最短约 30 分钟后可被发现。
