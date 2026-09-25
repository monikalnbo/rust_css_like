# 01. Rust CSS-Like 全景需求规格说明书与系统架构蓝图

> **文档编号**：`DOC-01-REQ-ARCH`  
> **文档定位**：定义 `rust_css_like` 项目的业务需求、性能指标、工程规范以及 12 个子模块的完整架构拓扑与数据流转模型。

---

## 目录
1. [项目定位与核心使命](#一-项目定位与核心使命)
2. [全景需求规格说明 (Requirements Specification)](#二-全景需求规格说明-requirements-specification)
   - [2.1 核心功能性需求 (Functional Requirements)](#21-核心功能性需求-functional-requirements)
   - [2.2 性能与非功能性需求 (Non-Functional Requirements)](#22-性能与非功能性需求-non-functional-requirements)
   - [2.3 商业落地与分发需求](#23-商业落地与分发需求)
3. [系统顶层架构拓扑 (System Architecture)](#三-系统顶层架构拓扑-system-architecture)
   - [3.1 架构分层设计](#31-架构分层设计)
   - [3.2 12 个核心子模块职责矩阵](#32-12-个核心子模块职责矩阵)
4. [核心数据流转模型 (End-to-End Pipeline)](#四-核心数据流转模型-end-to-end-pipeline)
   - [4.1 源码加载与规范化阶段](#41-源码加载与规范化阶段)
   - [4.2 语法分析与 AST 构建阶段](#42-语法分析与-ast-构建阶段)
   - [4.3 样式级联与优先级求解阶段](#43-样式级联与优先级求解阶段)
   - [4.4 布局计算与物理坐标求解阶段](#44-布局计算与物理坐标求解阶段)
   - [4.5 绘制指令流生成与渲染管线](#45-绘制指令流生成与渲染管线)
   - [4.6 交互事件驱动与局部脏刷新循环](#46-交互事件驱动与局部脏刷新循环)
5. [跨语言 FFI 与数据互联架构](#五-跨语言-ffi-与数据互联架构)

---

## 一、 项目定位与核心使命

现代桌面客户端开发长期饱受困扰：
* **Electron 系技术栈**：虽然开发门槛低，但打包动辄数百兆，空载内存占用超过 200MB，CPU 资源开销极大，且源码易被解包反编译；
* **传统原生 GUI（Qt / MFC / Win32）**：虽然性能卓越，但缺乏现代响应式标记语言与声明式样式体系，跨平台编译环境复杂，排版调试成本极高。

`rust_css_like` 旨在成为**面向 Windows 与 macOS 商业级原生软件的轻量、极致性能、全栈前端引擎**。它以纯 Rust 构建，从根源上彻底解决内存占用与样板代码臃肿问题。

---

## 二、 全景需求规格说明 (Requirements Specification)

### 2.1 核心功能性需求 (Functional Requirements)

1. **超精炼标记语言 (Minimalist Unified DSL)**：
   - **零文件头样板**：抛弃 HTML 的 `<!DOCTYPE>`, `<html>`, `<head>`, `<meta>` 标签体系；
   - **统一约束符**：纯粹以 `{}` 大括号界定作用域与层级；
   - **三位一体**：统一标记结构（`win`, `row`, `col`, `box`）、视觉样式（`pad=16`, `bg=#fff`）、响应式状态（`let count = 0`）与交互流（`btn -> count += 1`）。
2. **增量编译与即时热重载 (Live Runtime)**：
   - **作用域闭合监听**：监听字符流中的 `{ ... }` 闭合状态机，单次作用域重编译耗时 `< 0.2ms`；
   - **三级脏标记更新**：
     - `REPAINT`（纯视觉变化，如背景色、阴影）：直接提交 GPU 重绘（`< 1ms`）；
     - `RELAYOUT`（几何尺寸变动，如宽度、Padding、Flex）：仅标记局部子树触发 Taffy 重排（`< 3ms`）；
     - `RESTRUCTURE`（节点增删）：全量虚拟树重建。
3. **底层渲染与高分屏 (Render Backend)**：
   - 产生扁平、无状态的 `DisplayList` 绘制指令队列（矩形、盒阴影、文本、裁剪视口、自定义效果）；
   - 支持逻辑像素（DIP）与物理设备像素（Device Pixel）的动态 DPI 视网膜缩放映射（1.0x, 1.25x, 1.5x, 2.0x）；
   - 支持多层级 Z-Index 的树形点击命中测试（Hit Testing）；
   - 计算原生输入法（IME）拼音浮动候选窗的物理光标绝对锚点，防止输入法漂移。
4. **全字符集与国际化兼容 (Charset Compat)**：
   - 文件头 BOM 自动识别与剥离（UTF-8 BOM `EF BB BF`、UTF-16LE/BE）；
   - 中文 Windows 经典 GBK / GB18030 字符自动探测与容错转码，杜绝乱码；
   - Win32 原生宽字符（`wchar_t` / `[u16]`）与 Rust UTF-8 零成本双向互转；
   - CJK（中日韩统一表意文字及全角标点）等宽与非等宽排版列宽精确度量。
5. **商业安全与防逆向体系 (Crypto Pack)**：
   - 源码编译为紧凑二进制字节码（`.binui`，带 `RCSS` 校验魔数）；
   - 编译期符号脱敏：将明文变量、类名、方法名映射为 64 位 FNV-1a 不可逆混淆哈希（如 `_0x8f2a...`）；
   - 动态流加密：仅在 RAM 内存堆中解密执行，硬盘零明文临时文件，杜绝内存抓包与静态反编译。
6. **全栈直连与跨语言胶水 (Data Bridge & Polyglot FFI)**：
   - 前端直连单文件嵌入式 SQLite 与远程数据库；
   - 导出标准 C-ABI 头文件与动态库，原生打通 Python、Node.js、C++、Go 与 C#。

---

### 2.2 性能与非功能性需求 (Non-Functional Requirements)

1. **极致轻量**：
   - 静态编译二进制体积 `< 15MB`（压缩后绿色版 `< 8MB`）；
   - 空载冷启动内存消耗 `< 25MB`，彻底终结 Electron 的 200MB+ 内存噩梦。
2. **高刷满帧流畅度**：
   - 全程稳定支持 60 FPS / 120 FPS 高刷显示屏；
   - 动效单帧计算耗时 `< 8ms`，贝塞尔曲线状态过渡平滑无抖动。
3. **高并发与内存安全**：
   - 100% 遵循 Rust 内存安全原则，核心层零未定义行为（Zero UB）；
   - 多线程数据访问采用无锁或轻量级细粒度读写锁。

---

### 2.3 商业落地与分发需求

1. **零本地环境依赖**：开发者无需配置本地复杂的 C++ 编译链，文本编辑器编写代码即可推送编译；
2. **跨平台全自动 CI/CD 出包**：
   - GitHub Actions 云端并行产出 Windows `.exe` 绿色压缩包与 macOS Universal Binary `.dmg`；
   - 自动部署 WebAssembly 演练场至 GitHub Pages，支持浏览器免安装在线试用。

---

## 三、 系统顶层架构拓扑 (System Architecture)

### 3.1 架构分层设计

```
┌────────────────────────────────────────────────────────────────────────┐
│                          业务声明层 (Markup & Logic)                    │
│                 examples/app.ui  /  *.binui (加密字节码)                 │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
┌───────────────────────────────────▼────────────────────────────────────┐
│                    语法分析与运行时调度层 (Core Pipeline)                │
│   ┌─────────────────────┐   ┌───────────────────┐   ┌──────────────┐   │
│   │     dsl-parser      │──►│   element-core    │──►│ style-system │   │
│   │ 括号状态机/AST/Token │   │  虚拟节点树/State  │   │  7级优先级/计算  │   │
│   └─────────────────────┘   └───────────────────┘   └──────────────┘   │
│              ▲                        ▲                     │          │
│              │                        │                     │          │
│   ┌──────────┴──────────┐   ┌─────────┴─────────┐           ▼          │
│   │   charset-compat    │   │   script-engine   │   ┌──────────────┐   │
│   │  BOM/GBK/宽字符/CJK  │   │   求值器/插件槽位   │   │css-animation │   │
│   └─────────────────────┘   └───────────────────┘   │ 贝塞尔/Lerp   │   │
│              ▲                        ▲             └──────────────┘   │
│              │                        │                     │          │
│   ┌──────────┴──────────┐   ┌─────────┴─────────┐           │          │
│   │     crypto-pack     │   │    data-bridge    │           │          │
│   │   混淆/流加密/字节码 │   │  SQLite/FFI/Signal │           │          │
│   └─────────────────────┘   └───────────────────┘           │          │
└───────────────────────────────────────┬─────────────────────┼──────────┘
                                        │                     │
┌───────────────────────────────────────▼─────────────────────▼──────────┐
│                     排版求解与渲染呈现层 (Render Engine)                │
│   ┌────────────────────────────────────────┐   ┌───────────────────┐   │
│   │             layout-engine              │   │   live-runtime    │   │
│   │     Taffy 桥接 / LayoutRect 物理坐标    │   │  DirtyMask/增量   │   │
│   └───────────────────┬────────────────────┘   └─────────┬─────────┘   │
│                       │                                  │             │
│                       ▼                                  │             │
│   ┌──────────────────────────────────────────────────────┴─────────┐   │
│   │                         render-backend                         │   │
│   │     DrawCommand 图元 / DisplayList 队列 / DPI 缩放 / IME 锚点    │   │
│   └──────────────────────────────────┬─────────────────────────────┘   │
└──────────────────────────────────────┼─────────────────────────────────┘
                                       │
┌──────────────────────────────────────▼─────────────────────────────────┐
│                      操作系统物理层 (Hardware & OS)                     │
│           Windows (Win32/D3D12)   │   macOS (Metal/Cocoa)   │   WASM   │
└────────────────────────────────────────────────────────────────────────┘
```

---

### 3.2 12 个核心子模块职责矩阵

| 序号 | Crate 模块 | 职责与能力边界 | 核心产出数据结构 |
| :---: | :--- | :--- | :--- |
| 1 | `css-types` | 底层纯数据 POD 类型定义，零依赖，高紧凑内存布局 | `Color`, `Rect`, `Point`, `Size`, `Dimension`, `Display` |
| 2 | `dsl-parser` | 流式词法与语法分析，`{}` 约束符平衡状态机，高级控制流解析 | `Token`, `ScopeBlock`, `ScopeKind`, `BracketEvent`, `AST` |
| 3 | `element-core` | 对标 HTML 原语的轻量虚拟 DOM 树，位掩码交互状态机 | `ElementTag`, `ElementStateMask`, `ElementNode`, `ElementTree` |
| 4 | `style-system` | 紧凑型最终计算样式，7 级确定性特异度覆盖与主题变量表 | `ComputedStyle`, `PriorityLevel`, `StyledProperty`, `VariableTable` |
| 5 | `css-animation`| 纯数学缓动函数、三次贝塞尔拟合算法与帧进插值状态机 | `Easing`, `Lerp`, `Transition` |
| 6 | `layout-engine`| 将自研计算样式转译为 Taffy 0.7 样式，解算绝对物理几何坐标 | `LayoutBridge`, `LayoutRect` |
| 7 | `live-runtime` | 作用域闭合驱动的增量热重载生命周期中枢，分级脏标记 | `DirtyMask`, `LiveEvent` |
| 8 | `data-bridge`  | 本地嵌入式数据表抽象、响应式信号单元与跨语言标准 C-ABI | `DatabaseClient`, `DbRow`, `Signal`, `FFI C-ABI` |
| 9 | `crypto-pack`  | AOT 二进制字节码包装、FNV-1a 符号哈希脱敏与动态流加密 | `BinaryPackage`, `Obfuscator`, `StreamCrypto` |
| 10 | `charset-compat`| 全字符集编码识别、BOM 剥离、GBK 容错转码与 CJK 等宽度量 | `CharsetDetector`, `Transcoder`, `Win32String`, `UnicodeMetrics` |
| 11 | `render-backend`| 矢量图元指令队列、DPI 高分屏映射、Z-Index 命中测试与 IME 锚点 | `DrawCommand`, `DisplayList`, `DpiScale`, `HitTester`, `ImeCursorAnchor` |
| 12 | `script-engine` | 无 JS 引擎的微型求值器、模版变量插值、点属性查找与原生插件槽 | `ScriptValue`, `ScriptScope`, `Evaluator`, `ExtensionRegistry` |

---

## 四、 核心数据流转模型 (End-to-End Pipeline)

整个引擎的端到端执行流程分为以下六个阶段：

### 4.1 源码加载与规范化阶段
1. 接收输入流（明文 `.ui` 文本，或 `.binui` 加密字节流）；
2. 若为 `.binui`，`crypto-pack` 验证 `RCSS` 魔数并在内存堆中通过 `StreamCrypto` 解密；
3. `charset-compat` 介入，识别并剥除 BOM，处理 GBK/UTF-16 编码纠偏，输出纯净的标准 UTF-8 字符流。

### 4.2 语法分析与 AST 构建阶段
1. `dsl-parser::Lexer` 将字符流切分为包含精确定位 `Span` 的 Token 流；
2. `BracketTracker` 实时统计 `{` 与 `}` 的开闭配对深度；
3. `Parser` 生成多叉 `ScopeBlock` 抽象语法树（包含 `@import`, `let`, `component`, `for`, `if`, 元素原语及 `->` 动作代码）。

### 4.3 样式级联与优先级求解阶段
1. `style-system` 依据元素标签与局部继承关系初始化默认样式；
2. 按照 **7 级确定性特异度**（`Inherited` < `LayerBase` < `Mixin` < `Local` < `HoverFocus` < `Active` < `Important`）覆盖属性；
3. 变量符号表解析 `$var` 与表达式计算，产出紧凑的 `ComputedStyle`。

### 4.4 布局计算与物理坐标求解阶段
1. `layout-engine::LayoutBridge` 将节点的盒模型属性（Display, FlexDirection, Size, Padding 等）映射为 `taffy::style::Style`；
2. 递归注入 Taffy 布局树，执行 `taffy.compute_layout()` 算法；
3. 将解算完成的物理绝对像素尺寸与偏移量回填到各节点的 `LayoutRect` 中。

### 4.5 绘制指令流生成与渲染管线
1. 结合 `LayoutRect`、`ComputedStyle` 与当前节点的 `ElementStateMask`；
2. 构造 `DrawCommand::DrawRect`、`DrawShadow`、`DrawText` 或 `CustomEffect`；
3. 按 Z-Index 与树形深度将图元推入 `DisplayList`；
4. 乘以 `DpiScale` 缩放因子，提交给底层硬件或软件光栅化管线在屏幕上画出像素。

### 4.6 交互事件驱动与局部脏刷新循环
1. 用户在物理窗口触发鼠标点击 `(px, py)`；
2. `DpiScale::point_to_logical` 将物理坐标转换为逻辑坐标；
3. `HitTester` 倒序遍历候选节点，命中目标节点并触发绑定的 `->` 脚本；
4. 脚本修改响应式状态（`let count += 1`），触发 `Signal` 变更；
5. `live-runtime` 依据变动类型产生 `DirtyMask`，精准触发局部重排（`<3ms`）或局部重绘（`<1ms`）。

---

## 五、 跨语言 FFI 与数据互联架构

引擎对外输出纯 C 标准 ABI，各主流语言可无缝嵌入驱动：

```
                    ┌────────────────────────────┐
                    │    rust_css_like 核心库    │
                    │   (librust_css_engine.dll) │
                    └──────────────┬─────────────┘
                                   │
      ┌────────────────────────────┼────────────────────────────┐
      │                            │                            │
┌─────▼──────┐              ┌──────▼─────┐               ┌──────▼─────┐
│   Python   │              │   Node.js  │               │   C / C++  │
│  (PyO3 /   │              │  (napi-rs) │               │ (cbindgen  │
│  ctypes)   │              │            │               │  标准头)   │
└────────────┘              └────────────┘               └────────────┘
```

1. **Python (`PyO3`)**：适合快速拉起 AI 交互面板，直连本地大模型推理结果；
2. **Node.js (`napi-rs`)**：前端工程师编写 TypeScript 业务逻辑，由 Rust 负责原生高刷渲染，**内存占用仅为 Electron 的十分之一**；
3. **C / C++ (`cbindgen`)**：输出标准 `.h` 头文件，可无缝嵌入 Qt 既有工程、虚幻引擎或自研游戏引擎中作为高性能 UI 视口。
