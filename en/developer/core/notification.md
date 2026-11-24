# Notification System

Notifications are a core application component, available through `from app.core import Notify`. `app/core/notify/__init__.py` creates the global instance. `app/core/notify/manager.py` selects delivery targets from global or account settings and coordinates rendering, sending, and reporting the results. Application code decides when to send and supplies the content. For user settings, see [Push Notifications](../../docs/advanced-features/notification.md).

## Send a Notification

In an asynchronous application function, build a common message and choose the sending method that fits the use case.

```python
from app.core import Notify
from app.models.notification import NotifyPayload

<notification payload> = NotifyPayload(
    title=<notification title>,
    text=<plain-text body>,
)

<dispatch result> = await Notify.send_global(
    <notification payload>,
    include_system=<include system notifications>,
)
```

- **Message parameters**

  - `title=` is the notification title.
  - `text=` is a self-contained plain-text body. Targets that support rich text can use other bodies provided in the same message.

- **Delivery parameters**

  - `<notification payload>` is the `NotifyPayload` to send.
  - `include_system=` defaults to `False`. Set it to `True` to include the system notification channel if it is enabled.
  - `<dispatch result>` is the `DispatchResult` for this delivery.

### Choose a Sending Method

- **Send using one account's settings**

  ```python
  <dispatch result> = await Notify.send_user(<notification payload>, <account configuration>)
  ```

  - `<account configuration>` supplies the settings for this dispatch. The host resolves recipients from the channels enabled for that account.

- **Send to selected targets**

  ```python
  <dispatch result> = await Notify.dispatch(
      payload=<notification payload>,
      targets=<delivery targets>,
      attempts=<total attempts>,
      retry_delay=<retry delay in seconds>,
      skip_channel_ids=<already delivered target IDs>,
  )
  ```

  - `targets=` takes targets built by methods such as `Notify.global_target()` and `Notify.user_target(<account configuration>)`. Building targets does not send a message.
  - `attempts=` includes the first attempt and defaults to `1`.
  - `retry_delay=` is the interval in seconds between attempts and defaults to `0`.
  - `skip_channel_ids=` takes the stable IDs to skip and defaults to an empty collection. Pass the previous dispatch's `succeeded_ids` when retrying undelivered targets.

- **Send a system notification or test a Webhook directly**

  ```python
  await Notify.send_notice(<local notice>)
  await Notify.test_webhook(<notification payload>, <Webhook target snapshot>)
  ```

  - `<local notice>` is a `LocalNotice` with a title, a message, ticker text, and a display duration.
  - `<Webhook target snapshot>` is a `WebhookTargetSnapshot` with the URL, request method, template, headers, and other settings for this test. The settings page can test a draft before it is enabled.

## Messages and Images

Message types live in `app/models/notification/`. Application code can provide plain text, Markdown, HTML, a summary, and images in the same message. The host selects a body and resolves image references according to the target's capabilities.

### Provide Bodies and a Summary

```python
from app.models.notification import NotificationSummary, NotifyPayload

<notification payload> = NotifyPayload(
    title=<notification title>,
    text=<plain-text body>,
    markdown=<Markdown body>,
    html=<HTML body>,
    summary=NotificationSummary(
        text=<compact summary>,
        title=<summary title>,
        overflow_text=<fallback for an oversized body>,
    ),
)
```

- **Optional bodies**

  - `markdown=` and `html=` default to `None`, which means the format is absent. An empty string counts as a provided body.
  - `append_signature=` defaults to `True`. The renderer appends the shared signature when the message and target both allow it.
  - `body_title=` is an optional body heading. The target's title policy determines whether to use it.

- **Summary**

  - `text=` is the body for targets that need a compact display.
  - `title=` is used when the target allows the summary to replace the title. It defaults to `None`.
  - `overflow_text=` is used when the target supports overflow fallback and the body exceeds its byte budget. An absent or empty value falls back to the compact summary.

Use `DeferredContent(<content factory>)` for optional content that is expensive to generate, such as templates or images. The renderer generates it when a target needs it and reuses the result across targets and retries. If body generation fails, the renderer tries the next format in the target's preference order.

### Reference an Image in a Body

```python
from app.models.notification import NotificationImage, image_reference

<image resource> = NotificationImage(
    id=<image ID>,
    data=<raw image bytes>,
    url=<image URL>,
    alt=<alternative text>,
    mime_type=<image MIME type>,
)
<image reference> = image_reference(<image ID>)
```

- **Image resource**

  - `id=` identifies one image within the message. Start with a letter or digit. The rest can include letters, digits, underscores, periods, and hyphens.
  - `data=` and `url=` require at least one value. When both are provided, they should describe the same image.
  - `alt=` defaults to an empty string, and `mime_type=` defaults to `"image/png"`.

- **Body reference**

  - Add `<image resource>` to `NotifyPayload.images` and use `<image reference>` in an HTML `img src` or Markdown image link.
  - Mail can embed raw image bytes. Markdown images use an existing URL. The renderer preserves readable text when the target cannot display an image.
  - Escape dynamic text such as account names and errors before inserting it into HTML. Channel implementations handle Base64, MIME, and other protocol encodings.

