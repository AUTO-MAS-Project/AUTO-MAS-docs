# API Development Process

## 1. Define Data Models

Define data models in `app/models/schema.py`.

```python
from pydantic import BaseModel

class MyRequest(BaseModel):
    param1: str
    param2: int

class MyResponse(BaseModel):
    success: bool
    data: dict
```

## 2. Create an API Route

**Create the route in the corresponding module under `app/api/`.**

```python
from fastapi import APIRouter
from app.models.schema import MyRequest, MyResponse

router = APIRouter()

@router.post("/my-endpoint", response_model=MyResponse)
async def my_endpoint(request: MyRequest):
    # Business logic
    return MyResponse(success=True, data={})
```

## 3. Generate Frontend Code

1. **Start the backend**

```bash
python main.py
```

2. **Generate frontend code with openapi-generator-typescript**

```bash
cd frontend
yarn openapi
```

If there is no output, generation has completed. If an error occurs, read the error message.

::: warning Note
`frontend/src/api` is maintained by the OpenAPI generator. After backend schema or API contract changes, regenerate this directory. Do not manually edit generated files to fix types or interfaces.
:::

::: warning Note
If you are reading this while developing a new API, remember that when you run `yarn dev` directly, the frontend actually uses backend code from the GitHub `dev` branch, not your current local backend.

To test your API from the frontend, start the local backend first, then run the frontend. See [using a local backend](/en/developer/getting-start).
:::

## 4. Call the API from the Frontend

Frontend business code should not call the generated `Service.ts` directly across page components. First wrap business-oriented composables in `frontend/src/composables`, then call those composables from pages or components.

The generated `frontend/src/api/services/Service.ts` should be used only as the low-level API client:

```typescript
import { Service } from '@/api';

export function useMyApi() {
    const submitMyRequest = async (param1: string, param2: number) => {
        return Service.myEndpoint({
            param1,
            param2,
        });
    };

    return {
        submitMyRequest,
    };
}
```

Page components should focus on interaction, loading states, error messages, and data display. API parameter preparation, response compatibility handling, and reusable logic should stay inside composables.
