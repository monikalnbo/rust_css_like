# 🚀 Rust CSS-Like (`.ui`) — 声明式前端语言与原生 UI 引擎

<div align="center">

**一个将「标记结构、紧凑样式、响应式逻辑」三位一体的原生级前端编程语言与多端渲染引擎。**

[![CI - Workspace Check & Test](https://github.com/monikalnbo/rust_css_like/actions/workflows/ci.yml/badge.svg)](https://github.com/monikalnbo/rust_css_like/actions/workflows/ci.yml)
[![Cross-Platform Native Build](https://github.com/monikalnbo/rust_css_like/actions/workflows/build-native.yml/badge.svg)](https://github.com/monikalnbo/rust_css_like/actions/workflows/build-native.yml)
[![WebAssembly Live Playground](https://img.shields.io/badge/WASM_Playground-Online-brightgreen)](https://monikalnbo.github.io/rust_css_like/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[English](README.md) | [简体中文](README_zh.md)

---

### *没有 HTML 的标签闭合地狱。没有 CSS 的类名层叠焦虑。没有 JS/Redux 的沉重样板代码。*

[🌐 在线 WASM 演练场](https://monikalnbo.github.io/rust_css_like/) • [📘 语言完全参考手册](docs/LANGUAGE_GUIDE.md) • [💻 预编译发布包下载 (.exe/.dmg)](https://github.com/monikalnbo/rust_css_like/actions) • [📚 架构全景白皮书](docs/01_REQUIREMENTS_AND_ARCHITECTURE.md)

</div>

---

## 📖 目录
1. [为什么要做一门新的前端语言？（解决什么痛点）](#一-为什么要做一门新的前端语言解决什么痛点)
2. [底层运作机制：从代码到屏幕全流程](#二-底层运作机制从代码到屏幕全流程)
3. [前端语言语法速览 (Syntax Guide)](#三-前端语言语法速览-syntax-guide)
4. [真实实战应用示例 (Examples)](#四-真实实战应用示例-examples)
5. [多端运行与体验途径 (Run & Deploy)](#五-多端运行与体验途径-run--deploy)
6. [高扩展底层注册总线 (Extensibility)](#六-高扩展底层注册总线-extensibility)
7. [底层 15 个 Crate 微内核工程架构](#七-底层-15-个-crate-微内核工程架构)
8. [核心文档索引清单](#八-核心文档索引清单)

---

## 一、 为什么要做一门新的前端语言？（解决什么痛点）

过去二十年里，传统 Web 前端技术栈（HTML5 + CSS3 + JavaScript / TypeScript + React / Vue / Electron）积累了极其沉重的心智模型与运行时包袱：

| 传统前端技术栈痛点 | Rust CSS-Like (`.ui`) 解决方案 |
| :--- | :--- |
| **多语言思维割裂**：开发者在 HTML（结构）、CSS（样式）、JS/TS（逻辑）与 JSX 之间反复切换。 | **三位一体单一语言**：标记结构、样式属性、响应式状态与事件流，全部统一在 `{}` 作用域中表达。 |
| **标签闭合地狱**：面对海量无穷无尽的 `<div></div>`、`<span></span>` 闭合标签。 | **零闭合标签**：纯粹使用大括号 `{}` 自然界定层级范围，行内叶子节点直接以分号或换行收敛。 |
| **样式命名焦虑**：BEM 规范、CSS Modules、Tailwind 几千个原子类，仍然面临样式层叠优先级不可控。 | **属性即样式（Inline Properties）**：`pad=16 bg=#1e293b rad=8 flex=1` 一气呵成，直达底层计算。 |
| **运行时臃肿迟钝**：一个简单的计算器应用，打包 Electron 后动辄 150MB~300MB，常驻 100MB+ 内存。 | **极致轻量原生**：纯原生二进制仅 **~1.2 MB**，冷启动 **< 15ms**，内存常驻仅 **10MB ~ 25MB**！ |
| **无原生后端直连**：必须另起 Node.js / Python / Go 后端服务打通数据库。 | **UI 内置全栈数据库**：在 `.ui` 文件中直接调用 `db.query` 获得响应式数据流。 |

```scss
// 仅需 12 行代码：一个跨平台的原生响应式计数器应用
let count = 0

win "极简计数器应用" (400, 300) bg=#0f172a {
    col pad=32 gap=16 align=center justify=center flex=1 {
        txt "当前计数: $count" #f8fafc 24px bold
        row gap=12 {
            btn "减少 (-1)" pad=(8, 16) bg=#334155 rad=6 -> count -= 1
            btn "增加 (+1)" pad=(8, 16) bg=#4f46e5 rad=6 -> count += 1
        }
    }
}
```

---

## 二、 底层运作机制：从代码到屏幕全流程

我们是如何设计并实现这套前端语言引擎的？  
整个引擎基于 Rust 采用 **5 层严格单向无环图 (Strict DAG)** 微内核架构。以下是您的 `.ui` 代码被逐级转换为物理像素的过程：

```
┌────────────────────────────────────────────────────────────────────────┐
│                        阶段 1：词法分词与语法分析                      │
│   源码 (.ui) ──► dsl-parser (大括号闭合状态机) ──► 抽象语法树 AST      │
└───────────────────────────────────┬────────────────────────────────────┘
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                        阶段 2：虚拟 DOM 树与样式解算                   │
│   抽象语法树 AST ──► element-core (虚拟 DOM) ──► style-system (7级层叠)│
└───────────────────────────────────┬────────────────────────────────────┘
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                        阶段 3：几何排版与文本排印                      │
│   layout-engine (Taffy 0.7 Flexbox) ──► text-layout (cosmic-text)      │
└───────────────────────────────────┬────────────────────────────────────┘
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                        阶段 4：DisplayList 指令流装配                  │
│   render-backend: 收集硬件无关的绘制指令 (矩形、文字、阴影、视口裁剪)  │
└───────────────────────────────────┬────────────────────────────────────┘
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                        阶段 5：多端光栅化渲染呈现                      │
│   桌面端 (tiny-skia 软光栅 / winit)  │  浏览器端 (WebAssembly / Canvas)│
└────────────────────────────────────────────────────────────────────────┘
```

1. **语法分析层 (`dsl-parser`)**：使用高效字符流提取 Token，通过大括号状态机实时跟踪闭合深度，生成干净的 `ScopeBlock` 抽象语法树；
2. **虚拟 DOM 与样式层 (`element-core` & `style-system`)**：构建轻量虚拟节点树，通过 7 级确定性特异度（Priority Cascade）将用户样式合并为紧凑的 `ComputedStyle` 结构体；
3. **几何排版与文本排印 (`layout-engine` & `text-layout`)**：将样式桥接至 Taffy 0.7 求解绝对物理坐标 `LayoutRect`；集成 `cosmic-text` 进行字形塑形、自动换行与 CJK 全角度量；
4. **绘制指令流 (`render-backend`)**：生成硬件无关的 `DisplayList` 绘制指令队列；
5. **多端渲染呈现 (`app-shell` / `wasm-runtime`)**：桌面端通过 `tiny-skia` 软件光栅化直推物理窗口显存，Web 端通过 WebAssembly 直绘 HTML5 `<canvas>`。

---

## 三、 前端语言语法速览 (Syntax Guide)

### 3.1 窗口与布局容器
* `win "标题" (宽, 高)`：定义原生顶层主窗口；
* `row`：横向弹性流布局（子项自左向右排列）；
* `col`：纵向弹性流布局（子项自上而下排列）；
* `box`：通用盒模型容器（支持内边距、外边距、圆角与边框）；
* `card`：预设阴影圆角质感的卡片容器。

### 3.2 极简样式缩写表（对齐 CSS）
* **间距度量**：`pad=16` (padding), `margin=12` (margin), `gap=10` (弹性子项间距)；
* **尺寸度量**：`w=200`, `h=48`, `w=100%`, `flex=1` (flex-grow 剩余空间自适应)；
* **对齐方式**：`align=center|start|end` (交叉轴对齐), `justify=between|center|around` (主轴对齐)；
* **视觉渲染**：`bg=#1e293b` (背景色), `rad=8` (圆角半径), `border=(1, #334155)`, `opacity=0.9`；
* **文本排印**：`#hex` (文字颜色), `16px` (字号), `bold` (粗体), `italic` (斜体)。

### 3.3 响应式状态 (`let`) 与动作流 (`->`)
使用 `let` 声明的变量自动加入响应式依赖图谱，页面变动毫秒级热更新：
```scss
let is_dark = true
let username = "Alice"

// 模版插值：直接使用 $var
txt "欢迎回来, $username!" (is_dark ? #f8fafc : #0f172a)

// 单向箭头 '->' 引导用户交互动作
btn (is_dark ? "切换为浅色" : "切换为深色") -> is_dark = !is_dark
```

### 3.4 控制流：条件分支与列表渲染
语言在标记内部原生支持 `if` 和 `for` 语句，直接展开为 UI 节点：
```scss
col gap=8 {
    // 列表循环展开
    for task in tasks {
        row pad=12 bg=#1e293b rad=6 justify=between {
            txt task.title #fff 14px
            btn "完成" -> task.done = true
        }
    }

    // 条件分支渲染
    if len(tasks) == 0 {
        txt "暂无任何待办任务。" #94a3b8 12px
    }
}
```

### 3.5 模版组件与跨文件导入
```scss
// 声明自定义可复用组件
component StatCard(title, value, color) {
    col pad=16 bg=#1e293b rad=8 gap=6 flex=1 {
        txt title #94a3b8 12px
        txt value color 20px bold
    }
}

// 主界面复用组件
row gap=12 {
    StatCard(title="日活用户", value="12,480", color=#22c55e)
    StatCard(title="系统负载", value="18.2%", color=#3b82f6)
}
```

---

## 四、 真实实战应用示例 (Examples)

完整可运行的代码位于 [`examples/`](examples/) 目录下：

1. **[examples/counter.ui](examples/counter.ui)**：15 行代码的极简单向响应式计数器。
2. **[examples/todo_app.ui](examples/todo_app.ui)**：包含动态添加、双向输入绑定 (`bind=text`) 与勾选完成的待办事项应用。
3. **[examples/components_demo.ui](examples/components_demo.ui)**：自定义 `component` 声明与主题色彩复用示例。
4. **[examples/app.ui](examples/app.ui)**：包含 SQLite 本地数据库直连、顶部导航栏与 GPU 水波特效的企业级原生工作台。

---

## 五、 多端运行与体验途径 (Run & Deploy)

### 途径 1：🌐 浏览器在线即开即用（WebAssembly 演练场）
无需安装任何本地环境，直接在浏览器中打开：  
👉 **[https://monikalnbo.github.io/rust_css_like/](https://monikalnbo.github.io/rust_css_like/)**

### 途径 2：💻 下载预编译原生执行程序（Windows / macOS / Linux）
GitHub Actions 会在每次提交时，全自动交叉编译出各平台的原生绿色包：  
👉 前往 **[GitHub Actions 页面](https://github.com/monikalnbo/rust_css_like/actions)**，点击最新的 **Cross-Platform Native Build**，在页面底部的 **Artifacts** 区域即可一键下载：
* `windows-x64-executable.zip`：单文件 Windows 原生 `.exe`。
* `macos-universal-executable.tar.gz`：macOS 通用二进制（同时原生支持 Apple M 系列芯片与 Intel 芯片）。
* `linux-x64-executable.tar.gz`：Linux x86_64 原生独立可执行文件。

### 途径 3：🛠️ 本地编译与开发（Rust 1.75+）
```bash
# 1. 克隆代码仓库
git clone https://github.com/monikalnbo/rust_css_like.git
cd rust_css_like

# 2. 执行全工程 30 项单元与集成测试（100% 通过）
cargo test --workspace

# 3. 启动桌面端原生渲染主外壳
cargo run -p app-shell
```

---

## 六、 高扩展底层注册总线 (Extensibility)

引擎为开发者提供了 7 大底层注册槽位，无需修改引擎核心即可实现企业级深度定制：

| 槽位序号 | 注册 Trait | 解决的扩展需求 | `.ui` 对应语法 |
| :---: | :--- | :--- | :--- |
| **1** | `CustomPainter` | 挂载底层自定义 GPU 着色器（水波光晕、毛玻璃模糊、粒子流光） | `card effect="InteractiveRipple"` |
| **2** | `CustomComponentDriver` | 挂载自研 3D 视口、视频播放器或 EChart 图表为一等公民节点 | `plugin "CustomChart"` |
| **3** | `CustomLayoutStrategy` | 实现除 Flex 之外的非标算法（如瀑布流 Waterfall、环形表盘） | `box display="waterfall"` |
| **4** | `CustomPropertyHandler` | 扩充新样式字段并提供时间轴补间动画计算（Lerp） | `glow-speed=2.5` |
| **5** | `NativeHostFn` | 将系统底层能力（文件读写、剪贴板、系统托盘）注入脚本环境 | `btn -> fs.read("data.json")` |
| **6** | `StorageDriverFactory` | 插件化接入 DuckDB、RocksDB、Redis 或跨进程共享内存 IPC | `db.connect("duckdb://...")` |
| **7** | `AssetProtocolLoader` | 接管资源加载流程，支持私有加密包内存动态解密 | `@import "pak://secure.ui"` |

---

## 七、 底层 15 个 Crate 微内核工程架构

```
crates/
├── css-types/          # 基础纯 POD 数据类型（Color, Rect, Length, Dimension）
├── charset-compat/     # 字符集兼容层（自动剥离 BOM，GBK/UTF-16 互转，CJK 全角度量）
├── crypto-pack/        # 商业安全层（AOT 字节码封装 .binui，符号哈希脱敏，内存解密）
├── dsl-parser/         # 流式语法分析层（大括号状态机，Token 流，无环 AST 构建）
├── element-core/       # 虚拟 DOM 树（ElementTree，节点位掩码交互状态机）
├── style-system/       # 样式计算层（7级特异度打分，动态变量表，计算样式分流）
├── css-animation/      # 数学动效层（三次贝塞尔插值，通用数值/色彩补间状态机）
├── layout-engine/      # 几何排版层（基于纯 Rust Taffy 0.7 算法求解绝对物理坐标）
├── text-layout/        # 高性能文本塑形与换行（集成 cosmic-text 原生字形度量）
├── render-backend/     # 硬件无关渲染指令流（DisplayList，DPI 视网膜缩放，IME 锚点）
├── live-runtime/       # 热重载与脏标记（三级刷新 DirtyMask，避免昂贵全树重排）
├── script-engine/      # 微型脚本引擎（表达式求值，变量作用域链，原生插件扩展槽）
├── data-bridge/        # 全栈数据库与 FFI（SQLite 本地表，Signal 响应式信号，C-ABI 导出）
├── wasm-runtime/       # 浏览器 WebAssembly 运行时（导出 Canvas 2D 绘图后端）
└── app-shell/          # 桌面端主运行外壳（基于 winit 0.29 与 tiny-skia 软光栅）
```

---

## 八、 核心文档索引清单

* 📘 [语言完全参考手册 (`LANGUAGE_GUIDE.md`)](docs/LANGUAGE_GUIDE.md) — 完整的语言词法、语法字典与标准库规约。
* 📚 [系统架构蓝图 (`01_REQUIREMENTS_AND_ARCHITECTURE.md`)](docs/01_REQUIREMENTS_AND_ARCHITECTURE.md) — 深入了解 5 层微内核与数据流水线设计。
* 🌐 [HTML/CSS 对齐规范 (`03_HTML_CSS_PHP_ALIGNMENT_SPEC.md`)](docs/03_HTML_CSS_PHP_ALIGNMENT_SPEC.md) — 虚拟 DOM 状态、文本度量与 `calc()` 动态求解。
* 🔌 [底层扩展开发指南 (`04_EXTENSIBILITY_AND_PLUGIN_SYSTEM.md`)](docs/04_EXTENSIBILITY_AND_PLUGIN_SYSTEM.md) — 七大注册总线与插件开发完全教程。

---

## 📄 开源许可证
本项目基于 [MIT 许可证](LICENSE) 开源。欢迎 Star、提交 Issue 与参与共建！
