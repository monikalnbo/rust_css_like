# 🚀 Rust CSS-Like (`.ui`) — 专为高性能原生与 Web 打造的声明式前端语言

<div align="center">

[![CI - Workspace Check & Test](https://github.com/monikalnbo/rust_css_like/actions/workflows/ci.yml/badge.svg)](https://github.com/monikalnbo/rust_css_like/actions/workflows/ci.yml)
[![Cross-Platform Native Build](https://github.com/monikalnbo/rust_css_like/actions/workflows/build-native.yml/badge.svg)](https://github.com/monikalnbo/rust_css_like/actions/workflows/build-native.yml)
[![WebAssembly Live Playground](https://img.shields.io/badge/WASM_Playground-Online-brightgreen)](https://monikalnbo.github.io/rust_css_like/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**一个将「标记结构、紧凑样式、响应式逻辑」三位一体的原生级前端编程语言与多端渲染引擎。**  
*没有 HTML 的标签闭合地狱，没有 CSS 的类名命名焦虑，没有 JS/Redux 的沉重样板代码。*

[🌐 在线交互演练场](https://monikalnbo.github.io/rust_css_like/) • [📘 语言参考手册](docs/LANGUAGE_GUIDE.md) • [💻 预编译发布包下载](https://github.com/monikalnbo/rust_css_like/actions) • [📚 架构全景白皮书](docs/01_REQUIREMENTS_AND_ARCHITECTURE.md)

</div>

---

## 目录
1. [为什么要用它写前端？（语言诞生哲学）](#一-为什么要用它写前端语言诞生哲学)
2. [3 分钟语法极速上手 (Hello World)](#二-3-分钟语法极速上手-hello-world)
3. [核心语法体系剖析](#三-核心语法体系剖析)
   - [3.1 布局与标签原语 (`win`, `row`, `col`, `box`, `card`)](#31-布局与标签原语)
   - [3.2 极简样式属性表 (对齐 CSS)](#32-极简样式属性表)
   - [3.3 响应式状态与动作流 (`let`, `->`)](#33-响应式状态与动作流)
   - [3.4 控制流：条件与列表 (`if`, `for`)](#34-控制流条件与列表)
   - [3.5 模版组件与跨文件导入 (`component`, `@import`)](#35-模版组件与跨文件导入)
   - [3.6 全栈数据库与物理特效 (`db.query`, `effect`)](#36-全栈数据库与物理特效)
4. [三种多端运行与体验途径](#四-三种多端运行与体验途径)
5. [完整文档与索引清单 (Docs & Examples)](#五-完整文档与索引清单)
6. [底层 12 大微内核引擎矩阵 (Engine Internals)](#六-底层-12-大微内核引擎矩阵)

---

## 一、 为什么要用它写前端？（语言诞生哲学）

传统前端技术栈（HTML5 + CSS3 + JavaScript / TypeScript + React / Vue / Electron）历经数十年演化，带来了极其沉重的包袱：
1. **多语言认知割裂**：开发者必须在 HTML（结构）、CSS（样式）、JS/TS（逻辑）与 JSX 之间反复切换心智模型；
2. **闭合标签冗余**：面对无穷无尽的 `<div></div>`、`<span></span>` 闭合地狱；
3. **样式组织困扰**：BEM 命名规范、CSS Modules、Tailwind 冗长类名等层出不穷，却始终面临样式层叠覆盖不可控的风险；
4. **运行时庞大迟钝**：一个简单的计算器桌面应用，打包 Electron 后动辄 150MB~300MB，内存占用高达 100MB+，启动延迟 500ms 以上。

**Rust CSS-Like (`.ui`) 将前端开发回归至纯粹的声明式语言本源**：

```scss
// 这就是全部！没有 HTML 头，没有 CSS 文件，没有 npm 安装，毫秒级响应
let count = 0

win "极简原生应用" (400, 300) bg=#0f172a {
    col pad=32 gap=16 align=center justify=center flex=1 {
        txt "当前计数: $count" #f8fafc 24px bold
        row gap=12 {
            btn "减少 (-1)" pad=(8,16) bg=#334155 rad=6 -> count -= 1
            btn "增加 (+1)" pad=(8,16) bg=#4f46e5 rad=6 -> count += 1
        }
    }
}
```

* **体积仅 ~1.2 MB**：纯原生 Rust 编译产物，零 Chromium/Node.js 冗余！
* **启动仅 < 15 ms**：毫秒级直达物理屏幕，内存占用仅 10MB~25MB！
* **全平台统一分发**：一套 `.ui` 代码，一键出 Windows `.exe`、macOS `.app`、Linux 及 WebAssembly！

---

## 二、 3 分钟语法极速上手 (Hello World)

创建一个 `hello.ui` 文件：

```scss
let message = "你好，Rust CSS-Like 世界！"
let is_dark = true

win "我的首个前端应用" (500, 360) bg=(is_dark ? #0f172a : #f8fafc) {
    col pad=30 gap=20 align=center justify=center flex=1 {
        // 1. 文本图元展示响应式状态
        txt "$message" (is_dark ? #f8fafc : #0f172a) 20px bold

        // 2. 交互按钮与动作流绑定
        row gap=12 {
            btn (is_dark ? "切换为浅色" : "切换为深色") pad=(8, 16) bg=#4f46e5 rad=8 #fff -> {
                is_dark = !is_dark
            }
            btn "点赞" pad=(8, 16) bg=#22c55e rad=8 #fff -> {
                message = "感谢你的支持！❤️"
            }
        }
    }
}
```

---

## 三、 核心语法体系剖析

### 3.1 布局与标签原语
语言提供对齐现代化 Flexbox 的极简容器标签，所有嵌套以 `{}` 自动界定作用域：
* `win "标题" (宽, 高)`：定义原生顶层主窗口；
* `row`：横向弹性流布局（子项由左向右排布）；
* `col`：纵向弹性流布局（子项自上而下排布）；
* `box`：通用盒模型容器（支持绝对/相对定位、圆角与边框）；
* `card`：预设阴影质感的独立展示卡片；
* `txt "内容"`：高性能排版文本图元；
* `btn "按钮"`：内置物理交互动效的按钮控件；
* `inp placeholder="提示"`：带双向状态绑定的输入控件。

### 3.2 极简样式属性表
告别繁琐的 CSS 语法，样式属性直接以内联参数方式书写：
```scss
// 内边距 16px、外边距 12px、子项间距 10px、圆角 8px、背景色 #1e293b
box pad=16 margin=12 gap=10 rad=8 bg=#1e293b
```
* **盒模型度量**：`pad` (内边距), `margin` (外边距), `gap` (子间距), `w` (宽度), `h` (高度), `flex` (弹性占用比例)；
* **对齐方式**：`align=center|start|end` (交叉轴), `justify=between|center|around` (主轴)；
* **视觉渲染**：`bg` (背景色), `rad` (圆角半径), `border` (边框), `opacity` (不透明度), `shadow` (盒阴影)；
* **文本排版**：`#hex` (字体颜色), `16px` (字号), `bold` (加粗), `italic` (斜体)。

### 3.3 响应式状态与动作流
* **状态定义**：以 `let` 关键字声明，自动加入全局/局部响应式依赖图谱：
  ```scss
  let username = "管理员"
  let is_logged_in = false
  ```
* **动作流引导 (`->`)**：用户交互由单向箭头 `->` 触发，直接执行轻量脚本语句：
  ```scss
  btn "登录" -> is_logged_in = true
  ```

### 3.4 控制流：条件与列表
语言在标记内部原生支持 `if` 和 `for` 语句，直接与 UI 树深度融合：
```scss
col gap=8 {
    // 列表循环展开
    for item in tasks {
        row pad=12 bg=#1e293b rad=6 justify=between {
            txt item.title #fff 14px
            btn "完成" -> item.done = true
        }
    }

    // 条件分支渲染
    if len(tasks) == 0 {
        txt "暂无任何待办任务" #94a3b8 12px
    }
}
```

### 3.5 模版组件与跨文件导入
* **组件声明**：使用 `component` 封装高复用控件：
  ```scss
  component StatCard(title, value, color) {
      col pad=16 bg=#1e293b rad=8 gap=6 flex=1 {
          txt title #94a3b8 12px
          txt value color 20px bold
      }
  }
  ```
* **跨文件导入**：使用 `@import` 组织大型前端工程：
  ```scss
  @import "components/header.ui";
  @import "components/sidebar.ui";
  ```

### 3.6 全栈数据库与物理特效
* **前端直连 SQLite**：
  ```scss
  let tasks = db.query("SELECT id, title FROM tasks ORDER BY id DESC")
  btn "添加" -> db.execute("INSERT INTO tasks (title) VALUES (?)", [new_title])
  ```
* **挂载底层物理光晕特效**：
  ```scss
  // 鼠标移动或点击时，底层自动将物理坐标交由 GPU 计算产生真实水波扩散
  card pad=16 bg=#1e293b rad=8 effect="InteractiveRipple" {
      txt "点击卡片感受物理光晕水波" #f8fafc
  }
  ```

---

## 四、 三种多端运行与体验途径

### 1. 🌐 WebAssembly 在线演练场（免安装秒开）
无需配置任何本地环境，直接访问 GitHub Pages 在线沙盒：  
👉 **[https://monikalnbo.github.io/rust_css_like/](https://monikalnbo.github.io/rust_css_like/)**

### 2. 💻 下载预编译原生可执行包（Windows / macOS / Linux）
GitHub Actions 会在每次提交时，全自动交叉编译各平台发布包：  
👉 前往 **[GitHub Actions 页面](https://github.com/monikalnbo/rust_css_like/actions)**，点击最新的 **Cross-Platform Native Build**，在页面底部的 **Artifacts** 区域即可一键下载：
* `windows-x64-executable.zip`：包含单文件 Windows 原生 `.exe`
* `macos-universal-executable.tar.gz`：包含支持 Apple Silicon (M1/M2/M3/M4) 与 Intel 的 Universal 运行包
* `linux-x64-executable.tar.gz`：Linux x86_64 原生独立可执行文件

### 3. 🛠️ 本地编译与开发
```bash
# 1. 克隆本仓库
git clone https://github.com/monikalnbo/rust_css_like.git
cd rust_css_like

# 2. 执行整个工程全量单元测试 (30 项单元测试 100% 通过)
cargo test --workspace

# 3. 本地启动 App 渲染主外壳
cargo run -p app-shell
```

---

## 五、 完整文档与索引清单

| 文档 / 示例 | 路径 | 核心定位与说明 |
| :--- | :--- | :--- |
| 📘 **前端语言完全手册** | [docs/LANGUAGE_GUIDE.md](docs/LANGUAGE_GUIDE.md) | **必读**：完整的 `.ui` 语法字典、选择器、控制流、标准库函数与组件规约 |
| 📚 **系统架构设计蓝图** | [docs/01_REQUIREMENTS_AND_ARCHITECTURE.md](docs/01_REQUIREMENTS_AND_ARCHITECTURE.md) | 深入五层微内核设计、数据流水线模型与无环依赖规范 |
| 🌐 **HTML/CSS 对齐规范** | [docs/03_HTML_CSS_PHP_ALIGNMENT_SPEC.md](docs/03_HTML_CSS_PHP_ALIGNMENT_SPEC.md) | 虚拟 DOM 状态掩码、`cosmic-text` 富文本与 `calc()` 求解 |
| 🔌 **底层扩展与插件指南** | [docs/04_EXTENSIBILITY_AND_PLUGIN_SYSTEM.md](docs/04_EXTENSIBILITY_AND_PLUGIN_SYSTEM.md) | 七大注册总线（GPU 着色器、自定义节点、外接存储驱动）插件开发指南 |
| 💡 **实战应用 1：完整工作台** | [examples/app.ui](examples/app.ui) | 包含导航栏、SQLite 数据库、水波特效与任务管理的工业级示例 |
| 💡 **实战应用 2：极简计数器** | [examples/counter.ui](examples/counter.ui) | 极简 15 行代码演示单向响应式流 |
| 💡 **实战应用 3：待办事项清单** | [examples/todo_app.ui](examples/todo_app.ui) | 列表循环、输入框双向数据流与状态切换 |
| 💡 **实战应用 4：组件化仪表盘** | [examples/components_demo.ui](examples/components_demo.ui) | 自定义 `component` 封装与主题色彩复用 |

---

## 六、 底层 12 大微内核引擎矩阵

整个引擎采用**严格自底向上的单向无环依赖（Strict DAG）**设计，各模块结构严整：

```
crates/
├── css-types/          # 1. 基础纯 POD 数据类型（Color, Rect, Length, Dimension）
├── charset-compat/     # 2. 字符集兼容层（自动剥离 BOM，GBK/UTF-16 互转，CJK 全角度量）
├── crypto-pack/        # 3. 商业安全层（AOT 字节码封装 .binui，符号哈希脱敏，内存解密）
├── dsl-parser/         # 4. 流式语法分析层（大括号状态机，Token 流，无环 AST 构建）
├── element-core/       # 5. 虚拟 DOM 树（ElementTree，节点位掩码交互状态机）
├── style-system/       # 6. 样式计算层（7级特异度打分，动态变量表，计算样式分流）
├── css-animation/      # 7. 数学动效层（三次贝塞尔插值，通用数值/色彩补间状态机）
├── layout-engine/      # 8. 几何排版层（基于纯 Rust Taffy 0.7 算法求解绝对物理坐标）
├── render-backend/     # 9. 渲染指令流（DisplayList 硬件无关图元，DPI 视网膜缩放，IME 锚点）
├── live-runtime/       # 10. 热重载与脏标记（三级刷新 DirtyMask，避免昂贵全树重排）
├── script-engine/      # 11. 微型脚本引擎（表达式求值，变量作用域链，原生插件扩展槽）
├── data-bridge/        # 12. 全栈数据库与 FFI（SQLite 本地表，Signal 响应式信号，C-ABI 导出）
├── text-layout/        # 13. 高性能文本塑形与换行（集成 cosmic-text 原生字形度量）
├── wasm-runtime/       # 14. 浏览器 WebAssembly 运行时（导出 Canvas 2D 绘图后端）
└── app-shell/          # 15. 原生桌面主运行外壳（基于 winit 0.29 与 tiny-skia 软光栅）
```

---

## 许可证
本项目基于 [MIT 许可证](LICENSE) 开源。欢迎 Star 与贡献代码！
