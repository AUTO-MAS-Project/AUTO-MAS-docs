# MCP Service

Through the MCP service, AI clients can call tools and features provided by AUTO-MAS.

## What is MCP?

MCP (Model Context Protocol) is an open protocol designed to provide standardized interfaces for AI models, allowing them to connect to external data sources and tools. With MCP, AI can safely and efficiently call features without needing to understand the underlying implementation details.

## Configure the MCP Service

For any MCP client that supports SSE, provide the AUTO-MAS MCP URL:

```text
http://localhost:36163/mcp
```

Common MCP clients such as Claude Desktop, Cursor, and Windsurf also support the following configuration:

```json
{
  "mcpServers": {
    "auto-mas": {
      "url": "http://localhost:36163/mcp"
    }
  }
}
```

## Use the MCP Service

After configuration, start the AUTO-MAS app. The AI client will discover and connect to this MCP service automatically, then call tools provided by AUTO-MAS to perform almost all tasks supported by AUTO-MAS.
