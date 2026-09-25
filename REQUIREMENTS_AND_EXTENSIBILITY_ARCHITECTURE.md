# Rust CSS-Like 核心需求规格说明书与高拓展底层注册架构规范

> **定位**：本文档系统化阐述 `rust_css_like` 的全景需求规格（功能性、非功能性、全栈对齐），并重点设计一套**“高内聚、高拓展、支持底层深度介入注册（Low-Level Registration Bus）”**的插件化引擎架构，确保引擎不仅开箱可用，更能被专业开发者无缝扩展至 3D 渲染、自研着色器、原生硬件控制及自定义语言宿主中。

---

## 目录
1. [系统需求全景规格 (Requirements Specification)](#一-系统需求全景规格-requirements-specification)
   - [1.1 核心功能性需求 (Functional Requirements)](#11-核心功能性需求-functional-requirements)
   - [1.2 性能与非功能性需求 (Performance & Non-Functional)](#12-性能与非功能性需求-performance--non-functional)
   - [1.3 HTML / CSS / PHP 深度对齐需求](#13-html--css--php-深度对齐需求)
2. [高拓展性底层注册架构设计 (Extensibility Architecture)](#二-高拓展性底层注册架构设计-extensibility-architecture)
   - [2.1 统一引擎插件总线 (EnginePlugin & RegistryBus)](#21-统一引擎插件总线-engineplugin--registrybus)
   - [2.2 七大底层注册槽位 (The 7 Low-Level Registration Slots)](#22-七大底层注册槽位-the-7-low-level-registration-slots)
3. [七大底层注册槽位详细设计与 Trait 规约](#三-七大底层注册槽位详细设计与-trait-规约)
   - [槽位 1：底层 GPU 着色器与自定义绘制注册 (CustomPainter & Shader Pass)](#槽位-1底层-gpu-着色器与自定义绘制注册-custompainter--shader-pass)
   - [槽位 2：自定义底层组件与虚拟节点注册 (Custom Node & Component Plugin)](#槽位-2自定义底层组件与虚拟节点注册-custom-node--component-plugin)
   - [槽位 3：自定义布局算法注册 (Custom Layout Strategy)](#槽位-3自定义布局算法注册-custom-layout-strategy)
   - [槽位 4：自定义样式属性与插值器注册 (Custom Style Property & Interpolator)](#槽位-4自定义样式属性与插值器注册-custom-style-property--interpolator)
   - [槽位 5：脚本宿主原生函数与全局对象注册 (Host Functions & Native Objects)](#槽位-5脚本宿主原生函数与全局对象注册-host-functions--native-objects)
   - [槽位 6：自定义数据库与存储驱动注册 (Database & Storage Drivers)](#槽位-6自定义数据库与存储驱动注册-database--storage-drivers)
   - [槽位 7：自定义 URI 资源加载器协议注册 (Custom Protocol & Asset Loader)](#槽位-7自定义-uri-资源加载器协议注册-custom-protocol--asset-loader)
4. [开发者端到端接入实战范例](#四-开发者端到端接入实战范例)
   - [案例：注册一个带物理交互的自研 3D 视口组件与原生函数](#案例注册一个带物理交互的自研-3d-视口组件与原生函数)

---

## 一、 系统需求全景规格 (Requirements Specification)

### 1.1 核心功能性需求 (Functional Requirements)

1. **超精炼声明式 DSL**：
   - 彻底摒弃 HTML 的样板文件头（`<!DOCTYPE>`, `<html>`, `<head>`, `<body>`）；
   - 统一采用 `{}` 作为唯一的作用域与约束符；
   - 融合标记（Markup）、样式（Style）、响应式数据绑定（`let`）与动作流逻辑（`->`）。
2. **增量编译与即时热重载 (Live Runtime)**：
   - 监听编辑器输入流中的 `{ ... }` 闭合事件，单次作用域重解析延迟 `< 0.2ms`；
   - 支持三级脏标记：纯视觉变化仅重绘（Repaint `< 1ms`），几何盒模型变化仅重排（Relayout `< 3ms`），树形增删才重构（Restructure）。
3. **底层渲染与高分屏 (Render Backend)**：
   - 生成紧凑无状态绘制指令队列（`DisplayList`）；
   - 支持物理像素与逻辑像素的动态 DPI 视网膜缩放（Retina / 4K / 8K）；
   - 支持多层级 Z-Index 树形点击命中测试；
   - 原生输入法（IME）拼音浮动候选窗物理锚点精准对齐。
4. **全字符集与国际化容错 (Charset Compat)**：
   - 自动识别并剥除 UTF-8 BOM、UTF-16LE/BE；
   - Windows 经典 GBK/GB18030 字符自动探测与容错转码，彻底消除乱码；
   - 消除 CJK（中日韩汉字及全角标点）等宽与非等宽排版时的光标错位。
5. **商业安全防逆向 (Crypto Pack)**：
   - AOT 离线编译为紧凑二进制字节码（`.binui`，带 `RCSS` 校验魔数）；
   - 符号哈希脱敏：明文类名/变量名映射为不可逆的 FNV-1a 64位哈希（如 `_0x8f2a...`）；
   - 动态流加密：仅在 RAM 运行内存堆中解密执行，硬盘零明文临时文件。
6. **跨语言互操作 (Polyglot Bridge)**：
   - 输出标准 C-ABI 头文件与动态链接库（`.h`, `.dll`, `.so`, `.dylib`）；
   - 原生打通 Python、Node.js、C++、Go 与 C#。

---

### 1.2 性能与非功能性需求 (Performance & Non-Functional)

1. **极致轻量（终结 Electron 内存噩梦）**：
   - 纯引擎静态二进制体积 `< 15MB`；
   - 空载冷启动内存消耗 `< 25MB`，彻底告别 Chrome 内核 200MB+ 的内存包袱。
2. **全帧率极速渲染**：
   - 稳定运行于 60 FPS / 120 FPS 高刷显示屏；
   - 每帧渲染耗时控制在 `8ms` 以内，动画贝塞尔插值零卡顿。
3. **跨平台零本地环境依赖**：
   - 支持 Windows x64、macOS (Apple Silicon & Intel) 原生运行；
   - 支持编译至 WebAssembly (WASM)，在浏览器中通过 `<canvas>` 零安装实时演练。

---

### 1.3 HTML / CSS / PHP 深度对齐需求

| 对齐维度 | 核心需求指标 |
| :--- | :--- |
| **对标 HTML** | ① 完整的表单控件族（Checkbox 勾选态、Radio 互斥组、Select 下拉遮罩、Slider 滑块）；<br>② 行内富文本混合排版（同一行内部分文字加粗、变色、插入内嵌图标与链接）；<br>③ 完整的事件流（捕获、目标、冒泡、`stopPropagation`）；<br>④ 全局键盘 Tab 焦点链与光标划选。 |
| **对标 CSS** | ① 多维选择器引擎（类选择器 `.btn`、ID `#header`、属性 `[type=text]`、层级 `>`）；<br>② `calc(100% - 30px)`、`min()`、`max()`、`clamp()` 动态数学求解；<br>③ 伪元素虚拟盒子（`::before`、`::after`）；<br>④ 局部层叠上下文（Stacking Context）与作用域变量继承。 |
| **对标 PHP** | ① 真正的 `@import "path/header.ui"` 磁盘递归解析与模块缓存；<br>② 组件宏实参展开（`component Card(title) { ... }` 能够被实例化调用并传参）；<br>③ 开箱即用的内置标准库（字符串、数组过滤映射、日期格式化、JSON 编解码）；<br>④ 真正的单文件持久化存储（Session / LocalStorage）与参数化预编译 SQLite 驱动。 |

---

## 二、 高拓展性底层注册架构设计 (Extensibility Architecture)

为了保证系统的工业级生命力，引擎采用**“内核最小化，能力总线化（Microkernel + Bus Architecture）”**的设计哲学。

```
┌────────────────────────────────────────────────────────────────────────┐
│                        Rust CSS-Like 核心调度总线                       │
│                           (RegistryBus)                                │
├───────────┬───────────┬───────────┬───────────┬───────────┬────────────┤
│ 1. 渲染   │ 2. 节点   │ 3. 布局   │ 4. 样式   │ 5. 脚本   │ 6. 存储    │
│  Shader/  │  Custom   │  Layout   │  Property/│  Host Fn/ │  Database  │
│  Painter  │  Element  │  Strategy │  Lerp     │  Objects  │  Driver    │
└─────▲─────┴─────▲─────┴─────▲─────┴─────▲─────┴─────▲─────┴─────▲──────┘
      │           │           │           │           │           │
      └───────────┴───────────┴─────┬─────┴───────────┴───────────┘
                                    │
                         ┌──────────┴──────────┐
                         │   EnginePlugin 协议  │
                         └─────────────────────┘
                                    ▲
                   ┌────────────────┴────────────────┐
            [自研 3D 视口插件]              [自研图表 EChart 插件]
            [自研 SQLite/DuckDB]            [自研 WGSL 毛玻璃着色器]
```

### 2.1 统一引擎插件总线 (EnginePlugin & RegistryBus)

开发者可以通过实现 `EnginePlugin`，在引擎启动生命周期中一次性注入所有底层的自定义能力：

```rust
pub trait EnginePlugin: Send + Sync {
    /// 插件唯一名称
    fn name(&self) -> &str;
    /// 插件初始化：向引擎总线注册所有自定义能力
    fn register(&self, bus: &mut RegistryBus);
}
```

---

## 三、 七大底层注册槽位详细设计与 Trait 规约

### 槽位 1：底层 GPU 着色器与自定义绘制注册 (CustomPainter & Shader Pass)

允许开发者绕过引擎内置的简单边框/矩形，直接挂载自研的硬件着色器或复杂的 CPU 矢量画板。

```rust
use render_backend::{DisplayList, LayoutRect};

/// 开发者自研底层着色器/绘图器 Trait
pub trait CustomPainter: Send + Sync {
    /// 绘图器标识符，在 DSL 中直接使用 `effect="MyPainter"` 挂载
    fn name(&self) -> &str;
    
    /// 接收真实底层物理硬件交互（鼠标滑动坐标、点击）
    fn on_event(&mut self, event: &render_backend::InteractionEvent, bounds: LayoutRect);
    
    /// 每帧物理模拟时钟推进（如流光频率、水波扩散）
    fn update(&mut self, dt_secs: f32);
    
    /// 向 DisplayList 提交底层 GPU 着色器 Pipeline 指令或 Uniform 变量
    fn paint(&self, bounds: LayoutRect, list: &mut DisplayList);
}
```
* **注册方法**：`bus.painters.register(Arc::new(Mutex::new(MyGpuWaterRipple)));`
* **DSL 调用**：`row effect="MyGpuWaterRipple" { ... }`

---

### 槽位 2：自定义底层组件与虚拟节点注册 (Custom Node & Component Plugin)

允许开发者将非内置的标准控件（如自研视频播放器、3D 画布、EChart 图表）作为原生节点直接嵌入虚拟 DOM 树中。

```rust
use element_core::{ElementNode, NodeId};
use script_engine::ScriptValue;

pub trait CustomComponentDriver: Send + Sync {
    /// 对应 DSL 中的标签名，例如 `plugin "VideoPlayer"` 或 `videoplayer { ... }`
    fn tag_name(&self) -> &str;
    
    /// 当节点在虚拟 DOM 中被创建时调用
    fn on_mount(&self, node_id: NodeId, node: &ElementNode);
    
    /// 接收 DSL 动作脚本通过 `->` 触发的方法调用
    fn invoke_method(&self, node_id: NodeId, method: &str, args: &[ScriptValue]) -> Result<ScriptValue, String>;
    
    /// 当节点尺寸或属性更新时调用
    fn on_update(&self, node_id: NodeId, node: &ElementNode);
    
    /// 节点从树中卸载时释放原生资源
    fn on_unmount(&self, node_id: NodeId);
}
```
* **注册方法**：`bus.components.register(Arc::new(NativeVideoPlayerDriver));`
* **DSL 调用**：
  ```scss
  plugin "VideoPlayer" src="assets/intro.mp4" autoplay=true -> {
      player.seek(0)
  }
  ```

---

### 槽位 3：自定义布局算法注册 (Custom Layout Strategy)

除了 Taffy 提供的标准 Flex 和 Grid 之外，允许开发者针对特殊 UI 场景（如金融看板、瀑布流、环形表盘布局）注册自定义几何解算器。

```rust
use layout_engine::LayoutRect;
use style_system::ComputedStyle;

pub trait CustomLayoutStrategy: Send + Sync {
    /// 布局标识，例如在 DSL 中声明 `display: waterfall;` 或 `display: radial;`
    fn layout_name(&self) -> &str;
    
    /// 递归解算子节点的几何坐标
    fn compute_layout(
        &self,
        parent_style: &ComputedStyle,
        available_width: f32,
        available_height: f32,
        children_styles: &[ComputedStyle],
    ) -> Vec<LayoutRect>;
}
```
* **注册方法**：`bus.layouts.register(Box::new(WaterfallLayoutStrategy));`
* **DSL 调用**：`box display="waterfall" cols=3 gap=12 { ... }`

---

### 槽位 4：自定义样式属性与插值器注册 (Custom Style Property & Interpolator)

开发者可以扩充引擎不认识的特殊样式属性，并为其提供动画过渡帧计算器（Lerp）。

```rust
use css_animation::Lerp;
use script_engine::ScriptValue;

pub trait CustomPropertyHandler: Send + Sync {
    /// 自定义属性名称，例如 `rainbow-speed` 或 `glow-intensity`
    fn property_name(&self) -> &str;
    
    /// 将 DSL 中的字面量解析为紧凑型自定义值
    fn parse(&self, raw_value: &str) -> Result<ScriptValue, String>;
    
    /// 为该属性提供补间动画插值计算
    fn interpolate(&self, from: &ScriptValue, to: &ScriptValue, t: f32) -> ScriptValue;
}
```
* **注册方法**：`bus.properties.register(Box::new(GlowIntensityHandler));`
* **DSL 调用**：`btn "炫彩按钮" glow-intensity=0.8 transition=(glow-intensity, 0.3s)`

---

### 槽位 5：脚本宿主原生函数与全局对象注册 (Host Functions & Native Objects)

允许将宿主操作系统底层能力（读写文件、剪贴板、系统托盘、网络请求、硬件串口通讯）注入为 DSL 脚本中可直接调用的全局函数或命名空间对象。

```rust
use script_engine::{ScriptScope, ScriptValue};

/// 原生函数闭包签名
pub type HostFunction = Box<dyn Fn(&[ScriptValue]) -> Result<ScriptValue, String> + Send + Sync>;

pub trait NativeModuleRegistry {
    /// 注册一个全局原生函数：例如 `alert("hello")`
    fn register_function(&mut self, name: &str, func: HostFunction);
    
    /// 注册一个原生全局对象：例如 `fs.read_file("config.json")`
    fn register_module(&mut self, module_name: &str, methods: HashMap<String, HostFunction>);
}
```
* **注册方法**：
  ```rust
  bus.scripts.register_function("show_dialog", Box::new(|args| {
      // 调用 Windows MessageBoxW / macOS NSAlert
      Ok(ScriptValue::Bool(true))
  }));
  ```
* **DSL 调用**：`btn "导出" -> show_dialog("数据导出完毕！")`

---

### 槽位 6：自定义数据库与存储驱动注册 (Database & Storage Drivers)

允许开发者接入非默认 SQLite 的本地或远程数据源（如 DuckDB、RocksDB、Redis 本地内存缓存、IPC 跨进程总线）。

```rust
use data_bridge::{DatabaseClient, DbRow, DbValue};

pub trait StorageDriverFactory: Send + Sync {
    /// 协议 Scheme，例如 `duckdb://`、`rocksdb://` 或 `ipc://`
    fn scheme(&self) -> &str;
    
    /// 根据连接字符串实例化数据库客户端句柄
    fn create_client(&self, connection_uri: &str) -> Result<Arc<dyn DatabaseClient>, String>;
}
```
* **注册方法**：`bus.storage.register(Arc::new(DuckDbDriverFactory));`
* **DSL 调用**：`let analytics_db = db.connect("duckdb://local_data.parquet")`

---

### 槽位 7：自定义 URI 资源加载器协议注册 (Custom Protocol & Asset Loader)

允许开发者接管资源加载流程（如图片解密、ZIP 归档包文件流读取、网络动态模块引入）。

```rust
pub trait AssetLoader: Send + Sync {
    /// 支持的 URI 协议头，例如 `pak://`、`res://`、`encrypted://`
    fn protocol(&self) -> &str;
    
    /// 加载并返回解密后的标准字节流
    fn load_bytes(&self, uri: &str) -> Result<Vec<u8>, String>;
}
```
* **注册方法**：`bus.assets.register(Arc::new(EncryptedPackLoader));`
* **DSL 调用**：`@import "pak://themes/cyberpunk.ui";`，`img src="pak://images/logo.png"`

---

## 四、 开发者端到端接入实战范例

### 案例：注册一个带物理交互的自研 3D 视口组件与原生函数

以下完整演示第三方开发者如何编写一个自定义插件，并无缝注入 `rust_css_like` 引擎中：

```rust
use render_backend::{CustomEffect, InteractionEvent, LayoutRect, DisplayList, DrawCommand};
use script_engine::{ScriptValue, EnginePlugin, RegistryBus};
use std::sync::{Arc, Mutex};

// 1. 实现一个自研底层交互效果：物理流光边框
pub struct CyberGlowEffect {
    hue: f32,
    intensity: f32,
}

impl CustomEffect for CyberGlowEffect {
    fn name(&self) -> &str { "CyberGlow" }
    
    fn on_interaction(&mut self, event: &InteractionEvent, _bounds: LayoutRect) {
        if let InteractionEvent::CursorMoved { local_x, .. } = event {
            self.hue = (local_x % 360.0); // 鼠标移动改变流光色彩
        }
    }
    
    fn update(&mut self, dt_secs: f32) {
        self.intensity = (self.intensity + dt_secs * 2.0) % 1.0;
    }
    
    fn paint(&self, bounds: LayoutRect, list: &mut DisplayList) {
        list.push(DrawCommand::CustomEffect {
            effect_name: self.name().to_string(),
            bounds,
            uniforms: vec![("hue".to_string(), self.hue), ("intensity".to_string(), self.intensity)],
        });
    }
}

// 2. 打包为引擎标准插件
pub struct CyberpunkPlugin;

impl EnginePlugin for CyberpunkPlugin {
    fn name(&self) -> &str { "CyberpunkExtension" }
    
    fn register(&self, bus: &mut RegistryBus) {
        // 注册底层着色器
        bus.painters.register(Arc::new(Mutex::new(CyberGlowEffect { hue: 180.0, intensity: 0.5 })));
        
        // 注册宿主原生系统级函数
        bus.scripts.register_function("play_beep", Box::new(|_args| {
            println!(">> 触发原生物理蜂鸣器声音！");
            Ok(ScriptValue::Null)
        }));
    }
}

// 3. 在引擎启动时加载插件并拉起窗口
fn main() {
    let mut engine = rust_css_engine::Engine::new();
    
    // 底层注入注册！
    engine.use_plugin(CyberpunkPlugin);
    
    // 加载带有自定义能力的 DSL
    engine.load_dsl(r#"
        win "商业原生终端" (800, 600) {
            box effect="CyberGlow" pad=20 {
                txt "已成功挂载底层 CyberGlow 着色器！" #38bdf8 16px
                btn "测试原生声音" -> play_beep()
            }
        }
    "#);
    
    engine.run();
}
```

---
*本文档已同步固化至工作区根目录 [REQUIREMENTS_AND_EXTENSIBILITY_ARCHITECTURE.md](file:///d:/for_clone/rust_css_like/REQUIREMENTS_AND_EXTENSIBILITY_ARCHITECTURE.md)，定义了 `rust_css_like` 完整的业务需求与底层可拓展开放架构标准。*
