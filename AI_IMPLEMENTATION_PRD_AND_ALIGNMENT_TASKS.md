# Rust CSS-Like 全面对标实现任务指导书与工程实施 PRD (AI Agent 专用版)

> **文档性质**：面向后续接盘 AI（如 Claude 3.7 / GPT-4o / DeepSeek / Cursor Agent 等）的高精度工程实现任务说明书（Engineering PRD & Implementation Guide）。
> **核心目标**：提供无歧义、免猜测、可立即拆分为具体 Prompt 的实施方案，将 `rust_css_like` 从目前的“架构骨架/桩代码阶段”推进至“全功能、可运行、全面对标现代 Web/GUI 标准的生产级原生引擎”。

---

## 目录
1. [接盘 AI 须知与开发约束守则](#一-接盘-ai-须知与开发约束守则)
2. [全面对标全景差距矩阵 (What is Missing)](#二-全面对标全景差距矩阵-what-is-missing)
3. [标准技术栈选型矩阵 (Tech Stack Recommendations)](#三-标准技术栈选型矩阵-tech-stack-recommendations)
4. [工程实施工单 (Task Tickets)](#四-工程实施工单-task-tickets)
   - [TASK-01 (P0): 建立桌面应用主外壳与系统窗口事件泵 (app-shell)](#task-01-p0-建立桌面应用主外壳与系统窗口事件泵-app-shell)
   - [TASK-02 (P0): 实现 2D 软件/GPU 光栅化渲染驱动 (render-backend)](#task-02-p0-实现-2d-软件gpu-光栅化渲染驱动-render-backend)
   - [TASK-03 (P1): 布局引擎整树递归排版与 Taffy 闭环 (layout-engine)](#task-03-p1-布局引擎整树递归排版与-taffy-闭环-layout-engine)
   - [TASK-04 (P1): 基于 Pratt 算法的表达式求值与 AST 解释器 (script-engine)](#task-04-p1-基于-pratt-算法的表达式求值与-ast-解释器-script-engine)
   - [TASK-05 (P1): 替换伪 SQL，接入真实嵌入式 SQLite (data-bridge)](#task-05-p1-替换伪-sql接入真实嵌入式-sqlite-data-bridge)
   - [TASK-06 (P2): 字符塑形、文本断行测量与富文本混排 (Text Shaping)](#task-06-p2-字符塑形文本断行测量与富文本混排-text-shaping)
   - [TASK-07 (P2): 现代 CSS 选择器匹配与 calc() 动态混合计算 (style-system)](#task-07-p2-现代-css-选择器匹配与-calc-动态混合计算-style-system)
   - [TASK-08 (P2): 完整表单控件族状态机与双向绑定 (element-core)](#task-08-p2-完整表单控件族状态机与双向绑定-element-core)
   - [TASK-09 (P2): 标准 DOM 事件冒泡/捕获与全局 Tab 焦点链 (live-runtime)](#task-09-p2-标准-dom-事件冒泡捕获与全局-tab-焦点链-live-runtime)
   - [TASK-10 (P3): 真实 WebAssembly 演练场与跨平台构建 (WASM Playground)](#task-10-p3-真实-webassembly-演练场与跨平台构建-wasm-playground)
5. [用户可直接复制给 AI 的分步 Prompt 模板库](#五-用户可直接复制给-ai-的分步-prompt-模板库)

---

## 一、 接盘 AI 须知与开发约束守则

### 1.1 现状严正提醒
1. **千万不要继续写 Mock / Stub**：当前项目中存在大量只有枚举定义、没有执行逻辑的占位代码（如假的 SQLite `starts_with("INSERT")`、假的 WASM CI、假的布局转换）。接盘 AI 的首要任务是**做实逻辑，严禁产生新的虚假桩代码**。
2. **当前仓库没有 main.rs**：目前 12 个子模块全部为 `lib.rs`，拉取代码后无法 `cargo run`，导致任何渲染与交互无法得到物理验证。必须优先解决应用入口问题。
3. **目前总代码量**：全仓 58 个 `.rs` 文件，约 2,578 行代码。说明数据骨架清晰，但核心算法均处于待填充状态。

### 1.2 架构硬性约束 (Architectural Invariants)
- **单向无环依赖 (Strict DAG)**：严禁在下层 crate（如 `css-types`, `element-core`）反向引用上层 crate（如 `render-backend`, `layout-engine`）。
- **小文件规范**：每个 Rust 源文件代码控制在 **50 ~ 200 行以内**。复杂模块必须拆分子文件（如 `tokenizer.rs`, `pratt.rs`, `eval.rs`），严禁堆砌单文件上千行的巨石代码。
- **内存安全与高性能**：核心样式与布局计算路径禁止频繁堆分配，能用 SmallVec / POD 值的尽量复用。

---

## 二、 全面对标全景差距矩阵 (What is Missing)

若要将本项目全面对标成熟的 **HTML (控件/事件/文本)**、**CSS (层叠/选择器/动画/动态排版)**、**PHP/JS (脚本/数据直连)** 以及 **Flutter/Slint (原生渲染)**，各模块缺失如下：

```
┌─────────────────┬───────────────────────────────┬──────────────────────────────────────────┐
│   模块分类      │ 现有状态 (Current)            │ 生产级对标缺失 (Missing Requirements)    │
├─────────────────┼───────────────────────────────┼──────────────────────────────────────────┤
│ 1. 运行入口     │ 只有 12 个 lib.rs，无 binary  │ 操作系统视窗 (winit)、事件循环、物理上屏 │
├─────────────────┼───────────────────────────────┼──────────────────────────────────────────┤
│ 2. 渲染引擎     │ 仅有 DisplayList 指令队列     │ 真正的 2D 光栅化画笔 (tiny-skia / wgpu)  │
├─────────────────┼───────────────────────────────┼──────────────────────────────────────────┤
│ 3. 布局引擎     │ 仅单节点 Style 转换           │ 整树递归注入 TaffyTree、全局坐标回填     │
├─────────────────┼───────────────────────────────┼──────────────────────────────────────────┤
│ 4. 文本与排版   │ 仅支持固定字号静态字符串      │ 字体字形塑形 (Shaping)、换行自适应测量   │
├─────────────────┼───────────────────────────────┼──────────────────────────────────────────┤
│ 5. 样式系统     │ 仅支持内嵌属性简写            │ 类选择器、后代选择器、calc()、CSS 变量作用域 │
├─────────────────┼───────────────────────────────┼──────────────────────────────────────────┤
│ 6. 脚本引擎     │ 字符串 split/strip 简单匹配   │ Pratt 语法分析器、四则运算优先级、对象点访问 │
├─────────────────┼───────────────────────────────┼──────────────────────────────────────────┤
│ 7. 数据库接口   │ HashMap 模拟表，写死前缀判断  │ 真实 SQLite (rusqlite/C绑定)、参数化预编译 │
├─────────────────┼───────────────────────────────┼──────────────────────────────────────────┤
│ 8. DOM 与交互   │ 仅有顶层 HitTester 碰撞检测   │ 事件捕获与冒泡、键盘 Tab 焦点链、光标闪烁│
├─────────────────┼───────────────────────────────┼──────────────────────────────────────────┤
│ 9. 控件丰富度   │ 仅基础 Box/Btn/Txt，其余空枚举│ Checkbox/Radio 互斥态、Select 下拉框浮层 │
├─────────────────┼───────────────────────────────┼──────────────────────────────────────────┤
│ 10. Web 演练场  │ CI 脚本仅 cat 静态 HTML 假页面 │ wasm-bindgen 真实编译导出、Canvas 画布绘制 │
└─────────────────┴───────────────────────────────┴──────────────────────────────────────────┘
```

---

## 三、 标准技术栈选型矩阵 (Tech Stack Recommendations)

为了避免接盘 AI 随意引入不兼容或过于臃肿的库，统一规定关键基础设施的技术选型：

| 领域 / 功能 | 推荐采用的 Rust Crate | 选型理由 |
| :--- | :--- | :--- |
| **窗口与原生事件** | `winit = "0.29"` | Rust 生态事实标准的跨平台窗口管理底座，无 C 运行时依赖。 |
| **软件光栅化 (首选)**| `tiny-skia = "0.11"` | 纯 Rust 实现的 2D 绘图引擎，轻量、抗锯齿极高、零 GPU 驱动黑屏风险。 |
| **GPU 硬件加速 (次选)**| `wgpu = "22.0"` | WebGPU 标准跨平台后端，适合渲染复杂 Shader 与自研特效。 |
| **排版引擎** | `taffy = "0.7"` (已有) | 现成的高效 Flexbox / Grid 布局求解器，必须深入串联。 |
| **文本塑形与排版** | `cosmic-text = "0.12"` | 纯 Rust 高性能多语言字体分词、断行、测量与光栅化库。 |
| **嵌入式数据库** | `rusqlite = { version = "0.31", features = ["bundled"] }` | 真正的单文件 SQLite，开箱即用，支持内存模式与物理磁盘持久化。 |
| **Web 编译导出** | `wasm-bindgen = "0.2"`, `web-sys` | 标准的 Rust to WASM 桥梁。 |

---

## 四、 工程实施工单 (Task Tickets)

接盘 AI 应严格按照以下工单由浅入深推进：

---

### TASK-01 (P0): 建立桌面应用主外壳与系统窗口事件泵 (app-shell)

* **负责目录**：新建 `crates/app-shell` 或在根目录添加 `examples/desktop_app`
* **目标**：实现一个具有物理视窗的可执行程序，打通“解析 DSL ──► 计算样式 ──► 求解排版 ──► 发射指令 ──► 绘制并呈现”的主闭环。
* **依赖引入**：`winit = "0.29"`, `softbuffer = "0.4"` (或与 tiny-skia 配合呈现到窗口)
* **核心代码与架构指引**：
  ```rust
  // crates/app-shell/src/main.rs 架构原型
  use winit::event_loop::EventLoop;
  use winit::window::WindowBuilder;

  fn main() {
      let event_loop = EventLoop::new().unwrap();
      let window = WindowBuilder::new()
          .with_title("Rust CSS-Like Native Runtime")
          .with_inner_size(winit::dpi::LogicalSize::new(960.0, 640.0))
          .build(&event_loop)
          .unwrap();

      // 初始化 softbuffer 帧缓冲区
      // 读取 examples/app.ui 源码
      // 在 RedrawRequested 事件中执行完整渲染管线
  }
  ```
* **验收标准 (DoD)**：
  1. 运行 `cargo run -p app-shell`（或 `cargo run --example desktop_app`），能弹出原生系统窗口。
  2. 窗口 Resize 时，能正确触发重新排版与重绘，无崩溃无闪退。

---

### TASK-02 (P0): 实现 2D 软件/GPU 光栅化渲染驱动 (render-backend)

* **负责目录**：[crates/render-backend/src](file:///d:/for_clone/rust_css_like/crates/render-backend/src)
* **目标**：为现有 `DisplayList` 编写消费驱动（Renderer），将 `DrawCommand` 逐项翻译为物理像素。
* **新增文件**：
  - `crates/render-backend/src/software_renderer.rs`
  - `crates/render-backend/src/font_atlas.rs`
* **关键实现逻辑**：
  ```rust
  // 伪代码指引
  use tiny_skia::{PixmapMut, Paint, PathBuilder, Rect, Color as SkColor};
  use crate::command::DrawCommand;
  use crate::display_list::DisplayList;

  pub struct SoftwareRenderer;

  impl SoftwareRenderer {
      pub fn render_to_pixmap(list: &DisplayList, pixmap: &mut PixmapMut) {
          for cmd in list.commands() {
              match cmd {
                  DrawCommand::DrawRect { bounds, color, radius, border_color, border_width } => {
                      // 1. 使用 PathBuilder 绘制带圆角矩形
                      // 2. 填充内部背景色
                      // 3. 若 border_width > 0，绘制描边
                  }
                  DrawCommand::DrawText { text, font_size, color, position } => {
                      // 绘制文字图元
                  }
                  DrawCommand::PushClip { clip_rect } => {
                      // 设置裁剪区域
                  }
                  DrawCommand::PopClip => {
                      // 恢复裁剪区域
                  }
                  _ => {}
              }
          }
      }
  }
  ```
* **验收标准 (DoD)**：
  1. 单元测试验证：给定一个带有圆角和背景色的 `DrawRect` 指令，渲染出来的 `tiny_skia::Pixmap` 对应像素点颜色值完全匹配。
  2. 结合 TASK-01 能将 [examples/app.ui](file:///d:/for_clone/rust_css_like/examples/app.ui) 里的矩形卡片、圆角按钮真实画在窗口上。

---

### TASK-03 (P1): 布局引擎整树递归排版与 Taffy 闭环 (layout-engine)

* **负责目录**：[crates/layout-engine/src](file:///d:/for_clone/rust_css_like/crates/layout-engine/src)
* **目标**：实现从 `ElementTree`（虚拟 DOM 树）递归构建 `taffy::TaffyTree`，执行整树布局计算后，将各节点的物理坐标与尺寸回填保存。
* **文件拆分建议**：
  - `crates/layout-engine/src/tree_solver.rs`：整树递归调度与 Taffy NodeId 映射。
* **关键实现接口**：
  ```rust
  use element-core::{ElementTree, NodeId};
  use style-system::ComputedStyle;
  use taffy::TaffyTree;
  use std::collections::HashMap;

  pub struct LayoutEngineContext {
      taffy: TaffyTree,
      node_map: HashMap<NodeId, taffy::tree::NodeId>,
  }

  impl LayoutEngineContext {
      /// 1. 递归构建 Taffy 树结构
      pub fn build_taffy_tree(&mut self, tree: &ElementTree, styles: &HashMap<NodeId, ComputedStyle>) -> taffy::tree::NodeId;

      /// 2. 求解全局布局
      pub fn compute_layout(&mut self, root_taffy_id: taffy::tree::NodeId, available_width: f32, available_height: f32);

      /// 3. 将所有 Taffy 计算结果回填为物理 LayoutRect (x, y, w, h)
      pub fn collect_layout_rects(&self, tree: &ElementTree) -> HashMap<NodeId, crate::LayoutRect>;
  }
  ```
* **验收标准 (DoD)**：
  1. 支持 `col` 嵌套多个 `row`，子节点使用 `flex=1` 能自适应平分父级容器空间。
  2. 窗口从 800px 宽度拉伸到 1200px 宽度时，各节点重新求解出的 `LayoutRect.width` 准确伸缩。

---

### TASK-04 (P1): 基于 Pratt 算法的表达式求值与 AST 解释器 (script-engine)

* **负责目录**：[crates/script-engine/src](file:///d:/for_clone/rust_css_like/crates/script-engine/src)
* **目标**：彻底丢弃现有的简单字符串切割，实现一个生产级轻量表达式解析与解释器。
* **核心功能要求**：
  1. **四则混合运算优先级**：正确处理 `1 + 2 * 3 == 7`、`(10 - 2) / 4 == 2`。
  2. **比较与逻辑运算**：支持 `==`, `!=`, `<`, `<=`, `>`, `>=`, `&&`, `||`, `!`。
  3. **三元条件表达式**：支持 `is_dark ? #0f172a : #f8fafc`。
  4. **连续对象属性点访问**：支持 `task.author.name`。
  5. **动作执行器**：支持解析 `-> count += 1`、`-> is_dark = !is_dark` 并直接修改作用域变量。
* **架构拆分建议**：
  - `src/lexer.rs`：表达式专用 Token 流。
  - `src/ast.rs`：二元操作符、一元操作符、函数调用节点。
  - `src/pratt.rs`：Pratt 解析器（根据 binding power 绑定力解析运算符）。
  - `src/eval.rs`：AST 遍历求值。
* **验收标准 (DoD)**：
  1. 单元测试覆盖各类复杂混合表达式：`"count * 2 + 1 > 5 ? 100 : 0"` 能在指定变量上下文中返回正确的 `ScriptValue`。
  2. 模拟按钮点击派发动作流，作用域中的 `count` 变量正确递增。

---

### TASK-05 (P1): 替换伪 SQL，接入真实嵌入式 SQLite (data-bridge)

* **负责目录**：[crates/data-bridge/src](file:///d:/for_clone/rust_css_like/crates/data-bridge/src)
* **目标**：将 `sqlite_local.rs` 改造为使用真实原生 SQLite 引擎，支持单文件数据库以及内存数据库。
* **依赖引入**：`rusqlite = { version = "0.31", features = ["bundled"] }`
* **改造关键点**：
  - `EmbeddedDatabase` 内部包装 `Arc<Mutex<rusqlite::Connection>>`。
  - 支持 `db.execute(sql, params)` 执行真正的 `CREATE TABLE`, `INSERT`, `UPDATE`, `DELETE`。
  - 支持 `db.query(sql, params)` 返回真实的列名与动态行数据集合（`DbRow`），具备完整的 `INTEGER`, `REAL`, `TEXT`, `BLOB`, `NULL` 映射。
* **验收标准 (DoD)**：
  1. 单元测试：新建表 `tasks`，插入 3 条记录，使用 `SELECT * FROM tasks WHERE id = 2` 能精确查出对应真实记录。
  2. 支持在本地生成 `.db` 文件并持久化保存数据。

---

### TASK-06 (P2): 字符塑形、文本断行测量与富文本混排 (Text Shaping)

* **负责目录**：新建 `crates/text-layout` 或扩充 `layout-engine`
* **目标**：解决目前只能写死单行文本尺寸的问题，对标 HTML 的行内图文排版。
* **依赖引入**：`cosmic-text = "0.12"`
* **核心功能**：
  1. **字形测量 (Shaping)**：给定字符串、字体文件、字号，精确测量出包围盒（Bounding Box）。
  2. **自动断行 (Word Wrapping)**：根据父容器分配的最大可用宽度，自动将文本分切为多行，并计算出总体占用高度。
  3. **Taffy 测量闭环**：通过 Taffy 的 `measure_func` 回调，将文本的实际测量尺寸喂给 Flexbox 布局器，实现内在尺寸（Intrinsic Sizing）。
  4. **富文本切片 (Span)**：支持一个段落内局部加粗、不同文字颜色和背景色。
* **验收标准 (DoD)**：
  1. 长文本在 200px 宽度的容器内自动折行成多行，文本容器高度随行数自适应撑开。

---

### TASK-07 (P2): 现代 CSS 选择器匹配与 calc() 动态混合计算 (style-system)

* **负责目录**：[crates/style-system/src](file:///d:/for_clone/rust_css_like/crates/style-system/src)
* **目标**：对标 CSS3 核心层叠与动态计算规范。
* **核心任务**：
  1. **选择器语法与匹配器**：
     - 支持类选择器（`.btn-primary`）、ID 选择器（`#main`）、属性选择器（`[disabled]`）。
     - 支持层级选择器（子代 `>`、后代空格）。
     - 完善特异度打分算法（ID: 100, Class: 10, Tag: 1）。
  2. **`calc()` 动态混合计算求解器**：
     - 在 `Dimension` 中增加 `Calc(CalcExpr)` 变体。
     - 在布局排版阶段，传入父级容器的参考物理像素（如 `parent_width = 500.0`），动态解算出 `calc(100% - 32px) = 468px`。
  3. **作用域 CSS 变量继承**：
     - 将当前扁平全局的 `VariableTable` 改为树形链表结构，子节点可覆盖 `--primary-color` 而不污染父级。
* **验收标准 (DoD)**：
  1. 样式表中定义 `.card > .btn:hover`，当节点结构满足且状态为 Hover 时，对应样式正确层叠生效。
  2. 元素宽度声明为 `calc(50% + 10px)`，在 400px 宽度的父容器下正确解算为 210px。

---

### TASK-08 (P2): 完整表单控件族状态机与双向绑定 (element-core)

* **负责目录**：[crates/element-core/src](file:///d:/for_clone/rust_css_like/crates/element-core/src)
* **目标**：将只有枚举的空占位控件做实为具备完整内部状态流的原生组件。
* **需实装控件**：
  1. **Checkbox (复选框)**：维护 `checked: bool` 状态，响应点击事件切换状态，提供对勾绘制图元。
  2. **Radio (单选框)**：支持 `name` 分组，同组选项互斥单选。
  3. **Select & Option (下拉菜单)**：支持折叠态/展开态，展开时生成全局悬浮浮层（Overlay），支持键盘上下键与鼠标高亮选择。
  4. **Slider (滑动条)**：支持 `min`, `max`, `step`, `value`，支持鼠标拖拽滑块（Drag）并派发数值变动事件。
* **验收标准 (DoD)**：
  1. 复选框点击时勾选状态发生真实切换，并同步更新双向绑定的变量（`bind=is_agreed`）。

---

### TASK-09 (P2): 标准 DOM 事件冒泡/捕获与全局 Tab 焦点链 (live-runtime)

* **负责目录**：[crates/live-runtime/src](file:///d:/for_clone/rust_css_like/crates/live-runtime/src) 与 [element-core](file:///d:/for_clone/rust_css_like/crates/element-core)
* **目标**：实现对标浏览器 DOM 的事件分发体系与键盘交互。
* **核心机制**：
  1. **事件流三阶段**：
     - `Capturing Phase`（由根节点向下递送至目标节点）
     - `Target Phase`（目标节点执行绑定事件）
     - `Bubbling Phase`（由目标节点沿 `parent` 链向上冒泡）
     - 支持在事件处理器中调用 `e.stop_propagation()` 阻断向上冒泡。
  2. **全局焦点链 (Focus Chain)**：
     - 维护当前聚焦节点 `focused_node: Option<NodeId>`。
     - 按键盘 `Tab` 键自动聚焦到下一个可获得焦点的控件（`btn`, `inp`, `checkbox`），并自动为其附加 `:focus` 伪类样式。
  3. **文本输入光标 (Caret Blink)**：
     - 文本框获得焦点时，以 500ms 频率周期性发送重绘信号，渲染闪烁的文本输入竖线。
* **验收标准 (DoD)**：
  1. 子容器按钮被点击时，父容器能捕获到该事件；若子按钮调用阻止冒泡，父容器不会收到事件。
  2. 纯键盘操作：按 Tab 键能在 3 个输入框间依次轮转焦点。

---

### TASK-10 (P3): 真实 WebAssembly 演练场与跨平台构建 (WASM Playground)

* **负责目录**：新建 `crates/wasm-runtime` 与改造 [.github/workflows/preview-wasm.yml](file:///d:/for_clone/rust_css_like/.github/workflows/preview-wasm.yml)
* **目标**：彻底消灭“用 Shell cat 写入静态 HTML 假装是 WASM”的虚假流程，实现真正的在线代码编辑实时预览演练场。
* **实施步骤**：
  1. 新增 `crates/wasm-runtime`，导出真实 `wasm_bindgen` 接口：
     ```rust
     #[wasm_bindgen]
     pub fn render_dsl_to_canvas(canvas_id: &str, dsl_source: &str) -> Result<(), JsValue> {
         // 解析 dsl_source -> 计算样式 -> 排版 -> 用 web-sys CanvasRenderingContext2D 执行绘制
     }
     ```
  2. 在 GitHub Actions 中安装 `wasm-pack`，执行真正的 `wasm-pack build --target web --out-dir ./public/pkg`。
  3. 前端静态页引入真正的 WASM 胶水 JS，在 `<textarea>` 输入框变动时实时重新编译渲染。
* **验收标准 (DoD)**：
  1. 本地执行 `wasm-pack build` 能成功产出 `.wasm` 与 `.js` 桥接文件。
  2. 在浏览器中打开本地 Web 页面，左边修改 `.ui` 文本，右边 Canvas 原生发生变化。

---

## 五、 用户可直接复制给 AI 的分步 Prompt 模板库

当你把这个项目交给其他 AI 时，**请不要一次性把所有任务全部抛出**，建议每次选择一个工单，复制以下 Prompt 发送给接盘 AI：

### Prompt 示例 1：执行 TASK-01 与 TASK-02（打通物理窗口与基础光栅化）
```text
请阅读项目中的 AI_IMPLEMENTATION_PRD_AND_ALIGNMENT_TASKS.md 文档。
现在我们需要优先实施【TASK-01】和【TASK-02】：
1. 在项目中建立可执行入口（例如 crates/app-shell 或 examples/desktop_app），引入 winit 和 softbuffer 建立物理视窗。
2. 在 crates/render-backend 中引入 tiny-skia，编写 SoftwareRenderer，实现对 DisplayList 中 DrawRect（含圆角、边框、背景）和 DrawText 指令的真实光栅化像素输出。
3. 确保运行 `cargo run -p app-shell` 能够弹出一个原生系统窗口，并能在窗口中正确绘制出 examples/app.ui 中的基础卡片背景与按钮。
注意：严禁写任何 mock 或伪数据，遵循小文件规范（每个文件 50~200 行）。请一步步给出修改和新增的代码。
```

### Prompt 示例 2：执行 TASK-03（打通 Taffy 整树递归排版）
```text
请阅读项目中的 AI_IMPLEMENTATION_PRD_AND_ALIGNMENT_TASKS.md 文档。
现在我们需要实施【TASK-03: 布局引擎整树递归排版与 Taffy 闭环】：
1. 深入改造 crates/layout-engine，编写 TreeSolver。
2. 将 element-core 中的 ElementTree 虚拟 DOM 树递归构建进 taffy::TaffyTree，实现父子容器关系的完整挂载。
3. 执行 compute_layout 后，将 Taffy 计算得到的物理尺寸和相对坐标递归转化为绝对坐标，并回填到各个节点的 LayoutRect 中。
4. 编写完整的单元测试：测试在 800x600 视口下，嵌套的 flex 列和 flex 行各子元素是否获得了正确的宽度、高度与偏移。
```

### Prompt 示例 3：执行 TASK-04（做实 script-engine 的 Pratt 表达式解析）
```text
请阅读项目中的 AI_IMPLEMENTATION_PRD_AND_ALIGNMENT_TASKS.md 文档。
现在我们需要实施【TASK-04: 基于 Pratt 算法的表达式求值与 AST 解释器】：
1. 重构 crates/script-engine，彻底移除目前依靠 strip_prefix 和简单字符串匹配的粗糙实现。
2. 引入 Pratt 语法分析器，支持加减乘除四则混合运算优先级、比较运算符（<, >, ==）、三元表达式（cond ? a : b）以及连续点属性访问（如 task.author.name）。
3. 支持动作流赋值操作（如 count += 1, is_dark = !is_dark）。
4. 编写充分的单元测试验证四则运算结合性、优先级和逻辑短路行为。
```

### Prompt 示例 4：执行 TASK-05（做实 SQLite 真实数据库）
```text
请阅读项目中的 AI_IMPLEMENTATION_PRD_AND_ALIGNMENT_TASKS.md 文档。
现在我们需要实施【TASK-05: 替换伪 SQL，接入真实嵌入式 SQLite】：
1. 在 crates/data-bridge/Cargo.toml 中引入 rusqlite（启用 bundled 特性）。
2. 重构 sqlite_local.rs，将目前基于 HashMap 且只用 starts_with 判断 SQL 的假数据库替换为真实的 SQLite 连接封装。
3. 支持参数化查询与真正的 execute / query 映射，将 SQLite 数据行完整转换为 DbRow 与 DbValue。
4. 编写持久化测试用例，验证建表、插入、查询、删除的真实可用性。
```

---
> **维护与更新记录**：本文件由系统架构委员会于 2026 年整理，专门用于自动化 AI 编码助理的任务交接与工程对齐。

### 六、 全面对标工单完成归档 (Implementation Status Matrix)

| 工单编号 | 任务名称 | 实施状态 | 核心产出文件 | 规范检测 |
| :--- | :--- | :---: | :--- | :---: |
| **TASK-01 (P0)** | 桌面宿主与系统窗口事件泵 | ✅ 已完成 | [`crates/app-shell`](file:///d:/for_clone/rust_css_like/crates/app-shell) (`main.rs`, `window.rs`, `pipeline.rs`, `event_pump.rs`) | 通过 |
| **TASK-02 (P0)** | 2D 软件光栅化渲染驱动 | ✅ 已完成 | [`crates/render-backend`](file:///d:/for_clone/rust_css_like/crates/render-backend) (`software_renderer.rs`, `font_atlas.rs`) | 通过 |
| **TASK-03 (P1)** | 布局引擎整树递归排版与 Taffy 闭环 | ✅ 已完成 | [`crates/layout-engine`](file:///d:/for_clone/rust_css_like/crates/layout-engine) (`tree_solver.rs`) | 通过 |
| **TASK-04 (P1)** | 基于 Pratt 算法的表达式求值与 AST 解释器 | ✅ 已完成 | [`crates/script-engine`](file:///d:/for_clone/rust_css_like/crates/script-engine) (`token.rs`, `ast.rs`, `pratt.rs`, `eval.rs`) | 通过 |
| **TASK-05 (P1)** | 替换伪 SQL，接入真实嵌入式 SQLite | ✅ 已完成 | [`crates/data-bridge`](file:///d:/for_clone/rust_css_like/crates/data-bridge) (`sqlite_local.rs`) | 通过 |
| **TASK-06 (P2)** | 字符塑形、文本断行测量与富文本混排 | ✅ 已完成 | [`crates/text-layout`](file:///d:/for_clone/rust_css_like/crates/text-layout) (`shaper.rs`, `paragraph.rs`, `span.rs`) | 通过 |
| **TASK-07 (P2)** | CSS 选择器匹配与 calc() 动态混合计算 | ✅ 已完成 | [`crates/style-system`](file:///d:/for_clone/rust_css_like/crates/style-system) (`selector.rs`, `calc.rs`, `variables.rs`) | 通过 |
| **TASK-08 (P2)** | 完整表单控件族状态机与双向绑定 | ✅ 已完成 | [`crates/element-core`](file:///d:/for_clone/rust_css_like/crates/element-core) (`form.rs`) | 通过 |
| **TASK-09 (P2)** | 标准 DOM 事件冒泡/捕获与全局 Tab 焦点链 | ✅ 已完成 | [`crates/live-runtime`](file:///d:/for_clone/rust_css_like/crates/live-runtime) (`dom_event.rs`, `focus.rs`) | 通过 |
| **TASK-10 (P3)** | 真实 WebAssembly 演练场与跨平台构建 | ✅ 已完成 | [`crates/wasm-runtime`](file:///d:/for_clone/rust_css_like/crates/wasm-runtime) (`canvas_backend.rs`, `index.html`, `preview-wasm.yml`) | 通过 |
