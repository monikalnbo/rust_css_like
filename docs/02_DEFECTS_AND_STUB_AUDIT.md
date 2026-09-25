# 02. Rust CSS-Like 源码级缺陷审查与占位符清单 (Defects & Stub Audit)

> **文档编号**：`DOC-02-DEFECTS-AUDIT`  
> **文档定位**：以最高工程标准对 `rust_css_like` 现有全部 12 个子模块与 CI/CD 脚本进行“不回避、不粉饰”的源码级代码解剖，详细标出每一处假数据（Mock）、占位符（Stub）与未打通的关键断点，并提供修复策略。

---

## 目录
1. [审查综述与缺陷严重度评级矩阵](#一-审查综述与缺陷严重度评级矩阵)
2. [P0 级致命缺陷解剖（阻断运行）](#二-p0-级致命缺陷解剖阻断运行)
   - [2.1 缺少可执行主入口 (No main.rs / app-shell)](#21-缺少可执行主入口-no-mainrs--app-shell)
   - [2.2 虚假 CI 演练场与出包脚本 (Mock WASM & Empty Artifacts)](#22-虚假-ci-演练场与出包脚本-mock-wasm--empty-artifacts)
   - [2.3 缺少底层上屏光栅化驱动 (Render Backend 缺画笔)](#23-缺少底层上屏光栅化驱动-render-backend-缺画笔)
3. [P1 级核心功能断层解剖（逻辑未闭环）](#三-p1-级核心功能断层解剖逻辑未闭环)
   - [3.1 假 SQLite 本地数据库 (String Prefix Mock)](#31-假-sqlite-本地数据库-string-prefix-mock)
   - [3.2 布局引擎缺少整树排版递归调度 (Missing TaffyTree Recursion)](#32-布局引擎缺少整树排版递归调度-missing-taffytree-recursion)
   - [3.3 脚本引擎缺少 AST 与运算符优先级 (Fake Script Evaluator)](#33-脚本引擎缺少-ast-与运算符优先级-fake-script-evaluator)
4. [P2 级体验与细节缺失解剖（规范不完整）](#四-p2-级体验与细节缺失解剖规范不完整)
   - [4.1 假输入法定位 (Fake IME Cursor Anchor)](#41-假输入法定位-fake-ime-cursor-anchor)
   - [4.2 GBK 缺少全量汉字 Unicode 码表 (Heuristic GBK)](#42-gbk-缺少全量汉字-unicode-码表-heuristic-gbk)
   - [4.3 表单控件族缺少状态流与交互逻辑 (Empty Form States)](#43-表单控件族缺少状态流与交互逻辑-empty-form-states)
5. [缺陷清零改造行动方案](#五-缺陷清零改造行动方案)

---

## 一、 审查综述与缺陷严重度评级矩阵

| 缺陷编号 | 所在文件/模块 | 缺陷现象 | 严重度评级 | 影响范围 |
| :--- | :--- | :--- | :---: | :--- |
| **DEF-01** | 全局 Workspace | 全仓只有 12 个 `lib.rs`，没有 `main.rs` | 🔴 **P0 (致命)** | 无法本地 `cargo run`，没有任何可执行程序输出 |
| **DEF-02** | `.github/workflows/preview-wasm.yml` | `Build WASM` 步骤仅 `cat` 静态 HTML，无 WASM 编译 | 🔴 **P0 (致命)** | 云端 GitHub Pages 演练场纯属静态假页面 |
| **DEF-03** | `.github/workflows/build-native.yml` | 打包不存在的 `app-shell` 与 `*.exe` | 🔴 **P0 (致命)** | Actions 产出的打包文件完全为空白压缩包 |
| **DEF-04** | `render-backend::command / display_list` | 只有 `DisplayList` 收集，无任何 GPU/CPU 光栅化驱动 | 🔴 **P0 (致命)** | 屏幕上没有任何像素产出，黑屏/无窗口 |
| **DEF-05** | `data-bridge::sqlite_local` | SQL 执行用字符串前缀匹配，查询返回硬编码行 | 🟠 **P1 (严重)** | 本地任务管理完全无法持久化，假数据库 |
| **DEF-06** | `layout-engine::bridge / solver` | 仅支持单节点转换，未将 DOM 树递归注入 Taffy | 🟠 **P1 (严重)** | 无法自适应多层嵌套盒模型流式排版 |
| **DEF-07** | `script-engine::eval` | 仅靠 `strip_prefix` 字符串截取，无 Pratt/AST | 🟠 **P1 (严重)** | 无法解析四则运算优先级、复杂逻辑与条件判断 |
| **DEF-08** | `render-backend::ime` | 仅有数据结构，未调用 Win32/macOS 输入法 API | 🟡 **P2 (中等)** | 中文输入法候选框悬空与错位 |
| **DEF-09** | `charset-compat::detector / transcoder` | GBK 依赖启发式探测，未内置 2 万+字完整映射码表 | 🟡 **P2 (中等)** | 特殊生僻字与扩展汉字转码可能出现失真 |
| **DEF-10** | `element-core::tag / state` | `Checkbox`/`Select` 仅有枚举定义，无双向状态机 | 🟡 **P2 (中等)** | 无法直接构建带交互的复选框与下拉菜单 |

---

## 二、 P0 级致命缺陷解剖（阻断运行）

### 2.1 缺少可执行主入口 (No main.rs / app-shell)

* **缺陷定位**：根目录 [Cargo.toml](file:///d:/for_clone/rust_css_like/Cargo.toml) 与全部 12 个子 Crate。
* **现状代码**：
  ```toml
  members = [
      "crates/css-types",
      "crates/dsl-parser",
      "crates/element-core",
      ...全部为库 crate
  ]
  ```
* **技术穿透**：
  任何 GUI 系统必须有一个运行时 Shell（包含 OS 窗口初始化、事件循环主泵 `event_loop.run()`、帧缓冲绑定）。当前仓库没有任何一个可执行的二进制文件入口，导致开发者拉取代码后无法执行 `cargo run`。
* **修复策略**：
  在 `crates/app-shell` 或根目录 `examples/desktop_app` 中建立 `src/main.rs`，引入 `winit` 建立物理视窗，并将 `app.ui` 的解析与渲染完整串接起来。

---

### 2.2 虚假 CI 演练场与出包脚本 (Mock WASM & Empty Artifacts)

* **缺陷定位**：[.github/workflows/preview-wasm.yml:33-88](file:///d:/for_clone/rust_css_like/.github/workflows/preview-wasm.yml#L33-L88)
* **现场抓包**：
  ```yaml
  - name: Build WASM Core
    run: |
      mkdir -p public
      cat << 'EOF' > public/index.html
      ...
      <div id="preview">
        <div class="canvas-mock">
          <h4>WASM 实时原生视口渲染</h4>
          <p>每次输入完成一个 <code>{ ... }</code> 闭合，右侧自动触发增量渲染！</p>
        </div>
      </div>
  ```
* **技术穿透**：
  脚本声称“免安装在线 WebAssembly 演练场”，但实际只是用 Shell 命令写入了一个写死文本的 HTML。它**压根没有执行 `wasm-pack build`**，也没有在页面里引入任何 `.wasm` 胶水脚本。同理，[build-native.yml:75](file:///d:/for_clone/rust_css_like/.github/workflows/build-native.yml#L75) 试图打包不存在的 `app-shell`，产出全部为空。
* **修复策略**：
  1. 新增 `crates/wasm-runtime`，导出 `#[wasm_bindgen]` 函数 `render_dsl_to_canvas(canvas_id, dsl_source)`；
  2. 在 CI 中执行 `wasm-pack build --target web --out-dir public/pkg`；
  3. 在 `index.html` 中通过 JavaScript 监听 textarea 输入事件并实时调用 WASM 增量渲染。

---

### 2.3 缺少底层上屏光栅化驱动 (Render Backend 缺画笔)

* **缺陷定位**：[crates/render-backend/src/command.rs](file:///d:/for_clone/rust_css_like/crates/render-backend/src/command.rs) 与 [display_list.rs](file:///d:/for_clone/rust_css_like/crates/render-backend/src/display_list.rs)
* **技术穿透**：
  `render-backend` 能产出 `DrawCommand::DrawRect`、`DrawText`、`PushClip`，但**全仓没有任何一个消费者去执行这些指令**。
  - 没有写基于 `wgpu` 的着色器管线（Vertex Shader / Fragment Shader / PSO）；
  - 没有接入 `tiny-skia` / `skia-safe` 软件光栅化器。
  这就好比一个打印机驱动程序生成了纸张打印指令，但物理打印机根本没有装喷头与墨盒，导致物理屏幕无法亮起。
* **修复策略**：
  在 `render-backend` 中实现 `SoftwareRenderer`（基于 `tiny-skia`）或 `GpuRenderer`（基于 `wgpu`），遍历 `DisplayList` 并将矢量图元画入物理帧缓冲区（Framebuffer / Texture）。

---

## 三、 P1 级核心功能断层解剖（逻辑未闭环）

### 3.1 假 SQLite 本地数据库 (String Prefix Mock)

* **缺陷定位**：[crates/data-bridge/src/sqlite_local.rs:68-88](file:///d:/for_clone/rust_css_like/crates/data-bridge/src/sqlite_local.rs#L68-L88)
* **现场抓包**：
  ```rust
  impl DatabaseClient for EmbeddedDatabase {
      fn execute(&self, sql: &str) -> Result<usize, String> {
          if sql.to_uppercase().starts_with("INSERT") {
              Ok(1)
          } else if sql.to_uppercase().starts_with("DELETE") {
              Ok(1)
          } else {
              Ok(0)
          }
      }

      fn query(&self, sql: &str) -> Result<Vec<DbRow>, String> {
          let mut rows = Vec::new();
          let mut row = DbRow::default();
          row.columns.push(("title".to_string(), DbValue::Text("示例数据记录".to_string())));
          rows.push(row);
          Ok(rows)
      }
  }
  ```
* **技术穿透**：
  这是典型的 Stub 模拟桩。执行任何 `INSERT` 或 `DELETE` 语句，其实都只是做了字符串前缀匹配，并直接返回 `Ok(1)`；查询永远返回一条假数据。它无法处理 `WHERE` 过滤、没有真实的 `.db` 文件持久化、不支持参数化绑定（`?`）。
* **修复策略**：
  在 `data-bridge` 中接入真正的 `rusqlite`（使用 `bundled` 特性避免外部依赖），实现带预编译参数绑定的真正 SQL 执行器。

---

### 3.2 布局引擎缺少整树排版递归调度 (Missing TaffyTree Recursion)

* **缺陷定位**：[crates/layout-engine/src/bridge.rs](file:///d:/for_clone/rust_css_like/crates/layout-engine/src/bridge.rs) 与 [solver.rs](file:///d:/for_clone/rust_css_like/crates/layout-engine/src/solver.rs)
* **技术穿透**：
  `LayoutBridge` 目前只写了一个将单独的 `ComputedStyle` 转换成 `taffy::style::Style` 的转换器。但是：
  1. 缺少整树递归构建器：没有将 `element-core::ElementTree` 中的父子关系同步映射为 `taffy::TaffyTree` 的 `NodeId`；
  2. 缺少坐标回填器：没有调用 `taffy.compute_layout()` 并将计算好的每个节点的绝对坐标 `x, y, width, height` 递归赋值回 `LayoutRect`。
* **修复策略**：
  编写 `LayoutSolver::solve_tree(&mut ElementTree, window_width, window_height)`，完成整树 Taffy 节点的构建、计算与坐标回填。

---

### 3.3 脚本引擎缺少 AST 与运算符优先级 (Fake Script Evaluator)

* **缺陷定位**：[crates/script-engine/src/eval.rs:10-87](file:///d:/for_clone/rust_css_like/crates/script-engine/src/eval.rs#L10-L87)
* **现场抓包**：
  ```rust
  if let Some(var_name) = trimmed.strip_prefix('!') { ... }
  if let Some(inner) = trimmed.strip_prefix("len(")...
  if trimmed.contains('.') { ... }
  ```
* **技术穿透**：
  所有的求值逻辑都是平铺的 `if strip_prefix` 字符串扫描。如果开发者在 DSL 中写入复合表达式（例如 `(count + 1) * 2`、`is_dark && count > 5`、`total > 100 ? #22c55e : #ef4444`），当前求值器完全无法理解，甚至会将其直接当作字符串原样返回。
* **修复策略**：
  引入简易的 **Pratt Parser（普拉特解析器）**，将行内表达式构建为微型二叉表达式树（`Expr::BinaryOp`, `Expr::UnaryOp`, `Expr::Ternary`），严格按照数学与逻辑优先级递归求值。

---

## 四、 P2 级体验与细节缺失解剖（规范不完整）

### 4.1 假输入法定位 (Fake IME Cursor Anchor)
* **缺陷定位**：[crates/render-backend/src/ime.rs:1-25](file:///d:/for_clone/rust_css_like/crates/render-backend/src/ime.rs#L1-L25)
* **技术穿透**：
  `ImeCursorAnchor` 仅仅计算了一个 `(x, y + line_height)` 的数学点，完全没有调用 Windows 原生 `imm32.dll` 的 `ImmSetCandidateWindow` 或 macOS `NSTextInputClient`。输入框在打汉字时，拼音候选框依旧会飘在屏幕左上角。

### 4.2 GBK 缺少全量汉字 Unicode 码表 (Heuristic GBK)
* **缺陷定位**：[crates/charset-compat/src/detector.rs:37-63](file:///d:/for_clone/rust_css_like/crates/charset-compat/src/detector.rs#L37-L63)
* **技术穿透**：
  当前采用的是检测高字节在 `0x81..=0xFE`、低字节在 `0x40..=0xFE` 的启发式规则。解码失败时直接回退到 `String::from_utf8_lossy`，并未内嵌真正的 GB18030 二万多汉字映射码表，遇到生僻字可能出现字符替代丢失。

### 4.3 表单控件族缺少状态流与交互逻辑 (Empty Form States)
* **缺陷定位**：[crates/element-core/src/tag.rs:21-29](file:///d:/for_clone/rust_css_like/crates/element-core/src/tag.rs#L21-L29)
* **技术穿透**：
  `ElementTag` 中虽然列举了 `Checkbox`、`Select`、`Option`、`Slider`，但由于没有给它们定义专有的状态结构体（如 Checkbox 的 `checked: bool`、Slider 的 `value: f32, min: f32, max: f32`），目前在运行时它们只是一堆无行为的空盒子。

---

## 五、 缺陷清零改造行动方案

```
[行动计划：从 Stub 桩代码迈向工业级生产就绪]

第 1 步：创建 crates/app-shell (解决 DEF-01, DEF-04)
  └─ 编写 main.rs，引入 winit + tiny-skia，完成首帧窗口物理光栅化渲染。

第 2 步：做实 layout-engine 整树排版 (解决 DEF-06)
  └─ 打通 ElementTree -> TaffyTree 递归构建与 LayoutRect 坐标回填。

第 3 步：做实 data-bridge 真实 SQLite (解决 DEF-05)
  └─ 引入 rusqlite 替代内存 HashMap，打通真实单文件本地表增删改查。

第 4 步：重构 script-engine 引入 Pratt 解析器 (解决 DEF-07)
  └─ 支持 1 + 2 * 3 运算符优先级、比较逻辑与三元条件求解。

第 5 步：真实打通 preview-wasm.yml (解决 DEF-02, DEF-03)
  └─ 接入 wasm-pack build，在 HTML 中通过 Canvas 实现浏览器内真 WASM 渲染。
```
