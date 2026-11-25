# Troubleshooting with the Community Bot

<Links
  :items="[
    {
      name: 'Join the Official AUTO-MAS QQ Group',
      link: 'https://qm.qq.com/q/hdV065dcbu',
      image: '/icons/tencentqq.svg',
      desc: 'Group ID: 957750551'
    }
  ]"
/>

The official AUTO-MAS QQ group includes an automated bot that can analyze logs for you. Here is how to use it:

1. Send `debug [your specific question]`, for example: `debug 为什么 MaaEnd 运行超时`.
2. The bot will prompt you to upload a log bundle. In AUTO-MAS, go to **Settings → Log Management → Export Topic Log Package** (设置 → 日志管理 → 导出专项日志包) and export the log bundle matching your issue.
3. Upload the exported log bundle to the group and wait for the bot to reply.
4. If you need to add more logs, upload them directly to the group and the bot will capture them automatically. To provide additional details, `@` mention the bot with your description.
5. Once your issue is resolved, send `debug resolve` to close the issue. If there is no follow-up for 30 minutes, the bot closes it automatically.
6. If your issue is still unresolved, you can `@` a group admin for further help (just kidding).

Send `debug help` to view more bot commands and usage tips.
