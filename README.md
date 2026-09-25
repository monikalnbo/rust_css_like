# Rust CSS-Like 现代化原生标记与样式系统架构白皮书

> **定位**：面向 Windows 与 macOS 商业级原生软件的轻量、极致性能、全栈前端引擎。  
> **核心革新**：  
> 1. **零冗长头特征**：彻底抛弃 HTML 臃肿的 `<!DOCTYPE>`、`<html>`、`<head>`、`<meta>` 样板代码，纯粹以 `{}` 为约束符；  
> 2. **超短特性与强逻辑**：统一标记结构、视觉样式与响应式逻辑（`win`, `row`, `col`, `txt`, `btn`, `if`, `for`, `->` 动作流）；  
> 3. **真正低耦合微内核**：12 个子模块严格遵循单向依赖拓扑，中间表达（AST / DisplayList）完全解耦，可独立测试、替换与扩展；  
> 4. **底层渲染与高分屏基建**：GPU 绘制指令队列 (DisplayList)、DPI 视网膜缩放映射、树形命中测试与原生 IME 输入法底座；  
> 5. **全字符集与国际化兼容**：内建 BOM 自动剥离、Windows 宽字符互操作、GBK/GB18030 容错转码与 CJK 全角/半角排版计算；  
> 6. **全栈直连数据**：前端直连嵌入式 SQLite 与远程数据库，原生打通 Python / Node.js / C++ / Go / C#；  
> 7. **商业级 AOT 混淆加密**：源码离线编译为紧凑二进制字节码 (`.binui`)，符号哈希脱敏与内存流解密，杜绝逆向盗版；  
> 8. **高拓展底层注册总线**：开放 GPU 着色器、自定义虚拟节点、非标几何排版、原生系统函数等七大底层注册槽位。

---

