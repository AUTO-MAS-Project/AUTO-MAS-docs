# Frontend Development

## Generate the Frontend API Client

Whenever you add a FastAPI endpoint or modify request/response models, regenerate the frontend TypeScript types and API client.

Start your local backend as described in [Start Backend and Frontend Separately](../getting-start/environment.md#_4-1-start-backend-and-frontend-separately) and keep it running. Then open another terminal at the repository root and run:

```sh
cd frontend
vp run openapi
```

This command fetches `http://127.0.0.1:36164/openapi.json` and regenerates `frontend/src/api/`. FastAPI declarations are the single source of truth—never edit generated files under `frontend/src/api/` by hand.

If an expected route or field is missing from the generated output, check `http://127.0.0.1:36164/openapi.json` first to verify that the running backend exports it, then inspect the FastAPI route and schema declarations.
