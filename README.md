# AUTO-MAS Docs

## 📖 项目简介
AUTO-MAS Docs 是一个基于 **VitePress** 构建的文档站点，提供 **[AUTO-MAS](https://github.com/AUTO-MAS-Project/AUTO-MAS)** 项目的详细使用说明。


📌 **技术栈**：
- [VitePress](https://vitepress.dev/) - 轻量级静态文档生成器
- [TypeScript](https://www.typescriptlang.org/) - 提供类型安全
- [GitHub Pages](https://pages.github.com/) - 托管静态页面

## 🚀 快速开始

## 如果需要修改文档内容，请直接修改对应的md文件即可。

### 1️⃣ 安装依赖
确保你的机器上安装了 **Node.js (>=18.x)**，然后运行：
```sh
npm install  # 或者使用 pnpm install / yarn install
```

### 2️⃣ 本地运行
```sh
npm run docs:dev  # 或 pnpm run docs:dev / yarn docs:dev
```
访问 **`http://localhost:5173/`** 预览文档。

### 3️⃣ 生成静态文件
```sh
npm run docs:build  # 或 pnpm run docs:build / yarn docs:build
```
静态文件将生成到 `.vitepress/dist/` 目录。

### 4️⃣ 预览构建结果
```sh
npm run docs:preview  # 或 pnpm run docs:preview / yarn docs:preview
```


## ❤️ 贡献指南

欢迎贡献你的改进！
1. **Fork** 本仓库
2. **创建分支**
   ```sh
   git checkout -b feature/my-new-feature
   ```
3. **提交修改**
   ```sh
   git commit -m "feat: 添加新功能"
   ```
4. **推送分支**
   ```sh
   git push origin feature/my-new-feature
   ```
5. **创建 Pull Request**


## 📜 许可证
本项目采用 MIT License 许可。