## 目录
1. [项目设计哲学与低耦合原则](#一-项目设计哲学与低耦合原则)
2. [五层低耦合微内核架构图谱](#二-五层低耦合微内核架构图谱)
3. [12 大子模块与全文件细分全景拆解](#三-12-大子模块与全文件细分全景拆解)
   - [3.1 `css-types`：基础纯数据类型层](#31-css-types基础纯数据类型层)
   - [3.2 `charset-compat`：字符集与跨平台编码兼容层](#32-charset-compat字符集与跨平台编码兼容层)
   - [3.3 `crypto-pack`：商业安全与 AOT 字节码层](#33-crypto-pack商业安全与-aot-字节码层)
   - [3.4 `dsl-parser`：流式词法与语法分析层](#34-dsl-parser流式词法与语法分析层)
   - [3.5 `element-core`：虚拟节点树与状态机层](#35-element-core虚拟节点树与状态机层)
   - [3.6 `style-system`：样式计算与特异度层叠层](#36-style-system样式计算与特异度层叠层)
   - [3.7 `css-animation`：数学动效与贝塞尔插值层](#37-css-animation数学动效与贝塞尔插值层)
   - [3.8 `layout-engine`：几何排版与物理尺寸求解层](#38-layout-engine几何排版与物理尺寸求解层)
   - [3.9 `render-backend`：绘制指令流与渲染基建层](#39-render-backend绘制指令流与渲染基建层)
   - [3.10 `live-runtime`：增量闭合监听与脏标记中枢](#310-live-runtime增量闭合监听与脏标记中枢)
   - [3.11 `script-engine`：微型脚本求值与插件槽位层](#311-script-engine微型脚本求值与插件槽位层)
   - [3.12 `data-bridge`：全栈数据库与跨语言 FFI 胶水层](#312-data-bridge全栈数据库与跨语言-ffi-胶水层)
4. [高拓展性设计：七大底层注册槽位 (RegistryBus)](#四-高拓展性设计七大底层注册槽位-registrybus)
5. [声明式 DSL 语法实战展示 (examples/app.ui)](#五-声明式-dsl-语法实战展示-examplesappui)
6. [四大核心技术规范白皮书索引 (docs/)](#六-四大核心技术规范白皮书索引-docs)
7. [构建、测试与分发](#七-构建测试与分发)

---

## 一、 项目设计哲学与低耦合原则

传统 GUI 引擎经常陷入**“巨石架构（Monolithic Spaghetti）”**：排版层直接调用 GPU 绘制接口、DOM 节点紧密耦合样式计算细节、脚本运行时强绑定系统窗口事件。一旦需要更换排版算法或移植到新平台，往往牵一发而动全身。

`rust_css_like` 严格执行**五项低耦合架构原则（Low Coupling Principles）**：

1. **单向无环依赖 (Strict DAG)**：所有 Crate 之间的引用严格自底向上，杜绝任何循环依赖（Circular Dependencies）；
2. **纯数据中介抽象 (Intermediate Representation, IR)**：
   - 语法层与 DOM 层通过 **AST (`ScopeBlock`)** 隔离；
   - 样式层与排版层通过 **`ComputedStyle` POD 结构** 隔离；
   - 排版层与渲染层通过 **`LayoutRect` 物理矩形** 隔离；
   - 渲染层与操作系统物理显存通过 **`DisplayList` 指令流** 隔离；
3. **Trait 接口多态化**：数据库、渲染画笔、插件驱动全部以 Trait 形式定义（如 `DatabaseClient`、`CustomPainter`、`CustomComponentDriver`），核心内核只面向契约编程；
4. **叶子节点零外设**：基础类型库（`css-types`、`charset-compat`、`crypto-pack`）为纯逻辑纯数据库，零外部重量级系统依赖；
5. **微内核总线式扩展**：任何非核心功能（如自研着色器、非标瀑布流排版、DuckDB 数据源）均通过外部插件总线动态注入，不侵入核心库代码。

---

## 二、 五层低耦合微内核架构图谱

```
┌────────────────────────────────────────────────────────────────────────┐
│                   第 5 层：应用装配与跨语言层 (Application & FFI)       │
│               examples/app.ui   │   C-ABI (Python / Node.js / C++)     │
└───────────────────────────────────┬────────────────────────────────────┘
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                   第 4 层：逻辑、数据与运行时中枢 (Runtime & Data)       │
│        ┌─────────────────────────┐       ┌────────────────────────┐    │
│        │      live-runtime       │       │      data-bridge       │    │
│        │   增量热重载 / 脏标记   │       │  SQLite / Signal 响应式│    │
│        └────────────┬────────────┘       └───────────┬────────────┘    │
│                     │                                │                 │
│        ┌────────────┴────────────┐                   │                 │
│        │      script-engine      │◄──────────────────┘                 │
│        │   行内求值 / 原生插件槽 │                                     │
│        └────────────┬────────────┘                                     │
└─────────────────────┼──────────────────────────────────────────────────┘
                      ▼
┌────────────────────────────────────────────────────────────────────────┐
│                   第 3 层：几何排版与渲染指令层 (Layout & Pipeline)     │
│        ┌─────────────────────────┐       ┌────────────────────────┐    │
│        │      layout-engine      │       │     css-animation      │    │
│        │   Taffy 桥接 / 几何求解 │       │   贝塞尔曲线 / Lerp    │    │
│        └────────────┬────────────┘       └───────────┬────────────┘    │
│                     │                                │                 │
│                     ▼                                ▼                 │
│        ┌──────────────────────────────────────────────────────────┐    │
│        │                      render-backend                      │    │
│        │     DisplayList 指令流 / DPI 视网膜缩放 / IME 物理锚点   │    │
│        └────────────────────────┬─────────────────────────────────┘    │
└─────────────────────────────────┼──────────────────────────────────────┘
                                  ▼
┌────────────────────────────────────────────────────────────────────────┐
│                   第 2 层：语义、语法与样式模型层 (Semantics & Model)    │
│        ┌─────────────────────────┐       ┌────────────────────────┐    │
│        │       dsl-parser        │       │      style-system      │    │
│        │  括号状态机 / 词法语法  │       │  7级特异度 / 计算样式  │    │
│        └────────────┬────────────┘       └───────────┬────────────┘    │
│                     │                                │                 │
│                     ▼                                │                 │
│        ┌─────────────────────────────────────────────┴────────────┐    │
│        │                       element-core                       │    │
│        │              虚拟 DOM 树 / 交互状态位掩码                │    │
│        └────────────────────────┬─────────────────────────────────┘    │
└─────────────────────────────────┼──────────────────────────────────────┘
                                  ▼
┌────────────────────────────────────────────────────────────────────────┐
│                   第 1 层：基础契约与纯数据叶子层 (Foundation Types)     │
│     ┌─────────────────┐   ┌──────────────────┐   ┌────────────────┐    │
│     │    css-types    │   │  charset-compat  │   │  crypto-pack   │    │
│     │  紧凑几何/颜色  │   │  BOM/GBK/宽字符  │   │ 混淆/AOT字节码 │    │
│     └─────────────────┘   └──────────────────┘   └────────────────┘    │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 三、 12 大子模块与全文件细分全景拆解

系统所有源码遵循**小文件规范**（单文件控制在 30 ~ 200 行以内，结构严整）：

```
crates/
├── css-types/          # 1. 基础纯数据类型
├── charset-compat/     # 2. 字符集与跨平台编码
├── crypto-pack/        # 3. 商业安全与AOT字节码
├── dsl-parser/         # 4. 流式语法分析与括号状态机
├── element-core/       # 5. 虚拟节点树与状态机
├── style-system/       # 6. 样式计算与特异度层叠
├── css-animation/      # 7. 数学动效与贝塞尔插值
├── layout-engine/      # 8. 几何排版与物理尺寸求解
├── render-backend/     # 9. 绘制指令流与渲染基建
├── live-runtime/       # 10. 增量闭合监听与脏标记中枢
├── script-engine/      # 11. 微型脚本求值与插件槽位
└── data-bridge/        # 12. 全栈数据库与跨语言FFI
```

---

### 3.1 `css-types`：基础纯数据类型层
* **路径**：[crates/css-types](file:///d:/for_clone/rust_css_like/crates/css-types)
* **定位**：零依赖的纯 POD 数据结构基石，为全局其他 11 个模块提供紧凑内存表达。
* **文件细分剖析**：
  * [src/color.rs](file:///d:/for_clone/rust_css_like/crates/css-types/src/color.rs)：定义 4 字节紧凑型 `Color { r, g, b, a }`，支持十六进制色彩解析（`#fff`, `#4f46e5`, `#ffffff80`）与 `lerp` 色彩空间线性插值。
  * [src/geometry.rs](file:///d:/for_clone/rust_css_like/crates/css-types/src/geometry.rs)：定义泛型点 `Point<T>`、尺寸 `Size<T>`、盒模型矩形 `Rect<T>`（支持 `symmetric`, `trbl`）与独立 4 轴圆角 `BorderRadius`。
  * [src/layout_props.rs](file:///d:/for_clone/rust_css_like/crates/css-types/src/layout_props.rs)：定义 1 字节紧凑枚举：`Display` (Flex/Grid/None)、`Position` (Relative/Absolute)、`FlexDirection`、`JustifyContent`、`AlignItems` 与 `Overflow`。
  * [src/text_props.rs](file:///d:/for_clone/rust_css_like/crates/css-types/src/text_props.rs)：定义排版枚举：`FontWeight` (100~900)、`TextAlign` (Left/Center/Right/Justify) 与 `TextOverflow` (Clip/Ellipsis)。
  * [src/units.rs](file:///d:/for_clone/rust_css_like/crates/css-types/src/units.rs)：定义绝对与相对度量 `Length`（Px/Em/Rem）与支持百分比和自适应的统一维度 `Dimension`（Auto/Px/Percent），提供 `resolve_or()` 计算器。

---

### 3.2 `charset-compat`：字符集与跨平台编码兼容层
* **路径**：[crates/charset-compat](file:///d:/for_clone/rust_css_like/crates/charset-compat)
* **定位**：解决 Windows 平台历史遗留编码顽疾（锟斤拷、烫烫烫乱码、BOM 污染），提供统一规范化 UTF-8 字符流与 CJK 排版度量。
* **文件细分剖析**：
  * [src/detector.rs](file:///d:/for_clone/rust_css_like/crates/charset-compat/src/detector.rs)：定义 `EncodingKind` 枚举，实现 `detect_bom`（识别 UTF-8 BOM `EF BB BF`、UTF-16LE `FF FE`、UTF-16BE `FE FF`）与 `is_likely_gbk` 启发式双字节特征探测。
  * [src/transcoder.rs](file:///d:/for_clone/rust_css_like/crates/charset-compat/src/transcoder.rs)：跨字符集无损转码器 `Transcoder`，提供 `to_utf8()` 入口，自动剥离 BOM 并将 UTF-16/GBK 容错转码为纯净 Rust UTF-8 String。
  * [src/win32.rs](file:///d:/for_clone/rust_css_like/crates/charset-compat/src/win32.rs)：Win32 宽字符通道 `Win32String`，实现 Rust UTF-8 ↔ Windows `wchar_t` / `[u16]` 原生互转。
  * [src/grapheme.rs](file:///d:/for_clone/rust_css_like/crates/charset-compat/src/grapheme.rs)：Unicode 字符度量器 `UnicodeMetrics`，精确识别中日韩统一表意文字及全角标点区间，赋予全角 2 宽、ASCII 半角 1 宽，防止排版光标错位。

---

### 3.3 `crypto-pack`：商业安全与 AOT 字节码层
* **路径**：[crates/crypto-pack](file:///d:/for_clone/rust_css_like/crates/crypto-pack)
* **定位**：企业级三重防逆向保护体系，抹除明文字符串，防止商业软件被反编译破解。
* **文件细分剖析**：
  * [src/bytecode.rs](file:///d:/for_clone/rust_css_like/crates/crypto-pack/src/bytecode.rs)：AOT 二进制字节码包装器 `BinaryPackage`，定义 `BYTECODE_MAGIC = [0x52, 0x43, 0x53, 0x53]`（"RCSS"），提供跨平台序列化与反序列化。
  * [src/obfuscate.rs](file:///d:/for_clone/rust_css_like/crates/crypto-pack/src/obfuscate.rs)：符号哈希脱敏器 `Obfuscator`，基于快速 64 位非加密哈希算法（FNV-1a），将明文标识符映射为不可逆的混淆符号（如 `_0x8f2a4c1b`）。
  * [src/crypto.rs](file:///d:/for_clone/rust_css_like/crates/crypto-pack/src/crypto.rs)：内存流加解密引擎 `StreamCrypto`，提供基于动态伪随机密钥流的双向变换，运行时直接在 RAM 内存堆中解密，硬盘零临时明文文件。

---

### 3.4 `dsl-parser`：流式词法与语法分析层
* **路径**：[crates/dsl-parser](file:///d:/for_clone/rust_css_like/crates/dsl-parser)
* **定位**：负责将文本代码快速切词、解析为无环抽象语法树，并提供大括号闭合状态机。
* **文件细分剖析**：
  * [src/span.rs](file:///d:/for_clone/rust_css_like/crates/dsl-parser/src/span.rs)：源码精确定位结构 `Span { start, end, line, column }`，支持区间合并 `merge`，用于编译期诊断。
  * [src/token.rs](file:///d:/for_clone/rust_css_like/crates/dsl-parser/src/token.rs)：定义全部 Token 枚举 `TokenKind`（关键字 `@import`, `let`, `component`, `for`, `if`；字面量、操作符、界定符及动作流箭头 `->`）。
  * [src/bracket.rs](file:///d:/for_clone/rust_css_like/crates/dsl-parser/src/bracket.rs)：大括号闭合状态机 `BracketTracker`，实时追踪嵌套深度 `depth`，一旦触发完整闭合产生 `BracketEvent::ScopeClosed`，作为实时热预览的核心触发点。
  * [src/lexer.rs](file:///d:/for_clone/rust_css_like/crates/dsl-parser/src/lexer.rs)：流式字符分词器 `Lexer`，自动过滤单行注释 `//`，正确提取变量 `$var`、十六进制颜色 `#hex`、数字与字符串。
  * [src/ast.rs](file:///d:/for_clone/rust_css_like/crates/dsl-parser/src/ast.rs)：抽象语法树定义，包括 `ScopeKind`（Window, Element, Span, Import, ComponentDef, ForLoop, IfBranch, StateLet, Pseudo）、属性声明 `PropertyDecl` 与多叉作用域块 `ScopeBlock`。
  * [src/parser.rs](file:///d:/for_clone/rust_css_like/crates/dsl-parser/src/parser.rs)：增量容错解析器 `Parser`，解析顶级控制流与 `{ ... }` 内部属性键值对及 `->` 动作流代码。

---

### 3.5 `element-core`：虚拟节点树与状态机层
* **路径**：[crates/element-core](file:///d:/for_clone/rust_css_like/crates/element-core)
* **定位**：对标 HTML 基础原语的虚拟 DOM 节点树与交互状态标志位。
* **文件细分剖析**：
  * [src/tag.rs](file:///d:/for_clone/rust_css_like/crates/element-core/src/tag.rs)：定义标签枚举 `ElementTag`（包含 Window, Box, Row, Col, Text, Span, Button, Input, Checkbox, Select, Option, Slider, Image, ScrollView, Custom）。
  * [src/state.rs](file:///d:/for_clone/rust_css_like/crates/element-core/src/state.rs)：基于 `bitflags` 的交互状态掩码 `ElementStateMask`（NORMAL, HOVERED, PRESSED, FOCUSED, DISABLED，可复合叠加）。
  * [src/node.rs](file:///d:/for_clone/rust_css_like/crates/element-core/src/node.rs)：轻量级虚拟节点 `ElementNode`（包含节点全局唯一 ID `NodeId`、标签、父节点指针、子节点列表、行内样式表与文本内容）。
  * [src/tree.rs](file:///d:/for_clone/rust_css_like/crates/element-core/src/tree.rs)：扁平化高性能哈希存储的虚拟节点树 `ElementTree`，提供 `create_node`, `append_child`, `get_node_mut` 与遍历接口。

---

### 3.6 `style-system`：样式计算与特异度层叠层
* **路径**：[crates/style-system](file:///d:/for_clone/rust_css_like/crates/style-system)
* **定位**：对标 CSS 的层叠计算系统，解决多规则冲突打分并输出紧凑计算样式。
* **文件细分剖析**：
  * [src/priority.rs](file:///d:/for_clone/rust_css_like/crates/style-system/src/priority.rs)：7 级确定性特异度枚举 `PriorityLevel`（`Inherited` < `LayerBase` < `Mixin` < `Local` < `HoverFocus` < `Active` < `Important`），以及带优先级的样式包装器 `StyledProperty<T>`。
  * [src/computed.rs](file:///d:/for_clone/rust_css_like/crates/style-system/src/computed.rs)：紧凑型计算样式 `ComputedStyle`（分流为盒模型布局属性 `size, margin, padding, flex` 给 Taffy 消费，视觉属性 `background, border, shadow, opacity` 给 GPU 消费）。
  * [src/variables.rs](file:///d:/for_clone/rust_css_like/crates/style-system/src/variables.rs)：动态主题变量符号表 `VariableTable`，支持 `$theme.primary` 的符号解析与运行时热替换。

---

### 3.7 `css-animation`：数学动效与贝塞尔插值层
* **路径**：[crates/css-animation](file:///d:/for_clone/rust_css_like/crates/css-animation)
* **定位**：全帧率流畅动效数学库，负责时间演进过程中的数值与色彩补间。
* **文件细分剖析**：
  * [src/easing.rs](file:///d:/for_clone/rust_css_like/crates/css-animation/src/easing.rs)：缓动曲线枚举 `Easing`（Linear, EaseIn, EaseOut, EaseInOut），以及三次贝塞尔多项式逼近算法 `CubicBezier(p1x, p1y, p2x, p2y)`。
  * [src/lerp.rs](file:///d:/for_clone/rust_css_like/crates/css-animation/src/lerp.rs)：通用线性插值 Trait `Lerp`，为 `f32` 浮点数与 `Color` 颜色提供插值能力。
  * [src/transition.rs](file:///d:/for_clone/rust_css_like/crates/css-animation/src/transition.rs)：按帧推进的状态机 `Transition<T: Lerp>`，接收 `dt_secs` 时间差步进，输出当前帧过渡值，并在进度达到 1.0 时标记完成。

---

### 3.8 `layout-engine`：几何排版与物理尺寸求解层
* **路径**：[crates/layout-engine](file:///d:/for_clone/rust_css_like/crates/layout-engine)
* **定位**：基于纯 Rust Taffy 0.7 布局算法，解算最终物理绝对几何坐标。
* **文件细分剖析**：
  * [src/bridge.rs](file:///d:/for_clone/rust_css_like/crates/layout-engine/src/bridge.rs)：转译桥梁 `LayoutBridge`，将自研 `ComputedStyle`（Display, FlexDirection, JustifyContent, AlignItems, Dimension）转译为 `taffy::style::Style`。
  * [src/solver.rs](file:///d:/for_clone/rust_css_like/crates/layout-engine/src/solver.rs)：几何解算输出结构 `LayoutRect { x, y, width, height }`，提供点命中包含判定 `contains(px, py)`。

---

### 3.9 `render-backend`：绘制指令流与渲染基建层
* **路径**：[crates/render-backend](file:///d:/for_clone/rust_css_like/crates/render-backend)
* **定位**：图形渲染指令抽象、高分屏视网膜映射、命中测试与输入法定位底座。
* **文件细分剖析**：
  * [src/command.rs](file:///d:/for_clone/rust_css_like/crates/render-backend/src/command.rs)：硬件无关的底层图元指令 `DrawCommand`（DrawRect 支持 4 轴圆角与边框、DrawShadow 盒阴影、DrawText 文本、PushClip/PopClip 视口裁剪、CustomEffect 自定义效果）。
  * [src/display_list.rs](file:///d:/for_clone/rust_css_like/crates/render-backend/src/display_list.rs)：单帧绘制指令队列 `DisplayList`，负责收集图元并在帧结束时整体派发给光栅化管线。
  * [src/dpi.rs](file:///d:/for_clone/rust_css_like/crates/render-backend/src/dpi.rs)：DPI 缩放映射器 `DpiScale`，实现逻辑像素（DIP）与物理设备像素（Device Pixel）双向转换。
  * [src/hit_test.rs](file:///d:/for_clone/rust_css_like/crates/render-backend/src/hit_test.rs)：全局树形命中测试器 `HitTester`，按照 Z-Index 降序与兄弟节点逆序寻找最上层响应点击的节点。
  * [src/ime.rs](file:///d:/for_clone/rust_css_like/crates/render-backend/src/ime.rs)：输入法物理光标定位锚点 `ImeCursorAnchor`，计算拼音候选词弹窗的精确物理坐标。
  * [src/effect.rs](file:///d:/for_clone/rust_css_like/crates/render-backend/src/effect.rs)：底层交互式特效协议 `CustomEffect` 与注册中心 `EffectRegistry`，内置物理光晕水波 `InteractiveRippleEffect`。

---

### 3.10 `live-runtime`：增量闭合监听与脏标记中枢
* **路径**：[crates/live-runtime](file:///d:/for_clone/rust_css_like/crates/live-runtime)
* **定位**：增量热重载事件驱动中枢，通过脏标记避免全树昂贵重算。
* **文件细分剖析**：
  * [src/event.rs](file:///d:/for_clone/rust_css_like/crates/live-runtime/src/event.rs)：热重载事件枚举 `LiveEvent`（SourceTextChanged 文本变动、ScopeBlockClosed 作用域闭合、RequestRepaint 重绘请求、RequestRelayout 重排请求）。
  * [src/differ.rs](file:///d:/for_clone/rust_css_like/crates/live-runtime/src/differ.rs)：基于 `bitflags` 的三级刷新脏标记 `DirtyMask`（CLEAN 无需刷新、REPAINT 仅重绘 `<1ms`、RELAYOUT 仅重排 `<3ms`、RESTRUCTURE 全树重建）。

---

### 3.11 `script-engine`：微型脚本求值与插件槽位层
* **路径**：[crates/script-engine](file:///d:/for_clone/rust_css_like/crates/script-engine)
* **定位**：轻量级前端动态值系统、表达式求值器与第三方原生插件注册中心。
* **文件细分剖析**：
  * [src/value.rs](file:///d:/for_clone/rust_css_like/crates/script-engine/src/value.rs)：动态值枚举 `ScriptValue`（Null, Bool, Int, Float, String, List, Map），内置真值判定 `is_truthy()` 与格式化显示。
  * [src/scope.rs](file:///d:/for_clone/rust_css_like/crates/script-engine/src/scope.rs)：支持父子继承的作用域环境表 `ScriptScope`，线程安全且支持链式查找变量。
  * [src/eval.rs](file:///d:/for_clone/rust_css_like/crates/script-engine/src/eval.rs)：行内表达式与模版求值器 `Evaluator`，支持 `$var` 模版插值、点属性链查找 `task.title`、逻辑取反 `!var` 与内置函数 `len()`, `trim()`。
  * [src/extension.rs](file:///d:/for_clone/rust_css_like/crates/script-engine/src/extension.rs)：自定义扩展组件 Trait `CustomComponentPlugin` 与全局注册中心 `ExtensionRegistry`，为自研 3D/视频插件提供槽位。

---

### 3.12 `data-bridge`：全栈数据库与跨语言 FFI 胶水层
* **路径**：[crates/data-bridge](file:///d:/for_clone/rust_css_like/crates/data-bridge)
* **定位**：提供全栈本地/远程数据库连接，以及暴露给外部主流语言的标准 C-ABI。
* **文件细分剖析**：
  * [src/database.rs](file:///d:/for_clone/rust_css_like/crates/data-bridge/src/database.rs)：数据库查询值通用表示 `DbValue`、行数据结构 `DbRow` 与统一连接客户端抽象 `DatabaseClient` Trait。
  * [src/sqlite_local.rs](file:///d:/for_clone/rust_css_like/crates/data-bridge/src/sqlite_local.rs)：嵌入式本地数据库引擎驱动 `EmbeddedDatabase` 与本地表 `LocalTable`。
  * [src/reactive.rs](file:///d:/for_clone/rust_css_like/crates/data-bridge/src/reactive.rs)：响应式信号单元 `Signal<T>`（基于 `Arc<Mutex<T>>`），数据更新时触发 UI 脏刷新。
  * [src/ffi_c.rs](file:///d:/for_clone/rust_css_like/crates/data-bridge/src/ffi_c.rs)：导出给 Python/Node.js/C++/Go/C# 调用的标准 C-ABI 函数（`rust_css_engine_init`, `rust_css_engine_load_dsl`, `rust_css_engine_version`）。

---

## 四、 高拓展性设计：七大底层注册槽位 (RegistryBus)

为了保证工业级生命力，引擎采用**微内核 + 注册总线**架构，开发者可通过 `EnginePlugin` 一次性介入底层：

| 槽位序号 | 槽位名称 | 注册核心 Trait | 解决的拓展需求 | DSL 对应语法 |
| :---: | :--- | :--- | :--- | :--- |
| **1** | **GPU 着色器绘制** | `CustomPainter` | 挂载底层自定义着色器（水波光晕、毛玻璃模糊、粒子流光） | `row effect="WaterRipple"` |
| **2** | **自定义虚拟节点** | `CustomComponentDriver` | 挂载自研 3D 视口、视频播放器或 EChart 图表为一等公民节点 | `plugin "ModelViewer3D"` |
| **3** | **非标几何排版** | `CustomLayoutStrategy` | 实现除 Flex/Grid 外的特殊算法（如瀑布流 Waterfall、环形表盘） | `box display="waterfall"` |
| **4** | **自定义样式属性** | `CustomPropertyHandler` | 扩充非标样式字段并提供补间动画计算（Lerp） | `glow-speed=2.5 transition=(...)` |
| **5** | **系统原生函数** | `NativeHostFn` | 将操作系统底层能力（文件读写、剪贴板、系统托盘、物理蜂鸣）注入脚本 | `btn -> fs.read_text("...")` |
| **6** | **外部存储驱动** | `StorageDriverFactory` | 插件化接入 DuckDB、RocksDB、Redis 或跨进程共享内存 IPC | `db.connect("duckdb://...")` |
| **7** | **资源解密加载** | `AssetProtocolLoader` | 接管 `@import` 资源加载流程，支持私有加密包内存动态解密 | `@import "pak://dark.ui"` |

---

## 五、 声明式 DSL 语法实战展示 (examples/app.ui)

在 [examples/app.ui](file:///d:/for_clone/rust_css_like/examples/app.ui) 中，展示了使用该语言开发的原生桌面应用：

```scss
// 1. 响应式前端状态与数据库直连
let title = "Rust 原生商业工作台"
let is_dark = true
let count = 0
let username = "开发者"
let tasks = db.query("SELECT id, title, status FROM tasks ORDER BY id DESC")
let new_task_input = ""

// 2. 原生窗体声明：开门见山，零冗余样板代码
win "$title" (960, 640) bg=(is_dark ? #0f172a : #f8fafc) {

    // 顶部导航栏 (横向 Flex 流)
    row pad=16 bg=#1e293b align=center justify=between {
        row gap=10 align=center {
            box w=12 h=12 rad=6 bg=#22c55e // 在线状态小绿点
            txt "系统控制中心" #f8fafc 16px bold
        }

        row gap=8 align=center {
            txt "当前用户: $username" #94a3b8 13px
            // 动作流绑定：'->' 触发轻量脚本执行
            btn (is_dark ? "切换浅色" : "切换深色") pad=(6,12) bg=#334155 rad=6 -> is_dark = !is_dark
            btn "计数器: $count" pad=(6,14) bg=#4f46e5 rad=6 -> count += 1
        }
    }

    // 主内容区 (纵向排列)
    col flex=1 pad=20 gap=16 {

        // 快捷添加任务卡片 (挂载自研底层交互水波特效！)
        row pad=16 bg=#1e293b rad=8 gap=8 align=center effect="InteractiveRipple" {
            inp placeholder="输入要添加的本地任务..." bind=new_task_input flex=1
            btn "保存到 SQLite" bg=#4f46e5 rad=6 pad=(8,16) -> {
                db.execute("INSERT INTO tasks (title, status) VALUES (?, 'active')", [new_task_input])
                new_task_input = ""
            }
        }

        // 任务列表展示
        txt "待办任务列表" #94a3b8 14px bold

        col gap=8 {
            for item in tasks {
                row pad=12 bg=#1e293b rad=6 justify=between align=center {
                    txt item.title #f8fafc 14px
                    btn "删除" #ef4444 rad=4 pad=(4,8) -> db.execute("DELETE FROM tasks WHERE id = ?", [item.id])
                }
            }
        }
    }
}
```

---

## 六、 四大核心技术规范白皮书索引 (docs/)

为了保证系统的长期架构清晰度与工程可实施性，系统在 `docs/` 目录下建立了全套权威规范：

1. 📘 [01. 全景需求规格说明书与系统架构蓝图](file:///d:/for_clone/rust_css_like/docs/01_REQUIREMENTS_AND_ARCHITECTURE.md) (`DOC-01-REQ-ARCH`)：定义完整的业务需求指标、分层模型与六阶段端到端数据流水线。
2. 🔍 [02. 源码级缺陷审查与占位符清单](file:///d:/for_clone/rust_css_like/docs/02_DEFECTS_AND_STUB_AUDIT.md) (`DOC-02-DEFECTS-AUDIT`)：以最高工程标准解剖系统现有缺少 `main.rs`、虚假 WASM、缺画笔、假 SQLite 等断点并给出修复方案。
3. 🌐 [03. 与 HTML / CSS / PHP 深度集成对齐规范书](file:///d:/for_clone/rust_css_like/docs/03_HTML_CSS_PHP_ALIGNMENT_SPEC.md) (`DOC-03-ALIGN-SPEC`)：对齐表单双向流、`cosmic-text` 富文本排版、多维选择器、`calc()` 动态求解、`@import` 宏展开与真实预编译事务。
4. 🔌 [04. 高拓展性底层注册与插件开发规范指南](file:///d:/for_clone/rust_css_like/docs/04_EXTENSIBILITY_AND_PLUGIN_SYSTEM.md) (`DOC-04-EXT-PLUGIN`)：阐述七大底层注册槽位 Trait 规约与完整实战插件开发范式。

---

## 七、 构建、测试与分发

### 7.1 本地编译与单测
```bash
# 检查整个工作区各子模块代码规范
cargo check --workspace

# 执行全部子模块单元测试
cargo test --workspace
```

### 7.2 云端 GitHub Actions 全自动构建
* **Windows 与 macOS 出包**：推送至 `main` 分支触发 [.github/workflows/build-native.yml](file:///d:/for_clone/rust_css_like/.github/workflows/build-native.yml)，在微软和苹果官方虚拟机上自动化编译产出绿色 `.zip` 与 `.dmg`；
* **WebAssembly 演练场**：推送至 `main` 分支触发 [.github/workflows/preview-wasm.yml](file:///d:/for_clone/rust_css_like/.github/workflows/preview-wasm.yml)，自动化部署在线交互沙盒。

---
*本文档为 `rust_css_like` 顶级总览技术白皮书。*
