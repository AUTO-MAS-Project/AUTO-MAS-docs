# 脚本专项适配

任务调度是 AUTO-MAS 最为核心的机制，知道了任务调度的完整实现方法，也就知道了 AUTO-MAS 项目后端的几乎所有机制。因此，从零开始拆解整个任务调度逻辑不太现实。笔者选择直接带着各位走一遍为某个脚本专项适配的完整流程，希望能对您有所帮助。


为方便叙述，本文以 `NScript` 指代被适配的脚本名。

## 1. 添加脚本配置项

### 1.1 定义配置模板

源代码路径：`app\models\config.py`

1. 找到 `GeneralConfig`、`GeneralUserConfig`，将这两个类复制一份，重命名其为 `NScriptConfig`、`NScriptUserConfig`。
2. 修整两个类中的字段使其大致符合预期，注意 `NScriptConfig.UserData` 一定要修改成 `MultipleConfig([NScriptUserConfig])`。
3. `GlobalConfig.ScriptConfig` 中添加 `NScriptConfig` 类型，`CLASS_BOOK` 中添加 `"NScript": NScriptConfig`。

### 1.2 修整 Config 方法

源代码路径：`app\core\config.py`

1. 找到 `AppConfig.add_user()`，补充 `NScriptConfig` 与 `NScriptUserConfig` 的处理逻辑。

### 1.3 适配 API 定义

源代码路径：`app\models\schema.py`、`app\api\scripts.py`

1. 找到 `GeneralConfig`、`GeneralUserConfig` 相关定义，仿照该格式完成 `NScript` 的类型定义。
2. 搜索 `GeneralConfig`、`GeneralUserConfig`，在所有涉及这两个类型的地方补充 `NScript`的相关引用与处理方法。

### 1.4 调试配置项

1. 启动后端并访问 `http://localhost:36163/docs` 调试相关接口。

## 2. 添加脚本任务

### 2.1 复刻通用调度

源代码路径：`app\task\general`

1. 复制一份 `app\task\general` 文件夹到相同目录并重命名为 `NScript`。
2. 基于原有代码逻辑，修改使其适配新的脚本调度，以下罗列所需注意的函数方法 *（当然，最快的适配方法就是拿一份通用调度配置，挨个把代码中对应的变量替换成对应的参数就行）*。

#### TaskExecuteBase

这是一个任务执行基类，包含3个必须实现的函数

- `main_task`：主任务，执行任务时先执行该方法，该方法内允许出现未捕获的异常
- `final_task`：收尾任务，无论主任务是否正常完成，该任务都会执行，常用于资源回收与总结信息，该方法内允许出现未捕获的异常。
- `on_crash`：异常处理，任何异常发生时都会调用该方法，该方法内不允许出现未捕获的异常

**TaskExecuteBase** 支持在多层嵌套执行的情况下保证各级子任务按顺序中止并执行收尾任务，具体调用方法如下：

1. 主任务调用 `.execute()` 方法开始执行任务。
2. 在任意任务的 `.main_task()` 中都可以调用 `await self.spawn(TaskExecuteBase 方法)` 执行下一级子任务，不建议调用 `self.spawn()` 方法而不使用 `await`。
3. 主任务调用 `.cancel()` 方法时，各级子任务将按调用栈顺序中止并执行收尾任务，您可以使用 `await .accomplish.wait()` 方法等待所有子任务执行完毕。

#### TaskItem

这是一个任务信息记录类，包含一个任务中所有需要记录的数据，相关内容发生更改时会自动向前端发送 `WebSocket` 消息，开发者只需要及时把相应信息更新写入 `TaskItem` 类的相应字段即可。

### 2.2 任务调度器适配

源代码路径：`app\core\task_manager.py`

1. `Task.main_task()` 中添加对 `NScriptManager` 的适配。
2. 查找获取任务列表的 API，添加对 `NScriptManager` 的适配。

### 2.3 杂项适配

1. 搜索 `GeneralConfig`，检查所有相关内容边上有没有添上对 `NScript` 的适配。

### 2.4 任务调度试运行

1. 调用 `/api/dispatch/start` 接口，测试新脚本能否正常运行。

## 3. 添加相关页面

自行在前端添加相关页面展示即可。