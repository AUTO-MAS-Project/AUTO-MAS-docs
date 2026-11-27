# Declare Plugin Capabilities

Declare capabilities on the plugin class exported by the package entry point. Use service declarations for calls from other plugins and HTTP declarations for frontend requests. The host registers these capabilities after `on_enable()` succeeds and unregisters them on disable.

## Service Declaration

Use `provides_services` to map service names to plugin instance methods. The host registers each method so that consumers can look it up by name. See [Call Services from Other Plugins](./host-capabilities.md#call-services-from-other-plugins).

```python
from auto_mas_core import BasePlugin


class <plugin class>(BasePlugin):
    provides_services = {"<service name>": "<plugin method name>"}

    async def <plugin method name>(self, <service arguments>):
        return <service result>
```

## HTTP Interface Declarations

### FastAPI Routes

Assign an `APIRouter` to the plugin class's `api_router`. The host mounts the router and includes its endpoints in OpenAPI. Enable the plugin before following [Generate the Frontend API Client](../../core/api.md#generate-the-frontend-api-client).

```python
from auto_mas_core import BasePlugin
from fastapi import APIRouter

router = APIRouter(prefix="/api/plugin/<plugin ID>")


@router.get("/<relative path>")
async def <endpoint function>(<request parameters>) -> <return type>:
    return <response data>


class <plugin class>(BasePlugin):
    api_router = router
```

### Gateway Routes

Use `@plugin_route` to mark a public method on the plugin instance. The host dispatches requests by plugin ID, relative path, and HTTP method. Callers send requests to the agreed gateway URL.

```python
from auto_mas_core import BasePlugin, plugin_route
from fastapi import Request


class <plugin class>(BasePlugin):
    @plugin_route("/<relative path>", methods=["<HTTP method>"])
    async def <handler method>(self, request: Request) -> dict:
        return <response data>
```

- `<relative path>` is the path under the plugin gateway. The full address is `/api/plugin/<plugin ID>/<relative path>`.
- `<HTTP method>` is the request method handled by the route. The default is `GET` when you omit `methods`.
