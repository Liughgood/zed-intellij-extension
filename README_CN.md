# IntelliJ Server — Zed Extension (Unofficial)

**[English](README.md)**

将 JetBrains 的 **IntelliJ Server**（来自 VS Code 扩展 "Java and Kotlin by IntelliJ IDEA"）作为 LSP Server 接入 Zed 编辑器，提供 IntelliJ 级别的 Java 和 Kotlin 语言支持。

> **⚠️ 本项目为非官方社区项目，与 JetBrains 无任何关联。** "IntelliJ" 是 JetBrains s.r.o. 的注册商标。本扩展不包含、不分发任何 JetBrains 专有软件，用户需自行从 JetBrains 官方渠道获取 IntelliJ Server。

## 功能

- ✅ IntelliJ 级别代码补全、诊断、导航
- ✅ 代码格式化、优化导入
- ✅ 重构：重命名、提取变量/方法/常量/字段/参数
- ✅ 快速修复和代码检查
- ✅ Gradle、Maven、Bazel 项目导入
- ✅ IntelliJ 风格快捷键（Java/Kotlin 文件中生效）

## 前提条件

- [Zed](https://zed.dev) 编辑器
- [Rust](https://rustup.rs) 工具链（编译扩展所需）
- IntelliJ Server 文件（从 VS Code 扩展中提取，见下方）

---

## 第一步：获取 IntelliJ Server

### 方式一：从 VS Code 扩展提取

1. 从 [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=JetBrains.intellij-server) 下载 `.vsix` 文件
2. `.vsix` 本质是 ZIP 文件，直接解压

### 方式二：从已安装的 VS Code 扩展复制

**macOS：**
```bash
# 扩展通常在以下目录
ls ~/.vscode/extensions/jetbrains.intellij-server-*/extension/server/bin/
```

**Windows：**
```powershell
# 扩展通常在以下目录
dir "$env:USERPROFILE\.vscode\extensions\jetbrains.intellij-server-*\extension\server\bin\"
```

### 提取后的关键文件

```
extension/server/
  bin/
    intellij-server.exe    # Windows
    intellij-server        # macOS/Linux
    intellij-server.vmoptions
    idea.properties
  jbr/                     # JetBrains Runtime (JDK 25+，自带，无需额外安装)
  lib/                     # 核心 JAR 包
  plugins/                 # 语言插件
  product-info.json
```

### 验证 Server 可独立运行

**macOS：**
```bash
chmod +x path/to/intellij-server
./path/to/intellij-server --stdio --system-path ~/.intellij-server/system
```

**Windows：**
```powershell
& "path\to\intellij-server.exe" --stdio --system-path "$env:USERPROFILE\.intellij-server\system"
```

成功启动会输出类似：
```
IJ_JAVA_OPTIONS=
idea.config.path=...
idea.system.path=...
Log file: ...
```

> 按 `Ctrl+C` 退出验证。服务器以 stdio 模式等待输入，不会主动退出。

---

## 第二步：安装 Rust

Zed 扩展需要编译为 WebAssembly，因此需要 Rust 工具链。

**macOS：**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
rustup target add wasm32-wasip2
```

**Windows：**
```powershell
# 下载安装器
Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "$env:TEMP\rustup-init.exe"
& "$env:TEMP\rustup-init.exe" -y

# 如果没有 Visual Studio，切换到 GNU 工具链
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
rustup default stable-x86_64-pc-windows-gnu
rustup target add wasm32-wasip2
```

> **Windows 用户注意**：如果已安装 Visual Studio 或 Build Tools，可使用默认的 MSVC 工具链，无需切换 GNU。

验证安装：
```bash
rustc --version
cargo --version
```

---

## 第三步：安装扩展

### 1. 克隆本仓库

```bash
git clone https://github.com/your-name/zed-intellij-extension.git
```

### 2. 确保 Rust 在当前 PATH 中

**macOS：**
```bash
source "$HOME/.cargo/env"
```

**Windows：**
```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
```

### 3. 从命令行启动 Zed

> **重要**：必须从包含 Rust PATH 的环境启动 Zed，否则 Zed 找不到 `rustc` 无法编译扩展。

**macOS：**
```bash
zed
```

**Windows：**
```powershell
Start-Process "path\to\Zed.exe"
```

### 4. 安装 Dev Extension

在 Zed 中：
1. `Ctrl+Shift+P`（macOS: `Cmd+Shift+P`）→ 搜索 **`install dev extension`**
2. 选择本仓库的目录
3. 等待编译完成（首次约 30 秒）

安装成功后 Zed 日志会显示：
```
compiled Rust extension "...zed-intellij-extension"
extensions updated. loading 0, reloading 1, unloading 0
```

---

## 第四步：配置 Zed

编辑 Zed 的 settings 文件（`Cmd/Ctrl+Shift+P` → `zed: open settings`）：

### macOS 示例

```json
{
  "languages": {
    "Java": {
      "language_servers": ["intellij-server-java", "!jdtls", "..."]
    },
    "Kotlin": {
      "language_servers": ["intellij-server-kotlin", "!kotlin-lsp", "!kotlin-language-server", "..."]
    }
  },
  "lsp": {
    "intellij-server-java": {
      "initialization_options": {
        "serverPath": "/Users/you/intellij-server/server/bin/intellij-server",
        "systemPath": "/Users/you/.intellij-server/system"
      }
    },
    "intellij-server-kotlin": {
      "initialization_options": {
        "serverPath": "/Users/you/intellij-server/server/bin/intellij-server",
        "systemPath": "/Users/you/.intellij-server/system"
      }
    }
  }
}
```

### Windows 示例

```json
{
  "languages": {
    "Java": {
      "language_servers": ["intellij-server-java", "!jdtls", "..."]
    },
    "Kotlin": {
      "language_servers": ["intellij-server-kotlin", "!kotlin-lsp", "!kotlin-language-server", "..."]
    }
  },
  "lsp": {
    "intellij-server-java": {
      "initialization_options": {
        "serverPath": "D:/path/to/intellij-server/server/bin/intellij-server.exe",
        "systemPath": "C:/Users/you/.intellij-server/system"
      }
    },
    "intellij-server-kotlin": {
      "initialization_options": {
        "serverPath": "D:/path/to/intellij-server/server/bin/intellij-server.exe",
        "systemPath": "C:/Users/you/.intellij-server/system"
      }
    }
  }
}
```

### 配置项说明

| 配置项 | 必填 | 说明 |
|--------|------|------|
| `initialization_options.serverPath` | ✅ | `intellij-server` 可执行文件的完整路径 |
| `initialization_options.systemPath` | ❌ | 缓存/索引目录路径，默认为 `intellij-server`（会在当前目录下创建） |
| `languages.Java.language_servers` | ✅ | `!jdtls` 禁用默认 JDTLS，避免冲突 |
| `languages.Kotlin.language_servers` | ✅ | `!kotlin-lsp` 和 `!kotlin-language-server` 禁用默认 Kotlin LSP |

> **⚠️ 不要使用 `binary.path` 配置 server 路径**，否则 Zed 会覆盖扩展返回的启动参数（丢失 `--stdio`），导致服务器以 daemon 模式启动而无法通信。

---

## 快捷键

扩展在 Java（`.java`）和 Kotlin（`.kt`）文件中提供 IntelliJ 风格的快捷键，基于 Zed 的 JetBrains base_keymap 进行补充。

### 重构与代码操作

| 快捷键 | 功能 | 说明 |
|--------|------|------|
| `Alt+Enter` | 显示代码操作 | 快速修复、代码检查、重构入口 |
| `Ctrl+Alt+L` | 格式化代码 | |
| `Ctrl+Alt+O` | 优化导入 | |
| `Shift+F6` | 重命名符号 | |
| `Ctrl+Alt+Shift+T` | 重构菜单 | 打开 Code Actions 列表 |
| `Ctrl+Alt+V` | 提取变量 | 打开 Code Actions 列表，选择提取变量 |
| `Ctrl+Alt+M` | 提取方法 | 打开 Code Actions 列表，选择提取方法 |
| `Ctrl+Alt+C` | 提取常量 | 打开 Code Actions 列表，选择提取常量 |
| `Ctrl+Alt+F` | 提取字段 | 打开 Code Actions 列表，选择提取字段 |
| `Ctrl+Alt+P` | 提取参数 | 打开 Code Actions 列表，选择提取参数 |
| `Ctrl+Alt+T` | 包围代码 | if/while/try 等 |
| `F6` | 移动重构 | 打开 Code Actions 列表 |

> **说明**：提取变量/方法/常量等操作，Zed 没有对应的专用 action，统一通过 `editor::ShowCodeActions` 触发。按下快捷键后从 Code Action 列表中选择具体的重构操作。这与 IntelliJ 中快捷键直接执行重构不同。

### 导航与信息

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+B` | 跳转到定义 |
| `Ctrl+Alt+B` | 跳转到实现 |
| `Ctrl+Shift+B` | 跳转到类型定义 |
| `Alt+F7` | 查找所有引用 |
| `Ctrl+P` | 参数信息 |
| `Ctrl+Q` | 快速文档（Hover） |

> macOS 用户：将 `Ctrl` 替换为 `Cmd`，`Alt` 替换为 `Option`。

---

## 验证

1. 打开一个 Java 或 Kotlin 项目
2. 打开任意 `.java` 或 `.kt` 文件
3. Zed 底部状态栏应显示 `intellij-server-java` 或 `intellij-server-kotlin` 连接状态
4. 等待项目索引完成（首次打开较慢，后续会使用缓存）
5. 测试：输入代码触发补全，或按 `Alt+Enter` 查看代码操作

查看服务器日志：
```
<systemPath>/system/log/intellij-server.log
```

---

## IntelliJ Server 支持的功能

| 类别 | 说明 |
|------|------|
| 语言 | Java, Kotlin |
| 构建工具 | Gradle, Maven, Bazel, JPS |
| 内置插件 | Spring/Spring Boot, Lombok, JUnit, TestNG, EditorConfig |
| 调试 | DAP (JVM Debug Adapter Protocol)，需额外适配 |
| 代码智能 | 补全、导航、诊断、快速修复、重构、格式化 |

---

## 常见问题

### 安装扩展时 `running rustc: program not found`

Zed 找不到 Rust。确保从包含 `cargo` 的 PATH 环境中启动 Zed：

**macOS：**
```bash
source "$HOME/.cargo/env" && zed
```

**Windows：**
```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; Start-Process "path\to\Zed.exe"
```

### Windows 编译时 `link.exe not found`

默认 MSVC 工具链需要 Visual Studio。如未安装，切换到 GNU 工具链：

```powershell
rustup default stable-x86_64-pc-windows-gnu
rustup target add wasm32-wasip2
```

扩展仓库中的 `rust-toolchain.toml` 会自动指定 GNU 工具链。

### `Property intellij-server is not allowed`

Zed 的 `lsp` 配置只允许扩展注册的 server id。确保已安装本扩展（dev extension）。

### 服务器启动失败：`Address already in use: bind`

可能有残留的 `intellij-server` 进程。终止后重试：

**macOS：**
```bash
pkill -f intellij-server
```

**Windows：**
```powershell
Get-Process -Name "intellij-server" | Stop-Process -Force
```

### LSP 状态一直显示 "starting"

1. 确认 **没有** 在 Zed settings 中使用 `binary.path`（这会覆盖 `--stdio` 参数）
2. 检查 `initialization_options.serverPath` 路径是否正确
3. 查看服务器日志是否有错误

### `duplicate key java in table grammars`

Zed 已内置 Java/Kotlin Tree-sitter 语法。本扩展的 `extension.toml` 中不包含 `[grammars.*]` 定义，如遇到此错误，确保使用最新版本的扩展。

---

## 项目结构

```
zed-intellij-extension/
  extension.toml              # 扩展清单，注册 language server
  Cargo.toml                  # Rust 项目配置
  src/
    lib.rs                    # 扩展逻辑：读取 settings，启动 server
  keymaps/
    intellij-refactor.json    # IntelliJ 风格重构快捷键
  rust-toolchain.toml         # Rust 工具链配置（Windows GNU）
  README.md
```

扩展不包含 IntelliJ Server 二进制文件，用户需自行从 VS Code 扩展获取。

---

## 致谢

- [JetBrains](https://www.jetbrains.com/) — IntelliJ Server
- [Zed Industries](https://zed.dev/) — Zed 编辑器及扩展 API

## 免责声明

本项目为非官方社区项目，与 JetBrains s.r.o. 无任何关联、背书或赞助。IntelliJ 是 JetBrains s.r.o. 的注册商标。本扩展不包含、不分发任何 JetBrains 专有软件或二进制文件。用户需自行从 JetBrains 官方渠道（VS Code Marketplace）获取 IntelliJ Server 并遵守其许可协议（预览期 30 天，1.0 正式版将引入许可机制）。
