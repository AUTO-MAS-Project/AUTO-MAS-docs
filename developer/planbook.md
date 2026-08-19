# 计划表规范

## 1. 负责做什么

计划表框架只处理日期，不理解专项业务。
计划表框架返回完整 key，不按字段拆解，也不根据 key 内容作业务判断。Key 可以是字符串，也可以是多层嵌套对象。

## 2. 通用计划表结构

计划表的后端类型是 `XXXPlanConfig`。一张计划表包含基础信息和八个日期槽位：

```json
{
  "Info": {
    "Name": "材料周计划",
    "Mode": "Weekly"
  },
  "ALL": {
    "Key": {}
  },
  "Monday": {
    "Key": {}
  },
  "Tuesday": {
    "Key": {}
  },
  "Wednesday": {
    "Key": {}
  },
  "Thursday": {
    "Key": {}
  },
  "Friday": {
    "Key": {}
  },
  "Saturday": {
    "Key": {}
  },
  "Sunday": {
    "Key": {}
  }
}
```

各日期槽位只有一个框架字段：`Key`。框架把它当作不透明数据保存和返回。

`Info` 字段如下：

| 字段 | 可选值 | 默认值 | 说明 |
| --- | --- | --- | --- |
| `Name` | 字符串 | `新 MaaEnd 计划表` | 计划表名称 |
| `Mode` | `ALL`、`Weekly` | `ALL` | 全局模式或周计划模式 |

## 3. 维护位置

| 范围 | 位置 | 作用 |
| --- | --- | --- |
| 通用日期-key 容器 | `app/models/config.py` 中的 `WeeklyKeyPlanConfig` | 保存槽位并按日期返回 key |
| MaaEnd API 契约 | `app/models/schema.py` | 定义两种嵌套 key 及枚举约束 |
| MaaEnd key 转换 | `app/models/config.py` | 固定配置转换、、运行前校验 |
| MaaEnd 任务注入 | `app/task/maaend/AutoProxy.py` | 解释 key 并修改 MaaEnd 任务 |
| 前端 key 工具 | `frontend/src/utils/maaEndProtocolSpace.ts` | 编辑、显示、归一化和兼容旧结构 |
| 计划表界面 | `frontend/src/views/plan/tables/MaaEndPlanTable.vue` | 编辑日期对应的 MaaEnd key |

专项必须拥有自己的 key schema、编辑界面和消费逻辑。不要把专项字段塞回计划表框架，也不要让框架根据 key 内容分派业务。
