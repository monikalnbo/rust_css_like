# Rust CSS-Like (`.ui`) 前端声明式开发语言完全手册

> **定位**：一种专为下一代超高性能、零臃肿商业级桌面端与 Web 前端设计的声明式标记与逻辑一体化编程语言。  
> **核心标语**：**没有 HTML 的标签闭合地狱，没有 CSS 的繁琐类名派生，没有 JS/Redux 的重量级样板代码。**

---

## 目录
1. [语言设计理念](#一-语言设计理念)
2. [基础语法与标记结构](#二-基础语法与标记结构)
3. [样式缩写体系 (CSS 属性对齐)](#三-样式缩写体系-css-属性对齐)
4. [响应式状态与脚本求值 (State & Expressions)](#四-响应式状态与脚本求值-state--expressions)
5. [控制流：条件渲染与循环 (Control Flow)](#五-控制流条件渲染与循环-control-flow)
6. [组件化定义与模块导入 (Components & Imports)](#六-组件化定义与模块导入-components--imports)
7. [全栈本地数据库直连 (SQLite in UI)](#七-全栈本地数据库直连-sqlite-in-ui)
8. [物理特效挂载 (Custom Visual Effects)](#八-物理特效挂载-custom-visual-effects)
9. [与传统前端栈 (HTML + CSS + React/Vue) 对比](#九-与传统前端栈-html--css--reactvue-对比)

---

## 一、 语言设计理念

在传统 Web 前端中，写一个简单的卡片需要跨越 3 种语言：
* **HTML**：`<div>`, `<span>`, `</button>` 充满开闭冗余标签；
* **CSS**：定义 `.card-container { display: flex; padding: 16px; border-radius: 8px; }`，面临类名命名与层叠冲突；
* **JavaScript**：引入 `useState`, `onClick`, `onChange`，伴随打包工具与巨大依赖 node_modules。

**Rust CSS-Like (`.ui`) 将三者合而为一**：
* 纯大括号 `{}` 界定层级，零标签闭合；
* 样式即属性，内联直写，紧凑缩写（如 `pad=16 bg=#fff rad=8`）；
* 状态自响应，动作流单向引导（`-> count += 1`）。

---

## 二、 基础语法与标记结构

### 2.1 顶级窗口定义 (`win` / `window`)
每个 `.ui` 应用以 `win` 或 `window` 关键字作为顶级容器：
```scss
win "我的原生应用" (960, 640) bg=#0f172a {
    // 界面内容
}
```
* 第一个参数：窗口初始标题（支持 `$var` 响应式变量插值）；
* 第二个参数：窗口初始尺寸 `(宽, 高)`，单位物理像素；
* 后续属性：窗口默认背景、字体与边距。

### 2.2 核心布局容器
| 标签原语 | HTML 对照 | 语义说明 | 默认排版表现 |
| :--- | :--- | :--- | :--- |
| `row` | `<div style="display:flex; flex-direction:row">` | 横向弹性流排版 | 子节点水平向右排列 |
| `col` | `<div style="display:flex; flex-direction:column">` | 纵向弹性流排版 | 子节点垂直向下排列 |
| `box` | `<div>` | 通用矩形盒容器 | 基础容器，用于定位、装饰、裁剪 |
| `card` | `<section class="card">` | 预设阴影圆角卡片 | 带默认背景色、边框与内边距的展示卡片 |

### 2.3 文本与表单交互原语
| 标签原语 | HTML 对照 | 语法示例 | 行为说明 |
| :--- | :--- | :--- | :--- |
| `txt` | `<p>` / `<span>` | `txt "你好世界" #fff 16px bold` | 高性能文本图元，由 `cosmic-text` 进行字形塑形 |
| `btn` | `<button>` | `btn "点击提交" bg=#4f46e5 -> submit()` | 交互按钮，支持内置悬停与点击物理动效 |
| `inp` | `<input>` | `inp placeholder="搜索..." bind=keyword` | 原生文本输入框，与状态变量双向绑定 |

---

## 三、 样式缩写体系 (CSS 属性对齐)

为了追求极速编码体验，语言内置了一套直观的超短样式属性表，同时兼容标准全拼：

### 3.1 盒模型与弹性伸缩
| 缩写属性 | 标准 CSS 属性 | 接受值格式 | 示例 |
| :--- | :--- | :--- | :--- |
| `pad` | `padding` | 数值或四元组 | `pad=16` (全边), `pad=(8, 16)` (上下, 左右) |
| `margin` | `margin` | 数值或四元组 | `margin=12` |
| `gap` | `gap` | 数值 | `gap=10` (弹性子项间距 10px) |
| `w` | `width` | 像素/百分比/自适应 | `w=200`, `w=100%`, `w=auto` |
| `h` | `height` | 像素/百分比/自适应 | `h=48`, `h=100%` |
| `flex` | `flex-grow` | 数值 | `flex=1` (占用剩余全部可用空间) |
| `align` | `align-items` | 关键字 | `align=center`, `align=start`, `align=end` |
| `justify` | `justify-content` | 关键字 | `justify=between`, `justify=center`, `justify=around` |

### 3.2 视觉外观与圆角
| 缩写属性 | 标准 CSS 属性 | 接受值格式 | 示例 |
| :--- | :--- | :--- | :--- |
| `bg` | `background-color` | 十六进制色彩 / 表达式 | `bg=#1e293b`, `bg=#fff`, `bg=(is_dark ? #000 : #fff)` |
| `rad` | `border-radius` | 像素数值 | `rad=8` (四角均为 8px 圆角) |
| `border` | `border` | 复合或单值 | `border=(1, #334155)` (1px 宽边框) |
| `opacity` | `opacity` | 0.0 ~ 1.0 浮点数 | `opacity=0.85` |

---

## 四、 响应式状态与脚本求值 (State & Expressions)

### 4.1 状态声明 (`let`)
无需声明状态管理器或调用 Hook，所有全局/局部状态直接以 `let` 声明：
```scss
let count = 0
let is_dark = true
let username = "开发者"
```

### 4.2 模版字符串变量插值 (`$var`)
文本或样式中出现 `$var`，引擎会自动建立依赖图谱，并在数据变动时做增量热刷新：
```scss
txt "欢迎回来, $username!" #f8fafc 16px
txt "当前点击计数: $count" #94a3b8 14px
```

### 4.3 动态三元表达式
任何属性均可接收带有括号的动态表达式：
```scss
row bg=(is_dark ? #0f172a : #ffffff) {
    btn (is_dark ? "切换为浅色" : "切换为深色") -> is_dark = !is_dark
}
```

### 4.4 动作流引导符 (`->`)
使用 `->` 直接绑定用户交互事件（如点击、键盘），执行内置轻量脚本：
```scss
// 单条状态自增
btn "加一" -> count += 1

// 多行逻辑块
btn "重置状态" -> {
    count = 0
    username = "未登录"
}
```

---

## 五、 控制流：条件渲染与循环 (Control Flow)

### 5.1 循环控制流 (`for ... in`)
直接将列表数据展开为界面图元：
```scss
col gap=8 {
    for item in tasks {
        row pad=12 bg=#1e293b rad=6 justify=between {
            txt item.title #fff 14px
            btn "删除" #ef4444 -> remove_task(item.id)
        }
    }
}
```

### 5.2 条件渲染分支 (`if ... else`)
根据表达式动态切换 DOM 子树装配：
```scss
if is_logged_in {
    txt "当前用户: $username"
    btn "退出登录" -> logout()
} else {
    btn "立即登录" bg=#4f46e5 -> open_login_dialog()
}
```

---

## 六、 组件化定义与模块导入 (Components & Imports)

### 6.1 定义可复用组件 (`component`)
```scss
component UserCard(name, role, avatar_color) {
    row pad=16 bg=#1e293b rad=8 gap=12 align=center {
        box w=40 h=40 rad=20 bg=avatar_color
        col gap=4 {
            txt name #f8fafc 16px bold
            txt role #94a3b8 12px
        }
    }
}
```

### 6.2 跨文件模块导入 (`@import`)
支持将公共组件拆分到独立 `.ui` 文件中：
```scss
@import "components/header.ui";
@import "components/user_card.ui";

win "主界面" (800, 600) {
    Header(title="控制中心")
    UserCard(name="Alice", role="系统管理员", avatar_color=#6366f1)
}
```

---

## 七、 全栈本地数据库直连 (SQLite in UI)

无需启动臃肿的后端服务，语言内置了与本地 SQLite 数据库的直连查询与修改通道：

```scss
// 1. 直接将 SQL 查询绑定到响应式列表
let tasks = db.query("SELECT id, title, status FROM tasks ORDER BY id DESC")
let new_input = ""

// 2. 在界面中添加并实时持久化
row pad=12 bg=#1e293b rad=6 gap=8 {
    inp placeholder="添加本地任务..." bind=new_input flex=1
    btn "添加" bg=#4f46e5 -> {
        db.execute("INSERT INTO tasks (title, status) VALUES (?, 'active')", [new_input])
        new_input = ""
    }
}
```

---

## 八、 物理特效挂载 (Custom Visual Effects)

语言支持直接在元素上挂载底层 GPU 物理特效，例如水波光晕扩散：
```scss
// 用户在该卡片上移动或点击时，产生物理涟漪扩散
card pad=20 bg=#1e293b rad=12 effect="InteractiveRipple" {
    txt "点击体验水波扩散特效" #f8fafc 16px bold
}
```

---

## 九、 与传统前端栈对比

| 评估维度 | 传统 HTML + CSS + JS (React/Vue) | Rust CSS-Like (`.ui`) |
| :--- | :--- | :--- |
| **运行时体积** | 几 MB 到数十 MB (Node.js/Chromium/DOM) | **~1.2 MB** 纯原生二进制 |
| **启动时间** | 300ms ~ 2000ms (V8 冷启动) | **< 15ms** 毫秒级闪开 |
| **内存占用** | 80MB ~ 250MB | **10MB ~ 25MB** |
| **语言割裂度** | HTML / CSS / JS / JSX 互相切换 | **单一语法一体化** (`.ui`) |
| **多端编译分发** | 需配置 Electron / Tauri / Webpack 庞大链条 | **GitHub Actions 全自动出 Windows/macOS/WASM** |
| **安全性** | 明文代码、极易被逆向盗版 | **支持 AOT 二进制混淆加密 (`.binui`)** |
