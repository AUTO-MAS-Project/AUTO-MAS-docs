# Frontend Development

## Generate the Frontend API Client

When you add a FastAPI endpoint or change its request or response models, regenerate the frontend TypeScript types and request client.

Start the backend from your current checkout as described in [Start Backend and Frontend Separately](../getting-start/environment.md#_4-1-start-backend-and-frontend-separately). Leave it running and run the following commands from the repository root in another terminal.

```sh
cd frontend
yarn openapi
```

The command reads `http://127.0.0.1:36164/openapi.json` and updates `frontend/src/api/`. FastAPI declarations define the contract. Use this command to update the generated files.

If an expected route or field is missing from the generated client, look for it in `http://127.0.0.1:36164/openapi.json`, then check the running backend and its API declarations.
