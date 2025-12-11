# MCP 服务

利用 MCP 服务，AI 可以调用 AUTO-MAS 提供的各类工具和功能。

## 什么是 MCP

MCP（Model Context Protocol）是一种开放协议，旨在为 AI 模型提供标准化的接口，使其能够连接外部数据源和工具。通过 MCP，AI 可以安全、高效地调用各种功能，而无需关心底层实现细节。

## 配置 MCP 服务

对于任何支持 SSE 的 MCP 客户端，您只需提供 MCP 的网址即可。所有最流行的MCP客户端（Claude Desktop、Cursor 和 Windsurf）都支持使用以下配置：

```json
{
  "mcpServers": {
    "auto-mas": {
      "url": "http://localhost:36163/mcp"
    }
  }
}
```


## 使用 MCP 服务

配置完成后，启动 AUTO-MAS 应用，AI 会自动发现并连接到该 MCP 服务，并可以调用 AUTO-MAS 提供的工具，执行几乎所有 AUTO-MAS 支持的任务。