# 自定义外观包

外观包是一个 ZIP 文件，可以替换配色、整窗背景、侧栏菜单图标、鼠标光标，并在角落放一张装饰图。

## 导入与切换

在「设置 → 界面设置 → 主题模式」点击「导入外观」，选择 ZIP 文件。导入成功后会弹出预览，点击「应用外观」即可生效。

- 已导入的外观会出现在「主题模式」下拉框里，和跟随系统、浅色模式、深色模式放在一起，随时可以切回内置外观。
- 选中自定义外观时，旁边会出现「移除外观」按钮。
- 再次导入相同 ID 的外观包时会询问是否替换。
- 外观包保存在用户数据目录，升级 AUTO-MAS 不会清除。
- 使用自定义外观时主题色由外观包决定，「主题色」选项不可用。

## 包结构

外观包只能包含 JSON 和 PNG、JPEG、WebP 图片。CSS、JavaScript、HTML、SVG 一律不执行，也不会加载网络资源。

```text
my-appearance.zip
├── theme.json
├── preview.png                 # 可选，导入后的预览图
└── assets/
    ├── background.png          # 可选，整窗背景
    ├── mascot.webp             # 可选，角落装饰图
    ├── menu-home.png           # 可选，菜单图标（也可复用其他已声明的图片）
    └── cursor-pointer.png      # 可选，PNG 光标（1–64 px）
```

## theme.json 示例

```json
{
  "formatVersion": 1,
  "id": "my-appearance",
  "name": "我的外观",
  "description": "一个暖色外观示例",
  "mode": "light",
  "tokens": {
    "colorPrimary": "#1677ff",
    "colorBgLayout": "#f5f7fb",
    "colorBgContainer": "#ffffff",
    "colorBgElevated": "#ffffff",
    "colorText": "#1f2937",
    "colorTextSecondary": "#667085",
    "colorBorder": "#d0d5dd",
    "colorBorderSecondary": "#eaecf0",
    "borderRadius": 8
  },
  "background": {
    "path": "assets/background.png",
    "opacity": 1,
    "surfaceOpacity": 0.72,
    "position": "right bottom",
    "size": "cover"
  },
  "mascot": {
    "path": "assets/mascot.webp",
    "width": 144,
    "opacity": 1,
    "position": "bottom-right"
  },
  "preview": "preview.png",
  "menuIcons": {
    "home": "assets/menu-home.png",
    "settings": "assets/menu-home.png"
  },
  "cursors": {
    "pointer": {
      "path": "assets/cursor-pointer.png",
      "hotspotX": 0,
      "hotspotY": 0
    }
  }
}
```

## 字段说明

### 基本信息

- `id`：只能使用小写字母、数字、下划线和短横线，不能用 `con`、`nul`、`com1` 这类 Windows 保留名。
- `mode`：`light` 或 `dark`，决定界面按浅色还是深色绘制。
- `tokens`：Ant Design 配色。`colorPrimary` 必填，其余可省略。

### 背景 `background`

整窗固定背景，覆盖标题栏、侧栏和内容区；切换页面、滚动内容时图片保持原位。

- `path`：必须放在 `assets/` 目录。
- `opacity`：图片本身的不透明度，0 到 1，默认 1。
- `surfaceOpacity`：布局和卡片表面的不透明度，0 到 1，默认 0.72。0 表示表面全透明，1 表示不透明。
- 文字、图标、按钮和输入框不会跟着变透明；弹窗、抽屉和下拉菜单保持不透明。
- 只有提供了背景图片才启用透明表面，纯配色的外观包沿用正常背景。

### 角落装饰 `mascot`

- `path`：必须放在 `assets/` 目录。
- `position`：`top-left`、`top-right`、`bottom-left`、`bottom-right`，默认 `bottom-right`。
- 主界面会在装饰图所在一侧留出空间，避免遮挡内容。窗口宽度不超过 800px 时隐藏装饰图。
- 初始化页面和独立日志窗口使用外观包的配色与背景，但不显示装饰图。

### 菜单图标 `menuIcons`

每个值指向 `assets/` 下的 PNG、JPEG 或 WebP 图片，没写的键保留内置图标。同一张图片可以给多个键复用。侧栏按 24px 显示，建议用透明背景。

可用的键：`home`、`scripts`、`plans`、`emulators`、`queue`、`scheduler`、`gameSign`、`history`、`tools`、`settings`。

开发菜单另有 `testRouter`、`ocrDev`、`overlayMaskDev`、`updateDownloadDev`，只在开发环境显示。

### 光标 `cursors`

只支持 `default`、`pointer`、`text` 三个键。

- 图片必须是 `assets/` 下的 PNG，宽高均为 1 到 64 像素。
- `hotspotX`、`hotspotY` 是可选的非负整数，默认 0，必须落在图片范围内。
- 没写的键继续使用系统光标。

## 导入校验

导入时会检查以下内容，任何一项不通过都会拒绝整个包：

- 路径：不允许绝对路径、`..`、符号链接和 Windows 保留文件名。
- 图片：文件内容必须和扩展名一致。
- 文件：每个文件都必须在 `theme.json` 里引用，不能有多余文件。
- 大小：文件数量和解压后的总大小都有上限。
