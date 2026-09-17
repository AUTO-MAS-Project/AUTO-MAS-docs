# MCP 服务

接上 MCP 之后，你可以直接对 AI 说"帮我把今天的号都代理一遍"，由 AI 去调用 AUTO-MAS 完成——几乎所有能在界面上点的操作，AI 都能替你做。

MCP（Model Context Protocol）是一套让 AI 调用外部工具的通用接口。你不需要了解它怎么工作，只要把下面的地址填进你的 AI 客户端就行。

## 配置

只要你的 AI 客户端支持 MCP，填这个地址即可：`http://localhost:36163/mcp`

Claude Desktop、Cursor、Windsurf 这类客户端用配置文件的形式填写：

```json
{
  "mcpServers": {
    "auto-mas": {
      "url": "http://localhost:36163/mcp"
    }
  }
}
```


## 使用

配好之后，**保持 AUTO-MAS 开着**，AI 会自动连上，之后直接用自然语言指挥它就行。

::: warning AUTO-MAS 没开的话连不上
这个地址是 AUTO-MAS 自己提供的服务，软件关了服务就没了，AI 会提示连接失败。
:::