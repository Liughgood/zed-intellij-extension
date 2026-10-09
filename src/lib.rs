use zed_extension_api::{self as zed, LanguageServerId, Result};
use zed::settings::LspSettings;

const DEFAULT_SYSTEM_PATH: &str = "intellij-server";

struct IntelliJServerExtension;

impl IntelliJServerExtension {
    fn server_binary_path(&self, language_server_id: &LanguageServerId, worktree: &zed::Worktree) -> Result<String> {
        if let Some(binary) = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|s| s.binary)
        {
            if let Some(path) = binary.path {
                return Ok(path);
            }
        }
        Err("intellij-server binary path not configured. Please set `lsp.intellij-server-java.binary.path` in your Zed settings to point to intellij-server.exe")?
    }

    fn system_path_args(&self, language_server_id: &LanguageServerId, worktree: &zed::Worktree) -> Vec<String> {
        let mut args = vec!["--stdio".to_string(), "--system-path".to_string()];

        if let Some(settings) = LspSettings::for_worktree(language_server_id.as_ref(), worktree).ok() {
            if let Some(opts) = &settings.initialization_options {
                if let Some(path) = opts.get("systemPath").and_then(|v| v.as_str()) {
                    args.push(path.to_string());
                    return args;
                }
            }
        }

        args.push(DEFAULT_SYSTEM_PATH.to_string());
        args
    }
}

impl zed::Extension for IntelliJServerExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            "intellij-server-java" | "intellij-server-kotlin" => {
                let binary_path = self.server_binary_path(language_server_id, worktree)?;
                let args = self.system_path_args(language_server_id, worktree);
                Ok(zed::Command {
                    command: binary_path,
                    args,
                    env: Default::default(),
                })
            }
            _ => Err(format!(
                "Unrecognized language server: {language_server_id}"
            ))?,
        }
    }
}

zed::register_extension!(IntelliJServerExtension);