## Delivery Flow

`app/core/notify/manager.py` reads configuration and assembles senders. `app/core/notify/content.py` renders content for each target. `app/core/notify/dispatch.py` sends messages and combines results.

1. **Prepare targets.** The host checks enabled channels in the current scope and builds a `ChannelTarget` for each recipient. Each target captures the destination, content capabilities, and required transport settings for the dispatch. Stable IDs are used for retry and display names are stored separately.
2. **Render content.** Choose the first available format in the target's preference order, then apply its title, signature, image, and summary handling. When the body exceeds the target's limit, the renderer can use the supplied fallback summary according to the target's policy. A rendering failure is recorded for that target.
3. **Send.** Adapters in `app/core/notify/channels/` encode the protocol and attempt delivery. Retryable failures are retried up to the caller's limit. Each retry reuses the rendered content and target snapshot. Within a configuration transaction, dispatch runs in order in the same coroutine.
4. **Combine results.** Return results in input order. Include channel discovery and target preparation failures while allowing other resolved targets to proceed.

### Read the Dispatch Result

- **Delivery counts**

  - `attempted` counts targets handled by the dispatch. Retries for the same target do not increase this count.
  - `succeeded` and `failed` contain display names for successful targets and failures. Use `succeeded` or `succeeded_ids` to check whether any message was delivered.

- **Target IDs and failure reasons**

  - `succeeded_ids` and `failed_ids` contain stable IDs for resolved targets. They can be used for deduplication or retry.
  - `failures` reports the phase, category, retryability, and number of actual send attempts. Channel discovery and target preparation failures may have no target ID.
  - `queued` indicates that the host has accepted a task notification for later delivery. It does not indicate delivery success.

- **Empty-recipient policy | `empty_policy`**

  - `"send"` passes the target to the channel.
  - `"warn"` warns and counts it as a failure.
  - `"skip"` skips the target.

## Task Reports and Community Notifications

The report entry point lives in `app/core/notify/manager.py`. `app/core/notify/task_report.py` captures the report policy for each request. `app/core/notify/coordinator.py` handles waiting for, combining, and resending community sign-in summaries.

```python
<dispatch result> = await Notify.push_proxy_result(
    title=<report title>,
    report=<task result report>,
    task_info=<task information node>,
)
```

- **Task result report**

  - `report=` takes a `TaskResultReport` with start and end times, completed and incomplete account counts, and the results body. Put extra rendering data for specialized templates in `template_fields`.
  - `task_info=` takes the current `TaskItem` and links the report to the task's notification state. Omit it to use the current notification batch context.
  - `result_template=` selects an optional HTML template and defaults to `"general_result.html"`.

- **Statistics and immediate notifications**

  - `Notify.push_statistics_report()` sends a prepared statistics body from within a task. It selects targets using global and account statistics settings. The host manages delivery and failure feedback.
  - `Notify.send_statistics()` is for sending standalone statistics and returns the dispatch result.
  - `Notify.dispatch_task_notification()` is for task code that already has the message and targets. It uses the host notification lifecycle.

A task report briefly waits for a community sign-in summary to include. If sign-in finishes later, the coordinator sends the summary only to targets that have yet to receive it. Standalone sign-in notifications use `Notify.community` with the existing sign-in results. Callers submit results through their own entry points, and the host tracks which targets received the summary.

## Extend Built-in Channels

Channel implementations live in `app/core/notify/channels/<channel ID>/`. Each channel builds its targets, declares content capabilities, encodes its protocol, and attempts delivery once. The directory's `README.md` describes the manifest and registration contract.

1. **Declare a manifest.** Add `manifest.json` with the interface version, channel ID, registration entry point, and required modules. See the example below.
2. **Register the channel.** Provide `register() -> NotifyChannel` with settings metadata, scopes, the enable field, `target_builder`, `send_adapter`, and `sender_factory`. Registration assembles the declaration.
3. **Build targets.** `target_builder` reads the host's `ChannelConfig`, creates a `ChannelTarget` with a stable ID for each explicitly enabled recipient, and declares the target's capabilities.
4. **Implement one send attempt.** Use the injected transport dependencies to encode `RenderedNotification` as a protocol request and interpret the response. Returning `True` or `None` indicates success. Returning `False` or raising an exception indicates failure. Use `NotificationDeliveryError` to report whether the channel failure can be retried.
5. **Update configuration and verify behavior.** When adding fields or public API routes, follow the [Configuration Framework](./config.md) and [Frontend Development](./api.md) guides. Use the fake senders in `tests/core/test_notify.py` to check scope and activation, target resolution, content choice, and reported failures.

```json
{
  "interface_version": 1,
  "id": "<channel ID>",
  "entrypoint": "app.core.notify.channels.<channel ID>:register",
  "requires": ["<required third-party module>"]
}
```

- **Manifest parameters**

  - `id` is the stable channel ID and matches the declaration's `key`.
  - `entrypoint` is the channel package's registration function. The package directory matches `id`.
  - `requires` lists required top-level third-party module names. Use an empty list when there are none.
