use zed_extension_api::{self as zed, LanguageServerId, Result};

const SERVER_PATH: &str = "D:/code/vscode-jetbrain/jetbrains.intellij-server-0.0.13-win32-x64/extension/server/bin/intellij-server.exe";
const SYSTEM_PATH: &str = "C:/Users/admin/.intellij-server/system";

struct IntelliJServerExtension;

impl zed::Extension for IntelliJServerExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            "intellij-server-java" | "intellij-server-kotlin" => Ok(zed::Command {
                command: SERVER_PATH.to_string(),
                args: vec![
                    "--stdio".to_string(),
                    "--system-path".to_string(),
                    SYSTEM_PATH.to_string(),
                ],
                env: Default::default(),
            }),
            _ => Err(format!(
                "Unrecognized language server: {language_server_id}"
            ))?,
        }
    }
}

zed::register_extension!(IntelliJServerExtension);
