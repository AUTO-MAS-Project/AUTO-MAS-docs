# API 开发流程

## 定义数据模型

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

## 创造API路由

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

## 生成前端代码
首先，先**启动后端**
```bash
python main.py
```
待后端启动完成后，使用 openapi-generator-typescript 生成前端代码

```bash
openapi --input http://127.0.0.1:36163/openapi.json --output ./src/api --client axios
```
如果没有结果，代表已经生成完成了，如果报错了，请自行查阅报错内容

::: warning 注意
当你阅读至此，说明你正在开发一个新的 API，请你牢记，你使用 npm run dev时，后端实际上使用的是dev的分支，而非你当前开发的分支。

因此，为了在前端测试你的API，你必须使用你当前开发的分支启动后端，才能测试你新开发的API。

为了做到这一点，先开启后端，然后开启前端，跳过前端的初始化流程（可以使用调试面板）即可。

:::

##  **在前端调用 API   **

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