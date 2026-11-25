# AUTO-MAS Docs

## 项目简介

AUTO-MAS Docs 是一个基于 **VitePress** 构建的文档站点，提供 **[AUTO-MAS](https://github.com/AUTO-MAS-Project/AUTO-MAS)** 项目的详细使用说明。

### 技术栈

- [VitePress](https://vitepress.dev/) - 轻量级静态文档生成器
- [TypeScript](https://www.typescriptlang.org/) - 提供类型安全
- [GitHub Pages](https://pages.github.com/) - 托管静态页面

### 主要文件位置

| 文档 | 中文          | 英文             |
| ---- | ------------- | ---------------- |
| 首页 | `index.md`    | `en/index.md`    |
| 文档 | `docs/`       | `en/docs/`       |
| 开发 | `developer/`  | `en/developer/`  |
| 下载 | `download/`   | `en/download/`   |
| 公示 | `disclosure/` | `en/disclosure/` |

| 图像资源     | 位置                 |
| ------------ | -------------------- |
| 文档截图     | `public/docs/img/`   |
| 插件文档图片 | `public/plugin/img/` |
| 站点图标     | `public/icons/`      |
| 应用图片     | `public/app-images/` |

## 本地开发

以下命令均在仓库根目录运行。

### 选择包管理器

| 包管理器                            | 锁文件              | 本地安装       | 严格按锁文件安装                 |
| ----------------------------------- | ------------------- | -------------- | -------------------------------- |
| npm（推荐）                         | `package-lock.json` | `npm install`  | `npm ci`                         |
| pnpm 11（Node.js 22.13 或更新版本） | `pnpm-lock.yaml`    | `pnpm install` | `pnpm install --frozen-lockfile` |
| Yarn Classic 1.22.22                | `yarn.lock`         | `yarn install` | `yarn install --frozen-lockfile` |

### 开发与验证命令

| 用途                        | npm                    | pnpm                    | Yarn                    | 结果                                                        |
| --------------------------- | ---------------------- | ----------------------- | ----------------------- | ----------------------------------------------------------- |
| 启动开发服务器              | `npm run docs:dev`     | `pnpm run docs:dev`     | `yarn run docs:dev`     | 打开 `http://localhost:5174/`，修改 Markdown 后可实时查看。 |
| 构建文档站                  | `npm run docs:build`   | `pnpm run docs:build`   | `yarn run docs:build`   | 运行 VitePress，生成 `.vitepress/dist/`，同时检查站内死链。 |
| 预览构建结果                | `npm run docs:preview` | `pnpm run docs:preview` | `yarn run docs:preview` | 先构建，再运行预览命令并访问终端打印的地址。                |
| 编译 `src/` 下的 TypeScript | `npm run build`        | `pnpm run build`        | `yarn run build`        | 运行 `tsc`，生成 `dist/index.js`。                          |

## 参与贡献

欢迎纠正文档、补充示例和改进中英文翻译。提交前请先阅读[开发与贡献指南](developer/index.md)及其中的 **AUTO-MAS 开发者社区授权协议**。

## 关于

- [AUTO-MAS 官方文档站](https://doc.auto-mas.top/)
- [AUTO-MAS 主程序仓库](https://github.com/AUTO-MAS-Project/AUTO-MAS)
- 本仓库采用 [MIT 许可证](LICENSE)。图像与美术资源的使用条款见[资源许可协议](disclosure/assets-license.md)。

### 贡献者

感谢所有编写、校对和翻译文档的贡献者。

[![AUTO-MAS Docs 贡献者](https://contrib.rocks/image?repo=AUTO-MAS-Project/AUTO-MAS-docs)](https://github.com/AUTO-MAS-Project/AUTO-MAS-docs/graphs/contributors)

### 官方社区

- QQ 交流群：[957750551](https://qm.qq.com/q/bd9fISNoME)
- QQ 开发者交流群：[1094208135](https://qm.qq.com/q/MJYbfjwScM)
- Telegram：[@AUTO_MAS_top](https://t.me/AUTO_MAS_top)
