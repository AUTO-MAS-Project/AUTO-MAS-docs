
# 构筑与发布

## 开发环境构筑软件包
```bash
cd frontend
yarn build
```

构建产物位于 `frontend/dist/`

## 软件发布流程

1. 更新版本号
   - `res/version.json` 中的 `main_version`
   - `frontend/package.json` 中的 `version`
   - `app/core/config.py` 中的 `AppConfig.VERSION`

2. 检查 `res/version.json` 中的版本信息

3. 提交修改到仓库

4. 通知版本发布审核员 **DLmaster_361** 准备发布发行版
   - 这一步千万别跳过了，发布发行版时需要 **DLmaster_361** 进行签名批准。

5. 触发 `构建并发布应用程序` 工作流