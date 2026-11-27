# 声明插件能力

插件在包入口导出的插件类上声明能力。插件间调用使用服务声明，前端请求使用 HTTP 接口声明。宿主在插件的 `on_enable()` 成功后登记这些能力，停用时撤销登记。

## 服务声明

插件通过 `provides_services` 将服务名映射到插件实例的方法。宿主登记该方法，调用方可按服务名取得它。见[调用其他插件的服务](./host-capabilities.md#调用其他插件的服务)。

```python
from auto_mas_core import BasePlugin


class <插件类>(BasePlugin):
    provides_services = {"<服务名>": "<插件方法名>"}

    async def <插件方法名>(self, <服务参数>):
        return <服务结果>
```

## 接口声明

### FastAPI 路由

将 `APIRouter` 赋给插件类的 `api_router`。宿主挂载路由，接口定义随路由进入 OpenAPI。需要生成前端 API 时，先启用插件，再按[生成前端 API](../../core/api.md#生成前端-api)的步骤操作。

```python
from auto_mas_core import BasePlugin
from fastapi import APIRouter

router = APIRouter(prefix="/api/plugin/<插件 ID>")


@router.get("/<相对路径>")
async def <接口函数>(<请求参数>) -> <返回类型>:
    return <响应数据>


class <插件类>(BasePlugin):
    api_router = router
```

### 网关路由

使用 `@plugin_route` 标记插件实例的公开方法。宿主按插件 ID、相对路径和 HTTP 方法分发请求。网关路由由调用方按约定的 URL 请求。

```python
from auto_mas_core import BasePlugin, plugin_route
from fastapi import Request


class <插件类>(BasePlugin):
    @plugin_route("/<相对路径>", methods=["<HTTP 方法>"])
    async def <处理方法>(self, request: Request) -> dict:
        return <响应数据>
```

- `<相对路径>`：插件网关下的路径，完整地址为 `/api/plugin/<插件 ID>/<相对路径>`。
- `<HTTP 方法>`：该路由处理的请求方法。省略 `methods` 时使用 `GET`。
