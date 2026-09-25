# 系统当前占位符 (Placeholders)、未补全细节与对标差距清单

> **审查定位**：本文档以最高工程标准对 `rust_css_like` 当前全部 12 个子模块进行严格审查，**逐一坦白哪些模块仅实现了类型骨架/模拟占位符（Mock / Stub），哪些细节尚未完全打通**，并列出生产级补全路线。

---

## 目录
1. [占位符与未补全细节全景矩阵](#一-占位符与未补全细节全景矩阵)
2. [各模块占位细节逐项解剖](#二-各模块占位细节逐项解剖)
   - [2.1 `data-bridge::sqlite_local`: 内存 Mock 占位](#21-data-bridgesqlite_local-内存-mock-占位)
   - [2.2 `render-backend`: GPU 着色器上屏管线缺失](#22-render-backend-gpu-着色器上屏管线缺失)
   - [2.3 `script-engine::eval`: 复杂运算符与方法链未完全展开](#23-script-engineeval-复杂运算符与方法链未完全展开)
   - [2.4 `dsl-parser`: 控制流 AST 与 @import 尚未合并](#24-dsl-parser-控制流-ast-与-import-尚未合并)
   - [2.5 `style-system`: calc() 动态混合计算未做运行时求解器](#25-style-system-calc-动态混合计算未做运行时求解器)
3. [对标 CSS / HTML / PHP 的核心差距分析](#三-对标-css--html--php-的核心差距分析)
4. [下一步详细补全计划 (To-Do Matrix)](#四-下一步详细补全计划-to-do-matrix)

---

## 一、 占位符与未补全细节全景矩阵

| 子模块 Crate | 当前实现状态 | 是否存在占位符 / 假数据 | 距离生产级真正还缺什么 |
| :--- | :--- | :--- | :--- |
| **`data-bridge`** | **部分为占位符** | ⚠️ **存在占位符** | `sqlite_local.rs` 仅用 `HashMap` 模拟表，SQL 只做了字符串前缀判断，未真正链接 `sqlite3.c` 原生 C 库与 B 树文件存储。 |
| **`render-backend`** | **指令骨架已就绪** | ⚠️ **缺少真正绘制驱动** | 拥有 `DrawCommand` 和 `DisplayList` 数据结构，但未写 WGPU / DirectX 12 顶点缓冲区、着色器管线（Pipeline State Object）向屏幕输出真实像素。 |
| **`script-engine`** | **基础求值可用** | ⚠️ **缺少完整 AST 解释器** | 支持 `$var` 模版插值、布尔翻转与简单字面量，但无法解析 `1 + 2 * 3` 算术优先级、无法执行 `for in` 树节点自动循环展开。 |
| **`dsl-parser`** | **基础括号 AST 可用** | ⚠️ **缺少高级语法** | 支持 `{}` 约束块与属性键值对，缺少 `@import` 导入指令、`component` 模版定义与 `if/else` 控制流 AST 生成。 |
| **`style-system`** | **静态计算值已就绪** | ⚠️ **缺少运行时动态求解** | 具备紧凑的 `ComputedStyle` 内存布局，但无法在排版阶段动态计算 `calc(100% - 20px)`。 |
| **`element-core`** | **基础虚拟节点可用** | ⚠️ **缺少表单全族控件** | 具备 `Box`、`Text`、`Button`、`Input`，但缺少 `Checkbox`、`Select` 下拉菜单与行内富文本 `Span` 混排节点。 |
| **`css-types`** | **生产级已就绪** | ✅ **无占位符** | Color (RGBA/Hex/Lerp)、Rect、Size、Dimension、Display 等枚举均为完备的纯数据结构。 |
| **`css-animation`** | **数学级已就绪** | ✅ **无占位符** | Cubic-Bezier 贝塞尔公式逼近、Color Oklab Lerp、Transition 状态机计算逻辑已完整实现。 |
| **`charset-compat`** | **基础转码可用** | ⚠️ **需扩充完整 GBK 码表** | 支持 BOM 探测、UTF-16LE/BE 批量解码，但 GBK 采用启发式探测，未内嵌 2 万+汉字完整 Unicode 映射表（需引入完整码表）。 |
| **`crypto-pack`** | **加密算法可用** | ✅ **无占位符** | FNV-1a 符号哈希脱敏、流式加解密变换与二进制字节码打包协议已完整可测。 |
| **`layout-engine`** | **桥接定义已就绪** | ⚠️ **依赖 Taffy 完整树求解** | 能够将 `ComputedStyle` 转译为 `taffy::Style`，但尚未打通虚拟 DOM 树向 Taffy NodeId 树的整树递归映射。 |
| **`live-runtime`** | **事件驱动就绪** | ⚠️ **缺少局部重绘调度器** | 具备 `DirtyMask` 与 `BracketEvent`，缺少与底层 OS 窗口事件循环的绑定通道。 |

---

## 二、 各模块占位细节逐项解剖

### 2.1 `data-bridge::sqlite_local`: 内存 Mock 占位
* **当前现状**：
  ```rust
  // 当前使用的是粗糙的字符串前缀模拟判断！
  if sql.to_uppercase().starts_with("INSERT") {
      Ok(1)
  }
  ```
* **差距**：这并不是真正的 SQL 执行器。它不能处理 `WHERE id = 5`，不能创建真实的 `.db` 文件，不能处理事物回滚（ACID）。
* **需补全**：引入真实的轻量级 SQLite C 绑定，或内嵌微型 B-Tree 存储引擎，支持参数化绑定 `?`。

### 2.2 `render-backend`: GPU 着色器上屏管线缺失
* **当前现状**：定义了 `DrawCommand::DrawRect`、`DrawText`、`PushClip`，但执行 `DisplayList` 时目前没有任何画笔能够把这些指令绘制在物理窗口上。
* **差距**：没有编写与显卡打交道的顶点着色器（Vertex Shader）和片元着色器（Fragment Shader）。
* **需补全**：接入 `wgpu` 或纯 Rust 软件光栅化渲染器（如 `tiny-skia` / `femtovg`），将矩形、阴影、字体点阵真正画入显存 Framebuffer。

### 2.3 `script-engine::eval`: 复杂运算符与方法链未完全展开
* **当前现状**：只支持简单的单值取值，例如 `$username`，不支持对象属性连续访问（如 `$task.author.name`），不支持四则混合运算。
* **差距**：缺少逆波兰表达式（Shunting-Yard）算法，无法处理带优先级的算术与逻辑表达式。
* **需补全**：实现简易 Pratt 语法解析器，支持方法调用（如 `$list.len()`、`$text.trim()`）。

---

## 三、 对标 CSS / HTML / PHP 的核心差距分析

### 1. 对标 CSS 缺失的核心能力：
1. **`calc()` 动态混合计算**：无法在排版阶段动态解析 `width: calc(100% - 30px)`。
2. **伪元素虚拟盒子 (`::before` / `::after`)**：无法直接通过样式在组件前后自动附着图标或红点徽章。
3. **文本流双阶段测量 (Intrinsic Sizing: `min-content` / `max-content`)**：自适应文本换行排版尚未形成测量反馈闭环。

### 2. 对标 HTML 缺失的核心能力：
1. **富文本内联切片混排 (Inline Spans)**：单行内无法实现“部分字加粗、部分字变红、中间插个超链接”。
2. **完整表单控件族**：缺少 `Checkbox`、`Radio`、`Select`、`Slider` 等现代 GUI 标配控件。
3. **全局键盘 Tab 焦点链**：无法纯靠键盘 Tab 键在各输入框之间流转焦点，缺少光标划选文本的渲染。

### 3. 对标 PHP 缺失的核心能力：
1. **跨文件模块导入 (`@import "header.ui"`)**：无法将庞大的界面拆分到多个小文件协同编写。
2. **内建实用函数标准库 (Built-in Stdlib)**：缺少类似 PHP 的字符串截取 (`substr`)、时间格式化 (`date`)、JSON 解析 (`json_parse`) 等内建工具箱。
3. **持久化存储 (Session / LocalStorage)**：缺乏配置一键存盘、下次开机自动读取的持久化层。

---

## 四、 下一步详细补全计划 (To-Do Matrix)

```
[阶段 A：扩展语法体系 (立即行动)]
  ├─ 1. 在 dsl-parser 中实现 @import、for in 循环与 if/else 分支 AST
  ├─ 2. 在 element-core 中补齐 Checkbox, Select, Option, Span 等控件原语
  └─ 3. 在 script-engine 中实现对象点访问 (task.title) 与标准函数 (len, trim)

[阶段 B：把核心占位符做实]
  ├─ 1. 将 sqlite_local 替换为真正的嵌入式单文件参数化执行引擎
  └─ 2. 在 script-engine 中实现 Pratt 表达式解析器，支持完整算术优先级运算

[阶段 C：底层光栅化上屏验证]
  └─ 编写渲染适配器，将 DisplayList 的 DrawCommand 转化为实际像素呈现
```
