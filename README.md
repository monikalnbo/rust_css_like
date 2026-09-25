# 🚀 Rust CSS-Like (`.ui`)

<div align="center">

**A unified, declarative frontend programming language and high-performance native UI engine built with Rust.**

[![CI - Workspace Check & Test](https://github.com/monikalnbo/rust_css_like/actions/workflows/ci.yml/badge.svg)](https://github.com/monikalnbo/rust_css_like/actions/workflows/ci.yml)
[![Cross-Platform Native Build](https://github.com/monikalnbo/rust_css_like/actions/workflows/build-native.yml/badge.svg)](https://github.com/monikalnbo/rust_css_like/actions/workflows/build-native.yml)
[![WebAssembly Live Playground](https://img.shields.io/badge/WASM_Playground-Online-brightgreen)](https://monikalnbo.github.io/rust_css_like/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[English](README.md) | [简体中文](README_zh.md)

---

### *No HTML closing tag fatigue. No CSS cascading conflicts. No JS/Redux boilerplate.*

[🌐 Live WASM Playground](https://monikalnbo.github.io/rust_css_like/) • [📘 Language Reference Manual](docs/LANGUAGE_GUIDE.md) • [💻 Download Releases (.exe/.dmg)](https://github.com/monikalnbo/rust_css_like/actions) • [📚 Architecture Whitepaper](docs/01_REQUIREMENTS_AND_ARCHITECTURE.md)

</div>

---

## 📖 Table of Contents
1. [Why Another Frontend Language?](#-why-another-frontend-language)
2. [How It Works Under the Hood](#-how-it-works-under-the-hood)
3. [Language Tour & Syntax Guide](#-language-tour--syntax-guide)
4. [Real-World Examples](#-real-world-examples)
5. [How to Run & Deploy](#-how-to-run--deploy)
6. [Extensibility & Plugin Bus](#-extensibility--plugin-bus)
7. [Architecture Matrix (15 Crates)](#-architecture-matrix-15-crates)
8. [Documentation Index](#-documentation-index)

---

## 💡 Why Another Frontend Language?

Modern frontend engineering (HTML5, CSS3, JavaScript/TypeScript, React/Vue, Electron) has accumulated tremendous friction over the last two decades:

| Pain Points in Traditional Stack | How Rust CSS-Like (`.ui`) Solves It |
| :--- | :--- |
| **Cognitive Fragmentation**: Switching between HTML structure, CSS rules, JS hooks, and JSX files. | **Unified Language Paradigm**: Markup, styles, and reactive actions live in one clean syntax with `{}` scopes. |
| **Closing Tag Fatigue**: Infinite `<div></div>` and `</span>` trees. | **Zero Closing Tags**: Pure curly braces `{}` delineate scope naturally. |
| **CSS Naming Anxiety**: BEM, CSS Modules, utility-first classes, specificity overrides. | **Inline Shortened Properties**: `pad=16 bg=#1e293b rad=8 flex=1` written directly on elements. |
| **Bloated Runtimes**: Electron desktop apps start at 150MB+ and consume 100MB+ RAM. | **Ultralight & Blazingly Fast**: Pure native binary (~1.2 MB), <15ms cold start, only 10MB~25MB RAM. |
| **No Native Backend Connectivity**: Requires separate Node/REST/GraphQL backends. | **Direct Embedded Database**: Direct SQL queries (`db.query`) right inside your UI file. |

```scss
// A complete, reactive, cross-platform desktop application in 12 lines
let count = 0

win "Counter App" (400, 300) bg=#0f172a {
    col pad=32 gap=16 align=center justify=center flex=1 {
        txt "Counter: $count" #f8fafc 24px bold
        row gap=12 {
            btn "Decrease (-1)" pad=(8, 16) bg=#334155 rad=6 -> count -= 1
            btn "Increase (+1)" pad=(8, 16) bg=#4f46e5 rad=6 -> count += 1
        }
    }
}
```

---

## ⚙️ How It Works Under the Hood

The engine is engineered as a **5-Layer Strictly Unidirectional DAG (Directed Acyclic Graph)** microkernel. Here is how your `.ui` source code transforms into physical pixels:

```
┌────────────────────────────────────────────────────────────────────────┐
│                        Phase 1: Lexing & AST Parsing                   │
│   Source (.ui) ──► dsl-parser (Bracket State Machine) ──► Scope AST    │
└───────────────────────────────────┬────────────────────────────────────┘
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                        Phase 2: Virtual DOM & Styling                  │
│   Scope AST ──► element-core (Virtual DOM) ──► style-system (Cascading)│
└───────────────────────────────────┬────────────────────────────────────┘
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                        Phase 3: Geometry & Text Layout                 │
│   layout-engine (Taffy 0.7 Flexbox) ──► text-layout (cosmic-text)      │
└───────────────────────────────────┬────────────────────────────────────┘
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                        Phase 4: DisplayList Assembly                   │
│   render-backend: Collects DrawCommands (Rects, Text, Shadows, Clips)   │
└───────────────────────────────────┬────────────────────────────────────┘
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                        Phase 5: Multi-Target Output                    │
│   Desktop (tiny-skia / winit)  │  Browser (WebAssembly / HTML5 Canvas) │
└────────────────────────────────────────────────────────────────────────┘
```

1. **Parser Layer (`dsl-parser`)**: Tokenizes the code into tokens, tracks `{}` depths with a state machine, and builds a hierarchical `ScopeBlock` AST.
2. **DOM & Style Layer (`element-core` & `style-system`)**: Instantiates the virtual node tree and resolves 7-level CSS cascading priority into compact `ComputedStyle` structs.
3. **Geometry & Typography (`layout-engine` & `text-layout`)**: Bridges `ComputedStyle` into Taffy Flexbox layout, computing pixel-perfect `LayoutRect`s, while `cosmic-text` shapes UTF-8 text with word-wrapping and CJK full-width metrics.
4. **Drawing Pipeline (`render-backend`)**: Emits hardware-agnostic `DrawCommand` primitives into an immutable `DisplayList`.
5. **Rasterization (`app-shell` / `wasm-runtime`)**: Renders pixels directly to physical framebuffers via `tiny-skia` on desktop or HTML5 `<canvas>` via WebAssembly.

---

## 🎨 Language Tour & Syntax Guide

### 1. Window & Layout Containers
* `win "Title" (Width, Height)`: Top-level native window.
* `row`: Horizontal flexbox flow (left-to-right).
* `col`: Vertical flexbox flow (top-to-bottom).
* `box`: Generic box-model container (margin, padding, border, radius, shadow).
* `card`: Preset card element with shadow and rounded styling.

### 2. Built-in Shortened Styles (CSS Aligned)
* **Spacing**: `pad=16` (padding), `margin=12`, `gap=10` (flex item spacing).
* **Sizing**: `w=200`, `h=48`, `w=100%`, `flex=1` (flex-grow).
* **Alignment**: `align=center|start|end` (align-items), `justify=between|center|around` (justify-content).
* **Visuals**: `bg=#1e293b` (background), `rad=8` (border-radius), `border=(1, #334155)`, `opacity=0.9`.
* **Typography**: `#hex` (color), `16px` (font-size), `bold`, `italic`.

### 3. Reactive State (`let`) & Event Streams (`->`)
Variables declared with `let` are automatically reactive:
```scss
let is_dark = true
let username = "Alice"

// Template interpolation with $var
txt "Welcome, $username!" (is_dark ? #f8fafc : #0f172a)

// '->' arrow leads user actions directly
btn (is_dark ? "Light Mode" : "Dark Mode") -> is_dark = !is_dark
```

### 4. Control Flow: Loops & Conditions
```scss
col gap=8 {
    // List rendering
    for task in tasks {
        row pad=12 bg=#1e293b rad=6 justify=between {
            txt task.title #fff 14px
            btn "Done" -> task.done = true
        }
    }

    // Conditional branches
    if len(tasks) == 0 {
        txt "No pending tasks." #94a3b8 12px
    }
}
```

### 5. Reusable Components & Imports
```scss
component StatCard(title, value, color) {
    col pad=16 bg=#1e293b rad=8 gap=6 flex=1 {
        txt title #94a3b8 12px
        txt value color 20px bold
    }
}

// In main view:
row gap=12 {
    StatCard(title="Active Users", value="12,480", color=#22c55e)
    StatCard(title="Server Load", value="18.2%", color=#3b82f6)
}
```

---

## 💡 Real-World Examples

All working code is available in the [`examples/`](examples/) directory:

1. **[examples/counter.ui](examples/counter.ui)**: Minimal 15-line counter with bidirectional reactive flow.
2. **[examples/todo_app.ui](examples/todo_app.ui)**: Todo list with dynamic list additions, two-way input binding (`bind=text`), and item completion.
3. **[examples/components_demo.ui](examples/components_demo.ui)**: Custom component declarations and theme color reusability.
4. **[examples/app.ui](examples/app.ui)**: Full commercial application with SQLite integration, navigation bars, and GPU ripple shaders.

---

## 🚀 How to Run & Deploy

### Option 1: Instant Browser Sandbox (WebAssembly)
No local installation required! Open in your browser:  
👉 **[https://monikalnbo.github.io/rust_css_like/](https://monikalnbo.github.io/rust_css_like/)**

### Option 2: Download Precompiled Native Binaries
GitHub Actions automatically cross-compiles release binaries on every commit:  
👉 Go to **[GitHub Actions Releases](https://github.com/monikalnbo/rust_css_like/actions)**, select the latest **Cross-Platform Native Build**, and download from **Artifacts**:
* `windows-x64-executable.zip`: Standalone Windows `.exe`.
* `macos-universal-executable.tar.gz`: Universal macOS binary (Apple Silicon M1~M4 + Intel).
* `linux-x64-executable.tar.gz`: Linux x86_64 standalone binary.

### Option 3: Local Compilation (Rust 1.75+)
```bash
# 1. Clone repository
git clone https://github.com/monikalnbo/rust_css_like.git
cd rust_css_like

# 2. Run full workspace test suite (30 unit & integration tests, 100% passing)
cargo test --workspace

# 3. Launch native desktop shell
cargo run -p app-shell
```

---

## 🔌 Extensibility & Plugin Bus

The engine provides 7 low-level registration slots for deep enterprise customization without modifying the core:

| Slot | Trait | Use Case | `.ui` Syntax |
| :---: | :--- | :--- | :--- |
| **1** | `CustomPainter` | Mount custom GPU shaders (ripples, blurs, particles) | `card effect="InteractiveRipple"` |
| **2** | `CustomComponentDriver` | Register 3D viewports, video players, or charts | `plugin "CustomChart"` |
| **3** | `CustomLayoutStrategy` | Implement non-standard layouts (Waterfall, Radial) | `box display="waterfall"` |
| **4** | `CustomPropertyHandler` | Extend new CSS properties with Lerp animation interpolation | `glow-speed=2.5` |
| **5** | `NativeHostFn` | Expose OS capabilities (File I/O, Clipboard, System Tray) | `btn -> fs.read("data.json")` |
| **6** | `StorageDriverFactory` | Connect DuckDB, RocksDB, Redis, or IPC shared memory | `db.connect("duckdb://...")` |
| **7** | `AssetProtocolLoader` | Decrypt and load proprietary assets and stylesheets | `@import "pak://secure.ui"` |

---

## 📦 Architecture Matrix (15 Crates)

```
crates/
├── css-types/          # Pure POD foundation types (Color, Rect, Length, Dimension)
├── charset-compat/     # BOM stripping, GBK/UTF-16 transcoder, CJK full-width metrics
├── crypto-pack/        # AOT .binui bytecode, symbol obfuscation, stream crypto
├── dsl-parser/         # Lexer, bracket tracker, AST with control flow & components
├── element-core/       # ElementTree, NodeId, bitflags interactive state masks
├── style-system/       # 7-level specificity cascade, computed styles, variable table
├── css-animation/      # Cubic bezier easing, Lerp trait, transition state machine
├── layout-engine/      # Taffy 0.7 flexbox bridge, LayoutRect geometric solver
├── text-layout/        # cosmic-text font shaping, word-wrapping, paragraph layout
├── render-backend/     # Hardware-agnostic DisplayList, DrawCommand, DPI scaling
├── live-runtime/       # DirtyMask (Repaint, Relayout, Restructure), event hub
├── script-engine/      # Inline expression evaluator, scope chain, plugin registry
├── data-bridge/        # SQLite integration, reactive Signal<T>, C-ABI exports
├── wasm-runtime/       # Canvas 2D WebAssembly backend for browser execution
└── app-shell/          # Desktop runtime shell using winit 0.29 & tiny-skia
```

---

## 📚 Documentation Index

* 📘 [Language Reference Manual (`LANGUAGE_GUIDE.md`)](docs/LANGUAGE_GUIDE.md) — Comprehensive syntax specification.
* 📚 [Architecture Blueprint (`01_REQUIREMENTS_AND_ARCHITECTURE.md`)](docs/01_REQUIREMENTS_AND_ARCHITECTURE.md) — Architectural design and pipeline.
* 🌐 [HTML/CSS Alignment Specification (`03_HTML_CSS_PHP_ALIGNMENT_SPEC.md`)](docs/03_HTML_CSS_PHP_ALIGNMENT_SPEC.md) — Web standards alignment.
* 🔌 [Plugin System Guide (`04_EXTENSIBILITY_AND_PLUGIN_SYSTEM.md`)](docs/04_EXTENSIBILITY_AND_PLUGIN_SYSTEM.md) — Guide for custom extensions.

---

## 📄 License
This project is open-source under the [MIT License](LICENSE). Contributions, feedback, and GitHub Stars are warmly welcome!
