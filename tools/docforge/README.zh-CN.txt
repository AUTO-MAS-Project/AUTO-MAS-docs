# DocForge：AUTO-MAS 中文文档工坊

DocForge 是专门为本仓库制作的 Windows 绿色文档工具。它使用 Rust 构建，直接运行
`docforge.exe`，不会安装服务，也不会修改注册表、系统 PATH、文档站的 Node 依赖或
VitePress 配置。

## 第一次使用

1. 解压 `docforge-portable-windows-x64.zip`，不要只把 EXE 单独拖出来。
2. 双击 `docforge.exe`。
3. 点击左上角“打开文档站”，选择 `AUTO-MAS-docs` 仓库根目录。
4. 工具会自动打开“开始使用”文档；也可以从左侧资源管理器选择其他中文或英文文档。
5. 中间是带行号的 Markdown 编辑器，右侧是同步预览和检查结果。
6. 确认无误后点击右上角“保存文档”或按 `Ctrl+S`。

如果状态栏提示未找到 FFmpeg，请把 `ffmpeg.exe` 和 `ffprobe.exe` 放进绿色包的
`bin` 文件夹。PDF 首页图片预览需要 Poppler 的 `pdftoppm.exe`；没有它时仍可使用
PDF 文字预览和转换。

## 已支持的功能

- 自动识别本站的中文根目录、`en/` 英文目录，以及 `docs`、`developer`、
  `plugin`、`download`、`disclosure` 五个栏目。
- 编辑和预览 VitePress Markdown，检查一级标题、代码块、`::: / ::::` 容器以及本地图片。
- 保存时先写入同目录临时文件并同步到磁盘，再原子替换目标文件；不会写出文档站根目录。
- 使用 OpenAI-compatible `/chat/completions` 接口生成任意语言的翻译草稿。API Key
  只保存在本次运行的内存里，译文必须人工校对后才保存。
- 把当前编辑器里的内容导出为 PDF，标题、正文和本地图片都会写入；也支持 PDF 可复制
  文字导入 Markdown。安装 `pdftoppm.exe` 后会在导出页显示 PDF 首页。
- 导入 MP4、MKV、MOV、WebM 或 AVI 视频，在指定时间自由截图、按时间均匀截图，或用
  FFmpeg 场景识别自动提取关键画面。
- 自动把视频关键画面整理成“开始前准备、操作步骤、完成检查”教程；可以生成新的中英文
  站点文档，也可以追加到当前文档。每个步骤的标题和说明都可以在生成前修改。
- 对截图进行框选重点、箭头指向、自由涂画、马赛克遮挡和画面裁剪，支持撤销与清空。
- 在右侧文档预览中点击任意本地图片，可以直接进入画面编辑器；保存时会生成图片副本并
  自动替换当前 Markdown 引用，不会破坏原图。

## 翻译怎么用

1. 先打开一篇文档。
2. 点击左侧活动栏的“翻译”。
3. 填写接口地址、模型、源语言和目标语言；需要鉴权时再填写 API Key。
4. 点击“生成翻译草稿”，在下方逐段检查。
5. 可以把译文放入当前草稿，也可以原子保存到对应语言路径。

翻译会要求模型保留 frontmatter、代码、HTML/Vue 组件、VitePress 容器、URL、图片路径、
命令和配置键，但机器翻译仍可能出错，所以工具不会跳过人工校对直接覆盖。

## 导出当前文档 PDF

1. 从资源管理器选择文档，在编辑器中完成修改；不需要先保存 Markdown。
2. 点击左侧“PDF 导出”，确认页面显示的当前文档名称。
3. 点击“导出当前编辑内容为 PDF”，选择保存位置。
4. 导出成功后，右侧会显示 PDF 首页、页数和实际保存位置。

## PDF 的限制

- PDF 导入只提取可复制文字，复杂表格、扫描件、图片和多栏排版需要手动整理。
- PDF 导出保留标题、段落、列表、引用和本地 Markdown 图片；远程网络图片会保留文字提示。
- 导出中文 PDF 时，工具优先使用绿色包 `fonts` 目录中的兼容字体，再尝试 Windows 中文字体。
  如果仍提示缺字体，请点击“选择 PDF 字体”并选择有合法使用权限的 TTF 或 OTF 文件。

## 视频和画面编辑怎么用

1. 点击左侧“视频教程”，再点击“选择视频”。
2. 选择“自动识别关键画面”“均匀截取”或拖动时间位置后自由截图。
3. 为每张缩略图填写清晰的步骤标题和操作说明。
4. 点击缩略图，在右侧使用框选、箭头、涂画、遮挡或裁剪编辑画面。
5. 选择“生成新教程文件”或“追加到当前文档”。生成新文件时再选择语言、栏目和文件名。
6. 点击“生成并打开教程文档”；也可以直接点击右上角“一键截图并生成教程”。

教程图片会原子复制到对应栏目的 `img/generated`，Markdown 文件最后原子写入。生成新文件时，
如果目标已经存在，工具会要求修改文件名，不会直接覆盖原教程。

## 编辑当前文档里的图片

1. 在“编辑”页右侧预览中找到图片。
2. 点击图片卡片右上角“框选 / 箭头 / 涂抹”。
3. 在画面编辑器中添加标注并点击“保存副本并替换文档引用”。
4. 返回编辑页确认预览，最后点击“保存文档”。原图片会保留，可随时手动恢复引用。

## 给开发者

在仓库根目录检查和运行：

~~~powershell
$env:CARGO_HOME = "$PWD\tools\docforge\tmp\cargo-home"
$env:CARGO_TARGET_DIR = "$PWD\tools\docforge\target"
cargo test --locked --manifest-path tools\docforge\Cargo.toml
cargo run --locked --manifest-path tools\docforge\Cargo.toml -- --site .
~~~

构建绿色包：

~~~powershell
.\tools\docforge\package.ps1
~~~

如果需要把本地 FFmpeg、ffprobe、pdftoppm 和相关 DLL 一起装进绿色包：

~~~powershell
.\tools\docforge\package.ps1 -MediaToolsDirectory D:\media-tools\bin
~~~

只给本次 Cargo 进程使用代理，不写全局设置：

~~~powershell
.\tools\docforge\package.ps1 -Proxy http://127.0.0.1:7890
~~~

输出位于：

- `tools/docforge/out/docforge-portable/`
- `tools/docforge/out/docforge-portable-windows-x64.zip`

Cargo 缓存、编译目录和输出目录都限制在 `tools/docforge` 内，并由仓库的
`.gitignore` 忽略。
