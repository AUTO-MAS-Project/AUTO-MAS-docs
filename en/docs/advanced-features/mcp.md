# MCP Service

Once MCP is connected, you can just tell an AI "run the dailies on all my accounts today" and it drives AUTO-MAS to get it done. Almost anything you can click in the UI, the AI can do for you.

MCP (Model Context Protocol) is a common interface for AI to call external tools. You do not need to know how it works. Just put the address below into your AI client.

## Configuration

If your AI client supports MCP, enter this address: `http://localhost:36163/mcp`

Clients like Claude Desktop, Cursor, and Windsurf take it as a config file instead:

```json
{
  "mcpServers": {
    "auto-mas": {
      "url": "http://localhost:36163/mcp"
    }
  }
}
```

## Usage

Once it is set up, **keep AUTO-MAS running**. The AI connects on its own, and from there you direct it in plain language.

::: warning The AI cannot connect if AUTO-MAS is not running
That address is a service AUTO-MAS provides itself. Close the app and the service goes with it, so the AI will report a connection failure.
:::
