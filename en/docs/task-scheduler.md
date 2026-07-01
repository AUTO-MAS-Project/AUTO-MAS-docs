# Task Management

## Scheduling Queues

A **scheduling queue** is the AUTO-MAS module that organizes script tasks. It can define the run order for **multiple scripts**, run tasks automatically when the app starts, and run tasks on a schedule.

::: warning Reminder
Scheduling queues run scripts in sequence, which means the next script starts only after the previous script finishes.

If scripts in a single queue appear to run in parallel, check the general script settings.

Different start times in the same queue are not per-script start times.

Running a queue starts all scripts in that queue.

If you plan to use this feature, read this document carefully before asking questions.
:::

![scheduler](/docs/img/advanced-features/scheduler.png)

### Usage

1. Click **New Queue** in the upper-right corner to create a queue.
2. Enable **Run on startup** and/or **Scheduled run** according to your needs.

::: tip Tips
You can set AUTO-MAS to start with Windows and enable **Run on startup** in the scheduling queue. This lets scripts for emulators, such as MAA, run automatically when you start your computer.

If you have a machine that never stops, you can enable scheduled runs and let AUTO-MAS perform automation at the selected times.

If you have special login requirements, such as MAA custom infrastructure tasks, you can start MAA automatically before the custom infrastructure takes effect so MAA can switch infrastructure layouts.
:::

3. Click **Add schedule** and set the time when scripts should run. Remember to change the scheduled run status to **Enabled**.
4. Add tasks. A **task** here refers to a script task already configured in **Script Management**. If no tasks are available, read [Script Configuration](/en/docs/script-guide/).

## Auto-Proxy Strategy

The following describes AUTO-MAS task scheduling behavior in **auto-proxy** mode.

- Each **user** contains two subtasks: **Annihilation** and **Daily**. In simple mode, **Daily** is enabled by default. The app does not check whether the same user is running elsewhere. Scheduling order: **Annihilation > Daily**. This item applies only to MAA scripts.
- Each **script instance** contains multiple users. A **script instance task** is the sum of all tasks for its users. The same **script instance** cannot be started repeatedly. If it is already running, a new **script instance task** will be skipped. Scheduling order: **ascending user index**.
- Each **scheduling queue** contains multiple **script instance tasks**. A **scheduling queue task** is the sum of all **script instance tasks** in its task queue. The same **scheduling queue** can be started repeatedly. Scheduling order: **ascending task instance index**.
- Each **scheduler console** can run and display one **scheduling queue task**. Create multiple **scheduler consoles** to run multiple queues.

## Manual Review

**Manual review** is a mode for checking user automation status. It reviews users one by one and records the review result.

::: info Help me, developer

Currently only MAA is supported. More may come later.

:::

- Select **Manual review mode**, then click **Start task**.
- The app starts MAA and logs in to each user account in order.
- **After PRTS login completes**, manually check the automation status and confirm unfinished tasks.
- After the review ends, the system records the result in the status information section on the **User Management** page.
