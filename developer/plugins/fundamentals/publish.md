# 发布插件

插件先作为 Python 发行包发布到 PyPI，再由市场目录读取发行元数据。客户端通过目录展示名称、版本和更新说明，通过插件安装器安装并启用。源码调试见[开发与调试](./develop.md)。

## 准备发行包

1. **确认身份**：核对 `pyproject.toml` 的包名、`auto-mas-plugin` 和唯一的 `auto-mas-id:<插件 ID>`。
2. **确认依赖**：保留 `auto-mas-core` 依赖，按实际验证过的宿主和 Python 版本声明兼容范围。安装器会将宿主自带的 SDK 纳入求解。
3. **更新版本**：使用新的 PEP 440 版本，并核对 README、语言文件等需要随包交付的资源。
4. **构建与校验**：在插件项目根目录构建发行包，再检查 `dist/` 中的产物和元数据。

   ```sh
   uv build
   uvx twine check --strict dist/*
   ```

   独立构建和发布选项见 [uv 打包说明](https://docs.astral.sh/uv/guides/package/)。

## README 里的市场声明

通过 `readme = "README.md"` 将 README 放入发行元数据。市场声明是 README 起始位置的 HTML 注释。

```markdown
<!-- auto-mas:i18n
{
  "display_name_i18n": {"zh_CN": "<插件名称>", "en": "<English name>"},
  "description_i18n": {"zh_CN": "<插件简介>", "en": "<English description>"},
  "tags_i18n": {"zh_CN": ["<标签>"], "en": ["<English tag>"]}
}
-->
```

- **显示信息**

  - `display_name_i18n`、`description_i18n`：语言到名称和简介的映射，每项至少有一种非空语言。
  - `tags_i18n`：语言到字符串数组的映射。
  - 使用合法 JSON。语言键可用 `zh_CN`、`en`、`ja`，客户端按语言回退。

- **项目链接**

  - `project.urls` 中的文档、主页、源码和更新日志链接使用真实的 HTTP 或 HTTPS 地址。
  - 发布前检查 README 作为发行描述时仍以声明开头。目录要求有效声明，本地源码缺失时可用插件 ID 和简短描述展示。

## 在插件市场显示更新日志

在项目根目录的 `CHANGELOG.md` 使用与发行版本一致的二级标题。将可公开读取的文件链接写入 `[project.urls]`。

```toml
[project.urls]
Changelog = "https://github.com/<仓库所有者>/<插件仓库>/blob/<分支>/CHANGELOG.md"
```

目录生成器按已发布版本从对应章节读取说明。发布后可在[公开插件目录](https://data.auto-mas.top/api/v1/files/auto-mas/Client/plugins/download)核对条目的 `version` 和 `changelog`。

## 上传发行包

- **手动上传**

  - 在插件自己的项目中上传已检查的产物。受限测试可使用 TestPyPI。
  - 用 `uv publish` 或 `twine upload` 执行上传，凭据由发布环境提供。

- **Trusted Publishing**

  - 在 PyPI 的发布设置中登记仓库所有者、仓库、工作流文件名及所用的 GitHub 环境。
  - 分开构建与发布任务。发布任务只下载和发布本次构建产物，并向发布任务授予所需的 `id-token: write`。
  - 平台配置和首次发布方式见 [PyPI Trusted Publishing](https://docs.pypi.org/trusted-publishers/)。

## 数据站目录什么时候收录

[AUTO-MAS-Plugin-Index](https://github.com/AUTO-MAS-Project/AUTO-MAS-Plugin-Index) 负责读取 PyPI 元数据并生成目录。包名包含 `auto-mas` 或 `automas` 时可被自动发现，其他包名由维护者补录候选。收录仍要求合法关键词和完整的市场声明。

客户端优先使用有效的数据站目录。目录不可用时回退到 PyPI 和镜像检索。PyPI 已出现新版本而客户端仍显示旧目录时，先核对公开目录中的版本和更新日志。

## 验证发布结果

1. **检查 PyPI**：核对项目页的新版本、发行包和元数据。可通过 `https://pypi.org/pypi/<发行包名>/json` 检查 `info.version`、`info.keywords` 和 `info.description`。
2. **检查目录**：在公开插件目录按 `package_name` 查找包，核对名称、版本、标签和更新说明。
3. **检查实际安装**：移出同 ID 的源码项目，用独立测试配置从插件市场安装，按提示完成后端重启。再检查插件来源、版本、启用状态和最小业务流程。
4. **处理问题版本**：如果已发布版本需要撤回，按 [PyPI Yank 指南](https://docs.pypi.org/project-management/yanking/)标记该发行版，并发布更高版本的修复包。
