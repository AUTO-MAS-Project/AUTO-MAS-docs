# 前端开发

## 生成前端 API

新增 FastAPI 接口或修改请求、响应模型后，需要重新生成前端的 TypeScript 类型和请求客户端。

先按[启动后端服务](../getting-start/environment.md#_4-1-分别启动后端与前端)启动当前检出的后端。保持后端运行，在仓库根目录另开一个终端并执行以下命令。

```sh
cd frontend
yarn openapi
```

生成命令读取 `http://127.0.0.1:36164/openapi.json`，更新 `frontend/src/api/`。接口契约以 FastAPI 的声明为准，生成文件统一通过该命令更新。

如果生成结果中缺少预期的接口或字段，先打开 `http://127.0.0.1:36164/openapi.json` 确认它是否已出现在后端导出的定义里，再检查当前运行的后端和接口声明。
