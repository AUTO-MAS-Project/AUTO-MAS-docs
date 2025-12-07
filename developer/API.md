# API 开发流程

## 1. 定义数据模型

在 `app/models/schema.py` 中定义数据模型


```python
from pydantic import BaseModel

class MyRequest(BaseModel):
    param1: str
    param2: int

class MyResponse(BaseModel):
    success: bool
    data: dict
```

## 2. 创建 API 路由

**在 `app/api/` 对应模块中创建路由**

```python
from fastapi import APIRouter
from app.models.schema import MyRequest, MyResponse

router = APIRouter()

@router.post("/my-endpoint", response_model=MyResponse)
async def my_endpoint(request: MyRequest):
    # 业务逻辑
    return MyResponse(success=True, data={})
```

## 3. 生成前端代码

1. **启动后端**

```bash
python main.py
```
2. **使用 openapi-generator-typescript 生成前端代码**

```bash
openapi --input http://127.0.0.1:36163/openapi.json --output ./src/api --client axios
```
如果没有输出，代表已经生成完成了；如果报错了，请自行查阅报错内容。

由于后端部分接口使用了不兼容的类型定义，可能有部分 `枚举类` 生成错误，这是正常现象，自行撤销修改即可。

::: warning 注意
当你阅读至此，说明你正在开发一个新的 API，请你牢记，你直接使用 `yarn dev` 时，后端实际上使用的是 GitHub 上 `dev` 分支中的后端代码，而非你当前开发的后端。

因此，为了在前端测试你的 API，你必须先在本地启动后端，再运行前端，您可以参考 [使用来自本地的后端](/developer/getting-start#_4-2-使用本地的后端代码)。
:::

##  4. 在前端调用 API

使用新生成的 frontend/src/api/services/Service.ts，即可调用api：

```typescript
import { MyEndpointRequest, MyEndpointResponse, MyEndpoint } from '@/api/services/Service.ts';
const requestData: MyEndpointRequest = {
    param1: 'value',
    param2: 123,
};
const response: MyEndpointResponse = await MyEndpoint(requestData);
console.log(response.data);
```