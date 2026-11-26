# 前端开发

## 生成前端 API

新增 FastAPI 接口，或修改请求与响应模型后，需要重新生成前端的 TypeScript 类型和请求客户端。

先按[分别启动后端与前端](../getting-start/environment.md#_4-1-分别启动后端与前端)启动当前本地后端并保持运行，然后在仓库根目录另开一个终端执行：

```sh
cd frontend
vp run openapi
```

该命令会读取 `http://127.0.0.1:36164/openapi.json` 并自动更新 `frontend/src/api/`。接口契约以 FastAPI 的声明为准，切勿手动修改生成的客户端文件。

如果生成结果中缺少预期的接口或字段，请先打开 `http://127.0.0.1:36164/openapi.json` 确认后端已正确导出，再排查当前运行的后端进程与接口定义。
