# 在 Zed 中使用 IntelliJ Server 作为 Java/Kotlin LSP

本文记录了如何将 JetBrains 的 IntelliJ Server（来自 VS Code 扩展 "Java and Kotlin by IntelliJ IDEA"）配置为 Zed 编辑器的 LSP Server，替代默认的 jdtls 和 kotlin-lsp。

## 背景

JetBrains 发布了 VS Code 扩展 **"Java and Kotlin by IntelliJ IDEA"**，其中包含一个独立的 LSP Server（`intellij-server`），支持通过 `--stdio` 模式启动，使用标准 LSP 协议通信。这意味着它可以脱离 VS Code，被任何支持 LSP 的编辑器使用。

## 前提条件

- Zed 编辑器（已安装）
- IntelliJ Server VS Code 扩展包（解压后可获得 server 文件）
- Rust 工具链（Zed 编译扩展所需）

## 步骤

### 1. 获取 IntelliJ Server

从 VS Code 扩展中提取 server 文件。扩展的 VSIX 包本质上是一个 ZIP 文件，解压后目录结构如下：

```
jetbrains.intellij-server-0.0.13-win32-x64/
  extension/
    server/
      bin/
        intellij-server.exe          # LSP Server 主程序
        intellij-server.exe.vmoptions  # JVM 参数
        idea.properties               # IntelliJ 配置
      jbr/                            # JetBrains Runtime (JDK 25+)
      lib/                            # 核心 JAR 包
      plugins/                        # 语言插件 (java, kotlin, spring, etc.)
      product-info.json               # 产品信息
```

关键发现：`product-info.json` 中明确定义了服务器的启动方式：

```json
{
  "mainClass": "com.intellij.ls.server.MainImpl",
  "stdioRedirectArg": "--stdio"
}
```

### 2. 验证 Server 可独立运行

```powershell
& "path/to/intellij-server.exe" --stdio --system-path "$env:USERPROFILE\.intellij-server\system"
```

成功启动后会输出：

```
IJ_JAVA_OPTIONS=
idea.config.path=C:\Users\admin\.intellij-server\system\config
idea.system.path=C:\Users\admin\.intellij-server\system\system
Log file: C:\Users\admin\.intellij-server\system\system\log\intellij-server.log
```

服务器以 stdio 模式运行，通过标准输入/输出使用 LSP 协议通信。

### 3. 安装 Rust

Zed 扩展需要编译为 WebAssembly (wasm32-wasip2)，因此需要 Rust：

```powershell
# 下载并安装 rustup (Windows)
Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "$env:TEMP\rustup-init.exe"
& "$env:TEMP\rustup-init.exe" -y

# 如果没有 Visual Studio，使用 GNU 工具链
rustup default stable-x86_64-pc-windows-gnu
rustup target add wasm32-wasip2
```

### 4. 安装 Zed 扩展

扩展代码位于本仓库中。从包含 Rust PATH 的环境启动 Zed：

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
Start-Process "path/to/Zed.exe"
```

在 Zed 中：
1. `Ctrl+Shift+P` → 搜索 `install dev extension`
2. 选择本仓库目录
3. 等待编译完成

### 5. 配置 Zed settings.json

编辑 `C:\Users\admin\AppData\Roaming\Zed\settings.json`：

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
      "binary": {
        "path": "/path/to/intellij-server.exe"
      },
      "initialization_options": {
        "systemPath": "/path/to/intellij-server-system-dir"
      }
    },
    "intellij-server-kotlin": {
      "binary": {
        "path": "/path/to/intellij-server.exe"
      },
      "initialization_options": {
        "systemPath": "/path/to/intellij-server-system-dir"
      }
    }
  }
}
```

**必填配置项**：

| 配置项 | 说明 |
|--------|------|
| `binary.path` | `intellij-server.exe` 的完整路径 |
| `initialization_options.systemPath` | 缓存/索引目录路径（服务器会在此目录下创建 config/system 子目录） |

- `!jdtls` — 禁用默认的 Eclipse JDTLS
- `!kotlin-lsp` / `!kotlin-language-server` — 禁用默认的 Kotlin LSP
- `...` — 保留其他语言服务器（如 formatter）
- 如果未配置 `binary.path`，扩展会报错提示用户配置

## 验证

打开任意 Java 或 Kotlin 文件，应能看到：
- 代码补全（IntelliJ 级别）
- 代码诊断和检查
- 跳转定义 / 查找引用
- 代码格式化
- 重构支持

查看日志确认服务器已启动：
```
Log file: <systemPath>/system/log/intellij-server.log
```

## IntelliJ Server 支持的功能

| 功能 | 说明 |
|------|------|
| 语言 | Java, Kotlin |
| 构建工具 | Gradle, Maven, Bazel, JPS |
| 插件 | Spring/Spring Boot, Lombok, JUnit, TestNG, EditorConfig |
| 调试 | DAP (JVM Debug Adapter Protocol) |
| 其他 | 代码补全、导航、诊断、快速修复、重构、格式化 |

## 扩展架构

```
zed-intellij-extension/
  extension.toml           # 注册 intellij-server-java 和 intellij-server-kotlin
  Cargo.toml               # Rust 项目配置
  src/
    lib.rs                  # 从 Zed settings 读取 binary.path 和 systemPath，
                            # 启动 intellij-server.exe --stdio --system-path <path>
  rust-toolchain.toml      # GNU 工具链配置（无 Visual Studio 时必需）
```

扩展本身不包含 IntelliJ Server 二进制文件，用户需自行下载 VS Code 扩展获取。

## 踩坑记录

| 问题 | 原因 | 解决方案 |
|------|------|----------|
| `Property intellij-server is not allowed` | Zed 的 `lsp` 配置不接受未注册的自定义 server id | 创建 Zed 扩展注册 language server |
| `running rustc: program not found` | Zed 进程的 PATH 中没有 Rust | 从包含 cargo PATH 的命令行启动 Zed |
| `link.exe not found` | 默认 MSVC 工具链需要 Visual Studio | 切换到 GNU 工具链 + `rust-toolchain.toml` |
| `duplicate key java in table grammars` | Zed 已内置 Java/Kotlin Tree-sitter | `extension.toml` 中不要添加 `[grammars.*]` |
| `failed to fetch revision` | grammar git 仓库拉取失败 | 删除 grammar 定义，使用 Zed 内置语法 |
