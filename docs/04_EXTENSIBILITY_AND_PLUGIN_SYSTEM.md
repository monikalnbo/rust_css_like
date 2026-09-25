# 04. Rust CSS-Like 高拓展性底层注册与插件开发规范指南 (Extensibility & Plugin System)

> **文档编号**：`DOC-04-EXT-PLUGIN`  
> **文档定位**：详细阐述 `rust_css_like` 的开放式微内核架构与底层注册总线（Low-Level Registration Bus），指导开发者如何将自研的 GPU 着色器、3D 画布、特殊布局算法、宿主系统函数与自研数据库引擎深度注入到 UI 内核中。

---

## 目录
1. [开放式微内核设计哲学](#一-开放式微内核设计哲学)
2. [底层注册总线全景架构 (The Low-Level Registry Bus)](#二-底层注册总线全景架构-the-low-level-registry-bus)
3. [七大底层注册槽位详细规范与 Trait 签名](#三-七大底层注册槽位详细规范与-trait-签名)
   - [槽位 1：底层 GPU 着色器与自定义光栅化 (CustomPainter)](#槽位-1底层-gpu-着色器与自定义光栅化-custompainter)
   - [槽位 2：底层自定义节点与三方控件驱动 (CustomComponentDriver)](#槽位-2底层自定义节点与三方控件驱动-customcomponentdriver)
   - [槽位 3：自定义几何排版算法 (CustomLayoutStrategy)](#槽位-3自定义几何排版算法-customlayoutstrategy)
   - [槽位 4：自定义样式属性与动画插值器 (CustomPropertyHandler)](#槽位-4自定义样式属性与动画插值器-custompropertyhandler)
   - [槽位 5：宿主原生函数与系统级对象注入 (NativeHostFunctions)](#槽位-5宿主原生函数与系统级对象注入-nativehostfunctions)
   - [槽位 6：外部数据库与存储驱动工厂 (StorageDriverFactory)](#槽位-6外部数据库与存储驱动工厂-storagedriverfactory)
   - [槽位 7：自定义协议与资源解密加载器 (AssetProtocolLoader)](#槽位-7自定义协议与资源解密加载器-assetprotocolloader)
4. [统一插件协议与生命周期 (EnginePlugin & PluginContext)](#四-统一插件协议与生命周期-engineplugin--plugincontext)
5. [两个工业级端到端实战开发案例](#五-两个工业级端到端实战开发案例)
   - [实战 1：编写一个高性能物理水波 GPU 着色器插件](#实战-1编写一个高性能物理水波-gpu-着色器插件)
   - [实战 2：编写一个宿主原生操作系统能力与 DuckDB 引擎插件](#实战-2编写一个宿主原生操作系统能力与-duckdb-引擎插件)
6. [DSL 语法消费与无缝协同体验](#六-dsl-语法消费与无缝协同体验)

---

## 一、 开放式微内核设计哲学

成熟的商业级原生软件绝不可能仅靠框架内置的 `box`, `btn`, `txt` 完成所有业务需求。在实际工程落地中，开发者必然会面临：
* **图形极客需求**：需要挂载自研的 WGSL 片元着色器（如粒子光效、流光渐变、毛玻璃背景模糊）；
* **复杂交互需求**：需要将自研 3D 视口（Vello / WGPU / OpenGL 离屏纹理）、视频播放器或 ECharts 原生挂载为虚拟 DOM 的一等公民节点；
* **非标排版需求**：需要实现金融环形表盘（Radial Layout）、自适应瀑布流（Waterfall Layout）或停靠式窗口（Docking Layout）；
* **底层系统级互操作**：需要从 UI 行内脚本直接调用操作系统的系统托盘、硬件串口通信或原生文件系统。

因此，`rust_css_like` 秉承**“内核极小化，能力总线化（Microkernel + Bus Architecture）”**的原则，在引擎的**排版、渲染、脚本、组件树、样式表、数据连接与资源加载**这七大底层关键节点，全部开放拦截与注册槽位。

---

## 二、 底层注册总线全景架构 (The Low-Level Registry Bus)

```
┌────────────────────────────────────────────────────────────────────────┐
│                        Rust CSS-Like 统一注册总线                       │
│                              (RegistryBus)                             │
├──────────────┬──────────────┬──────────────┬─────────────┬─────────────┤
│ 1. 渲染绘制  │ 2. 虚拟节点  │ 3. 布局排版  │ 4. 样式拓展 │ 5. 宿主函数 │
│ CustomPainter│ ComponentDrv │ LayoutStrat  │ PropHandler │ HostFnRegistry
├──────────────┴──────────────┴──────────────┴─────────────┴─────────────┤
│ 6. 存储驱动 (StorageDriver)       │ 7. 资源协议 (AssetProtocolLoader)   │
└─────────────────────────────────────┴───────────────────────────────────┘
                                   ▲
                                   │ 一键注册总线
                        ┌──────────┴──────────┐
                        │   EnginePlugin 规范 │
                        └─────────────────────┘
                                   ▲
                 ┌─────────────────┴─────────────────┐
       [自研 3D/着色器插件]                [自研系统级扩展插件]
```

---

## 三、 七大底层注册槽位详细规范与 Trait 签名

### 槽位 1：底层 GPU 着色器与自定义光栅化 (CustomPainter)

允许开发者接管组件的绘制阶段，直接提交底层着色器 Uniform 参数或利用光栅化器绘制物理像素。

```rust
use render_backend::{DisplayList, LayoutRect, InteractionEvent};

/// 开发者自研着色器与绘图器
pub trait CustomPainter: Send + Sync {
    /// 唯一标识名，对应 DSL 中的 `effect="ShaderName"`
    fn name(&self) -> &str;
    
    /// 接收底层物理输入事件（鼠标移动轨迹、点击、按压深度）
    fn on_interaction(&mut self, event: &InteractionEvent, bounds: LayoutRect);
    
    /// 帧进物理模拟时钟推进（dt_secs 为上一帧至今的真实物理时间）
    fn update(&mut self, dt_secs: f32);
    
    /// 向渲染指令队列注入底层 Uniform 变量或自定义图元
    fn paint(&self, bounds: LayoutRect, list: &mut DisplayList);
}
```

---

### 槽位 2：底层自定义节点与三方控件驱动 (CustomComponentDriver)

允许开发者将任何自研的复杂系统（如视频播放器、3D 模型查看器）伪装为普通 UI 标签注册入虚拟 DOM 树中。

```rust
use element_core::{ElementNode, NodeId};
use script_engine::ScriptValue;

pub trait CustomComponentDriver: Send + Sync {
    /// 标签原语名称，例如 `plugin "ModelViewer3D"`
    fn tag_name(&self) -> &str;
    
    /// 当节点挂载进虚拟 DOM 树时触发（初始化原生资源）
    fn on_mount(&self, node_id: NodeId, node: &ElementNode);
    
    /// 接收 DSL 动作脚本通过 `->` 发送的方法调用
    fn invoke_method(&self, node_id: NodeId, method: &str, args: &[ScriptValue]) -> Result<ScriptValue, String>;
    
    /// 当节点尺寸或排版约束变动时触发
    fn on_update(&self, node_id: NodeId, node: &ElementNode);
    
    /// 当节点销毁时触发（释放底层显存与句柄）
    fn on_unmount(&self, node_id: NodeId);
}
```

---

### 槽位 3：自定义几何排版算法 (CustomLayoutStrategy)

允许开发者实现除标准 Flex / Grid 之外的特殊流式排版算法。

```rust
use layout_engine::LayoutRect;
use style_system::ComputedStyle;

pub trait CustomLayoutStrategy: Send + Sync {
    /// 布局标识，例如在 DSL 中声明 `display: "waterfall"`
    fn layout_name(&self) -> &str;
    
    /// 自定义几何解算：根据父级约束计算所有子节点的物理布局矩形
    fn compute_layout(
        &self,
        parent_style: &ComputedStyle,
        available_width: f32,
        available_height: f32,
        children: &[ComputedStyle],
    ) -> Vec<LayoutRect>;
}
```

---

### 槽位 4：自定义样式属性与动画插值器 (CustomPropertyHandler)

允许开发者扩充引擎原本未定义的样式字段，并为其提供帧过渡补间动画计算（Lerp）。

```rust
use script_engine::ScriptValue;

pub trait CustomPropertyHandler: Send + Sync {
    /// 自定义样式属性名，例如 `neon-pulse-frequency`
    fn property_name(&self) -> &str;
    
    /// 将 DSL 中的字面量解析为类型化数据
    fn parse_value(&self, raw: &str) -> Result<ScriptValue, String>;
    
    /// 补间动画插值计算 (0.0 <= t <= 1.0)
    fn interpolate(&self, from: &ScriptValue, to: &ScriptValue, t: f32) -> ScriptValue;
}
```

---

### 槽位 5：宿主原生函数与系统级对象注入 (NativeHostFunctions)

允许将操作系统的原生系统调用注入为 UI 脚本中的可调用函数或命名空间模块。

```rust
use script_engine::ScriptValue;
use std::collections::HashMap;

pub type NativeHostFn = Box<dyn Fn(&[ScriptValue]) -> Result<ScriptValue, String> + Send + Sync>;

pub trait NativeHostRegistry {
    /// 注册一个全局原生函数：例如 `alert("hello")`
    fn register_fn(&mut self, name: &str, func: NativeHostFn);
    
    /// 注册一个全局命名空间模块：例如 `fs.read_text("path")`
    fn register_module(&mut self, module_name: &str, methods: HashMap<String, NativeHostFn>);
}
```

---

### 槽位 6：外部数据库与存储驱动工厂 (StorageDriverFactory)

允许插件化替换或拓展数据层，接入 DuckDB、RocksDB、Redis 或跨进程共享内存总线。

```rust
use data_bridge::DatabaseClient;
use std::sync::Arc;

pub trait StorageDriverFactory: Send + Sync {
    /// 协议 Scheme，例如 `duckdb://` 或 `rocksdb://`
    fn scheme(&self) -> &str;
    
    /// 根据连接串实例化真正的数据库客户端
    fn connect(&self, uri: &str) -> Result<Arc<dyn DatabaseClient>, String>;
}
```

---

### 槽位 7：自定义协议与资源解密加载器 (AssetProtocolLoader)

允许开发者拦截 `@import` 或资源读取流程，支持加密包直接内存解密或虚拟归档包读取。

```rust
pub trait AssetProtocolLoader: Send + Sync {
    /// 支持的 URI 协议头，例如 `pak://`、`encrypted://`
    fn protocol(&self) -> &str;
    
    /// 根据相对或绝对 URI 加载字节流
    fn load_bytes(&self, uri: &str) -> Result<Vec<u8>, String>;
}
```

---

## 四、 统一插件协议与生命周期 (EnginePlugin & PluginContext)

为了使扩展高度内聚且便于第三方包分发，所有底层注册全部通过实现 `EnginePlugin` 统一打包：

```rust
pub struct PluginContext<'a> {
    pub bus: &'a mut RegistryBus,
    pub engine_version: &'static str,
}

pub trait EnginePlugin: Send + Sync {
    /// 插件全称与版本
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    
    /// 插件装载：向各个槽位注册能力
    fn on_load(&self, ctx: &mut PluginContext);
    
    /// 引擎每帧时钟推进（可选）
    fn on_frame(&self, _dt_secs: f32) {}
    
    /// 引擎关闭时优雅清理原生资源
    fn on_unload(&self) {}
}
```

---

## 五、 两个工业级端到端实战开发案例

### 实战 1：编写一个高性能物理水波 GPU 着色器插件

```rust
use render_backend::{CustomEffect, InteractionEvent, LayoutRect, DisplayList, DrawCommand};
use script_engine::{EnginePlugin, PluginContext};
use std::sync::{Arc, Mutex};

// 1. 实现 CustomEffect Trait
pub struct WaterRipplePainter {
    center: (f32, f32),
    radius: f32,
    max_radius: f32,
    intensity: f32,
}

impl WaterRipplePainter {
    pub fn new(max_radius: f32) -> Self {
        Self { center: (0.0, 0.0), radius: 0.0, max_radius, intensity: 0.0 }
    }
}

impl CustomEffect for WaterRipplePainter {
    fn name(&self) -> &str { "WaterRippleShader" }
    
    fn on_interaction(&mut self, event: &InteractionEvent, _bounds: LayoutRect) {
        if let InteractionEvent::PointerDown { local_x, local_y, .. } = *event {
            self.center = (local_x, local_y);
            self.radius = 0.0;
            self.intensity = 1.0; // 点击激发水波扩散
        }
    }
    
    fn update(&mut self, dt_secs: f32) {
        if self.intensity > 0.0 {
            self.radius += dt_secs * 350.0;
            self.intensity -= dt_secs * 1.8;
            if self.radius >= self.max_radius { self.intensity = 0.0; }
        }
    }
    
    fn render(&self, bounds: LayoutRect, list: &mut DisplayList) {
        list.push(DrawCommand::CustomEffect {
            effect_name: self.name().to_string(),
            bounds,
            uniforms: vec![
                ("u_center_x".to_string(), self.center.0),
                ("u_center_y".to_string(), self.center.1),
                ("u_radius".to_string(), self.radius),
                ("u_intensity".to_string(), self.intensity.max(0.0)),
            ],
        });
    }
}

// 2. 封装为插件
pub struct WaterRipplePlugin;

impl EnginePlugin for WaterRipplePlugin {
    fn name(&self) -> &str { "WaterRippleEffectPlugin" }
    fn version(&self) -> &str { "1.0.0" }
    
    fn on_load(&self, ctx: &mut PluginContext) {
        ctx.bus.painters.register(Arc::new(Mutex::new(WaterRipplePainter::new(200.0))));
    }
}
```

---

### 实战 2：编写一个宿主原生操作系统能力与 DuckDB 引擎插件

```rust
use script_engine::{EnginePlugin, PluginContext, ScriptValue};
use std::collections::HashMap;

pub struct SystemToolsPlugin;

impl EnginePlugin for SystemToolsPlugin {
    fn name(&self) -> &str { "SystemToolsBridge" }
    fn version(&self) -> &str { "0.1.0" }
    
    fn on_load(&self, ctx: &mut PluginContext) {
        // 1. 注册原生文件读取函数
        let mut fs_methods = HashMap::new();
        fs_methods.insert("read_text".to_string(), Box::new(|args: &[ScriptValue]| {
            if let Some(ScriptValue::String(path)) = args.get(0) {
                let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
                return Ok(ScriptValue::String(content));
            }
            Err("参数错误：需提供合法文件路径".to_string())
        }) as script_engine::NativeHostFn);
        
        ctx.bus.scripts.register_module("fs", fs_methods);
        
        // 2. 注册系统原生蜂鸣器调用
        ctx.bus.scripts.register_fn("beep", Box::new(|_| {
            #[cfg(target_os = "windows")]
            unsafe { winapi::um::utilapiset::Beep(750, 300); }
            Ok(ScriptValue::Null)
        }));
    }
}
```

---

## 六、 DSL 语法消费与无缝协同体验

一旦第三方插件通过底层注册总线挂载完毕，在 DSL 源码中即可享受如同内置关键字般优雅流畅的编码体验：

```scss
// 1. 跨文件引入公共主题
@import "pak://themes/cyberpunk.ui"

// 2. 直接调用底层插件注册的 fs 原生模块
let local_license = fs.read_text("license.key")

win "商业终端平台" (900, 650) {
    
    // 3. 直接挂载底层 WaterRipple 着色器！
    // 鼠标在卡片内点击时，GPU 自动渲染物理水波扩散！
    row pad=20 bg=#1e293b rad=8 effect="WaterRippleShader" {
        txt "点击此区域产生水波着色器渲染" #38bdf8 16px bold
        
        // 4. 点击直接触发原生底层注册函数
        btn "触发系统物理蜂鸣" bg=#4f46e5 pad=(8,16) rad=6 -> beep()
    }
}
```

---
*本文档已同步固化至工作区 [docs/04_EXTENSIBILITY_AND_PLUGIN_SYSTEM.md](file:///d:/for_clone/rust_css_like/docs/04_EXTENSIBILITY_AND_PLUGIN_SYSTEM.md)，构成了 `rust_css_like` 工业级可拓展插件体系的底层权威技术规范。*
