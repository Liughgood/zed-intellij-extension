# IntelliJ Server — Zed Extension (Unofficial)

**[中文文档](README_CN.md)**

Use JetBrains' **IntelliJ Server** (from the VS Code extension "Java and Kotlin by IntelliJ IDEA") as an LSP Server in Zed, bringing IntelliJ-level Java and Kotlin language support.

> **⚠️ This is an unofficial community project and is not affiliated with JetBrains.** "IntelliJ" is a registered trademark of JetBrains s.r.o. This extension does not contain or distribute any JetBrains proprietary software — users must obtain IntelliJ Server from JetBrains' official channels.

## Features

- ✅ IntelliJ-level code completion, diagnostics, and navigation
- ✅ Code formatting and import optimization
- ✅ Refactoring: rename, extract variable/method/constant/field/parameter
- ✅ Quick fixes and code inspections
- ✅ Gradle, Maven, and Bazel project import
- ✅ IntelliJ-style keybindings for Java/Kotlin files

## Prerequisites

- [Zed](https://zed.dev) editor
- [Rust](https://rustup.rs) toolchain (required to compile the extension)
- IntelliJ Server files (extracted from the VS Code extension, see below)

---

## Step 1: Get IntelliJ Server

### Option A: Extract from the VS Code extension

1. Download the `.vsix` file from the [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=JetBrains.intellij-server)
2. A `.vsix` file is a ZIP archive — extract it directly

### Option B: Copy from an existing VS Code installation

**macOS:**
```bash
# Extensions are typically located at
ls ~/.vscode/extensions/jetbrains.intellij-server-*/extension/server/bin/
```

**Windows:**
```powershell
dir "$env:USERPROFILE\.vscode\extensions\jetbrains.intellij-server-*\extension\server\bin\"
```

### Key files after extraction

```
extension/server/
  bin/
    intellij-server.exe    # Windows
    intellij-server        # macOS/Linux
    intellij-server.vmoptions
    idea.properties
  jbr/                     # JetBrains Runtime (JDK 25+, bundled)
  lib/                     # Core JARs
  plugins/                 # Language plugins
  product-info.json
```

### Verify the server runs independently

**macOS:**
```bash
chmod +x path/to/intellij-server
./path/to/intellij-server --stdio --system-path ~/.intellij-server/system
```

**Windows:**
```powershell
& "path\to\intellij-server.exe" --stdio --system-path "$env:USERPROFILE\.intellij-server\system"
```

A successful startup will output something like:
```
IJ_JAVA_OPTIONS=
idea.config.path=...
idea.system.path=...
Log file: ...
```

> Press `Ctrl+C` to exit. The server runs in stdio mode and waits for input — it won't exit on its own.

---

## Step 2: Install Rust

Zed extensions are compiled to WebAssembly, so you need the Rust toolchain.

**macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
rustup target add wasm32-wasip2
```

**Windows:**
```powershell
# Download the installer
Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "$env:TEMP\rustup-init.exe"
& "$env:TEMP\rustup-init.exe" -y

# If you don't have Visual Studio, switch to the GNU toolchain
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
rustup default stable-x86_64-pc-windows-gnu
rustup target add wasm32-wasip2
```

> **Windows note:** If you have Visual Studio or Build Tools installed, you can use the default MSVC toolchain without switching to GNU.

Verify the installation:
```bash
rustc --version
cargo --version
```

---

## Step 3: Install the Extension

### 1. Clone this repository

```bash
git clone https://github.com/your-name/zed-intellij-extension.git
```

### 2. Ensure Rust is in your current PATH

**macOS:**
```bash
source "$HOME/.cargo/env"
```

**Windows:**
```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
```

### 3. Launch Zed from the command line

> **Important:** Zed must be launched from an environment where `rustc` is in PATH, otherwise it cannot compile the extension.

**macOS:**
```bash
zed
```

**Windows:**
```powershell
Start-Process "path\to\Zed.exe"
```

### 4. Install as a Dev Extension

In Zed:
1. `Ctrl+Shift+P` (macOS: `Cmd+Shift+P`) → search for **`install dev extension`**
2. Select this repository's directory
3. Wait for compilation to complete (~30 seconds on first run)

A successful installation will show in Zed's log:
```
compiled Rust extension "...zed-intellij-extension"
extensions updated. loading 0, reloading 1, unloading 0
```

---

## Step 4: Configure Zed

Edit Zed's settings file (`Cmd/Ctrl+Shift+P` → `zed: open settings`):

### macOS Example

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

### Windows Example

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

### Configuration Reference

| Setting | Required | Description |
|---------|----------|-------------|
| `initialization_options.serverPath` | ✅ | Full path to the `intellij-server` executable |
| `initialization_options.systemPath` | ❌ | Cache/index directory path. Defaults to `intellij-server` (created in current directory) |
| `languages.Java.language_servers` | ✅ | `!jdtls` disables the default Eclipse JDTLS to avoid conflicts |
| `languages.Kotlin.language_servers` | ✅ | `!kotlin-lsp` and `!kotlin-language-server` disable the default Kotlin LSPs |

> **⚠️ Do NOT use `binary.path` to configure the server path.** Zed will override the extension's launch arguments (dropping `--stdio`), causing the server to start in daemon mode instead of stdio mode, which breaks communication.

---

## Keybindings

The extension provides IntelliJ-style keybindings in Java (`.java`) and Kotlin (`.kt`) files, complementing Zed's built-in JetBrains base keymap.

### Refactoring & Code Actions

| Shortcut | Action | Notes |
|----------|--------|-------|
| `Alt+Enter` | Show Code Actions | Quick fixes, inspections, refactoring entry point |
| `Ctrl+Alt+L` | Reformat Code | |
| `Ctrl+Alt+O` | Optimize Imports | |
| `Shift+F6` | Rename Symbol | |
| `Ctrl+Alt+Shift+T` | Refactor Menu | Opens Code Actions list |
| `Ctrl+Alt+V` | Extract Variable | Opens Code Actions list, select "Extract Variable" |
| `Ctrl+Alt+M` | Extract Method | Opens Code Actions list, select "Extract Method" |
| `Ctrl+Alt+C` | Extract Constant | Opens Code Actions list, select "Extract Constant" |
| `Ctrl+Alt+F` | Extract Field | Opens Code Actions list, select "Extract Field" |
| `Ctrl+Alt+P` | Extract Parameter | Opens Code Actions list, select "Extract Parameter" |
| `Ctrl+Alt+T` | Surround With | if/while/try etc. |
| `F6` | Move Refactoring | Opens Code Actions list |

> **Note:** Extract variable/method/constant etc. do not have dedicated Zed actions. They all go through `editor::ShowCodeActions`. After pressing the shortcut, select the specific refactoring from the Code Action list. This differs from IntelliJ where the shortcut directly executes the refactoring.

### Navigation & Information

| Shortcut | Action |
|----------|--------|
| `Ctrl+B` | Go to Definition |
| `Ctrl+Alt+B` | Go to Implementation |
| `Ctrl+Shift+B` | Go to Type Definition |
| `Alt+F7` | Find All References |
| `Ctrl+P` | Parameter Info |
| `Ctrl+Q` | Quick Documentation (Hover) |

> macOS users: replace `Ctrl` with `Cmd`, and `Alt` with `Option`.

---

## Verification

1. Open a Java or Kotlin project
2. Open any `.java` or `.kt` file
3. Zed's bottom status bar should show `intellij-server-java` or `intellij-server-kotlin` connection status
4. Wait for project indexing to complete (first open is slower; subsequent opens use cache)
5. Test: type code to trigger completion, or press `Alt+Enter` to see code actions

Server logs are located at:
```
<systemPath>/system/log/intellij-server.log
```

---

## IntelliJ Server Capabilities

| Category | Details |
|----------|---------|
| Languages | Java, Kotlin |
| Build Tools | Gradle, Maven, Bazel, JPS |
| Built-in Plugins | Spring/Spring Boot, Lombok, JUnit, TestNG, EditorConfig |
| Debugging | DAP (JVM Debug Adapter Protocol) — requires additional adaptation |
| Code Intelligence | Completion, navigation, diagnostics, quick fixes, refactoring, formatting |

---

## Troubleshooting

### `running rustc: program not found` during extension install

Zed cannot find Rust. Make sure to launch Zed from an environment with `cargo` in PATH:

**macOS:**
```bash
source "$HOME/.cargo/env" && zed
```

**Windows:**
```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; Start-Process "path\to\Zed.exe"
```

### `link.exe not found` on Windows

The default MSVC toolchain requires Visual Studio. If not installed, switch to the GNU toolchain:

```powershell
rustup default stable-x86_64-pc-windows-gnu
rustup target add wasm32-wasip2
```

The `rust-toolchain.toml` in this repository automatically specifies the GNU toolchain.

### `Property intellij-server is not allowed`

Zed's `lsp` settings only accept server IDs registered by extensions. Make sure this extension is installed as a dev extension.

### Server fails to start: `Address already in use: bind`

A stale `intellij-server` process may still be running. Kill it and retry:

**macOS:**
```bash
pkill -f intellij-server
```

**Windows:**
```powershell
Get-Process -Name "intellij-server" | Stop-Process -Force
```

### LSP status stays at "starting"

1. Confirm you are **NOT** using `binary.path` in Zed settings (this overrides the `--stdio` argument)
2. Check that `initialization_options.serverPath` points to the correct executable
3. Check the server log for errors

### `duplicate key java in table grammars`

Zed already includes Java and Kotlin Tree-sitter grammars. This extension's `extension.toml` does not include `[grammars.*]` definitions. If you encounter this error, make sure you're using the latest version of the extension.

---

## Project Structure

```
zed-intellij-extension/
  extension.toml              # Extension manifest, registers language servers
  Cargo.toml                  # Rust project configuration
  src/
    lib.rs                    # Extension logic: reads settings, launches server
  keymaps/
    intellij-refactor.json    # IntelliJ-style refactoring keybindings
  rust-toolchain.toml         # Rust toolchain configuration (Windows GNU)
  README.md                   # This file (English)
  README_CN.md                # Chinese documentation
```

This extension does **not** contain IntelliJ Server binaries. Users must obtain them from the VS Code extension independently.

---

## Acknowledgements

- [JetBrains](https://www.jetbrains.com/) — IntelliJ Server
- [Zed Industries](https://zed.dev/) — Zed editor and extension API

## Disclaimer

This is an unofficial community project and is not affiliated with, endorsed by, or sponsored by JetBrains s.r.o. IntelliJ is a registered trademark of JetBrains s.r.o. This extension does not contain or distribute any JetBrains proprietary software or binaries. Users must obtain IntelliJ Server from JetBrains' official channels (VS Code Marketplace) and comply with its license agreement (30-day preview period; licensing will be introduced with the 1.0 release).
